use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub id: i32,
    pub name: String,
    #[serde(default)]
    pub gender: Option<String>,
    #[serde(default)]
    pub age: i32,
    #[serde(default)]
    pub player_id: Option<i32>,
    #[serde(default)]
    pub is_alive: bool,
    #[serde(default)]
    pub is_leader: bool,
    #[serde(default)]
    pub is_heir: bool,
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub religion: Option<String>,
    #[serde(default)]
    pub spouse_id: Option<i32>,
    #[serde(default)]
    pub father_id: Option<i32>,
    #[serde(default)]
    pub mother_id: Option<i32>,
    #[serde(default)]
    pub traits: Vec<String>,
    #[serde(default)]
    pub rating_courage: i32,
    #[serde(default)]
    pub rating_discipline: i32,
    #[serde(default)]
    pub rating_charisma: i32,
    #[serde(default)]
    pub rating_wisdom: i32,
}
