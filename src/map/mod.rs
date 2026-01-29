pub mod colors;
pub mod data;
pub mod hex;
pub mod minimap;

use crate::client::types::{City, Player, Tile};
use crate::error::Result;

pub use data::MapData;

/// Render a compact minimap (default for `owcli map`)
pub fn render_map(tiles: &[Tile], cities: &[City], players: &[Player]) -> Result<String> {
    let map_data = MapData::from_game_data(tiles, cities, players);
    minimap::render_minimap(&map_data)
}

/// Render detailed hex grid (for future interactive mode)
#[allow(dead_code)]
pub fn render_hex_map(tiles: &[Tile], cities: &[City], players: &[Player]) -> Result<String> {
    let map_data = MapData::from_game_data(tiles, cities, players);
    hex::render_hex_grid(&map_data)
}
