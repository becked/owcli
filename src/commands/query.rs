use crate::client::ApiClient;
use crate::error::{OwcliError, Result};
use crate::output::TypedResponse;
use crate::path_parser::{parse_path, EndpointType};

/// Execute a query for the given path
pub async fn execute_query(client: &ApiClient, path_str: &str) -> Result<TypedResponse> {
    let api_path = parse_path(path_str)?;

    match api_path.endpoint_type {
        EndpointType::State => Ok(TypedResponse::State(client.get_state().await?)),
        EndpointType::Config => Ok(TypedResponse::Config(client.get_config().await?)),
        EndpointType::Players => Ok(TypedResponse::Players(client.get_players().await?)),
        EndpointType::Player => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::Player(client.get_player(index).await?))
        }
        EndpointType::PlayerUnits => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerUnits(
                client.get_player_units(index).await?,
            ))
        }
        EndpointType::PlayerTechs => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerTechs(
                client.get_player_techs(index).await?,
            ))
        }
        EndpointType::PlayerFamilies => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerFamilies(
                client.get_player_families(index).await?,
            ))
        }
        EndpointType::PlayerReligion => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerReligion(
                client.get_player_religion(index).await?,
            ))
        }
        EndpointType::PlayerGoals => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerGoals(
                client.get_player_goals(index).await?,
            ))
        }
        EndpointType::PlayerDecisions => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerDecisions(
                client.get_player_decisions(index).await?,
            ))
        }
        EndpointType::PlayerLaws => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerLaws(
                client.get_player_laws(index).await?,
            ))
        }
        EndpointType::PlayerMissions => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerMissions(
                client.get_player_missions(index).await?,
            ))
        }
        EndpointType::PlayerResources => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerResources(
                client.get_player_resources(index).await?,
            ))
        }
        EndpointType::Cities => Ok(TypedResponse::Cities(client.get_cities().await?)),
        EndpointType::City => {
            let id = extract_index(&api_path.path, "city")?;
            Ok(TypedResponse::City(client.get_city(id).await?))
        }
        EndpointType::Characters => Ok(TypedResponse::Characters(client.get_characters().await?)),
        EndpointType::Character => {
            let id = extract_index(&api_path.path, "character")?;
            Ok(TypedResponse::Character(client.get_character(id).await?))
        }
        EndpointType::Units => Ok(TypedResponse::Units(client.get_units().await?)),
        EndpointType::Unit => {
            let id = extract_index(&api_path.path, "unit")?;
            Ok(TypedResponse::Unit(client.get_unit(id).await?))
        }
        EndpointType::Map => Ok(TypedResponse::Map(client.get_map().await?)),
        EndpointType::Tiles => {
            // Default pagination
            Ok(TypedResponse::Tiles(
                client.get_tiles(Some(0), Some(100)).await?,
            ))
        }
        EndpointType::Tile => {
            // Parse tile path - could be tile/<id> or tile/<x>/<y>
            let parts: Vec<&str> = api_path.path.split('/').collect();
            match parts.as_slice() {
                ["tile", id] => {
                    let tile_id = id
                        .parse::<i32>()
                        .map_err(|_| OwcliError::InvalidPath(format!("Invalid tile ID: {}", id)))?;
                    Ok(TypedResponse::Tile(client.get_tile_by_id(tile_id).await?))
                }
                ["tile", x, y] => {
                    let x_coord = x.parse::<i32>().map_err(|_| {
                        OwcliError::InvalidPath(format!("Invalid x coordinate: {}", x))
                    })?;
                    let y_coord = y.parse::<i32>().map_err(|_| {
                        OwcliError::InvalidPath(format!("Invalid y coordinate: {}", y))
                    })?;
                    Ok(TypedResponse::Tile(
                        client.get_tile_by_coords(x_coord, y_coord).await?,
                    ))
                }
                _ => Err(OwcliError::InvalidPath(format!(
                    "Invalid tile path: {}",
                    api_path.path
                ))),
            }
        }
        EndpointType::Tribes => Ok(TypedResponse::Tribes(client.get_tribes().await?)),
        EndpointType::Tribe => {
            let tribe_type = extract_string_param(&api_path.path, "tribe")?;
            Ok(TypedResponse::Tribe(client.get_tribe(&tribe_type).await?))
        }
        EndpointType::Religions => Ok(TypedResponse::Religions(client.get_religions().await?)),
        EndpointType::TeamDiplomacy => Ok(TypedResponse::TeamDiplomacy(
            client.get_team_diplomacy().await?,
        )),
        EndpointType::TeamAlliances => Ok(TypedResponse::TeamAlliances(
            client.get_team_alliances().await?,
        )),
        EndpointType::TribeDiplomacy => Ok(TypedResponse::TribeDiplomacy(
            client.get_tribe_diplomacy().await?,
        )),
        EndpointType::TribeAlliances => Ok(TypedResponse::TribeAlliances(
            client.get_tribe_alliances().await?,
        )),
        EndpointType::CharacterEvents => Ok(TypedResponse::CharacterEvents(
            client.get_character_events().await?,
        )),
        EndpointType::UnitEvents => Ok(TypedResponse::UnitEvents(client.get_unit_events().await?)),
        EndpointType::CityEvents => Ok(TypedResponse::CityEvents(client.get_city_events().await?)),
    }
}

/// Execute a query for tiles with pagination
pub async fn execute_tiles_query(
    client: &ApiClient,
    offset: u32,
    limit: u32,
) -> Result<TypedResponse> {
    Ok(TypedResponse::Tiles(
        client
            .get_tiles(Some(offset as i32), Some(limit as i32))
            .await?,
    ))
}

/// Extract an integer index from a path like "player/0" or "city/123"
fn extract_index(path: &str, prefix: &str) -> Result<i32> {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() >= 2 && parts[0] == prefix {
        parts[1]
            .parse::<i32>()
            .map_err(|_| OwcliError::InvalidPath(format!("Invalid {} index: {}", prefix, parts[1])))
    } else {
        Err(OwcliError::InvalidPath(format!(
            "Could not extract index from path: {}",
            path
        )))
    }
}

/// Extract a string parameter from a path like "tribe/TRIBE_GAUL"
fn extract_string_param(path: &str, prefix: &str) -> Result<String> {
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() >= 2 && parts[0] == prefix {
        Ok(parts[1].to_string())
    } else {
        Err(OwcliError::InvalidPath(format!(
            "Could not extract parameter from path: {}",
            path
        )))
    }
}
