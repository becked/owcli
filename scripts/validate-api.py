#!/usr/bin/env python3
"""
Validate Old World API responses against OpenAPI specification.
Generates a report documenting all schema mismatches.

Usage: python3 scripts/validate-api.py [--host HOST] [--port PORT]

Requires: pip install requests pyyaml
"""

import argparse
import json
import sys
from datetime import datetime
from pathlib import Path
from typing import Any

import requests
import yaml


BASE_URL = "http://localhost:9877"

# Static endpoints that don't require dynamic IDs
# NOTE: The API still uses old paths (team-diplomacy, etc.) even though
# the spec defines new paths (diplomacy/teams, etc.)
STATIC_ENDPOINTS = [
    # (path, schema_name, is_array)
    ("/state", "GameState", False),
    ("/config", "GameConfig", False),
    ("/players", "Player", True),
    ("/cities", "City", True),
    ("/characters", "Character", True),
    ("/units", "Unit", True),
    ("/tribes", "Tribe", True),
    ("/religions", "Religion", True),
    ("/map", "MapMetadata", False),
    ("/tiles?limit=10", "PaginatedTiles", False),
    # Old paths that the API actually uses (spec says different paths)
    ("/team-diplomacy", "TeamDiplomacy", True),
    ("/team-alliances", "TeamAlliance", True),
    ("/tribe-diplomacy", "TribeDiplomacy", True),
    ("/tribe-alliances", "TribeAlliance", True),
    ("/character-events", "CharacterEvent", True),
    ("/unit-events", "UnitEvent", True),
    ("/city-events", "CityEvent", True),
]

# Dynamic endpoints - IDs will be discovered from list endpoints
# (path_template, schema_name, id_source, id_field)
DYNAMIC_ENDPOINTS = [
    ("/player/{id}", "Player", "/players", "index"),
    ("/player/{id}/units", "Unit", "/players", "index", True),
    ("/player/{id}/techs", "PlayerTechs", "/players", "index"),
    ("/player/{id}/families", "PlayerFamilies", "/players", "index"),
    ("/player/{id}/religion", "PlayerReligion", "/players", "index"),
    ("/player/{id}/goals", "PlayerGoals", "/players", "index"),
    ("/player/{id}/decisions", "PlayerDecisions", "/players", "index"),
    ("/player/{id}/laws", "PlayerLaws", "/players", "index"),
    ("/player/{id}/missions", "PlayerMissions", "/players", "index"),
    ("/player/{id}/resources", "PlayerResources", "/players", "index"),
    ("/city/{id}", "City", "/cities", "id"),
    ("/character/{id}", "Character", "/characters", "id"),
    ("/unit/{id}", "Unit", "/units", "id"),
    ("/tile/{id}", "Tile", "/tiles?limit=10", "id"),
    ("/tribe/{id}", "Tribe", "/tribes", "tribeType"),
]


