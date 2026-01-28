use super::YieldMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub index: i32,
    pub team: i32,
    pub nation: String,
    #[serde(default)]
    pub leader_id: Option<i32>,
    pub cities: i32,
    pub units: i32,
    pub legitimacy: i32,
    #[serde(default)]
    pub stockpiles: YieldMap,
    #[serde(default)]
    pub rates: YieldMap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerTech {
    pub tech_type: String,
    pub is_researched: bool,
    #[serde(default)]
    pub progress: i32,
    #[serde(default)]
    pub cost: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFamily {
    pub family_type: String,
    pub opinion: i32,
    #[serde(default)]
    pub head_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerReligion {
    pub religion_type: Option<String>,
    #[serde(default)]
    pub is_state_religion: bool,
}
