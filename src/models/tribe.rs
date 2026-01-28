use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tribe {
    pub tribe_type: String,
    #[serde(default)]
    pub strength: i32,
    #[serde(default)]
    pub units: i32,
    #[serde(default)]
    pub camps: i32,
    #[serde(default)]
    pub settlements: i32,
}
