//! Table formatters for API types.
//!
//! Format functions are auto-generated from openapi.yaml.
//! Regenerate with: cargo run --bin gen_formatters > src/output/table_generated.rs

use crate::client::types;

// === Utility ===

/// Shorten game type strings (e.g., "NATION_ROME" -> "Rome")
fn shorten_type(s: &str) -> String {
    s.split('_')
        .skip(1)
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first
                    .to_uppercase()
                    .chain(chars.map(|c| c.to_lowercase().next().unwrap_or(c)))
                    .collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

// Include auto-generated formatters
include!("table_generated.rs");
