use crate::client::types::{City, Player, Tile};
use std::collections::HashMap;

/// Marker type for cities on the map
#[derive(Debug, Clone, Copy)]
pub enum CityMarker {
    Capital,
    Regular,
}

/// Processed tile data ready for rendering
#[derive(Debug)]
pub struct RenderTile {
    pub terrain: Option<String>,
    pub height: Option<String>,
    pub vegetation: Option<String>,
    pub owner_nation: Option<String>,
    pub city_marker: Option<CityMarker>,
}

/// All data needed to render the map
#[derive(Debug)]
pub struct MapData {
    pub tiles: HashMap<(i32, i32), RenderTile>,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

impl MapData {
    pub fn from_game_data(tiles: &[Tile], cities: &[City], players: &[Player]) -> Self {
        // Build player index -> nation mapping (players are returned in index order)
        let player_nations: Vec<Option<&str>> = players
            .iter()
            .map(|p| p.nation.as_deref())
            .collect();

        // Build city location -> marker map
        let city_markers: HashMap<(i32, i32), CityMarker> = cities
            .iter()
            .filter_map(|c| {
                let x = c.x? as i32;
                let y = c.y? as i32;
                let marker = if c.is_capital.unwrap_or(false) {
                    CityMarker::Capital
                } else {
                    CityMarker::Regular
                };
                Some(((x, y), marker))
            })
            .collect();

        let mut tile_map = HashMap::new();
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for tile in tiles {
            let x = tile.x.unwrap_or(0);
            let y = tile.y.unwrap_or(0);

            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);

            // Map owner index to nation name
            // owner is "NONE" for unowned tiles, or a player index like "0", "1", etc.
            let owner_nation = tile.owner.as_ref().and_then(|owner| {
                if owner == "NONE" {
                    None
                } else {
                    // Parse player index and look up nation
                    owner.parse::<usize>().ok().and_then(|idx| {
                        player_nations.get(idx).and_then(|n| n.map(String::from))
                    })
                }
            });

            let render_tile = RenderTile {
                terrain: tile.terrain.clone(),
                height: tile.height.clone(),
                vegetation: tile.vegetation.clone(),
                owner_nation,
                city_marker: city_markers.get(&(x, y)).copied(),
            };

            tile_map.insert((x, y), render_tile);
        }

        Self {
            tiles: tile_map,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}