class SchemaValidator:
    def __init__(self, spec: dict):
        self.schemas = spec.get("components", {}).get("schemas", {})
        self.issues: list[dict] = []

    def get_schema(self, name: str) -> dict | None:
        return self.schemas.get(name)

    def resolve_ref(self, ref: str) -> dict | None:
        if ref.startswith("#/components/schemas/"):
            schema_name = ref.split("/")[-1]
            return self.get_schema(schema_name)
        return None

    def get_type_description(self, schema: dict) -> str:
        if "$ref" in schema:
            return schema["$ref"].split("/")[-1]
        if "enum" in schema:
            return f"enum[{len(schema['enum'])} values]"
        if schema.get("type") == "array":
            items = schema.get("items", {})
            item_type = self.get_type_description(items)
            return f"array<{item_type}>"
        if schema.get("type") == "object":
            if "additionalProperties" in schema:
                val_type = self.get_type_description(schema["additionalProperties"])
                return f"object<string, {val_type}>"
            return "object"
        return schema.get("type", "unknown")

    def infer_actual_type(self, value: Any) -> str:
        if value is None:
            return "null"
        if isinstance(value, bool):
            return "boolean"
        if isinstance(value, int):
            return "integer"
        if isinstance(value, float):
            return "number"
        if isinstance(value, str):
            return "string"
        if isinstance(value, list):
            if len(value) > 0:
                return f"array<{self.infer_actual_type(value[0])}>"
            return "array<empty>"
        if isinstance(value, dict):
            return "object"
        return "unknown"

    def validate(
        self,
        data: Any,
        schema: dict,
        path: str = "",
        schema_name: str = "",
        check_missing: bool = True,
    ) -> None:
        if "$ref" in schema:
            resolved = self.resolve_ref(schema["$ref"])
            if resolved:
                ref_name = schema["$ref"].split("/")[-1]
                self.validate(data, resolved, path, ref_name, check_missing)
            return

        expected_type = schema.get("type")
        nullable = schema.get("nullable", False)

        if data is None:
            if not nullable:
                self.issues.append({
                    "schema": schema_name,
                    "path": path or "(root)",
                    "issue": "unexpected_null",
                    "expected": expected_type,
                    "actual": "null",
                    "value": None,
                })
            return

        if expected_type == "object":
            if not isinstance(data, dict):
                self.issues.append({
                    "schema": schema_name,
                    "path": path or "(root)",
                    "issue": "type_mismatch",
                    "expected": "object",
                    "actual": self.infer_actual_type(data),
                    "value": self._truncate(data),
                })
                return

            spec_props = schema.get("properties", {})
            additional_props = schema.get("additionalProperties")
            required_fields = set(schema.get("required", []))

            if additional_props:
                # Map/dict type - validate values against additionalProperties schema
                for key, val in data.items():
                    self.validate(val, additional_props, f"{path}.{key}", schema_name, check_missing)
            else:
                # Regular object - validate against properties
                for key, val in data.items():
                    if key in spec_props:
                        self.validate(val, spec_props[key], f"{path}.{key}", schema_name, check_missing)
                    else:
                        self.issues.append({
                            "schema": schema_name,
                            "path": f"{path}.{key}",
                            "issue": "extra_field",
                            "expected": None,
                            "actual": self.infer_actual_type(val),
                            "value": self._truncate(val),
                        })

                # Check for missing required fields
                if check_missing:
                    for field in required_fields:
                        if field not in data:
                            field_schema = spec_props.get(field, {})
                            self.issues.append({
                                "schema": schema_name,
                                "path": f"{path}.{field}",
                                "issue": "missing_required",
                                "expected": self.get_type_description(field_schema),
                                "actual": "missing",
                                "value": None,
                            })

                    # Also report optional fields that are consistently missing (for info)
                    for field, field_schema in spec_props.items():
                        if field not in data and field not in required_fields:
                            # Only report if not nullable
                            if not field_schema.get("nullable", False):
                                self.issues.append({
                                    "schema": schema_name,
                                    "path": f"{path}.{field}",
                                    "issue": "missing_optional",
                                    "expected": self.get_type_description(field_schema),
                                    "actual": "missing",
                                    "value": None,
                                })

        elif expected_type == "array":
            if not isinstance(data, list):
                self.issues.append({
                    "schema": schema_name,
                    "path": path or "(root)",
                    "issue": "type_mismatch",
                    "expected": "array",
                    "actual": self.infer_actual_type(data),
                    "value": self._truncate(data),
                })
                return

            items_schema = schema.get("items", {})
            # Validate up to 10 items for better coverage
            for i, item in enumerate(data[:10]):
                self.validate(item, items_schema, f"{path}[{i}]", schema_name, check_missing)

        elif expected_type == "string":
            if not isinstance(data, str):
                self.issues.append({
                    "schema": schema_name,
                    "path": path or "(root)",
                    "issue": "type_mismatch",
                    "expected": "string",
                    "actual": self.infer_actual_type(data),
                    "value": self._truncate(data),
                })
            elif "enum" in schema and data not in schema["enum"]:
                self.issues.append({
                    "schema": schema_name,
                    "path": path or "(root)",
                    "issue": "enum_mismatch",
                    "expected": f"one of {len(schema['enum'])} enum values",
                    "actual": f'"{data}"',
                    "value": data,
                })

        elif expected_type == "integer":
            if not isinstance(data, int) or isinstance(data, bool):
                self.issues.append({
                    "schema": schema_name,
                    "path": path or "(root)",
                    "issue": "type_mismatch",
                    "expected": "integer",
                    "actual": self.infer_actual_type(data),
                    "value": self._truncate(data),
                })

        elif expected_type == "number":
            if not isinstance(data, (int, float)) or isinstance(data, bool):
                self.issues.append({
                    "schema": schema_name,
                    "path": path or "(root)",
                    "issue": "type_mismatch",
                    "expected": "number",
                    "actual": self.infer_actual_type(data),
                    "value": self._truncate(data),
                })

        elif expected_type == "boolean":
            if not isinstance(data, bool):
                self.issues.append({
                    "schema": schema_name,
                    "path": path or "(root)",
                    "issue": "type_mismatch",
                    "expected": "boolean",
                    "actual": self.infer_actual_type(data),
                    "value": self._truncate(data),
                })

    def _truncate(self, value: Any, max_len: int = 50) -> str:
        s = str(value)
        if len(s) > max_len:
            return s[:max_len] + "..."
        return s


