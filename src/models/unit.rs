use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    pub id: i32,
    pub unit_type: String,
    pub tile_id: i32,
    pub x: i32,
    pub y: i32,
    pub hp: i32,
    #[serde(default)]
    pub is_alive: bool,
    #[serde(default)]
    pub owner_id: Option<i32>,
    #[serde(default)]
    pub hp_max: i32,
    #[serde(default)]
    pub damage: i32,
    #[serde(default)]
    pub xp: i32,
    #[serde(default)]
    pub level: i32,
    #[serde(default)]
    pub turn_steps: i32,
    #[serde(default)]
    pub cooldown_turns: i32,
    #[serde(default)]
    pub fortify_turns: i32,
    #[serde(default)]
    pub create_turn: i32,
    #[serde(default)]
    pub general_id: Option<i32>,
    #[serde(default)]
    pub has_general: bool,
    #[serde(default)]
    pub is_sleep: bool,
    #[serde(default)]
    pub is_sentry: bool,
    #[serde(default)]
    pub is_pass: bool,
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub has_family: bool,
    #[serde(default)]
    pub religion: Option<String>,
    #[serde(default)]
    pub has_religion: bool,
    #[serde(default)]
    pub promotions: Vec<String>,
}
