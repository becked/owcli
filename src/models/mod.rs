#![allow(dead_code)]

pub mod character;
pub mod city;
pub mod command;
pub mod config;
pub mod diplomacy;
pub mod events;
pub mod player;
pub mod tile;
pub mod tribe;
pub mod unit;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A map of yield types to integer values (e.g., YIELD_FOOD -> 100)
pub type YieldMap = HashMap<String, i32>;

/// Common location information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub tile_id: i32,
    pub x: i32,
    pub y: i32,
}
