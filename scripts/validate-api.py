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

ENDPOINTS = [
    # (path, schema_name, is_array)
    ("/state", "GameState", False),
    ("/config", "GameConfig", False),
    ("/players", "Player", True),
    ("/player/0", "Player", False),
    ("/player/0/units", "Unit", True),
    ("/player/0/techs", "PlayerTechs", False),
    ("/player/0/families", "PlayerFamilies", False),
    ("/player/0/religion", "PlayerReligion", False),
    ("/player/0/goals", "PlayerGoals", False),
    ("/player/0/decisions", "PlayerDecisions", False),
    ("/player/0/laws", "PlayerLaws", False),
    ("/player/0/missions", "PlayerMissions", False),
    ("/player/0/resources", "PlayerResources", False),
    ("/cities", "City", True),
    ("/city/0", "City", False),
    ("/characters", "Character", True),
    ("/character/0", "Character", False),
    ("/units", "Unit", True),
    ("/unit/0", "Unit", False),
    ("/tribes", "Tribe", True),
    ("/religions", "Religion", True),
    ("/map", "MapMetadata", False),
    ("/tiles?limit=10", "TilesPaginated", False),
    ("/tile/0", "Tile", False),
    ("/team-diplomacy", "TeamDiplomacy", True),
    ("/team-alliances", "TeamAlliance", True),
    ("/tribe-diplomacy", "TribeDiplomacy", True),
    ("/tribe-alliances", "TribeAlliance", True),
    ("/character-events", "CharacterEvent", True),
    ("/unit-events", "UnitEvent", True),
    ("/city-events", "CityEvent", True),
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
            return f"enum{schema['enum']}"
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
            return "array<unknown>"
        if isinstance(value, dict):
            return "object"
        return "unknown"

    def validate(
        self,
        data: Any,
        schema: dict,
        path: str = "",
        schema_name: str = "",
    ) -> None:
        if "$ref" in schema:
            resolved = self.resolve_ref(schema["$ref"])
            if resolved:
                ref_name = schema["$ref"].split("/")[-1]
                self.validate(data, resolved, path, ref_name)
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

            if additional_props:
                for key, val in data.items():
                    self.validate(val, additional_props, f"{path}.{key}", schema_name)
            else:
                for key, val in data.items():
                    if key in spec_props:
                        self.validate(val, spec_props[key], f"{path}.{key}", schema_name)
                    else:
                        self.issues.append({
                            "schema": schema_name,
                            "path": f"{path}.{key}",
                            "issue": "extra_field",
                            "expected": None,
                            "actual": self.infer_actual_type(val),
                            "value": self._truncate(val),
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
            for i, item in enumerate(data[:5]):
                self.validate(item, items_schema, f"{path}[{i}]", schema_name)

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
                    "expected": f"enum{schema['enum']}",
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
            return None, f"404 Not Found"
        elif resp.status_code == 503:
            return None, "503 Game Not Available"
        else:
            return None, f"{resp.status_code} {resp.text[:100]}"
    except requests.exceptions.ConnectionError:
        return None, "Connection refused"
    except Exception as e:
        return None, str(e)


def generate_report(
    results: list[dict],
    all_issues: list[dict],
    output_path: Path,
) -> None:
    now = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    success_count = sum(1 for r in results if r["status"] == "ok")
    error_count = sum(1 for r in results if r["status"] == "error")
    issues_count = sum(1 for r in results if r["status"] == "ok" and r["issues"] > 0)

    issues_by_schema: dict[str, list[dict]] = {}
    for issue in all_issues:
        schema = issue["schema"]
        if schema not in issues_by_schema:
            issues_by_schema[schema] = []
        issues_by_schema[schema].append(issue)

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
        f"- **With schema issues:** {issues_count}",
        f"- **Total issues found:** {len(all_issues)}",
        "",
    ]

    if error_count > 0:
        lines.append("## Connection Errors")
        lines.append("")
        for r in results:
            if r["status"] == "error":
                lines.append(f"- `{r['path']}`: {r['error']}")
        lines.append("")

    if issues_by_schema:
        lines.append("## Schema Discrepancies")
        lines.append("")

        for schema_name in sorted(issues_by_schema.keys()):
            issues = issues_by_schema[schema_name]
            lines.append(f"### {schema_name}")
            lines.append("")
            lines.append("| Path | Issue | Expected | Actual | Example |")
            lines.append("|------|-------|----------|--------|---------|")

            seen = set()
            for issue in issues:
                key = (issue["path"], issue["issue"], issue["expected"], issue["actual"])
                if key in seen:
                    continue
                seen.add(key)

                path = issue["path"].lstrip(".")
                issue_type = issue["issue"].replace("_", " ")
                expected = issue["expected"] or "-"
                actual = issue["actual"]
                value = str(issue["value"]).replace("|", "\\|")[:30]

                lines.append(f"| `{path}` | {issue_type} | {expected} | {actual} | `{value}` |")

            lines.append("")
    else:
        lines.append("## No Schema Discrepancies Found")
        lines.append("")

    lines.append("## Endpoints Tested")
    lines.append("")
    lines.append("| Endpoint | Schema | Status | Issues |")
    lines.append("|----------|--------|--------|--------|")
    for r in results:
        status = r["status"]
        issues = r.get("issues", 0)
        if status == "error":
            status_display = f"error: {r['error'][:20]}"
        elif issues > 0:
            status_display = f"issues: {issues}"
        else:
            status_display = "ok"
        lines.append(f"| `{r['path']}` | {r['schema']} | {status_display} | {issues} |")
    lines.append("")

    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text("\n".join(lines))
    print(f"Report written to: {output_path}")


def main():
    parser = argparse.ArgumentParser(description="Validate API against OpenAPI spec")
    parser.add_argument("--host", default="localhost", help="API host")
    parser.add_argument("--port", type=int, default=9877, help="API port")
    args = parser.parse_args()

    base_url = f"http://{args.host}:{args.port}"

    script_dir = Path(__file__).parent
    project_root = script_dir.parent
    spec_path = project_root / "openapi.yaml"
    output_path = project_root / "docs" / "api-spec-validation.md"

    print(f"Loading spec from: {spec_path}")
    with open(spec_path) as f:
        spec = yaml.safe_load(f)

    validator = SchemaValidator(spec)

    print(f"Testing against: {base_url}")
    print(f"Endpoints to test: {len(ENDPOINTS)}")
    print()

    results = []
    all_issues = []

    for path, schema_name, is_array in ENDPOINTS:
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
            for i, item in enumerate(data[:3]):
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

    generate_report(results, all_issues, output_path)


if __name__ == "__main__":
    main()
