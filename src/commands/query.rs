use crate::client::{fetch, ApiClient};
use crate::error::{OwcliError, Result};
use crate::output::TypedResponse;
use crate::path_parser::{parse_path, EndpointType};

/// Execute a query for the given path
pub async fn execute_query(
    client: &ApiClient,
    path_str: &str,
    fields: Option<&str>,
) -> Result<TypedResponse> {
    let api_path = parse_path(path_str)?;

    match api_path.endpoint_type {
        EndpointType::State => Ok(TypedResponse::State(fetch(client.inner.get_state()).await?)),
        EndpointType::Config => Ok(TypedResponse::Config(fetch(client.inner.get_config()).await?)),
        EndpointType::Players => Ok(TypedResponse::Players(fetch(client.inner.get_players()).await?)),
        EndpointType::Player => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::Player(fetch(client.inner.get_player(index as i64)).await?))
        }
        EndpointType::PlayerUnits => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerUnits(
                fetch(client.inner.get_player_units(index as i64)).await?,
            ))
        }
        EndpointType::PlayerTechs => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerTechs(
                fetch(client.inner.get_player_techs(index as i64)).await?,
            ))
        }
        EndpointType::PlayerFamilies => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerFamilies(
                fetch(client.inner.get_player_families(index as i64)).await?,
            ))
        }
        EndpointType::PlayerReligion => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerReligion(
                fetch(client.inner.get_player_religion(index as i64)).await?,
            ))
        }
        EndpointType::PlayerGoals => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerGoals(
                fetch(client.inner.get_player_goals(index as i64)).await?,
            ))
        }
        EndpointType::PlayerDecisions => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerDecisions(
                fetch(client.inner.get_player_decisions(index as i64)).await?,
            ))
        }
        EndpointType::PlayerLaws => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerLaws(
                fetch(client.inner.get_player_laws(index as i64)).await?,
            ))
        }
        EndpointType::PlayerMissions => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerMissions(
                fetch(client.inner.get_player_missions(index as i64)).await?,
            ))
        }
        EndpointType::PlayerResources => {
            let index = extract_index(&api_path.path, "player")?;
            Ok(TypedResponse::PlayerResources(
                fetch(client.inner.get_player_resources(index as i64)).await?,
            ))
        }
        EndpointType::Cities => Ok(TypedResponse::Cities(fetch(client.inner.get_cities()).await?)),
        EndpointType::City => {
            let id = extract_index(&api_path.path, "city")?;
            Ok(TypedResponse::City(fetch(client.inner.get_city(id as i64)).await?))
        }
        EndpointType::Characters => Ok(TypedResponse::Characters(fetch(client.inner.get_characters()).await?)),
        EndpointType::Character => {
            let id = extract_index(&api_path.path, "character")?;
            Ok(TypedResponse::Character(fetch(client.inner.get_character(id as i64)).await?))
        }
        EndpointType::Units => Ok(TypedResponse::Units(fetch(client.inner.get_units()).await?)),
        EndpointType::Unit => {
            let id = extract_index(&api_path.path, "unit")?;
            Ok(TypedResponse::Unit(fetch(client.inner.get_unit(id as i64)).await?))
        }
        EndpointType::Map => Ok(TypedResponse::Map(fetch(client.inner.get_map()).await?)),
        EndpointType::Tiles => {
            // Default pagination
            let response = fetch(client.inner.get_tiles(fields, Some(100), Some(0))).await?;
            Ok(TypedResponse::Tiles(response.tiles))
        }
        EndpointType::Tile => {
            // Parse tile path - could be tile/<id> or tile/<x>/<y>
            let parts: Vec<&str> = api_path.path.split('/').collect();
            match parts.as_slice() {
                ["tile", id] => {
                    let tile_id = id
                        .parse::<i32>()
                        .map_err(|_| OwcliError::InvalidPath(format!("Invalid tile ID: {}", id)))?;
                    Ok(TypedResponse::Tile(
                        fetch(client.inner.get_tile(tile_id as i64, fields)).await?,
                    ))
                }
                ["tile", x, y] => {
                    let x_coord = x.parse::<i32>().map_err(|_| {
                        OwcliError::InvalidPath(format!("Invalid x coordinate: {}", x))
                    })?;
                    let y_coord = y.parse::<i32>().map_err(|_| {
                        OwcliError::InvalidPath(format!("Invalid y coordinate: {}", y))
                    })?;
                    Ok(TypedResponse::Tile(
                        fetch(client.inner.get_tile_by_coords(x_coord as i64, y_coord as i64, fields))
                            .await?,
                    ))
                }
                _ => Err(OwcliError::InvalidPath(format!(
                    "Invalid tile path: {}",
                    api_path.path
                ))),
            }
        }
        EndpointType::Tribes => Ok(TypedResponse::Tribes(fetch(client.inner.get_tribes()).await?)),
        EndpointType::Tribe => {
            let tribe_type = extract_string_param(&api_path.path, "tribe")?;
            Ok(TypedResponse::Tribe(fetch(client.inner.get_tribe(&tribe_type)).await?))
        }
        EndpointType::Religions => Ok(TypedResponse::Religions(fetch(client.inner.get_religions()).await?)),
        EndpointType::TeamDiplomacy => Ok(TypedResponse::TeamDiplomacy(
            fetch(client.inner.get_diplomacy_teams()).await?,
        )),
        EndpointType::TeamAlliances => Ok(TypedResponse::TeamAlliances(
            fetch(client.inner.get_alliances_teams()).await?,
        )),
        EndpointType::TribeDiplomacy => Ok(TypedResponse::TribeDiplomacy(
            fetch(client.inner.get_diplomacy_tribes()).await?,
        )),
        EndpointType::TribeAlliances => Ok(TypedResponse::TribeAlliances(
            fetch(client.inner.get_alliances_tribes()).await?,
        )),
        EndpointType::CharacterEvents => Ok(TypedResponse::CharacterEvents(
            fetch(client.inner.get_turn_summary_characters()).await?,
        )),
        EndpointType::UnitEvents => Ok(TypedResponse::UnitEvents(
            fetch(client.inner.get_turn_summary_units()).await?,
        )),
        EndpointType::CityEvents => Ok(TypedResponse::CityEvents(
            fetch(client.inner.get_turn_summary_cities()).await?,
        )),
    }
}

