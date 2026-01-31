//! Code generator for type formatters from OpenAPI spec.
//!
//! Usage:
//!   cargo run --bin gen_formatters > src/output/table_generated.rs
//!
//! This parses openapi.yaml and generates format_*() functions for all entity types,
//! displaying ALL fields.

use serde_yaml::Value;
use std::collections::HashSet;
use std::fs;

fn main() {
    let spec_path = "openapi.yaml";
    let content = fs::read_to_string(spec_path).expect("Failed to read openapi.yaml");
    let spec: Value = serde_yaml::from_str(&content).expect("Failed to parse YAML");

    let schemas = &spec["components"]["schemas"];

    // Schemas to generate formatters for (entity types, not command params)
    let include_schemas: HashSet<&str> = [
        "Player",
        "Unit",
        "City",
        "Character",
        "Tile",
        "Tribe",
        "Religion",
        "GameConfig",
        "GameState",
        "MapMetadata",
        "TeamDiplomacy",
        "TeamAlliance",
        "TribeDiplomacy",
        "TribeAlliance",
        "CharacterEvent",
        "CityEvent",
        "UnitEvent",
        "WonderEvent",
        "PlayerTechs",
        "PlayerFamilies",
        "PlayerReligion",
        "PlayerGoals",
        "PlayerDecisions",
        "PlayerLaws",
        "PlayerMissions",
        "PlayerResources",
    ]
    .into_iter()
    .collect();

    // Fields that are enums (not strings) and need {:?} formatting
    let enum_fields: HashSet<&str> = [
        "eventType",
        "buildType",
    ]
    .into_iter()
    .collect();

    // Parse schemas
    let mut schema_defs: Vec<SchemaDef> = Vec::new();

    if let Value::Mapping(map) = schemas {
        for (key, value) in map {
            if let Value::String(name) = key {
                if include_schemas.contains(name.as_str()) {
                    let fields = parse_schema_fields(value, &enum_fields);
                    schema_defs.push(SchemaDef {
                        name: name.clone(),
                        fields,
                    });
                }
            }
        }
    }

    // Sort for consistent output
    schema_defs.sort_by(|a, b| a.name.cmp(&b.name));

    // Generate output
    generate_preamble();
    for schema in &schema_defs {
        if !skip_single_formatter(&schema.name) {
            generate_single_formatter(schema);
            println!();
        }
        // Generate list formatter for collection types
        if should_have_list_formatter(&schema.name) {
            generate_list_formatter(schema);
            println!();
        }
    }
}

#[derive(Debug)]
struct SchemaDef {
    name: String,
    fields: Vec<FieldDef>,
}

#[derive(Debug)]
struct FieldDef {
    name: String,       // Original JSON name: "capitalCityID"
    rust_name: String,  // Snake case: "capital_city_id"
    field_type: FieldType,
    is_enum: bool,
}

#[derive(Debug, Clone)]
enum FieldType {
    Integer,
    Float,
    String,
    Boolean,
    Array(Box<FieldType>),
    HashMap(Box<FieldType>),  // HashMap<String, T> - simple value type
    JsonMap,                  // serde_json::Map<String, Value> - complex/untyped
    Object(String),
}

fn parse_schema_fields(schema: &Value, enum_fields: &HashSet<&str>) -> Vec<FieldDef> {
    let mut fields = Vec::new();

    if let Value::Mapping(props) = &schema["properties"] {
        for (key, value) in props {
            if let Value::String(name) = key {
                let field_type = parse_field_type(value);
                let is_enum = enum_fields.contains(name.as_str());

                fields.push(FieldDef {
                    name: name.clone(),
                    rust_name: escape_rust_keyword(&to_snake_case(name)),
                    field_type,
                    is_enum,
                });
            }
        }
    }

    // Sort fields alphabetically
    fields.sort_by(|a, b| a.name.cmp(&b.name));
    fields
}

fn parse_field_type(schema: &Value) -> FieldType {
    if let Some(ref_path) = schema["$ref"].as_str() {
        let type_name = ref_path.split('/').last().unwrap_or("Unknown");
        return FieldType::Object(type_name.to_string());
    }

    match schema["type"].as_str() {
        Some("integer") => FieldType::Integer,
        Some("number") => FieldType::Float,
        Some("string") => FieldType::String,
        Some("boolean") => FieldType::Boolean,
        Some("array") => {
            let items = &schema["items"];
            FieldType::Array(Box::new(parse_field_type(items)))
        }
        Some("object") => {
            if let Some(additional) = schema.get("additionalProperties") {
                // Check if additionalProperties has a simple type
                match additional["type"].as_str() {
                    Some("integer") | Some("number") | Some("string") | Some("boolean") => {
                        FieldType::HashMap(Box::new(parse_field_type(additional)))
                    }
                    _ => {
                        // Complex or untyped additionalProperties -> serde_json::Map
                        FieldType::JsonMap
                    }
                }
            } else {
                // Plain object without additionalProperties -> serde_json::Map
                FieldType::JsonMap
            }
        }
        _ => FieldType::String,
    }
}

