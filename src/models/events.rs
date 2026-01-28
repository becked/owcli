use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacterEvent {
    pub event_type: String,
    pub character_id: i32,
    #[serde(default)]
    pub character_name: Option<String>,
    pub turn: i32,
    #[serde(default)]
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitEvent {
    pub event_type: String,
    pub unit_id: i32,
    #[serde(default)]
    pub unit_type: Option<String>,
    pub turn: i32,
    #[serde(default)]
    pub tile_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CityEvent {
    pub event_type: String,
    pub city_id: i32,
    #[serde(default)]
    pub city_name: Option<String>,
    pub turn: i32,
    #[serde(default)]
    pub player_id: Option<i32>,
}