def fetch_endpoint(base_url: str, path: str) -> tuple[dict | list | None, str | None]:
    try:
        url = f"{base_url}{path}"
        resp = requests.get(url, timeout=10)
        if resp.status_code == 200:
            return resp.json(), None
        elif resp.status_code == 404:
            return None, "404 Not Found"
        elif resp.status_code == 503:
            return None, "503 Game Not Available"
        else:
            return None, f"{resp.status_code} {resp.text[:100]}"
    except requests.exceptions.ConnectionError:
        return None, "Connection refused"
    except Exception as e:
        return None, str(e)


def discover_ids(base_url: str) -> dict[str, list[Any]]:
    """Fetch list endpoints and extract IDs for dynamic endpoint testing."""
    ids: dict[str, list[Any]] = {}

    sources = [
        ("/players", "playerInt"),  # API uses playerInt, not index
        ("/cities", "id"),
        ("/characters", "id"),
        ("/units", "id"),
        ("/tiles?limit=10", "id"),
        ("/tribes", "tribeType"),
    ]

    for path, id_field in sources:
        data, error = fetch_endpoint(base_url, path)
        if error or data is None:
            continue

        # Handle paginated responses
        if isinstance(data, dict) and "tiles" in data:
            data = data["tiles"]

        if isinstance(data, list) and len(data) > 0:
            extracted = []
            for item in data[:3]:  # Get up to 3 IDs
                if isinstance(item, dict) and id_field in item:
                    extracted.append(item[id_field])
            if extracted:
                ids[path] = extracted

    return ids


