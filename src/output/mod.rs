pub mod table;

use crate::client::types;
use crate::error::Result;

/// Typed response enum for all API endpoints
#[derive(Debug)]
pub enum TypedResponse {
    State(types::GameState),
    Config(types::GameConfig),
    Players(Vec<types::Player>),
    Player(types::Player),
    PlayerUnits(Vec<types::Unit>),
    PlayerTechs(types::PlayerTechs),
    PlayerFamilies(types::PlayerFamilies),
    PlayerReligion(types::PlayerReligion),
    PlayerGoals(types::PlayerGoals),
    PlayerDecisions(types::PlayerDecisions),
    PlayerLaws(types::PlayerLaws),
    PlayerMissions(types::PlayerMissions),
    PlayerResources(types::PlayerResources),
    Cities(Vec<types::City>),
    City(types::City),
    Characters(Vec<types::Character>),
    Character(types::Character),
    Units(Vec<types::Unit>),
    Unit(types::Unit),
    Map(types::MapMetadata),
    Tiles(Vec<types::Tile>),
    Tile(types::Tile),
    Tribes(Vec<types::Tribe>),
    Tribe(types::Tribe),
    Religions(Vec<types::Religion>),
    TeamDiplomacy(Vec<types::TeamDiplomacy>),
    TeamAlliances(Vec<types::TeamAlliance>),
    TribeDiplomacy(Vec<types::TribeDiplomacy>),
    TribeAlliances(Vec<types::TribeAlliance>),
    CharacterEvents(Vec<types::CharacterEvent>),
    UnitEvents(Vec<types::UnitEvent>),
    CityEvents(Vec<types::CityEvent>),
}

impl TypedResponse {
    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String> {
        let json = match self {
            Self::State(v) => serde_json::to_string_pretty(v)?,
            Self::Config(v) => serde_json::to_string_pretty(v)?,
            Self::Players(v) => serde_json::to_string_pretty(v)?,
            Self::Player(v) => serde_json::to_string_pretty(v)?,
            Self::PlayerUnits(v) => serde_json::to_string_pretty(v)?,
            Self::PlayerTechs(v) => serde_json::to_string_pretty(v)?,
            Self::PlayerFamilies(v) => serde_json::to_string_pretty(v)?,
            Self::PlayerReligion(v) => serde_json::to_string_pretty(v)?,
            Self::PlayerGoals(v) => serde_json::to_string_pretty(v)?,
            Self::PlayerDecisions(v) => serde_json::to_string_pretty(v)?,
            Self::PlayerLaws(v) => serde_json::to_string_pretty(v)?,
            Self::PlayerMissions(v) => serde_json::to_string_pretty(v)?,
            Self::PlayerResources(v) => serde_json::to_string_pretty(v)?,
            Self::Cities(v) => serde_json::to_string_pretty(v)?,
            Self::City(v) => serde_json::to_string_pretty(v)?,
            Self::Characters(v) => serde_json::to_string_pretty(v)?,
            Self::Character(v) => serde_json::to_string_pretty(v)?,
            Self::Units(v) => serde_json::to_string_pretty(v)?,
            Self::Unit(v) => serde_json::to_string_pretty(v)?,
            Self::Map(v) => serde_json::to_string_pretty(v)?,
            Self::Tiles(v) => serde_json::to_string_pretty(v)?,
            Self::Tile(v) => serde_json::to_string_pretty(v)?,
            Self::Tribes(v) => serde_json::to_string_pretty(v)?,
            Self::Tribe(v) => serde_json::to_string_pretty(v)?,
            Self::Religions(v) => serde_json::to_string_pretty(v)?,
            Self::TeamDiplomacy(v) => serde_json::to_string_pretty(v)?,
            Self::TeamAlliances(v) => serde_json::to_string_pretty(v)?,
            Self::TribeDiplomacy(v) => serde_json::to_string_pretty(v)?,
            Self::TribeAlliances(v) => serde_json::to_string_pretty(v)?,
            Self::CharacterEvents(v) => serde_json::to_string_pretty(v)?,
            Self::UnitEvents(v) => serde_json::to_string_pretty(v)?,
            Self::CityEvents(v) => serde_json::to_string_pretty(v)?,
        };
        Ok(json)
    }