fn should_have_list_formatter(name: &str) -> bool {
    matches!(
        name,
        "Player"
            | "Unit"
            | "City"
            | "Character"
            | "Tile"
            | "Tribe"
            | "Religion"
            | "TeamDiplomacy"
            | "TeamAlliance"
            | "TribeDiplomacy"
            | "TribeAlliance"
            | "CharacterEvent"
            | "CityEvent"
            | "UnitEvent"
            | "WonderEvent"
    )
}

/// Types that only have list formatters (mod.rs passes Vec, not single item)
fn skip_single_formatter(name: &str) -> bool {
    matches!(
        name,
        "TeamDiplomacy"
            | "TribeDiplomacy"
            | "TeamAlliance"
            | "TribeAlliance"
            | "CharacterEvent"
            | "CityEvent"
            | "UnitEvent"
            | "WonderEvent"
    )
}

fn generate_preamble() {
    println!("// This file is auto-generated by `cargo run --bin gen_formatters`");
    println!("// Do not edit manually. Regenerate with:");
    println!("//   cargo run --bin gen_formatters > src/output/table_generated.rs");
    println!("//");
    println!("// Included via include!() in table.rs");
    println!();
}

fn generate_single_formatter(schema: &SchemaDef) {
    let fn_name = single_formatter_name(&schema.name);
    let type_name = &schema.name;

    println!("/// Format a single {} for display (all fields)", type_name);
    println!(
        "pub fn {}(item: &types::{}) -> String {{",
        fn_name, type_name
    );
    println!("    let mut lines = Vec::new();");
    println!();
    println!("    lines.push(\"{}\".to_string());", type_name);
    println!();

    for field in &schema.fields {
        generate_field_display(field);
    }

    println!();
    println!("    lines.join(\"\\n\")");
    println!("}}");
}

fn generate_field_display(field: &FieldDef) {
    let rust_name = &field.rust_name;
    let display_name = to_display_name(&field.name);

    // Enum fields use debug format
    if field.is_enum {
        println!("    if let Some(v) = &item.{} {{", rust_name);
        println!(
            "        lines.push(format!(\"  {}: {{:?}}\", v));",
            display_name
        );
        println!("    }}");
        return;
    }

    match &field.field_type {
        FieldType::Integer | FieldType::Float => {
            println!("    if let Some(v) = item.{} {{", rust_name);
            println!(
                "        lines.push(format!(\"  {}: {{}}\", v));",
                display_name
            );
            println!("    }}");
        }
        FieldType::String => {
            if is_type_constant_field(&field.name) {
                println!("    if let Some(v) = &item.{} {{", rust_name);
                println!(
                    "        lines.push(format!(\"  {}: {{}}\", shorten_type(v)));",
                    display_name
                );
                println!("    }}");
            } else {
                println!("    if let Some(v) = &item.{} {{", rust_name);
                println!(
                    "        lines.push(format!(\"  {}: {{}}\", v));",
                    display_name
                );
                println!("    }}");
            }
        }
        FieldType::Boolean => {
            println!("    if let Some(v) = item.{} {{", rust_name);
            println!("        if v {{");
            println!("            lines.push(\"  {}: Yes\".to_string());", display_name);
            println!("        }}");
            println!("    }}");
        }
        FieldType::Array(inner) => {
            println!("    if !item.{}.is_empty() {{", rust_name);
            match inner.as_ref() {
                FieldType::String => {
                    if is_type_constant_field(&field.name) {
                        println!(
                            "        let items: Vec<String> = item.{}.iter().map(|s| shorten_type(s)).collect();",
                            rust_name
                        );
                        println!(
                            "        lines.push(format!(\"  {}: {{}}\", items.join(\", \")));",
                            display_name
                        );
                    } else {
                        println!(
                            "        lines.push(format!(\"  {}: {{}}\", item.{}.join(\", \")));",
                            display_name, rust_name
                        );
                    }
                }
                FieldType::Integer => {
                    println!(
                        "        let items: Vec<String> = item.{}.iter().map(|v| v.to_string()).collect();",
                        rust_name
                    );
                    println!(
                        "        lines.push(format!(\"  {}: {{}}\", items.join(\", \")));",
                        display_name
                    );
                }
                _ => {
                    println!(
                        "        lines.push(format!(\"  {}: {{}} items\", item.{}.len()));",
                        display_name, rust_name
                    );
                }
            }
            println!("    }}");
        }
        FieldType::HashMap(inner) => {
            println!("    if !item.{}.is_empty() {{", rust_name);
            println!("        lines.push(\"  {}:\".to_string());", display_name);
            match inner.as_ref() {
                FieldType::Integer | FieldType::Float => {
                    println!("        for (k, v) in &item.{} {{", rust_name);
                    println!("            lines.push(format!(\"    {{}}: {{}}\", shorten_type(k), v));");
                    println!("        }}");
                }
                FieldType::Boolean => {
                    println!("        for (k, v) in &item.{} {{", rust_name);
                    println!("            if *v {{");
                    println!("                lines.push(format!(\"    {{}}\", shorten_type(k)));");
                    println!("            }}");
                    println!("        }}");
                }
                FieldType::String => {
                    println!("        for (k, v) in &item.{} {{", rust_name);
                    println!("            lines.push(format!(\"    {{}}: {{}}\", shorten_type(k), v));");
                    println!("        }}");
                }
                _ => {
                    println!("        for (k, v) in &item.{} {{", rust_name);
                    println!("            lines.push(format!(\"    {{}}: {{:?}}\", shorten_type(k), v));");
                    println!("        }}");
                }
            }
            println!("    }}");
        }
        FieldType::JsonMap => {
            // serde_json::Map<String, Value> - not Option-wrapped
            println!("    if !item.{}.is_empty() {{", rust_name);
            println!("        lines.push(\"  {}:\".to_string());", display_name);
            println!("        for (k, v) in &item.{} {{", rust_name);
            println!("            lines.push(format!(\"    {{}}: {{:?}}\", shorten_type(k), v));");
            println!("        }}");
            println!("    }}");
        }
        FieldType::Object(type_name) => {
            if type_name == "Location" {
                println!("    if let Some(loc) = &item.{} {{", rust_name);
                println!("        lines.push(format!(\"  {}: ({{:?}}, {{:?}})\", loc.x, loc.y));", display_name);
                println!("    }}");
            } else {
                println!("    if item.{}.is_some() {{", rust_name);
                println!(
                    "        lines.push(\"  {}: (nested {})\".to_string());",
                    display_name, type_name
                );
                println!("    }}");
            }
        }
    }
}

