use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameConfig {
    pub turn: i32,
    pub year: i32,
    #[serde(default)]
    pub player_count: i32,
    #[serde(default)]
    pub team_count: i32,
}