    /// Format as table
    pub fn to_table(&self) -> String {
        match self {
            Self::State(v) => table::format_state(v),
            Self::Config(v) => table::format_config(v),
            Self::Players(v) => table::format_players(v),
            Self::Player(v) => table::format_player(v),
            Self::PlayerUnits(v) | Self::Units(v) => table::format_units(v),
            Self::PlayerTechs(v) => table::format_player_techs(v),
            Self::PlayerFamilies(v) => table::format_player_families(v),
            Self::PlayerReligion(v) => table::format_player_religion(v),
            Self::PlayerGoals(v) => table::format_player_goals(v),
            Self::PlayerDecisions(v) => table::format_player_decisions(v),
            Self::PlayerLaws(v) => table::format_player_laws(v),
            Self::PlayerMissions(v) => table::format_player_missions(v),
            Self::PlayerResources(v) => table::format_player_resources(v),
            Self::Cities(v) => table::format_cities(v),
            Self::City(v) => table::format_city(v),
            Self::Characters(v) => table::format_characters(v),
            Self::Character(v) => table::format_character(v),
            Self::Unit(v) => table::format_unit(v),
            Self::Map(v) => table::format_map(v),
            Self::Tiles(v) => table::format_tiles(v),
            Self::Tile(v) => table::format_tile(v),
            Self::Tribes(v) => table::format_tribes(v),
            Self::Tribe(v) => table::format_tribe(v),
            Self::Religions(v) => table::format_religions(v),
            Self::TeamDiplomacy(v) => table::format_team_diplomacy(v),
            Self::TeamAlliances(v) => table::format_team_alliances(v),
            Self::TribeDiplomacy(v) => table::format_tribe_diplomacy(v),
            Self::TribeAlliances(v) => table::format_tribe_alliances(v),
            Self::CharacterEvents(v) => table::format_character_events(v),
            Self::UnitEvents(v) => table::format_unit_events(v),
            Self::CityEvents(v) => table::format_city_events(v),
        }
    }
}

/// Format typed response based on output mode
pub fn format_typed_output(response: &TypedResponse, json_mode: bool) -> Result<String> {
    if json_mode {
        response.to_json()
    } else {
        Ok(response.to_table())
    }
}

/// Format a command response
pub fn format_command_response(
    success: bool,
    error: Option<&str>,
    request_id: Option<&str>,
    json_mode: bool,
) -> String {
    if json_mode {
        serde_json::json!({
            "success": success,
            "error": error,
            "requestId": request_id
        })
        .to_string()
    } else if success {
        match request_id {
            Some(id) => format!("Success (request: {})", id),
            None => "Success".to_string(),
        }
    } else {
        match error {
            Some(e) => format!("Error: {}", e),
            None => "Error: Unknown error".to_string(),
        }
    }
}

/// Format a bulk command response
pub fn format_bulk_response(response: &types::BulkCommandResult, json_mode: bool) -> String {
    if json_mode {
        serde_json::to_string_pretty(response).unwrap_or_else(|_| "{}".to_string())
    } else {
        let mut output = String::new();
        let succeeded = response
            .results
            .iter()
            .filter(|r| r.success.unwrap_or(false))
            .count();
        let failed = response.results.len() - succeeded;

        output.push_str(&format!(
            "Bulk execution: {} succeeded, {} failed\n",
            succeeded, failed
        ));

        if let Some(ref id) = response.request_id {
            output.push_str(&format!("Request ID: {}\n", id));
        }

        for result in &response.results {
            let status = if result.success.unwrap_or(false) {
                "OK"
            } else {
                "FAIL"
            };
            let error_msg = result
                .error
                .as_ref()
                .map(|e| format!(" - {}", e))
                .unwrap_or_default();
            let index = result.index.unwrap_or(-1);
            output.push_str(&format!("  [{}] Command {}{}\n", status, index, error_msg));
        }

        output
    }
}
