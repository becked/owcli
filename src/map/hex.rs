use crate::error::Result;
use crossterm::style::Stylize;

use super::colors::{get_foreground_for_background, nation_to_color, terrain_to_color};
use super::data::{CityMarker, MapData};

/// Render the hex grid to a colored string (for future interactive mode)
#[allow(dead_code)]
///
/// Hex pattern (flat-top, 4 chars wide, 2 rows per hex):
/// ```text
///  / \ / \ / \ / \
/// |   |   | * |   |
///  \ / \ / \ / \ /
///   |   |   |   |
///  / \ / \ / \ / \
/// ```
///
/// Odd columns are offset down by half a hex height.
pub fn render_hex_grid(map_data: &MapData) -> Result<String> {
    let mut output = String::new();

    // For each logical row, we need to render in two passes:
    // 1. Even columns at the "up" position
    // 2. Odd columns at the "down" position (offset by one screen row)

    // Build output line by line

    // Top edge for first row (even columns)
    output.push_str(&render_top_edge(map_data));
    output.push('\n');

    for row in map_data.min_y..=map_data.max_y {
        // Content row (the |   | part)
        output.push_str(&render_content_row(map_data, row));
        output.push('\n');

        // Bottom edge / connector row
        if row < map_data.max_y {
            output.push_str(&render_connector_row(map_data, row));
            output.push('\n');
        }
    }

    // Bottom edge for last row
    output.push_str(&render_bottom_edge(map_data));
    output.push('\n');

    Ok(output)
}

fn render_top_edge(map_data: &MapData) -> String {
    let mut line = String::new();

    for col in map_data.min_x..=map_data.max_x {
        let is_odd = (col - map_data.min_x) % 2 != 0;

        if col == map_data.min_x {
            // First hex
            if is_odd {
                line.push_str("   ");
            }
            line.push_str(" /");
        }

        line.push_str(" \\");

        if !is_odd && col < map_data.max_x {
            line.push_str(" /");
        }
    }

    line
}

fn render_content_row(map_data: &MapData, row: i32) -> String {
    let mut line = String::new();

    for col in map_data.min_x..=map_data.max_x {
        let tile = map_data.tiles.get(&(col, row));
        let is_odd = (col - map_data.min_x) % 2 != 0;

        // Determine colors
        let bg_color = match tile {
            Some(t) => {
                if let Some(ref nation) = t.owner_nation {
                    nation_to_color(nation)
                } else {
                    terrain_to_color(
                        t.terrain.as_deref(),
                        t.height.as_deref(),
                        t.vegetation.as_deref(),
                    )
                }
            }
            None => crossterm::style::Color::Black,
        };

        let fg_color = get_foreground_for_background(bg_color);

        // Center character (city marker or space)
        let center = match tile.and_then(|t| t.city_marker) {
            Some(CityMarker::Capital) => "\u{2605}", // Black star ★
            Some(CityMarker::Regular) => "\u{2022}", // Bullet •
            None => " ",
        };

        // Left edge
        if col == map_data.min_x {
            if is_odd {
                line.push_str("  ");
            }
            line.push('|');
        }

        // Content with background color
        let content = format!(" {} ", center);
        line.push_str(&content.on(bg_color).with(fg_color).to_string());

        // Right edge
        line.push('|');
    }

    line
}

fn render_connector_row(map_data: &MapData, _row: i32) -> String {
    let mut line = String::new();

    for col in map_data.min_x..=map_data.max_x {
        let is_odd = (col - map_data.min_x) % 2 != 0;

        if col == map_data.min_x {
            if is_odd {
                line.push_str("  ");
            }
            line.push_str(" \\");
        } else if !is_odd {
            line.push_str(" \\");
        }

        if is_odd || col == map_data.max_x {
            line.push_str(" /");
        }

        if is_odd && col < map_data.max_x {
            line.push_str(" \\");
        }
    }

    line
}

fn render_bottom_edge(map_data: &MapData) -> String {
    let mut line = String::new();

    for col in map_data.min_x..=map_data.max_x {
        let is_odd = (col - map_data.min_x) % 2 != 0;

        if col == map_data.min_x {
            if is_odd {
                line.push_str("  ");
            }
            line.push_str(" \\");
        } else if !is_odd {
            line.push_str(" \\");
        }

        if col < map_data.max_x {
            if is_odd {
                line.push_str(" / \\");
            } else {
                line.push_str(" /");
            }
        } else {
            // Last column
            if is_odd {
                line.push_str(" /");
            }
        }
    }

    line
}
