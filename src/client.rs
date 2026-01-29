use crate::config::Config;
use crate::error::{OwcliError, Result};

// Include generated progenitor code
mod generated {
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

pub use generated::types;
pub use generated::Client as ProgenitorClient;

pub struct ApiClient {
    inner: ProgenitorClient,
}

impl ApiClient {
    pub fn new(config: &Config) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        let base_url = config.base_url();
        let inner = ProgenitorClient::new_with_client(&base_url, http_client);

        Ok(Self { inner })
    }

    // === Query endpoints (using generated client) ===

    pub async fn get_state(&self) -> Result<types::GameState> {
        self.inner
            .get_state()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_config(&self) -> Result<types::GameConfig> {
        self.inner
            .get_config()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_players(&self) -> Result<Vec<types::Player>> {
        self.inner
            .get_players()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_player(&self, index: i32) -> Result<types::Player> {
        self.inner
            .get_player_by_index(index as u64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_player_units(&self, index: i32) -> Result<Vec<types::Unit>> {
        self.inner
            .get_player_units(index as u64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_player_techs(&self, index: i32) -> Result<types::PlayerTechs> {
        self.inner
            .get_player_techs(index as u64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_player_families(&self, index: i32) -> Result<types::PlayerFamilies> {
        self.inner
            .get_player_families(index as u64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_player_religion(&self, index: i32) -> Result<types::PlayerReligion> {
        self.inner
            .get_player_religion(index as u64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_player_goals(&self, index: i32) -> Result<types::PlayerGoals> {
        self.inner
            .get_player_goals(index as u64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_player_decisions(&self, index: i32) -> Result<types::PlayerDecisions> {
        self.inner
            .get_player_decisions(index as u64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_player_laws(&self, index: i32) -> Result<types::PlayerLaws> {
        self.inner
            .get_player_laws(index as u64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_player_missions(&self, index: i32) -> Result<types::PlayerMissions> {
        self.inner
            .get_player_missions(index as u64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_player_resources(&self, index: i32) -> Result<types::PlayerResources> {
        self.inner
            .get_player_resources(index as u64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_cities(&self) -> Result<Vec<types::City>> {
        self.inner
            .get_cities()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_city(&self, id: i32) -> Result<types::City> {
        self.inner
            .get_city_by_id(id as i64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_characters(&self) -> Result<Vec<types::Character>> {
        self.inner
            .get_characters()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_character(&self, id: i32) -> Result<types::Character> {
        self.inner
            .get_character_by_id(id as i64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_units(&self) -> Result<Vec<types::Unit>> {
        self.inner
            .get_units()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_unit(&self, id: i32) -> Result<types::Unit> {
        self.inner
            .get_unit_by_id(id as i64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_map(&self) -> Result<types::MapMetadata> {
        self.inner
            .get_map()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_tiles(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<types::Tile>> {
        self.inner
            .get_tiles(limit.map(|l| l as i64), offset.map(|o| o as u64))
            .await
            .map(|r| r.into_inner().tiles)
            .map_err(map_progenitor_error)
    }

    pub async fn get_tile_by_id(&self, id: i32) -> Result<types::Tile> {
        self.inner
            .get_tile_by_id(id as i64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_tile_by_coords(&self, x: i32, y: i32) -> Result<types::Tile> {
        self.inner
            .get_tile_by_coords(x as i64, y as i64)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_tribes(&self) -> Result<Vec<types::Tribe>> {
        self.inner
            .get_tribes()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_tribe(&self, tribe_type: &str) -> Result<types::Tribe> {
        self.inner
            .get_tribe_by_type(tribe_type)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_religions(&self) -> Result<Vec<types::Religion>> {
        self.inner
            .get_religions()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_team_diplomacy(&self) -> Result<Vec<types::TeamDiplomacy>> {
        self.inner
            .get_team_diplomacy()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_team_alliances(&self) -> Result<Vec<types::TeamAlliance>> {
        self.inner
            .get_team_alliances()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_tribe_diplomacy(&self) -> Result<Vec<types::TribeDiplomacy>> {
        self.inner
            .get_tribe_diplomacy()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_tribe_alliances(&self) -> Result<Vec<types::TribeAlliance>> {
        self.inner
            .get_tribe_alliances()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_character_events(&self) -> Result<Vec<types::CharacterEvent>> {
        self.inner
            .get_character_events()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_unit_events(&self) -> Result<Vec<types::UnitEvent>> {
        self.inner
            .get_unit_events()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    pub async fn get_city_events(&self) -> Result<Vec<types::CityEvent>> {
        self.inner
            .get_city_events()
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    // === Command endpoints (using generated client) ===

    /// Execute a single game command
    pub async fn execute_command(&self, command: &types::GameCommand) -> Result<types::CommandResult> {
        self.inner
            .execute_command(command)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }

    /// Execute multiple commands in sequence
    pub async fn execute_bulk_commands(
        &self,
        bulk: &types::BulkCommand,
    ) -> Result<types::BulkCommandResult> {
        self.inner
            .execute_bulk_commands(bulk)
            .await
            .map(|r| r.into_inner())
            .map_err(map_progenitor_error)
    }
}

fn map_progenitor_error<T: std::fmt::Debug>(err: progenitor_client::Error<T>) -> OwcliError {
    match &err {
        progenitor_client::Error::ErrorResponse(resp) => {
            let status = resp.status();
            match status.as_u16() {
                503 => OwcliError::GameUnavailable,
                404 => OwcliError::NotFound(format!("{:?}", err)),
                code => OwcliError::Api {
                    message: format!("{:?}", err),
                    code: Some(code),
                },
            }
        }
        progenitor_client::Error::CommunicationError(_) => {
            OwcliError::Other(format!("Communication error: {:?}", err))
        }
        _ => OwcliError::Other(format!("{:?}", err)),
    }
}

/// Helper to check if a command succeeded
pub fn command_succeeded(result: &types::CommandResult) -> bool {
    result.success.unwrap_or(true)
}
