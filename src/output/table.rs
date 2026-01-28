use crate::path_parser::EndpointType;
use serde_json::Value;
use tabled::{Table, Tabled};

/// Format data as a table based on endpoint type
pub fn format_table(data: &Value, endpoint_type: &EndpointType) -> String {
    match endpoint_type {
        // Table-formatted endpoints
        EndpointType::Players => format_players(data),
        EndpointType::Player => format_player(data),
        EndpointType::PlayerUnits | EndpointType::Units => format_units(data),
        EndpointType::Cities => format_cities(data),
        EndpointType::City => format_city(data),
        EndpointType::Characters => format_characters(data),
        EndpointType::Character => format_character(data),
        EndpointType::Unit => format_unit(data),
        EndpointType::Tiles => format_tiles(data),
        EndpointType::Tile => format_tile(data),
        EndpointType::Tribes => format_tribes(data),
        EndpointType::Tribe => format_tribe(data),
        EndpointType::Config => format_config(data),
        EndpointType::TeamDiplomacy => format_team_diplomacy(data),
        EndpointType::TribeDiplomacy => format_tribe_diplomacy(data),
        EndpointType::CharacterEvents => format_character_events(data),
        EndpointType::UnitEvents => format_unit_events(data),
        EndpointType::CityEvents => format_city_events(data),

        // Additional table-formatted endpoints
        EndpointType::State => format_state(data),
        EndpointType::Map => format_map(data),
        EndpointType::Religions => format_religions(data),
        EndpointType::TeamAlliances => format_team_alliances(data),
        EndpointType::TribeAlliances => format_tribe_alliances(data),
        EndpointType::PlayerTechs => format_player_techs(data),
        EndpointType::PlayerFamilies => format_player_families(data),
        EndpointType::PlayerReligion => format_player_religion(data),
        EndpointType::PlayerGoals => format_player_goals(data),
        EndpointType::PlayerDecisions => format_player_decisions(data),
        EndpointType::PlayerLaws => format_player_laws(data),
        EndpointType::PlayerMissions => format_player_missions(data),
        EndpointType::PlayerResources => format_player_resources(data),
    }
}

#[derive(Tabled)]
struct PlayerRow {
    #[tabled(rename = "Idx")]
    index: i32,
    #[tabled(rename = "Nation")]
    nation: String,
    #[tabled(rename = "Cities")]
    cities: i32,
    #[tabled(rename = "Units")]
    units: i32,
    #[tabled(rename = "Legitimacy")]
    legitimacy: i32,
    #[tabled(rename = "Food")]
    food: i32,
    #[tabled(rename = "Gold")]
    gold: i32,
}