def generate_report(
    results: list[dict],
    all_issues: list[dict],
    output_path: Path,
    json_output_path: Path,
) -> None:
    now = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    success_count = sum(1 for r in results if r["status"] == "ok")
    error_count = sum(1 for r in results if r["status"] == "error")

    # Count issues by type
    type_mismatches = [i for i in all_issues if i["issue"] == "type_mismatch"]
    extra_fields = [i for i in all_issues if i["issue"] == "extra_field"]
    missing_required = [i for i in all_issues if i["issue"] == "missing_required"]
    missing_optional = [i for i in all_issues if i["issue"] == "missing_optional"]
    enum_mismatches = [i for i in all_issues if i["issue"] == "enum_mismatch"]
    unexpected_nulls = [i for i in all_issues if i["issue"] == "unexpected_null"]

    # Group issues by schema and deduplicate
    issues_by_schema: dict[str, dict[str, dict]] = {}
    for issue in all_issues:
        schema = issue["schema"]
        if schema not in issues_by_schema:
            issues_by_schema[schema] = {}

        # Use path + issue type as dedup key
        key = (issue["path"], issue["issue"], issue["expected"], issue["actual"])
        if key not in issues_by_schema[schema]:
            issues_by_schema[schema][key] = issue

    # Count unique issues per schema
    unique_issues_count = sum(len(v) for v in issues_by_schema.values())

    lines = [
        "# API Spec Validation Report",
        "",
        f"Generated: {now}",
        "",
        "## Summary",
        "",
        f"- **Endpoints tested:** {len(results)}",
        f"- **Successful:** {success_count}",
        f"- **Failed to fetch:** {error_count}",
        f"- **Unique schema issues:** {unique_issues_count}",
        "",
        "### Issues by Type",
        "",
        f"| Type | Count |",
        f"|------|-------|",
        f"| Type mismatch | {len(set((i['schema'], i['path']) for i in type_mismatches))} |",
        f"| Extra field (in response, not in spec) | {len(set((i['schema'], i['path']) for i in extra_fields))} |",
        f"| Missing required field | {len(set((i['schema'], i['path']) for i in missing_required))} |",
        f"| Missing optional field | {len(set((i['schema'], i['path']) for i in missing_optional))} |",
        f"| Enum value mismatch | {len(set((i['schema'], i['path']) for i in enum_mismatches))} |",
        f"| Unexpected null | {len(set((i['schema'], i['path']) for i in unexpected_nulls))} |",
        "",
    ]

    if error_count > 0:
        lines.append("## Connection Errors")
        lines.append("")
        for r in results:
            if r["status"] == "error":
                lines.append(f"- `{r['path']}`: {r['error']}")
        lines.append("")

    # Type mismatches are the most important - these break deserialization
    if type_mismatches:
        lines.append("## Type Mismatches (Critical)")
        lines.append("")
        lines.append("These fields have incorrect types in the spec and will cause deserialization failures.")
        lines.append("")

        for schema_name in sorted(issues_by_schema.keys()):
            schema_type_issues = [
                v for k, v in issues_by_schema[schema_name].items()
                if v["issue"] == "type_mismatch"
            ]
            if not schema_type_issues:
                continue

            lines.append(f"### {schema_name}")
            lines.append("")
            lines.append("| Field | Spec Type | Actual Type | Example Value |")
            lines.append("|-------|-----------|-------------|---------------|")

            for issue in sorted(schema_type_issues, key=lambda x: x["path"]):
                path = issue["path"].lstrip(".")
                # Remove array indices for cleaner display
                import re
                path = re.sub(r'\[\d+\]', '', path).lstrip(".")
                expected = issue["expected"] or "-"
                actual = issue["actual"]
                value = str(issue["value"]).replace("|", "\\|")[:40]
                lines.append(f"| `{path}` | {expected} | {actual} | `{value}` |")

            lines.append("")

    # Extra fields - informational, spec may need updating
    if extra_fields:
        lines.append("## Extra Fields (Spec Missing)")
        lines.append("")
        lines.append("These fields appear in API responses but are not defined in the spec.")
        lines.append("")

        for schema_name in sorted(issues_by_schema.keys()):
            schema_extra = [
                v for k, v in issues_by_schema[schema_name].items()
                if v["issue"] == "extra_field"
            ]
            if not schema_extra:
                continue

            lines.append(f"### {schema_name}")
            lines.append("")
            lines.append("| Field | Actual Type | Example Value |")
            lines.append("|-------|-------------|---------------|")

            seen_paths = set()
            for issue in sorted(schema_extra, key=lambda x: x["path"]):
                path = issue["path"].lstrip(".")
                import re
                path = re.sub(r'\[\d+\]', '', path).lstrip(".")
                if path in seen_paths:
                    continue
                seen_paths.add(path)
                actual = issue["actual"]
                value = str(issue["value"]).replace("|", "\\|")[:40]
                lines.append(f"| `{path}` | {actual} | `{value}` |")

            lines.append("")

    # Missing required fields
    if missing_required:
        lines.append("## Missing Required Fields")
        lines.append("")
        lines.append("These fields are marked required in spec but missing from responses.")
        lines.append("")

        for schema_name in sorted(issues_by_schema.keys()):
            schema_missing = [
                v for k, v in issues_by_schema[schema_name].items()
                if v["issue"] == "missing_required"
            ]
            if not schema_missing:
                continue

            lines.append(f"### {schema_name}")
            lines.append("")

            seen_paths = set()
            for issue in sorted(schema_missing, key=lambda x: x["path"]):
                path = issue["path"].lstrip(".")
                import re
                path = re.sub(r'\[\d+\]', '', path).lstrip(".")
                if path in seen_paths:
                    continue
                seen_paths.add(path)
                lines.append(f"- `{path}` (expected: {issue['expected']})")

            lines.append("")

    lines.append("## Endpoints Tested")
    lines.append("")
    lines.append("| Endpoint | Schema | Status |")
    lines.append("|----------|--------|--------|")
    for r in results:
        status = r["status"]
        issues = r.get("issues", 0)
        if status == "error":
            status_display = f"error: {r['error'][:30]}"
        elif issues > 0:
            status_display = f"{issues} issues"
        else:
            status_display = "ok"
        lines.append(f"| `{r['path']}` | {r['schema']} | {status_display} |")
    lines.append("")

    # Write markdown report
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text("\n".join(lines))
    print(f"Markdown report: {output_path}")

    # Write JSON report
    json_report = {
        "generated": now,
        "summary": {
            "endpoints_tested": len(results),
            "successful": success_count,
            "failed": error_count,
            "unique_issues": unique_issues_count,
        },
        "issues_by_type": {
            "type_mismatch": len(set((i['schema'], i['path']) for i in type_mismatches)),
            "extra_field": len(set((i['schema'], i['path']) for i in extra_fields)),
            "missing_required": len(set((i['schema'], i['path']) for i in missing_required)),
            "missing_optional": len(set((i['schema'], i['path']) for i in missing_optional)),
            "enum_mismatch": len(set((i['schema'], i['path']) for i in enum_mismatches)),
            "unexpected_null": len(set((i['schema'], i['path']) for i in unexpected_nulls)),
        },
        "results": results,
        "issues": all_issues,
    }
    json_output_path.write_text(json.dumps(json_report, indent=2))
    print(f"JSON report: {json_output_path}")