fn generate_list_formatter(schema: &SchemaDef) {
    let fn_name = list_formatter_name(&schema.name);
    let type_name = &schema.name;
    let plural = pluralize_name(&schema.name);

    println!("/// Format a list of {} (all fields for each)", plural);
    println!(
        "pub fn {}(items: &[types::{}]) -> String {{",
        fn_name, type_name
    );
    println!("    if items.is_empty() {{");
    println!("        return \"No {} found\".to_string();", plural);
    println!("    }}");
    println!();

    // If single formatter exists, call it; otherwise inline the formatting
    if skip_single_formatter(&schema.name) {
        // Inline formatting for types without single formatter
        println!("    items");
        println!("        .iter()");
        println!("        .map(|item| {{");
        println!("            let mut lines = Vec::new();");
        println!("            lines.push(\"{}\".to_string());", type_name);
        for field in &schema.fields {
            print!("            ");
            generate_field_display_inline(field);
        }
        println!("            lines.join(\"\\n\")");
        println!("        }})");
        println!("        .collect::<Vec<_>>()");
        println!("        .join(\"\\n\\n\")");
    } else {
        let single_fn = single_formatter_name(&schema.name);
        println!("    items");
        println!("        .iter()");
        println!("        .map(|item| {}(item))", single_fn);
        println!("        .collect::<Vec<_>>()");
        println!("        .join(\"\\n\\n\")");
    }
    println!("}}");
}

fn generate_field_display_inline(field: &FieldDef) {
    let rust_name = &field.rust_name;
    let display_name = to_display_name(&field.name);

    if field.is_enum {
        println!("if let Some(v) = &item.{} {{", rust_name);
        println!(
            "                lines.push(format!(\"  {}: {{:?}}\", v));",
            display_name
        );
        println!("            }}");
        return;
    }

    match &field.field_type {
        FieldType::Integer | FieldType::Float => {
            println!("if let Some(v) = item.{} {{", rust_name);
            println!(
                "                lines.push(format!(\"  {}: {{}}\", v));",
                display_name
            );
            println!("            }}");
        }
        FieldType::String => {
            if is_type_constant_field(&field.name) {
                println!("if let Some(v) = &item.{} {{", rust_name);
                println!(
                    "                lines.push(format!(\"  {}: {{}}\", shorten_type(v)));",
                    display_name
                );
                println!("            }}");
            } else {
                println!("if let Some(v) = &item.{} {{", rust_name);
                println!(
                    "                lines.push(format!(\"  {}: {{}}\", v));",
                    display_name
                );
                println!("            }}");
            }
        }
        FieldType::Boolean => {
            println!("if let Some(v) = item.{} {{", rust_name);
            println!("                if v {{");
            println!("                    lines.push(\"  {}: Yes\".to_string());", display_name);
            println!("                }}");
            println!("            }}");
        }
        FieldType::Array(inner) => {
            println!("if !item.{}.is_empty() {{", rust_name);
            match inner.as_ref() {
                FieldType::Integer => {
                    println!(
                        "                let vals: Vec<String> = item.{}.iter().map(|v| v.to_string()).collect();",
                        rust_name
                    );
                    println!(
                        "                lines.push(format!(\"  {}: {{}}\", vals.join(\", \")));",
                        display_name
                    );
                }
                _ => {
                    println!(
                        "                lines.push(format!(\"  {}: {{}} items\", item.{}.len()));",
                        display_name, rust_name
                    );
                }
            }
            println!("            }}");
        }
        FieldType::HashMap(_) | FieldType::JsonMap | FieldType::Object(_) => {
            // Skip complex fields in inline format for simplicity
        }
    }
}

