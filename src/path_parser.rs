use crate::error::{OwcliError, Result};

/// Represents a parsed API path
#[derive(Debug, Clone)]
pub struct ApiPath {
    /// The full path to send to the API
    pub path: String,
    /// The endpoint type for output formatting
    pub endpoint_type: EndpointType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EndpointType {
    State,
    Config,
    Players,
    Player,
    PlayerUnits,
    PlayerTechs,
    PlayerFamilies,
    PlayerReligion,
    PlayerGoals,
    PlayerDecisions,
    PlayerLaws,
    PlayerMissions,
    PlayerResources,
    Cities,
    City,
    Characters,
    Character,
    Units,
    Unit,
    Map,
    Tiles,
    Tile,
    Tribes,
    Tribe,
    Religions,
    TeamDiplomacy,
    TeamAlliances,
    TribeDiplomacy,
    TribeAlliances,
    CharacterEvents,
    UnitEvents,
    CityEvents,
}

/// Parse a slash-notation path into an API path
pub fn parse_path(input: &str) -> Result<ApiPath> {
    let input = input.trim().trim_start_matches('/');
    let parts: Vec<&str> = input.split('/').filter(|s| !s.is_empty()).collect();

    if parts.is_empty() {
        return Err(OwcliError::InvalidPath("Empty path".to_string()));
    }

    let (path, endpoint_type) = match parts.as_slice() {
        // Simple endpoints
        ["state"] => ("state".to_string(), EndpointType::State),
        ["config"] => ("config".to_string(), EndpointType::Config),
        ["players"] => ("players".to_string(), EndpointType::Players),
        ["cities"] => ("cities".to_string(), EndpointType::Cities),
        ["characters"] => ("characters".to_string(), EndpointType::Characters),
        ["units"] => ("units".to_string(), EndpointType::Units),
        ["map"] => ("map".to_string(), EndpointType::Map),
        ["tiles"] => ("tiles".to_string(), EndpointType::Tiles),
        ["tribes"] => ("tribes".to_string(), EndpointType::Tribes),
        ["religions"] => ("religions".to_string(), EndpointType::Religions),
        ["team-diplomacy"] => ("team-diplomacy".to_string(), EndpointType::TeamDiplomacy),
        ["team-alliances"] => ("team-alliances".to_string(), EndpointType::TeamAlliances),
        ["tribe-diplomacy"] => ("tribe-diplomacy".to_string(), EndpointType::TribeDiplomacy),
        ["tribe-alliances"] => ("tribe-alliances".to_string(), EndpointType::TribeAlliances),
        ["character-events"] => (
            "character-events".to_string(),
            EndpointType::CharacterEvents,
        ),
        ["unit-events"] => ("unit-events".to_string(), EndpointType::UnitEvents),
        ["city-events"] => ("city-events".to_string(), EndpointType::CityEvents),

        // Player with index
        ["player", index] => {
            validate_integer(index)?;
            (format!("player/{}", index), EndpointType::Player)
        }
        ["player", index, "units"] => {
            validate_integer(index)?;
            (format!("player/{}/units", index), EndpointType::PlayerUnits)
        }
        ["player", index, "techs"] => {
            validate_integer(index)?;
            (format!("player/{}/techs", index), EndpointType::PlayerTechs)
        }
        ["player", index, "families"] => {
            validate_integer(index)?;
            (
                format!("player/{}/families", index),
                EndpointType::PlayerFamilies,
            )
        }
        ["player", index, "religion"] => {
            validate_integer(index)?;
            (
                format!("player/{}/religion", index),
                EndpointType::PlayerReligion,
            )
        }
        ["player", index, "goals"] => {
            validate_integer(index)?;
            (format!("player/{}/goals", index), EndpointType::PlayerGoals)
        }
        ["player", index, "decisions"] => {
            validate_integer(index)?;
            (
                format!("player/{}/decisions", index),
                EndpointType::PlayerDecisions,
            )
        }
        ["player", index, "laws"] => {
            validate_integer(index)?;
            (format!("player/{}/laws", index), EndpointType::PlayerLaws)
        }
        ["player", index, "missions"] => {
            validate_integer(index)?;
            (
                format!("player/{}/missions", index),
                EndpointType::PlayerMissions,
            )
        }
        ["player", index, "resources"] => {
            validate_integer(index)?;
            (
                format!("player/{}/resources", index),
                EndpointType::PlayerResources,
            )
        }

        // City by ID
        ["city", id] => {
            validate_integer(id)?;
            (format!("city/{}", id), EndpointType::City)
        }

        // Character by ID
        ["character", id] => {
            validate_integer(id)?;
            (format!("character/{}", id), EndpointType::Character)
        }

        // Unit by ID
        ["unit", id] => {
            validate_integer(id)?;
            (format!("unit/{}", id), EndpointType::Unit)
        }

        // Tile by ID or coordinates
        ["tile", id] => {
            validate_integer(id)?;
            (format!("tile/{}", id), EndpointType::Tile)
        }
        ["tile", x, y] => {
            validate_integer(x)?;
            validate_integer(y)?;
            (format!("tile/{}/{}", x, y), EndpointType::Tile)
        }

        // Tribe by type
        ["tribe", tribe_type] => (format!("tribe/{}", tribe_type), EndpointType::Tribe),

        _ => {
            return Err(OwcliError::InvalidPath(format!(
                "Unknown path: '{}'. Run 'owcli --help' for valid paths.",
                parts.join("/")
            )));
        }
    };

    Ok(ApiPath {
        path,
        endpoint_type,
    })
}

fn validate_integer(s: &str) -> Result<()> {
    s.parse::<i32>()
        .map(|_| ())
        .map_err(|_| OwcliError::InvalidPath(format!("Expected integer, got '{}'", s)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_paths() {
        let path = parse_path("state").unwrap();
        assert_eq!(path.path, "state");
        assert_eq!(path.endpoint_type, EndpointType::State);

        let path = parse_path("players").unwrap();
        assert_eq!(path.path, "players");
        assert_eq!(path.endpoint_type, EndpointType::Players);
    }

    #[test]
    fn test_player_paths() {
        let path = parse_path("player/0").unwrap();
        assert_eq!(path.path, "player/0");
        assert_eq!(path.endpoint_type, EndpointType::Player);

        let path = parse_path("player/0/units").unwrap();
        assert_eq!(path.path, "player/0/units");
        assert_eq!(path.endpoint_type, EndpointType::PlayerUnits);
    }

    #[test]
    fn test_tile_paths() {
        let path = parse_path("tile/42").unwrap();
        assert_eq!(path.path, "tile/42");
        assert_eq!(path.endpoint_type, EndpointType::Tile);

        let path = parse_path("tile/5/12").unwrap();
        assert_eq!(path.path, "tile/5/12");
        assert_eq!(path.endpoint_type, EndpointType::Tile);
    }

    #[test]
    fn test_leading_slash() {
        let path = parse_path("/players").unwrap();
        assert_eq!(path.path, "players");
    }

    #[test]
    fn test_invalid_integer() {
        let result = parse_path("player/abc");
        assert!(result.is_err());
    }
}