/// Execute a query for tiles with pagination
pub async fn execute_tiles_query(
    client: &ApiClient,
    offset: u32,
    limit: u32,
    fields: Option<&str>,
) -> Result<TypedResponse> {
    let response =
        fetch(client.inner.get_tiles(fields, Some(limit as i64), Some(offset as i64))).await?;
    Ok(TypedResponse::Tiles(response.tiles))
}

/// Execute a query for all tiles (parallel batched fetching)
pub async fn execute_all_tiles_query(
    client: &ApiClient,
    fields: Option<&str>,
) -> Result<TypedResponse> {
    use futures::stream::{self, StreamExt};

    const BATCH_SIZE: i64 = 1000;
    const MAX_CONCURRENT: usize = 4;

    // Get total tile count from map metadata
    let map = fetch(client.inner.get_map()).await?;
    let total = map.num_tiles.unwrap_or(0) as i64;

    if total == 0 {
        return Ok(TypedResponse::Tiles(vec![]));
    }

    // Calculate offsets for all batches
    let offsets: Vec<i64> = (0..total).step_by(BATCH_SIZE as usize).collect();

    // Fetch batches in parallel with concurrency limit (buffered preserves order)
    let results: Vec<_> = stream::iter(offsets)
        .map(|offset| async move {
            fetch(client.inner.get_tiles(fields, Some(BATCH_SIZE), Some(offset))).await
        })
        .buffered(MAX_CONCURRENT)
        .collect()
        .await;

    // Combine results, preserving order by offset
    let mut all_tiles = Vec::with_capacity(total as usize);
    for result in results {
        all_tiles.extend(result?.tiles);
    }

    Ok(TypedResponse::Tiles(all_tiles))
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