fn format_players(data: &Value) -> String {
    let rows: Vec<PlayerRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|p| {
            Some(PlayerRow {
                index: p.get("index")?.as_i64()? as i32,
                nation: shorten_type(p.get("nation")?.as_str()?),
                cities: p.get("cities")?.as_i64()? as i32,
                units: p.get("units")?.as_i64()? as i32,
                legitimacy: p.get("legitimacy")?.as_i64().unwrap_or(0) as i32,
                food: get_stockpile(p, "YIELD_FOOD"),
                gold: get_stockpile(p, "YIELD_MONEY"),
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No players found".to_string();
    }

    Table::new(rows).to_string()
}

fn format_player(data: &Value) -> String {
    let mut lines = Vec::new();

    if let Some(index) = data.get("index").and_then(|v| v.as_i64()) {
        lines.push(format!("Player {}", index));
    }
    if let Some(nation) = data.get("nation").and_then(|v| v.as_str()) {
        lines.push(format!("  Nation: {}", shorten_type(nation)));
    }
    if let Some(cities) = data.get("cities").and_then(|v| v.as_i64()) {
        lines.push(format!("  Cities: {}", cities));
    }
    if let Some(units) = data.get("units").and_then(|v| v.as_i64()) {
        lines.push(format!("  Units: {}", units));
    }
    if let Some(legitimacy) = data.get("legitimacy").and_then(|v| v.as_i64()) {
        lines.push(format!("  Legitimacy: {}", legitimacy));
    }

    if let Some(stockpiles) = data.get("stockpiles").and_then(|v| v.as_object()) {
        lines.push("  Stockpiles:".to_string());
        for (key, val) in stockpiles {
            if let Some(v) = val.as_i64() {
                lines.push(format!("    {}: {}", shorten_type(key), v));
            }
        }
    }

    if let Some(rates) = data.get("rates").and_then(|v| v.as_object()) {
        lines.push("  Per Turn:".to_string());
        for (key, val) in rates {
            if let Some(v) = val.as_i64() {
                let sign = if v >= 0 { "+" } else { "" };
                lines.push(format!("    {}: {}{}", shorten_type(key), sign, v));
            }
        }
    }

    lines.join("\n")
}

#[derive(Tabled)]
struct UnitRow {
    #[tabled(rename = "ID")]
    id: i32,
    #[tabled(rename = "Type")]
    unit_type: String,
    #[tabled(rename = "Owner")]
    owner: String,
    #[tabled(rename = "HP")]
    hp: String,
    #[tabled(rename = "Pos")]
    pos: String,
    #[tabled(rename = "Status")]
    status: String,
}

fn format_units(data: &Value) -> String {
    let rows: Vec<UnitRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|u| {
            Some(UnitRow {
                id: u.get("id")?.as_i64()? as i32,
                unit_type: shorten_type(u.get("unitType")?.as_str()?),
                owner: u.get("ownerId").and_then(|v| v.as_i64()).map(|v| v.to_string()).unwrap_or("-".to_string()),
                hp: format!("{}/{}",
                    u.get("hp").and_then(|v| v.as_i64()).unwrap_or(0),
                    u.get("hpMax").and_then(|v| v.as_i64()).unwrap_or(0)
                ),
                pos: format!("({},{})",
                    u.get("x").and_then(|v| v.as_i64()).unwrap_or(0),
                    u.get("y").and_then(|v| v.as_i64()).unwrap_or(0)
                ),
                status: get_unit_status(u),
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No units found".to_string();
    }

    Table::new(rows).to_string()
}

fn format_unit(data: &Value) -> String {
    let mut lines = Vec::new();

    if let Some(id) = data.get("id").and_then(|v| v.as_i64()) {
        let unit_type = data.get("unitType").and_then(|v| v.as_str()).unwrap_or("Unknown");
        lines.push(format!("Unit {} - {}", id, shorten_type(unit_type)));
    }

    if let Some(owner) = data.get("ownerId").and_then(|v| v.as_i64()) {
        lines.push(format!("  Owner: Player {}", owner));
    }

    let hp = data.get("hp").and_then(|v| v.as_i64()).unwrap_or(0);
    let hp_max = data.get("hpMax").and_then(|v| v.as_i64()).unwrap_or(0);
    lines.push(format!("  HP: {}/{}", hp, hp_max));

    let x = data.get("x").and_then(|v| v.as_i64()).unwrap_or(0);
    let y = data.get("y").and_then(|v| v.as_i64()).unwrap_or(0);
    let tile_id = data.get("tileId").and_then(|v| v.as_i64()).unwrap_or(0);
    lines.push(format!("  Position: ({}, {}) [tile {}]", x, y, tile_id));

    lines.push(format!("  Status: {}", get_unit_status(data)));

    if let Some(xp) = data.get("xp").and_then(|v| v.as_i64()) {
        let level = data.get("level").and_then(|v| v.as_i64()).unwrap_or(0);
        lines.push(format!("  XP: {} (Level {})", xp, level));
    }

    if let Some(promotions) = data.get("promotions").and_then(|v| v.as_array()) {
        if !promotions.is_empty() {
            let promo_str: Vec<String> = promotions.iter()
                .filter_map(|p| p.as_str().map(shorten_type))
                .collect();
            lines.push(format!("  Promotions: {}", promo_str.join(", ")));
        }
    }

    lines.join("\n")
}

#[derive(Tabled)]
struct CityRow {
    #[tabled(rename = "ID")]
    id: i32,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Owner")]
    owner: String,
    #[tabled(rename = "Pop")]
    citizens: i32,
    #[tabled(rename = "HP")]
    hp: String,
    #[tabled(rename = "Pos")]
    pos: String,
}

fn format_cities(data: &Value) -> String {
    let rows: Vec<CityRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|c| {
            Some(CityRow {
                id: c.get("id")?.as_i64()? as i32,
                name: c.get("name")?.as_str()?.to_string(),
                owner: c.get("ownerId").and_then(|v| v.as_i64()).map(|v| v.to_string()).unwrap_or("-".to_string()),
                citizens: c.get("citizens").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                hp: format!("{}/{}",
                    c.get("hp").and_then(|v| v.as_i64()).unwrap_or(0),
                    c.get("hpMax").and_then(|v| v.as_i64()).unwrap_or(0)
                ),
                pos: format!("({},{})",
                    c.get("x").and_then(|v| v.as_i64()).unwrap_or(0),
                    c.get("y").and_then(|v| v.as_i64()).unwrap_or(0)
                ),
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No cities found".to_string();
    }

    Table::new(rows).to_string()
}

fn format_city(data: &Value) -> String {
    let mut lines = Vec::new();

    let name = data.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let id = data.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    lines.push(format!("{} (ID: {})", name, id));

    if let Some(owner) = data.get("ownerId").and_then(|v| v.as_i64()) {
        lines.push(format!("  Owner: Player {}", owner));
    }

    if let Some(nation) = data.get("nation").and_then(|v| v.as_str()) {
        lines.push(format!("  Nation: {}", shorten_type(nation)));
    }

    let is_capital = data.get("isCapital").and_then(|v| v.as_bool()).unwrap_or(false);
    if is_capital {
        lines.push("  Capital: Yes".to_string());
    }

    let citizens = data.get("citizens").and_then(|v| v.as_i64()).unwrap_or(0);
    lines.push(format!("  Population: {}", citizens));

    let hp = data.get("hp").and_then(|v| v.as_i64()).unwrap_or(0);
    let hp_max = data.get("hpMax").and_then(|v| v.as_i64()).unwrap_or(0);
    lines.push(format!("  HP: {}/{}", hp, hp_max));

    let x = data.get("x").and_then(|v| v.as_i64()).unwrap_or(0);
    let y = data.get("y").and_then(|v| v.as_i64()).unwrap_or(0);
    lines.push(format!("  Position: ({}, {})", x, y));

    if let Some(family) = data.get("family").and_then(|v| v.as_str()) {
        lines.push(format!("  Family: {}", shorten_type(family)));
    }

    if let Some(yields) = data.get("yields").and_then(|v| v.as_object()) {
        lines.push("  Yields:".to_string());
        for (key, val) in yields {
            if let Some(v) = val.as_i64() {
                lines.push(format!("    {}: {}", shorten_type(key), v));
            }
        }
    }

    lines.join("\n")
}

#[derive(Tabled)]
struct CharacterRow {
    #[tabled(rename = "ID")]
    id: i32,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Age")]
    age: i32,
    #[tabled(rename = "Player")]
    player: String,
    #[tabled(rename = "Role")]
    role: String,
}

fn format_characters(data: &Value) -> String {
    let rows: Vec<CharacterRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|c| {
            Some(CharacterRow {
                id: c.get("id")?.as_i64()? as i32,
                name: c.get("name")?.as_str()?.to_string(),
                age: c.get("age").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                player: c.get("playerId").and_then(|v| v.as_i64()).map(|v| v.to_string()).unwrap_or("-".to_string()),
                role: get_character_role(c),
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No characters found".to_string();
    }

    Table::new(rows).to_string()
}

fn format_character(data: &Value) -> String {
    let mut lines = Vec::new();

    let name = data.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let id = data.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    lines.push(format!("{} (ID: {})", name, id));

    if let Some(gender) = data.get("gender").and_then(|v| v.as_str()) {
        let age = data.get("age").and_then(|v| v.as_i64()).unwrap_or(0);
        lines.push(format!("  {} years old, {}", age, gender));
    }

    if let Some(player) = data.get("playerId").and_then(|v| v.as_i64()) {
        lines.push(format!("  Player: {}", player));
    }

    let role = get_character_role(data);
    if role != "-" {
        lines.push(format!("  Role: {}", role));
    }

    if let Some(family) = data.get("family").and_then(|v| v.as_str()) {
        lines.push(format!("  Family: {}", shorten_type(family)));
    }

    // Ratings
    let courage = data.get("ratingCourage").and_then(|v| v.as_i64()).unwrap_or(0);
    let discipline = data.get("ratingDiscipline").and_then(|v| v.as_i64()).unwrap_or(0);
    let charisma = data.get("ratingCharisma").and_then(|v| v.as_i64()).unwrap_or(0);
    let wisdom = data.get("ratingWisdom").and_then(|v| v.as_i64()).unwrap_or(0);
    lines.push(format!("  Ratings: COU {} / DIS {} / CHA {} / WIS {}", courage, discipline, charisma, wisdom));

    if let Some(traits) = data.get("traits").and_then(|v| v.as_array()) {
        if !traits.is_empty() {
            let trait_str: Vec<String> = traits.iter()
                .filter_map(|t| t.as_str().map(shorten_type))
                .collect();
            lines.push(format!("  Traits: {}", trait_str.join(", ")));
        }
    }

    lines.join("\n")
}

#[derive(Tabled)]
struct TileRow {
    #[tabled(rename = "ID")]
    id: i32,
    #[tabled(rename = "Pos")]
    pos: String,
    #[tabled(rename = "Terrain")]
    terrain: String,
    #[tabled(rename = "Resource")]
    resource: String,
    #[tabled(rename = "Improvement")]
    improvement: String,
    #[tabled(rename = "Owner")]
    owner: String,
}

fn format_tiles(data: &Value) -> String {
    let rows: Vec<TileRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|t| {
            Some(TileRow {
                id: t.get("id")?.as_i64()? as i32,
                pos: format!("({},{})",
                    t.get("x").and_then(|v| v.as_i64()).unwrap_or(0),
                    t.get("y").and_then(|v| v.as_i64()).unwrap_or(0)
                ),
                terrain: t.get("terrain").and_then(|v| v.as_str()).map(shorten_type).unwrap_or("-".to_string()),
                resource: t.get("resource").and_then(|v| v.as_str()).map(shorten_type).unwrap_or("-".to_string()),
                improvement: t.get("improvement").and_then(|v| v.as_str()).map(shorten_type).unwrap_or("-".to_string()),
                owner: t.get("ownerId").and_then(|v| v.as_i64()).map(|v| v.to_string()).unwrap_or("-".to_string()),
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No tiles found".to_string();
    }

    Table::new(rows).to_string()
}

fn format_tile(data: &Value) -> String {
    let mut lines = Vec::new();

    let id = data.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
    let x = data.get("x").and_then(|v| v.as_i64()).unwrap_or(0);
    let y = data.get("y").and_then(|v| v.as_i64()).unwrap_or(0);
    lines.push(format!("Tile {} at ({}, {})", id, x, y));

    if let Some(terrain) = data.get("terrain").and_then(|v| v.as_str()) {
        lines.push(format!("  Terrain: {}", shorten_type(terrain)));
    }
    if let Some(height) = data.get("height").and_then(|v| v.as_str()) {
        lines.push(format!("  Height: {}", shorten_type(height)));
    }
    if let Some(vegetation) = data.get("vegetation").and_then(|v| v.as_str()) {
        lines.push(format!("  Vegetation: {}", shorten_type(vegetation)));
    }
    if let Some(resource) = data.get("resource").and_then(|v| v.as_str()) {
        lines.push(format!("  Resource: {}", shorten_type(resource)));
    }
    if let Some(improvement) = data.get("improvement").and_then(|v| v.as_str()) {
        let pillaged = data.get("isPillaged").and_then(|v| v.as_bool()).unwrap_or(false);
        let suffix = if pillaged { " (pillaged)" } else { "" };
        lines.push(format!("  Improvement: {}{}", shorten_type(improvement), suffix));
    }
    if let Some(owner) = data.get("ownerId").and_then(|v| v.as_i64()) {
        lines.push(format!("  Owner: Player {}", owner));
    }
    if let Some(city) = data.get("cityId").and_then(|v| v.as_i64()) {
        lines.push(format!("  City: {}", city));
    }
    let has_road = data.get("hasRoad").and_then(|v| v.as_bool()).unwrap_or(false);
    if has_road {
        lines.push("  Road: Yes".to_string());
    }

    lines.join("\n")
}

#[derive(Tabled)]
struct TribeRow {
    #[tabled(rename = "Type")]
    tribe_type: String,
    #[tabled(rename = "Strength")]
    strength: i32,
    #[tabled(rename = "Units")]
    units: i32,
    #[tabled(rename = "Camps")]
    camps: i32,
    #[tabled(rename = "Settlements")]
    settlements: i32,
}

fn format_tribes(data: &Value) -> String {
    let rows: Vec<TribeRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|t| {
            Some(TribeRow {
                tribe_type: shorten_type(t.get("tribeType")?.as_str()?),
                strength: t.get("strength").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                units: t.get("units").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                camps: t.get("camps").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
                settlements: t.get("settlements").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No tribes found".to_string();
    }

    Table::new(rows).to_string()
}

fn format_tribe(data: &Value) -> String {
    let mut lines = Vec::new();

    if let Some(tribe_type) = data.get("tribeType").and_then(|v| v.as_str()) {
        lines.push(format!("Tribe: {}", shorten_type(tribe_type)));
    }
    if let Some(strength) = data.get("strength").and_then(|v| v.as_i64()) {
        lines.push(format!("  Strength: {}", strength));
    }
    if let Some(units) = data.get("units").and_then(|v| v.as_i64()) {
        lines.push(format!("  Units: {}", units));
    }
    if let Some(camps) = data.get("camps").and_then(|v| v.as_i64()) {
        lines.push(format!("  Camps: {}", camps));
    }
    if let Some(settlements) = data.get("settlements").and_then(|v| v.as_i64()) {
        lines.push(format!("  Settlements: {}", settlements));
    }

    lines.join("\n")
}

fn format_config(data: &Value) -> String {
    let mut lines = Vec::new();

    lines.push("Game Configuration".to_string());
    if let Some(turn) = data.get("turn").and_then(|v| v.as_i64()) {
        lines.push(format!("  Turn: {}", turn));
    }
    if let Some(year) = data.get("year").and_then(|v| v.as_i64()) {
        lines.push(format!("  Year: {}", year));
    }
    if let Some(players) = data.get("numPlayers").and_then(|v| v.as_i64()) {
        lines.push(format!("  Players: {}", players));
    }
    if let Some(teams) = data.get("numTeams").and_then(|v| v.as_i64()) {
        lines.push(format!("  Teams: {}", teams));
    }
    if let Some(tiles) = data.get("numTiles").and_then(|v| v.as_i64()) {
        lines.push(format!("  Tiles: {}", tiles));
    }

    lines.join("\n")
}

#[derive(Tabled)]
struct TeamDiplomacyRow {
    #[tabled(rename = "From")]
    from: i32,
    #[tabled(rename = "To")]
    to: i32,
    #[tabled(rename = "Relation")]
    relation: String,
    #[tabled(rename = "Opinion")]
    opinion: i32,
}

fn format_team_diplomacy(data: &Value) -> String {
    let rows: Vec<TeamDiplomacyRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|d| {
            Some(TeamDiplomacyRow {
                from: d.get("fromTeam")?.as_i64()? as i32,
                to: d.get("toTeam")?.as_i64()? as i32,
                relation: d.get("relation").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                opinion: d.get("opinion").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No diplomacy data found".to_string();
    }

    Table::new(rows).to_string()
}

#[derive(Tabled)]
struct TribeDiplomacyRow {
    #[tabled(rename = "Tribe")]
    tribe: String,
    #[tabled(rename = "Team")]
    team: i32,
    #[tabled(rename = "Relation")]
    relation: String,
}

fn format_tribe_diplomacy(data: &Value) -> String {
    let rows: Vec<TribeDiplomacyRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|d| {
            Some(TribeDiplomacyRow {
                tribe: shorten_type(d.get("tribeType")?.as_str()?),
                team: d.get("team")?.as_i64()? as i32,
                relation: d.get("relation").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No tribe diplomacy data found".to_string();
    }

    Table::new(rows).to_string()
}

#[derive(Tabled)]
struct CharacterEventRow {
    #[tabled(rename = "Turn")]
    turn: i32,
    #[tabled(rename = "Event")]
    event: String,
    #[tabled(rename = "Character")]
    character: String,
}

fn format_character_events(data: &Value) -> String {
    let rows: Vec<CharacterEventRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|e| {
            Some(CharacterEventRow {
                turn: e.get("turn")?.as_i64()? as i32,
                event: e.get("eventType").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                character: e.get("characterName")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| {
                        e.get("characterId")
                            .and_then(|v| v.as_i64())
                            .map(|id| format!("#{}", id))
                            .unwrap_or("-".to_string())
                    }),
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No character events found".to_string();
    }

    Table::new(rows).to_string()
}

#[derive(Tabled)]
struct UnitEventRow {
    #[tabled(rename = "Turn")]
    turn: i32,
    #[tabled(rename = "Event")]
    event: String,
    #[tabled(rename = "Unit")]
    unit: String,
    #[tabled(rename = "Tile")]
    tile: String,
}

fn format_unit_events(data: &Value) -> String {
    let rows: Vec<UnitEventRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|e| {
            Some(UnitEventRow {
                turn: e.get("turn")?.as_i64()? as i32,
                event: e.get("eventType").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                unit: e.get("unitType")
                    .and_then(|v| v.as_str())
                    .map(shorten_type)
                    .unwrap_or_else(|| {
                        e.get("unitId")
                            .and_then(|v| v.as_i64())
                            .map(|id| format!("#{}", id))
                            .unwrap_or("-".to_string())
                    }),
                tile: e.get("tileId")
                    .and_then(|v| v.as_i64())
                    .map(|id| id.to_string())
                    .unwrap_or("-".to_string()),
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No unit events found".to_string();
    }

    Table::new(rows).to_string()
}

#[derive(Tabled)]
struct CityEventRow {
    #[tabled(rename = "Turn")]
    turn: i32,
    #[tabled(rename = "Event")]
    event: String,
    #[tabled(rename = "City")]
    city: String,
    #[tabled(rename = "Player")]
    player: String,
}

fn format_city_events(data: &Value) -> String {
    let rows: Vec<CityEventRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|e| {
            Some(CityEventRow {
                turn: e.get("turn")?.as_i64()? as i32,
                event: e.get("eventType").and_then(|v| v.as_str()).unwrap_or("-").to_string(),
                city: e.get("cityName")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| {
                        e.get("cityId")
                            .and_then(|v| v.as_i64())
                            .map(|id| format!("#{}", id))
                            .unwrap_or("-".to_string())
                    }),
                player: e.get("playerId")
                    .and_then(|v| v.as_i64())
                    .map(|id| id.to_string())
                    .unwrap_or("-".to_string()),
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No city events found".to_string();
    }

    Table::new(rows).to_string()
}

// === State (full game snapshot - show summary) ===

fn format_state(data: &Value) -> String {
    let mut lines = Vec::new();

    lines.push("Game State Summary".to_string());
    lines.push("==================".to_string());

    if let Some(turn) = data.get("turn").and_then(|v| v.as_i64()) {
        lines.push(format!("Turn: {}", turn));
    }
    if let Some(year) = data.get("year").and_then(|v| v.as_i64()) {
        lines.push(format!("Year: {}", year));
    }

    if let Some(players) = data.get("players").and_then(|v| v.as_array()) {
        lines.push(format!("Players: {}", players.len()));
    }
    if let Some(cities) = data.get("cities").and_then(|v| v.as_array()) {
        lines.push(format!("Cities: {}", cities.len()));
    }
    if let Some(units) = data.get("units").and_then(|v| v.as_array()) {
        lines.push(format!("Units: {}", units.len()));
    }
    if let Some(characters) = data.get("characters").and_then(|v| v.as_array()) {
        lines.push(format!("Characters: {}", characters.len()));
    }

    lines.push("".to_string());
    lines.push("Use specific endpoints for details:".to_string());
    lines.push("  players, cities, units, characters".to_string());

    lines.join("\n")
}

// === Map metadata ===

fn format_map(data: &Value) -> String {
    let mut lines = Vec::new();

    lines.push("Map Information".to_string());
    if let Some(num_tiles) = data.get("numTiles").and_then(|v| v.as_i64()) {
        lines.push(format!("  Total Tiles: {}", num_tiles));
    }

    lines.join("\n")
}

// === Religions ===

#[derive(Tabled)]
struct ReligionRow {
    #[tabled(rename = "Religion")]
    religion: String,
    #[tabled(rename = "Founded")]
    founded: String,
    #[tabled(rename = "Holy City")]
    holy_city: String,
}

fn format_religions(data: &Value) -> String {
    let rows: Vec<ReligionRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|r| {
            Some(ReligionRow {
                religion: shorten_type(r.get("religionType")?.as_str()?),
                founded: r.get("isFounded")
                    .and_then(|v| v.as_bool())
                    .map(|b| if b { "Yes" } else { "No" })
                    .unwrap_or("-")
                    .to_string(),
                holy_city: r.get("holyCityId")
                    .and_then(|v| v.as_i64())
                    .map(|id| id.to_string())
                    .unwrap_or("-".to_string()),
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No religions found".to_string();
    }

    Table::new(rows).to_string()
}

// === Team Alliances ===

#[derive(Tabled)]
struct TeamAllianceRow {
    #[tabled(rename = "Team A")]
    team_a: i32,
    #[tabled(rename = "Team B")]
    team_b: i32,
}

fn format_team_alliances(data: &Value) -> String {
    let rows: Vec<TeamAllianceRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|a| {
            Some(TeamAllianceRow {
                team_a: a.get("teamA")?.as_i64()? as i32,
                team_b: a.get("teamB")?.as_i64()? as i32,
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No team alliances found".to_string();
    }

    Table::new(rows).to_string()
}

// === Tribe Alliances ===

#[derive(Tabled)]
struct TribeAllianceRow {
    #[tabled(rename = "Tribe")]
    tribe: String,
    #[tabled(rename = "Player")]
    player: i32,
}

fn format_tribe_alliances(data: &Value) -> String {
    let rows: Vec<TribeAllianceRow> = match data.as_array() {
        Some(arr) => arr.iter().filter_map(|a| {
            Some(TribeAllianceRow {
                tribe: shorten_type(a.get("tribeType")?.as_str()?),
                player: a.get("playerIndex")?.as_i64()? as i32,
            })
        }).collect(),
        None => return format_generic(data),
    };

    if rows.is_empty() {
        return "No tribe alliances found".to_string();
    }

    Table::new(rows).to_string()
}

// === Player Techs ===

fn format_player_techs(data: &Value) -> String {
    let mut lines = Vec::new();

    lines.push("Research Status".to_string());

    // Currently researching
    if let Some(researching) = data.get("researching").and_then(|v| v.as_str()) {
        let progress_map = data.get("progress").and_then(|v| v.as_object());
        let progress = progress_map
            .and_then(|m| m.get(researching))
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        lines.push(format!("  Currently: {} ({} progress)", shorten_type(researching), progress));
    } else {
        lines.push("  Currently: None".to_string());
    }

    lines.push("".to_string());

    // Available techs
    if let Some(available) = data.get("available").and_then(|v| v.as_array()) {
        lines.push("Available:".to_string());
        if available.is_empty() {
            lines.push("  (none)".to_string());
        } else {
            for tech in available {
                if let Some(t) = tech.as_str() {
                    lines.push(format!("  - {}", shorten_type(t)));
                }
            }
        }
    }

    lines.push("".to_string());

    // Researched techs
    if let Some(researched) = data.get("researched").and_then(|v| v.as_array()) {
        lines.push("Researched:".to_string());
        if researched.is_empty() {
            lines.push("  (none)".to_string());
        } else {
            for tech in researched {
                if let Some(t) = tech.as_str() {
                    lines.push(format!("  - {}", shorten_type(t)));
                }
            }
        }
    }

    lines.join("\n")
}

// === Player Families ===

#[derive(Tabled)]
struct FamilyRow {
    #[tabled(rename = "Family")]
    family: String,
    #[tabled(rename = "Opinion Rate")]
    opinion_rate: i32,
}

fn format_player_families(data: &Value) -> String {
    // API returns {"families": [...]}
    let families_data = data.get("families").and_then(|v| v.as_array());

    let rows: Vec<FamilyRow> = match families_data {
        Some(arr) => arr.iter().filter_map(|f| {
            Some(FamilyRow {
                family: shorten_type(f.get("family")?.as_str()?),
                opinion_rate: f.get("opinionRate").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
            })
        }).collect(),
        None => return "No families found".to_string(),
    };

    if rows.is_empty() {
        return "No families found".to_string();
    }

    Table::new(rows).to_string()
}

// === Player Religion ===

fn format_player_religion(data: &Value) -> String {
    let mut lines = Vec::new();

    lines.push("Player Religion".to_string());

    // State religion (may be null)
    if let Some(state_religion) = data.get("stateReligion").and_then(|v| v.as_str()) {
        lines.push(format!("  State Religion: {}", shorten_type(state_religion)));
    } else {
        lines.push("  State Religion: None".to_string());
    }

    // Religion counts
    if let Some(counts) = data.get("religionCounts").and_then(|v| v.as_object()) {
        if !counts.is_empty() {
            lines.push("  Followers:".to_string());
            for (religion, count) in counts {
                if let Some(c) = count.as_i64() {
                    lines.push(format!("    {}: {}", shorten_type(religion), c));
                }
            }
        }
    }

    lines.join("\n")
}

// === Player Goals ===

fn format_player_goals(data: &Value) -> String {
    let mut lines = Vec::new();

    lines.push("Player Goals".to_string());

    // Check for note (API limitation message)
    if let Some(note) = data.get("note").and_then(|v| v.as_str()) {
        lines.push(format!("  Note: {}", note));
    }

    // Format any goals if present
    if let Some(goals) = data.get("goals").and_then(|v| v.as_array()) {
        if !goals.is_empty() {
            for goal in goals {
                if let Some(goal_type) = goal.get("goalType").and_then(|v| v.as_str()) {
                    lines.push(format!("  - {}", shorten_type(goal_type)));
                }
            }
        } else if data.get("note").is_none() {
            lines.push("  No goals".to_string());
        }
    }

    lines.join("\n")
}

// === Player Decisions ===

fn format_player_decisions(data: &Value) -> String {
    let mut lines = Vec::new();

    lines.push("Player Decisions".to_string());

    // Check for note (API limitation message)
    if let Some(note) = data.get("note").and_then(|v| v.as_str()) {
        lines.push(format!("  Note: {}", note));
    }

    // Format any decisions if present
    if let Some(decisions) = data.get("decisions").and_then(|v| v.as_array()) {
        if !decisions.is_empty() {
            for decision in decisions {
                if let Some(decision_type) = decision.get("decisionType").and_then(|v| v.as_str()) {
                    lines.push(format!("  - {}", shorten_type(decision_type)));
                }
            }
        } else if data.get("note").is_none() {
            lines.push("  No pending decisions".to_string());
        }
    }

    lines.join("\n")
}

// === Player Laws ===

fn format_player_laws(data: &Value) -> String {
    let mut lines = Vec::new();

    lines.push("Player Laws".to_string());

    // Check for note (API limitation message)
    if let Some(note) = data.get("note").and_then(|v| v.as_str()) {
        lines.push(format!("  Note: {}", note));
    }

    // Format active laws if present
    if let Some(active_laws) = data.get("activeLaws").and_then(|v| v.as_object()) {
        if !active_laws.is_empty() {
            for (law, value) in active_laws {
                lines.push(format!("  - {}: {}", shorten_type(law), value));
            }
        } else if data.get("note").is_none() {
            lines.push("  No active laws".to_string());
        }
    }

    lines.join("\n")
}

// === Player Missions ===

fn format_player_missions(data: &Value) -> String {
    let mut lines = Vec::new();

    lines.push("Player Missions".to_string());

    // Check for note (API limitation message)
    if let Some(note) = data.get("note").and_then(|v| v.as_str()) {
        lines.push(format!("  Note: {}", note));
    }

    // Format any missions if present
    if let Some(missions) = data.get("missions").and_then(|v| v.as_array()) {
        if !missions.is_empty() {
            for mission in missions {
                if let Some(mission_type) = mission.get("missionType").and_then(|v| v.as_str()) {
                    lines.push(format!("  - {}", shorten_type(mission_type)));
                }
            }
        } else if data.get("note").is_none() {
            lines.push("  No active missions".to_string());
        }
    }

    lines.join("\n")
}

// === Player Resources ===

fn format_player_resources(data: &Value) -> String {
    let mut lines = Vec::new();

    lines.push("Player Resources".to_string());

    // Check for note (API limitation message)
    if let Some(note) = data.get("note").and_then(|v| v.as_str()) {
        lines.push(format!("  Note: {}", note));
    }

    // Format resources if present
    if let Some(resources) = data.get("resources").and_then(|v| v.as_object()) {
        if !resources.is_empty() {
            lines.push("  Resources:".to_string());
            for (resource, amount) in resources {
                if let Some(a) = amount.as_i64() {
                    lines.push(format!("    {}: {}", shorten_type(resource), a));
                }
            }
        }
    }

    // Format luxuries if present
    if let Some(luxuries) = data.get("luxuries").and_then(|v| v.as_object()) {
        if !luxuries.is_empty() {
            lines.push("  Luxuries:".to_string());
            for (luxury, amount) in luxuries {
                if let Some(a) = amount.as_i64() {
                    lines.push(format!("    {}: {}", shorten_type(luxury), a));
                }
            }
        }
    }

    lines.join("\n")
}

/// Generic fallback formatting for unknown types
fn format_generic(data: &Value) -> String {
    serde_json::to_string_pretty(data).unwrap_or_else(|_| data.to_string())
}

/// Shorten game type strings (e.g., "NATION_ROME" -> "Rome")
fn shorten_type(s: &str) -> String {
    s.split('_')
        .skip(1)
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars.map(|c| c.to_lowercase().next().unwrap_or(c))).collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn get_stockpile(player: &Value, yield_type: &str) -> i32 {
    player.get("stockpiles")
        .and_then(|s| s.get(yield_type))
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32
}

fn get_unit_status(unit: &Value) -> String {
    if unit.get("isSleep").and_then(|v| v.as_bool()).unwrap_or(false) {
        return "Sleep".to_string();
    }
    if unit.get("isSentry").and_then(|v| v.as_bool()).unwrap_or(false) {
        return "Sentry".to_string();
    }
    if unit.get("isPass").and_then(|v| v.as_bool()).unwrap_or(false) {
        return "Pass".to_string();
    }
    let fortify = unit.get("fortifyTurns").and_then(|v| v.as_i64()).unwrap_or(0);
    if fortify > 0 {
        return format!("Fortified ({})", fortify);
    }
    "Active".to_string()
}

fn get_character_role(character: &Value) -> String {
    let is_leader = character.get("isLeader").and_then(|v| v.as_bool()).unwrap_or(false);
    let is_heir = character.get("isHeir").and_then(|v| v.as_bool()).unwrap_or(false);

    if is_leader {
        "Leader".to_string()
    } else if is_heir {
        "Heir".to_string()
    } else {
        "-".to_string()
    }
}