// === Naming helpers ===

fn single_formatter_name(name: &str) -> String {
    match name {
        "GameState" => "format_state".to_string(),
        "GameConfig" => "format_config".to_string(),
        "MapMetadata" => "format_map".to_string(),
        _ => format!("format_{}", to_snake_case(name)),
    }
}

fn list_formatter_name(name: &str) -> String {
    match name {
        "City" => "format_cities".to_string(),
        "TeamDiplomacy" => "format_team_diplomacy".to_string(),
        "TribeDiplomacy" => "format_tribe_diplomacy".to_string(),
        "TeamAlliance" => "format_team_alliances".to_string(),
        "TribeAlliance" => "format_tribe_alliances".to_string(),
        "CharacterEvent" => "format_character_events".to_string(),
        "CityEvent" => "format_city_events".to_string(),
        "UnitEvent" => "format_unit_events".to_string(),
        "WonderEvent" => "format_wonder_events".to_string(),
        _ => format!("format_{}s", to_snake_case(name)),
    }
}

fn pluralize_name(name: &str) -> String {
    match name {
        "City" => "cities".to_string(),
        "TeamDiplomacy" | "TribeDiplomacy" => format!("{} entries", to_snake_case(name).replace('_', " ")),
        _ => format!("{}s", to_snake_case(name).replace('_', " ")),
    }
}

fn is_type_constant_field(name: &str) -> bool {
    let name_lower = name.to_lowercase();
    name_lower.contains("type")
        || name_lower.contains("nation")
        || name_lower.contains("family")
        || name_lower.contains("religion")
        || name_lower.contains("terrain")
        || name_lower.contains("resource")
        || name_lower.contains("improvement")
        || name_lower.contains("tribe")
        || name_lower.contains("height")
        || name_lower.contains("vegetation")
        || name_lower.contains("yield")
        || name_lower.ends_with("unit")  // More precise to avoid "count"
        || name_lower.contains("project")
        || name_lower.contains("effect")
        || name_lower.contains("council")
        || name_lower.contains("job")
        || name_lower.contains("trait")
        || name_lower.contains("goal")
        || name_lower.contains("law")
        || name_lower.contains("mission")
        || name_lower.contains("tech")
        || name_lower.contains("diplomacy")
        || name_lower.contains("development")
        || name_lower.contains("difficulty")
}

/// Convert camelCase to snake_case, handling acronyms like ID correctly
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            let prev_upper = i > 0 && chars[i - 1].is_uppercase();
            let next_lower = i + 1 < chars.len() && chars[i + 1].is_lowercase();

            // Add underscore before this char if:
            // - Not first char AND
            // - Either previous was lowercase OR (previous was upper AND next is lower)
            if i > 0 && (!prev_upper || (prev_upper && next_lower)) {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

/// Convert camelCase to "Title Case With Spaces", keeping acronyms together
/// e.g., "birthCityID" -> "Birth City ID", "capitalCityID" -> "Capital City ID"
fn to_display_name(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() && i > 0 {
            let prev_upper = chars[i - 1].is_uppercase();
            let next_lower = i + 1 < chars.len() && chars[i + 1].is_lowercase();

            // Add space before this char if:
            // - Previous was lowercase (start of new word), OR
            // - Previous was upper AND next is lower (end of acronym like "ID" before "Name")
            if !prev_upper || (prev_upper && next_lower) {
                result.push(' ');
            }
        }

        if i == 0 {
            result.push(c.to_uppercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

fn escape_rust_keyword(name: &str) -> String {
    // Progenitor uses trailing underscore for keywords, not r# prefix
    // e.g., "type" becomes "type_"
    const KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "crate", "else", "enum", "extern",
        "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
        "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
        "super", "trait", "true", "type", "unsafe", "use", "where", "while",
        "async", "await", "dyn", "abstract", "become", "box", "do", "final",
        "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
    ];
    if KEYWORDS.contains(&name) {
        format!("{}_", name)
    } else {
        name.to_string()
    }
}
