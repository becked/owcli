use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamDiplomacy {
    pub from_team: i32,
    pub to_team: i32,
    pub relation: String,
    #[serde(default)]
    pub opinion: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamAlliance {
    pub team_a: i32,
    pub team_b: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TribeDiplomacy {
    pub tribe_type: String,
    pub team: i32,
    pub relation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TribeAlliance {
    pub tribe_type: String,
    pub player_index: i32,
}
