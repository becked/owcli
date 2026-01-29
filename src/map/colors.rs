use crossterm::style::Color;

/// Map nation type string to RGB color
/// Based on docs/nation-colors.md
pub fn nation_to_color(nation: &str) -> Color {
    match nation {
        "NATION_ROME" => Color::Rgb {
            r: 136,
            g: 13,
            b: 86,
        }, // #880D56 Purple
        "NATION_GREECE" => Color::Rgb {
            r: 35,
            g: 96,
            b: 188,
        }, // #2360BC Blue
        "NATION_EGYPT" => Color::Rgb {
            r: 188,
            g: 99,
            b: 4,
        }, // #BC6304 Orange
        "NATION_PERSIA" => Color::Rgb {
            r: 192,
            g: 78,
            b: 74,
        }, // #C04E4A Red
        "NATION_BABYLON" | "NATION_BABYLONIA" => Color::Rgb {
            r: 130,
            g: 200,
            b: 62,
        }, // #82C83E Green
        "NATION_ASSYRIA" => Color::Rgb {
            r: 250,
            g: 220,
            b: 59,
        }, // #FADC3B Yellow
        "NATION_HITTITE" => Color::Rgb {
            r: 128,
            g: 227,
            b: 232,
        }, // #80E3E8 Cyan
        "NATION_CARTHAGE" => Color::Rgb {
            r: 246,
            g: 239,
            b: 225,
        }, // #F6EFE1 Beige
        "NATION_AKSUM" => Color::Rgb {
            r: 248,
            g: 163,
            b: 180,
        }, // #F8A3B4 Pink
        "NATION_KUSH" => Color::Rgb {
            r: 255,
            g: 255,
            b: 182,
        }, // #FFFFB6 Light Yellow
        _ => Color::DarkGrey, // Fallback for unknown nations
    }
}

/// Map tribe type string to RGB color
#[allow(dead_code)]
pub fn tribe_to_color(tribe: &str) -> Color {
    match tribe {
        "TRIBE_GAUL" | "TRIBE_GAULS" => Color::Rgb {
            r: 200,
            g: 71,
            b: 50,
        }, // #C84732
        "TRIBE_VANDAL" | "TRIBE_VANDALS" => Color::Rgb {
            r: 135,
            g: 219,
            b: 64,
        }, // #87DB40
        "TRIBE_DANE" | "TRIBE_DANES" => Color::Rgb {
            r: 156,
            g: 93,
            b: 255,
        }, // #9C5DFF
        "TRIBE_THRACIAN" | "TRIBE_THRACIANS" => Color::Rgb {
            r: 60,
            g: 205,
            b: 194,
        }, // #3CCDC2
        "TRIBE_SCYTHIAN" | "TRIBE_SCYTHIANS" => Color::Rgb {
            r: 216,
            g: 154,
            b: 24,
        }, // #D89A18
        "TRIBE_NUMIDIAN" | "TRIBE_NUMIDIANS" => Color::Rgb {
            r: 230,
            g: 225,
            b: 202,
        }, // #E6E1CA
        _ => Color::DarkMagenta, // Fallback
    }
}

/// Map terrain/vegetation/height to background color for unowned tiles
/// Priority: height (mountains/hills) > vegetation (forest) > terrain (water/desert/plains)
pub fn terrain_to_color(
    terrain: Option<&str>,
    height: Option<&str>,
    vegetation: Option<&str>,
) -> Color {
    // Height takes precedence (mountains, hills)
    if let Some(h) = height {
        match h {
            s if s.contains("MOUNTAIN") => {
                return Color::Rgb {
                    r: 128,
                    g: 128,
                    b: 128,
                }
            } // Gray
            s if s.contains("HILL") => {
                return Color::Rgb {
                    r: 139,
                    g: 90,
                    b: 43,
                }
            } // Brown
            _ => {}
        }
    }

    // Then check vegetation
    if let Some(v) = vegetation {
        match v {
            s if s.contains("FOREST") || s.contains("JUNGLE") => {
                return Color::Rgb {
                    r: 34,
                    g: 85,
                    b: 34,
                }; // Dark green
            }
            _ => {}
        }
    }

    // Finally terrain type
    match terrain {
        Some(t) if t.contains("WATER") || t.contains("OCEAN") || t.contains("COAST") => {
            Color::Rgb {
                r: 30,
                g: 90,
                b: 150,
            } // Blue
        }
        Some(t) if t.contains("DESERT") || t.contains("ARID") => Color::Rgb {
            r: 194,
            g: 178,
            b: 128,
        }, // Sandy yellow
        Some(t) if t.contains("TUNDRA") || t.contains("SNOW") => Color::Rgb {
            r: 220,
            g: 220,
            b: 230,
        }, // Off-white
        _ => Color::Rgb {
            r: 86,
            g: 125,
            b: 70,
        }, // Light green (plains/temperate default)
    }
}

/// Get appropriate foreground color for contrast against background
pub fn get_foreground_for_background(bg: Color) -> Color {
    match bg {
        Color::Rgb { r, g, b } => {
            // Simple luminance check - light backgrounds get dark text
            let luminance = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0;
            if luminance > 0.5 {
                Color::Black
            } else {
                Color::White
            }
        }
        Color::White => Color::Black,
        _ => Color::White,
    }
}