def main():
    parser = argparse.ArgumentParser(description="Validate API against OpenAPI spec")
    parser.add_argument("--host", default="localhost", help="API host")
    parser.add_argument("--port", type=int, default=9877, help="API port")
    args = parser.parse_args()

    base_url = f"http://{args.host}:{args.port}"

    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    spec_path = project_root / "openapi.yaml"
    output_path = project_root / "docs" / "api-validation-report.md"
    json_output_path = project_root / "docs" / "api-validation-report.json"

    print(f"Loading spec from: {spec_path}")
    with open(spec_path) as f:
        spec = yaml.safe_load(f)

    validator = SchemaValidator(spec)

    print(f"Testing against: {base_url}")
    print()

    # First, discover IDs from list endpoints
    print("Discovering IDs from list endpoints...")
    discovered_ids = discover_ids(base_url)
    for source, ids in discovered_ids.items():
        print(f"  {source}: {ids[:3]}")
    print()

    results = []

    # Test static endpoints
    print(f"Testing {len(STATIC_ENDPOINTS)} static endpoints...")
    for path, schema_name, is_array in STATIC_ENDPOINTS:
        print(f"  {path} ... ", end="", flush=True)

        data, error = fetch_endpoint(base_url, path)

        if error:
            print(f"ERROR: {error}")
            results.append({
                "path": path,
                "schema": schema_name,
                "status": "error",
                "error": error,
            })
            continue

        schema = validator.get_schema(schema_name)
        if not schema:
            print(f"SKIP (schema not found: {schema_name})")
            results.append({
                "path": path,
                "schema": schema_name,
                "status": "error",
                "error": f"Schema not found: {schema_name}",
            })
            continue

        issues_before = len(validator.issues)

        if is_array and isinstance(data, list):
            for i, item in enumerate(data[:5]):
                validator.validate(item, schema, f"[{i}]", schema_name)
        else:
            validator.validate(data, schema, "", schema_name)

        new_issues = len(validator.issues) - issues_before

        if new_issues > 0:
            print(f"ISSUES: {new_issues}")
        else:
            print("OK")

        results.append({
            "path": path,
            "schema": schema_name,
            "status": "ok",
            "issues": new_issues,
        })

    # Test dynamic endpoints with discovered IDs
    print()
    print("Testing dynamic endpoints with discovered IDs...")
    for endpoint_def in DYNAMIC_ENDPOINTS:
        if len(endpoint_def) == 5:
            path_template, schema_name, id_source, id_field, is_array = endpoint_def
        else:
            path_template, schema_name, id_source, id_field = endpoint_def
            is_array = False

        # Get IDs for this endpoint
        ids = discovered_ids.get(id_source, [])
        if not ids:
            print(f"  {path_template} ... SKIP (no IDs from {id_source})")
            results.append({
                "path": path_template,
                "schema": schema_name,
                "status": "error",
                "error": f"No IDs discovered from {id_source}",
            })
            continue

        # Test with first discovered ID
        test_id = ids[0]
        path = path_template.replace("{id}", str(test_id))
        print(f"  {path} ... ", end="", flush=True)

        data, error = fetch_endpoint(base_url, path)

        if error:
            print(f"ERROR: {error}")
            results.append({
                "path": path,
                "schema": schema_name,
                "status": "error",
                "error": error,
            })
            continue

        schema = validator.get_schema(schema_name)
        if not schema:
            print(f"SKIP (schema not found: {schema_name})")
            results.append({
                "path": path,
                "schema": schema_name,
                "status": "error",
                "error": f"Schema not found: {schema_name}",
            })
            continue

        issues_before = len(validator.issues)

        if is_array and isinstance(data, list):
            for i, item in enumerate(data[:5]):
                validator.validate(item, schema, f"[{i}]", schema_name)
        else:
            validator.validate(data, schema, "", schema_name)

        new_issues = len(validator.issues) - issues_before

        if new_issues > 0:
            print(f"ISSUES: {new_issues}")
        else:
            print("OK")

        results.append({
            "path": path,
            "schema": schema_name,
            "status": "ok",
            "issues": new_issues,
        })

    all_issues = validator.issues

    print()
    print(f"Total issues found: {len(all_issues)}")

    generate_report(results, all_issues, output_path, json_output_path)


if __name__ == "__main__":
    main()
