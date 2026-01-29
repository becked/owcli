use crate::error::Result;
use crossterm::style::Stylize;

use super::colors::{get_foreground_for_background, nation_to_color, terrain_to_color};
use super::data::{CityMarker, MapData};

/// Render a compact minimap where each tile is two characters wide
///
/// Layout uses hex offset: odd rows are shifted right by 1 character
/// ```text
/// ████████████████████████
///  ██████████★█████████████
/// ████████████████████████
///  ███████████████████████
/// ```
pub fn render_minimap(map_data: &MapData) -> Result<String> {
    let mut output = String::new();

    for row in map_data.min_y..=map_data.max_y {
        // Hex offset: odd rows shift right by 1 character
        let is_odd_row = (row - map_data.min_y) % 2 != 0;
        if is_odd_row {
            output.push(' ');
        }

        for col in map_data.min_x..=map_data.max_x {
            let tile = map_data.tiles.get(&(col, row));

            // Determine tile color
            let tile_color = match tile {
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

            // For city markers, use contrasting foreground on tile background
            // For regular tiles, use block characters in tile color
            let cell = match tile.and_then(|t| t.city_marker) {
                Some(CityMarker::Capital) => {
                    let fg = get_foreground_for_background(tile_color);
                    "\u{2605} ".on(tile_color).with(fg).to_string() // ★ with space
                }
                Some(CityMarker::Regular) => {
                    let fg = get_foreground_for_background(tile_color);
                    "\u{2022} ".on(tile_color).with(fg).to_string() // • with space
                }
                None => {
                    // Two block characters in tile color (no background needed)
                    "\u{2588}\u{2588}".with(tile_color).to_string() // ██
                }
            };

            output.push_str(&cell);
        }

        output.push('\n');
    }

    Ok(output)
}
