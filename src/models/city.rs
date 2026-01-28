use super::YieldMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct City {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub owner_id: Option<i32>,
    pub tile_id: i32,
    pub x: i32,
    pub y: i32,
    pub nation: String,
    #[serde(default)]
    pub is_capital: bool,
    #[serde(default)]
    pub citizens: i32,
    #[serde(default)]
    pub hp: i32,
    #[serde(default)]
    pub hp_max: i32,
    #[serde(default)]
    pub culture: i32,
    #[serde(default)]
    pub happiness: i32,
    #[serde(default)]
    pub discontent: i32,
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub governor_id: Option<i32>,
    #[serde(default)]
    pub religion: Option<String>,
    #[serde(default)]
    pub yields: YieldMap,
    #[serde(default)]
    pub build_queue: Vec<BuildQueueItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildQueueItem {
    pub build_type: String,
    pub item_type: String,
    #[serde(default)]
    pub progress: i32,
    #[serde(default)]
    pub threshold: i32,
    #[serde(default)]
    pub turns_left: Option<i32>,
    #[serde(default)]
    pub is_hurried: bool,
}
