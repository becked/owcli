use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tile {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    #[serde(default)]
    pub terrain: Option<String>,
    #[serde(default)]
    pub height: Option<String>,
    #[serde(default)]
    pub vegetation: Option<String>,
    #[serde(default)]
    pub resource: Option<String>,
    #[serde(default)]
    pub improvement: Option<String>,
    #[serde(default)]
    pub is_pillaged: bool,
    #[serde(default)]
    pub owner_id: Option<i32>,
    #[serde(default)]
    pub city_id: Option<i32>,
    #[serde(default)]
    pub has_road: bool,
    #[serde(default)]
    pub river_edges: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MapInfo {
    pub num_tiles: i32,
}
