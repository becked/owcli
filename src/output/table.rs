use crate::client::types;
use tabled::{Table, Tabled};

// === Players ===

#[derive(Tabled)]
struct PlayerRow {
    #[tabled(rename = "Idx")]
    index: i64,
    #[tabled(rename = "Nation")]
    nation: String,
    #[tabled(rename = "Cities")]
    cities: i64,
    #[tabled(rename = "Units")]
    units: i64,
    #[tabled(rename = "Legitimacy")]
    legitimacy: i64,
    #[tabled(rename = "Food")]
    food: i64,
    #[tabled(rename = "Gold")]
    gold: i64,
}

pub fn format_players(players: &[types::Player]) -> String {
    if players.is_empty() {
        return "No players found".to_string();
    }

    let rows: Vec<PlayerRow> = players
        .iter()
        .map(|p| PlayerRow {
            index: p.index.unwrap_or(0),
            nation: p.nation.as_deref().map(shorten_type).unwrap_or_default(),
            cities: p.cities.unwrap_or(0),
            units: p.units.unwrap_or(0),
            legitimacy: p.legitimacy.unwrap_or(0),
            food: *p.stockpiles.get("YIELD_FOOD").unwrap_or(&0),
            gold: *p.stockpiles.get("YIELD_MONEY").unwrap_or(&0),
        })
        .collect();

    Table::new(rows).to_string()
}

pub fn format_player(player: &types::Player) -> String {
    let mut lines = Vec::new();

    if let Some(index) = player.index {
        lines.push(format!("Player {}", index));
    }
    if let Some(nation) = &player.nation {
        lines.push(format!("  Nation: {}", shorten_type(nation)));
    }
    if let Some(cities) = player.cities {
        lines.push(format!("  Cities: {}", cities));
    }
    if let Some(units) = player.units {
        lines.push(format!("  Units: {}", units));
    }
    if let Some(legitimacy) = player.legitimacy {
        lines.push(format!("  Legitimacy: {}", legitimacy));
    }

    if !player.stockpiles.is_empty() {
        lines.push("  Stockpiles:".to_string());
        for (key, val) in &player.stockpiles {
            lines.push(format!("    {}: {}", shorten_type(key), val));
        }
    }

    if !player.rates.is_empty() {
        lines.push("  Per Turn:".to_string());
        for (key, val) in &player.rates {
            let sign = if *val >= 0 { "+" } else { "" };
            lines.push(format!("    {}: {}{}", shorten_type(key), sign, val));
        }
    }

    lines.join("\n")
}

// === Units ===

#[derive(Tabled)]
struct UnitRow {
    #[tabled(rename = "ID")]
    id: i64,
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

pub fn format_units(units: &[types::Unit]) -> String {
    if units.is_empty() {
        return "No units found".to_string();
    }

    let rows: Vec<UnitRow> = units
        .iter()
        .map(|u| UnitRow {
            id: u.id.unwrap_or(0),
            unit_type: u.unit_type.as_deref().map(shorten_type).unwrap_or_default(),
            owner: u.owner_id.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
            hp: format!("{}/{}", u.hp.unwrap_or(0), u.hp_max.unwrap_or(0)),
            pos: format!("({},{})", u.x.unwrap_or(0), u.y.unwrap_or(0)),
            status: get_unit_status(u),
        })
        .collect();

    Table::new(rows).to_string()
}

pub fn format_unit(unit: &types::Unit) -> String {
    let mut lines = Vec::new();

    let id = unit.id.unwrap_or(0);
    let unit_type = unit.unit_type.as_deref().map(shorten_type).unwrap_or_else(|| "Unknown".to_string());
    lines.push(format!("Unit {} - {}", id, unit_type));

    if let Some(owner) = unit.owner_id {
        lines.push(format!("  Owner: Player {}", owner));
    }

    lines.push(format!("  HP: {}/{}", unit.hp.unwrap_or(0), unit.hp_max.unwrap_or(0)));

    lines.push(format!(
        "  Position: ({}, {}) [tile {}]",
        unit.x.unwrap_or(0),
        unit.y.unwrap_or(0),
        unit.tile_id.unwrap_or(0)
    ));

    lines.push(format!("  Status: {}", get_unit_status(unit)));

    if let Some(xp) = unit.xp {
        lines.push(format!("  XP: {} (Level {})", xp, unit.level.unwrap_or(0)));
    }

    if !unit.promotions.is_empty() {
        let promo_str: Vec<String> = unit.promotions.iter().map(|p| shorten_type(p)).collect();
        lines.push(format!("  Promotions: {}", promo_str.join(", ")));
    }

    lines.join("\n")
}

fn get_unit_status(unit: &types::Unit) -> String {
    if unit.is_sleep.unwrap_or(false) {
        return "Sleep".to_string();
    }
    if unit.is_sentry.unwrap_or(false) {
        return "Sentry".to_string();
    }
    if unit.is_pass.unwrap_or(false) {
        return "Pass".to_string();
    }
    let fortify = unit.fortify_turns.unwrap_or(0);
    if fortify > 0 {
        return format!("Fortified ({})", fortify);
    }
    "Active".to_string()
}

// === Cities ===

#[derive(Tabled)]
struct CityRow {
    #[tabled(rename = "ID")]
    id: i64,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Owner")]
    owner: String,
    #[tabled(rename = "Pop")]
    citizens: i64,
    #[tabled(rename = "HP")]
    hp: String,
    #[tabled(rename = "Pos")]
    pos: String,
}

pub fn format_cities(cities: &[types::City]) -> String {
    if cities.is_empty() {
        return "No cities found".to_string();
    }

    let rows: Vec<CityRow> = cities
        .iter()
        .map(|c| CityRow {
            id: c.id.unwrap_or(0),
            name: c.name.clone().unwrap_or_default(),
            owner: c.owner_id.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
            citizens: c.citizens.unwrap_or(0),
            hp: format!("{}/{}", c.hp.unwrap_or(0), c.hp_max.unwrap_or(0)),
            pos: format!("({},{})", c.x.unwrap_or(0), c.y.unwrap_or(0)),
        })
        .collect();

    Table::new(rows).to_string()
}

pub fn format_city(city: &types::City) -> String {
    let mut lines = Vec::new();

    let name = city.name.as_deref().unwrap_or("Unknown");
    let id = city.id.unwrap_or(0);
    lines.push(format!("{} (ID: {})", name, id));

    if let Some(owner) = city.owner_id {
        lines.push(format!("  Owner: Player {}", owner));
    }

    if let Some(nation) = &city.nation {
        lines.push(format!("  Nation: {}", shorten_type(nation)));
    }

    if city.is_capital.unwrap_or(false) {
        lines.push("  Capital: Yes".to_string());
    }

    lines.push(format!("  Population: {}", city.citizens.unwrap_or(0)));
    lines.push(format!("  HP: {}/{}", city.hp.unwrap_or(0), city.hp_max.unwrap_or(0)));
    lines.push(format!("  Position: ({}, {})", city.x.unwrap_or(0), city.y.unwrap_or(0)));

    if !city.yields.is_empty() {
        lines.push("  Yields per turn:".to_string());
        for (key, val) in &city.yields {
            if let Some(per_turn) = val.per_turn {
                lines.push(format!("    {}: {}", shorten_type(key), per_turn));
            }
        }
    }

    lines.join("\n")
}

// === Characters ===

#[derive(Tabled)]
struct CharacterRow {
    #[tabled(rename = "ID")]
    id: i64,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Age")]
    age: i64,
    #[tabled(rename = "Player")]
    player: String,
    #[tabled(rename = "Role")]
    role: String,
}

pub fn format_characters(characters: &[types::Character]) -> String {
    if characters.is_empty() {
        return "No characters found".to_string();
    }

    let rows: Vec<CharacterRow> = characters
        .iter()
        .map(|c| CharacterRow {
            id: c.id.unwrap_or(0),
            name: c.name.clone().unwrap_or_default(),
            age: c.age.unwrap_or(0),
            player: c.player_id.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
            role: get_character_role(c),
        })
        .collect();

    Table::new(rows).to_string()
}

pub fn format_character(character: &types::Character) -> String {
    let mut lines = Vec::new();

    let name = character.name.as_deref().unwrap_or("Unknown");
    let id = character.id.unwrap_or(0);
    lines.push(format!("{} (ID: {})", name, id));

    if let Some(gender) = &character.gender {
        let age = character.age.unwrap_or(0);
        lines.push(format!("  {} years old, {:?}", age, gender));
    }

    if let Some(player) = character.player_id {
        lines.push(format!("  Player: {}", player));
    }

    let role = get_character_role(character);
    if role != "-" {
        lines.push(format!("  Role: {}", role));
    }

    if let Some(nation) = &character.nation {
        lines.push(format!("  Nation: {}", shorten_type(nation)));
    }

    // Ratings from hashmap
    if !character.ratings.is_empty() {
        let courage = character.ratings.get("RATING_COURAGE").copied().unwrap_or(0);
        let discipline = character.ratings.get("RATING_DISCIPLINE").copied().unwrap_or(0);
        let charisma = character.ratings.get("RATING_CHARISMA").copied().unwrap_or(0);
        let wisdom = character.ratings.get("RATING_WISDOM").copied().unwrap_or(0);
        lines.push(format!(
            "  Ratings: COU {} / DIS {} / CHA {} / WIS {}",
            courage, discipline, charisma, wisdom
        ));
    }

    if !character.traits.is_empty() {
        let trait_str: Vec<String> = character.traits.iter().map(|t| shorten_type(t)).collect();
        lines.push(format!("  Traits: {}", trait_str.join(", ")));
    }

    lines.join("\n")
}

fn get_character_role(character: &types::Character) -> String {
    if character.is_leader.unwrap_or(false) {
        "Leader".to_string()
    } else if character.is_heir.unwrap_or(false) {
        "Heir".to_string()
    } else {
        "-".to_string()
    }
}

// === Tiles ===

#[derive(Tabled)]
struct TileRow {
    #[tabled(rename = "ID")]
    id: i64,
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

pub fn format_tiles(tiles: &[types::Tile]) -> String {
    if tiles.is_empty() {
        return "No tiles found".to_string();
    }

    let rows: Vec<TileRow> = tiles
        .iter()
        .map(|t| TileRow {
            id: t.id.unwrap_or(0),
            pos: format!("({},{})", t.x.unwrap_or(0), t.y.unwrap_or(0)),
            terrain: t.terrain.as_deref().map(shorten_type).unwrap_or_else(|| "-".to_string()),
            resource: t.resource.as_deref().map(shorten_type).unwrap_or_else(|| "-".to_string()),
            improvement: t.improvement.as_deref().map(shorten_type).unwrap_or_else(|| "-".to_string()),
            owner: t.owner_id.map(|v| v.to_string()).unwrap_or_else(|| "-".to_string()),
        })
        .collect();

    Table::new(rows).to_string()
}

pub fn format_tile(tile: &types::Tile) -> String {
    let mut lines = Vec::new();

    lines.push(format!(
        "Tile {} at ({}, {})",
        tile.id.unwrap_or(0),
        tile.x.unwrap_or(0),
        tile.y.unwrap_or(0)
    ));

    if let Some(terrain) = &tile.terrain {
        lines.push(format!("  Terrain: {}", shorten_type(terrain)));
    }
    if let Some(height) = &tile.height {
        lines.push(format!("  Height: {}", shorten_type(height)));
    }
    if let Some(vegetation) = &tile.vegetation {
        lines.push(format!("  Vegetation: {}", shorten_type(vegetation)));
    }
    if let Some(resource) = &tile.resource {
        lines.push(format!("  Resource: {}", shorten_type(resource)));
    }
    if let Some(improvement) = &tile.improvement {
        let pillaged = tile.is_pillaged.unwrap_or(false);
        let suffix = if pillaged { " (pillaged)" } else { "" };
        lines.push(format!("  Improvement: {}{}", shorten_type(improvement), suffix));
    }
    if let Some(owner) = tile.owner_id {
        lines.push(format!("  Owner: Player {}", owner));
    }
    if let Some(city) = tile.city_id {
        lines.push(format!("  City: {}", city));
    }

    lines.join("\n")
}

// === Tribes ===

#[derive(Tabled)]
struct TribeRow {
    #[tabled(rename = "Type")]
    tribe_type: String,
    #[tabled(rename = "Strength")]
    strength: i64,
    #[tabled(rename = "Units")]
    num_units: i64,
    #[tabled(rename = "Cities")]
    num_cities: i64,
}

pub fn format_tribes(tribes: &[types::Tribe]) -> String {
    if tribes.is_empty() {
        return "No tribes found".to_string();
    }

    let rows: Vec<TribeRow> = tribes
        .iter()
        .map(|t| TribeRow {
            tribe_type: t.tribe_type.as_deref().map(shorten_type).unwrap_or_default(),
            strength: t.strength.unwrap_or(0),
            num_units: t.num_units.unwrap_or(0),
            num_cities: t.num_cities.unwrap_or(0),
        })
        .collect();

    Table::new(rows).to_string()
}

pub fn format_tribe(tribe: &types::Tribe) -> String {
    let mut lines = Vec::new();

    if let Some(tribe_type) = &tribe.tribe_type {
        lines.push(format!("Tribe: {}", shorten_type(tribe_type)));
    }
    if let Some(strength) = tribe.strength {
        lines.push(format!("  Strength: {}", strength));
    }
    if let Some(num_units) = tribe.num_units {
        lines.push(format!("  Units: {}", num_units));
    }
    if let Some(num_cities) = tribe.num_cities {
        lines.push(format!("  Cities: {}", num_cities));
    }
    if tribe.is_alive.unwrap_or(false) {
        lines.push("  Status: Alive".to_string());
    }

    lines.join("\n")
}

// === Config ===

pub fn format_config(config: &types::GameConfig) -> String {
    let mut lines = Vec::new();

    lines.push("Game Configuration".to_string());
    if let Some(turn) = config.turn {
        lines.push(format!("  Turn: {}", turn));
    }
    if let Some(year) = config.year {
        lines.push(format!("  Year: {}", year));
    }
    if let Some(players) = config.num_players {
        lines.push(format!("  Players: {}", players));
    }
    if let Some(teams) = config.num_teams {
        lines.push(format!("  Teams: {}", teams));
    }
    if let Some(tiles) = config.num_tiles {
        lines.push(format!("  Tiles: {}", tiles));
    }

    lines.join("\n")
}

// === Diplomacy ===

#[derive(Tabled)]
struct TeamDiplomacyRow {
    #[tabled(rename = "From")]
    from: i64,
    #[tabled(rename = "To")]
    to: i64,
    #[tabled(rename = "Diplomacy")]
    diplomacy: String,
    #[tabled(rename = "War Score")]
    war_score: i64,
}

pub fn format_team_diplomacy(diplomacy: &[types::TeamDiplomacy]) -> String {
    if diplomacy.is_empty() {
        return "No diplomacy data found".to_string();
    }

    let rows: Vec<TeamDiplomacyRow> = diplomacy
        .iter()
        .map(|d| TeamDiplomacyRow {
            from: d.from_team.unwrap_or(0),
            to: d.to_team.unwrap_or(0),
            diplomacy: d.diplomacy.clone().unwrap_or_else(|| "-".to_string()),
            war_score: d.war_score.unwrap_or(0),
        })
        .collect();

    Table::new(rows).to_string()
}

#[derive(Tabled)]
struct TribeDiplomacyRow {
    #[tabled(rename = "Tribe")]
    tribe: String,
    #[tabled(rename = "Team")]
    to_team: i64,
    #[tabled(rename = "Diplomacy")]
    diplomacy: String,
}

pub fn format_tribe_diplomacy(diplomacy: &[types::TribeDiplomacy]) -> String {
    if diplomacy.is_empty() {
        return "No tribe diplomacy data found".to_string();
    }

    let rows: Vec<TribeDiplomacyRow> = diplomacy
        .iter()
        .map(|d| TribeDiplomacyRow {
            tribe: d.tribe.as_deref().map(shorten_type).unwrap_or_default(),
            to_team: d.to_team.unwrap_or(0),
            diplomacy: d.diplomacy.clone().unwrap_or_else(|| "-".to_string()),
        })
        .collect();

    Table::new(rows).to_string()
}

// === Events ===

#[derive(Tabled)]
struct CharacterEventRow {
    #[tabled(rename = "Event")]
    event: String,
    #[tabled(rename = "Character")]
    character: String,
    #[tabled(rename = "Player")]
    player: String,
}

pub fn format_character_events(events: &[types::CharacterEvent]) -> String {
    if events.is_empty() {
        return "No character events found".to_string();
    }

    let rows: Vec<CharacterEventRow> = events
        .iter()
        .map(|e| CharacterEventRow {
            event: e.event_type.as_ref().map(|t| format!("{:?}", t)).unwrap_or_else(|| "-".to_string()),
            character: e.character_id.map(|id| format!("#{}", id)).unwrap_or_else(|| "-".to_string()),
            player: e.player_id.map(|id| id.to_string()).unwrap_or_else(|| "-".to_string()),
        })
        .collect();

    Table::new(rows).to_string()
}

#[derive(Tabled)]
struct UnitEventRow {
    #[tabled(rename = "Event")]
    event: String,
    #[tabled(rename = "Unit")]
    unit: String,
    #[tabled(rename = "Location")]
    location: String,
}

pub fn format_unit_events(events: &[types::UnitEvent]) -> String {
    if events.is_empty() {
        return "No unit events found".to_string();
    }

    let rows: Vec<UnitEventRow> = events
        .iter()
        .map(|e| UnitEventRow {
            event: e.event_type.as_ref().map(|t| format!("{:?}", t)).unwrap_or_else(|| "-".to_string()),
            unit: e
                .unit_type
                .as_deref()
                .map(shorten_type)
                .unwrap_or_else(|| e.unit_id.map(|id| format!("#{}", id)).unwrap_or_else(|| "-".to_string())),
            location: e.location.as_ref()
                .map(|loc| format!("({},{})", loc.x.unwrap_or(0), loc.y.unwrap_or(0)))
                .unwrap_or_else(|| "-".to_string()),
        })
        .collect();

    Table::new(rows).to_string()
}

#[derive(Tabled)]
struct CityEventRow {
    #[tabled(rename = "Event")]
    event: String,
    #[tabled(rename = "City")]
    city: String,
    #[tabled(rename = "Location")]
    location: String,
}

pub fn format_city_events(events: &[types::CityEvent]) -> String {
    if events.is_empty() {
        return "No city events found".to_string();
    }

    let rows: Vec<CityEventRow> = events
        .iter()
        .map(|e| CityEventRow {
            event: e.event_type.as_ref().map(|t| format!("{:?}", t)).unwrap_or_else(|| "-".to_string()),
            city: e
                .city_name
                .clone()
                .unwrap_or_else(|| e.city_id.map(|id| format!("#{}", id)).unwrap_or_else(|| "-".to_string())),
            location: e.location.as_ref()
                .map(|loc| format!("({},{})", loc.x.unwrap_or(0), loc.y.unwrap_or(0)))
                .unwrap_or_else(|| "-".to_string()),
        })
        .collect();

    Table::new(rows).to_string()
}

// === State (full game snapshot - show summary) ===

pub fn format_state(state: &types::GameState) -> String {
    let mut lines = Vec::new();

    lines.push("Game State Summary".to_string());
    lines.push("==================".to_string());

    if let Some(turn) = state.turn {
        lines.push(format!("Turn: {}", turn));
    }
    if let Some(year) = state.year {
        lines.push(format!("Year: {}", year));
    }

    lines.push(format!("Players: {}", state.players.len()));
    lines.push(format!("Cities: {}", state.cities.len()));
    lines.push(format!("Tribes: {}", state.tribes.len()));
    lines.push(format!("Characters: {}", state.characters.len()));

    lines.push("".to_string());
    lines.push("Use specific endpoints for details:".to_string());
    lines.push("  players, cities, units, characters".to_string());

    lines.join("\n")
}

// === Map metadata ===

pub fn format_map(map: &types::MapMetadata) -> String {
    let mut lines = Vec::new();

    lines.push("Map Information".to_string());
    if let Some(num_tiles) = map.num_tiles {
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

pub fn format_religions(religions: &[types::Religion]) -> String {
    if religions.is_empty() {
        return "No religions found".to_string();
    }

    let rows: Vec<ReligionRow> = religions
        .iter()
        .map(|r| ReligionRow {
            religion: r.religion_type.as_deref().map(shorten_type).unwrap_or_default(),
            founded: if r.is_founded.unwrap_or(false) { "Yes" } else { "No" }.to_string(),
            holy_city: r.holy_city_id.map(|id| id.to_string()).unwrap_or_else(|| "-".to_string()),
        })
        .collect();

    Table::new(rows).to_string()
}

// === Team Alliances ===

#[derive(Tabled)]
struct TeamAllianceRow {
    #[tabled(rename = "Team")]
    team: i64,
    #[tabled(rename = "Ally Team")]
    ally_team: i64,
}

pub fn format_team_alliances(alliances: &[types::TeamAlliance]) -> String {
    if alliances.is_empty() {
        return "No team alliances found".to_string();
    }

    let rows: Vec<TeamAllianceRow> = alliances
        .iter()
        .map(|a| TeamAllianceRow {
            team: a.team.unwrap_or(0),
            ally_team: a.ally_team.unwrap_or(0),
        })
        .collect();

    Table::new(rows).to_string()
}

// === Tribe Alliances ===

#[derive(Tabled)]
struct TribeAllianceRow {
    #[tabled(rename = "Tribe")]
    tribe: String,
    #[tabled(rename = "Ally Player")]
    ally_player: i64,
    #[tabled(rename = "Ally Team")]
    ally_team: i64,
}

pub fn format_tribe_alliances(alliances: &[types::TribeAlliance]) -> String {
    if alliances.is_empty() {
        return "No tribe alliances found".to_string();
    }

    let rows: Vec<TribeAllianceRow> = alliances
        .iter()
        .map(|a| TribeAllianceRow {
            tribe: a.tribe.as_deref().map(shorten_type).unwrap_or_default(),
            ally_player: a.ally_player_id.unwrap_or(0),
            ally_team: a.ally_team.unwrap_or(0),
        })
        .collect();

    Table::new(rows).to_string()
}

// === Player Techs ===

pub fn format_player_techs(techs: &types::PlayerTechs) -> String {
    let mut lines = Vec::new();

    lines.push("Research Status".to_string());

    // Currently researching
    if let Some(researching) = &techs.researching {
        let progress = techs.progress.get(researching).copied().unwrap_or(0);
        lines.push(format!("  Currently: {} ({} progress)", shorten_type(researching), progress));
    } else {
        lines.push("  Currently: None".to_string());
    }

    lines.push("".to_string());

    // Available techs
    lines.push("Available:".to_string());
    if techs.available.is_empty() {
        lines.push("  (none)".to_string());
    } else {
        for tech in &techs.available {
            lines.push(format!("  - {}", shorten_type(tech)));
        }
    }

    lines.push("".to_string());

    // Researched techs
    lines.push("Researched:".to_string());
    if techs.researched.is_empty() {
        lines.push("  (none)".to_string());
    } else {
        for tech in &techs.researched {
            lines.push(format!("  - {}", shorten_type(tech)));
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
    opinion_rate: i64,
}

pub fn format_player_families(families: &types::PlayerFamilies) -> String {
    if families.families.is_empty() {
        return "No families found".to_string();
    }

    let rows: Vec<FamilyRow> = families
        .families
        .iter()
        .map(|f| FamilyRow {
            family: f.family.as_deref().map(shorten_type).unwrap_or_default(),
            opinion_rate: f.opinion_rate.unwrap_or(0),
        })
        .collect();

    Table::new(rows).to_string()
}

// === Player Religion ===

pub fn format_player_religion(religion: &types::PlayerReligion) -> String {
    let mut lines = Vec::new();

    lines.push("Player Religion".to_string());

    if let Some(state_religion) = &religion.state_religion {
        lines.push(format!("  State Religion: {}", shorten_type(state_religion)));
    } else {
        lines.push("  State Religion: None".to_string());
    }

    if !religion.religion_counts.is_empty() {
        lines.push("  Followers:".to_string());
        for (rel, count) in &religion.religion_counts {
            lines.push(format!("    {}: {}", shorten_type(rel), count));
        }
    }

    lines.join("\n")
}

// === Player Goals ===

pub fn format_player_goals(goals: &types::PlayerGoals) -> String {
    let mut lines = Vec::new();

    lines.push("Player Goals".to_string());

    if let Some(note) = &goals.note {
        lines.push(format!("  Note: {}", note));
    }

    if goals.goals.is_empty() && goals.note.is_none() {
        lines.push("  No goals".to_string());
    }

    lines.join("\n")
}

// === Player Decisions ===

pub fn format_player_decisions(decisions: &types::PlayerDecisions) -> String {
    let mut lines = Vec::new();

    lines.push("Player Decisions".to_string());

    if let Some(note) = &decisions.note {
        lines.push(format!("  Note: {}", note));
    }

    if decisions.decisions.is_empty() && decisions.note.is_none() {
        lines.push("  No pending decisions".to_string());
    }

    lines.join("\n")
}

// === Player Laws ===

pub fn format_player_laws(laws: &types::PlayerLaws) -> String {
    let mut lines = Vec::new();

    lines.push("Player Laws".to_string());

    if let Some(note) = &laws.note {
        lines.push(format!("  Note: {}", note));
    }

    if laws.active_laws.is_empty() && laws.note.is_none() {
        lines.push("  No active laws".to_string());
    } else {
        for (law, value) in &laws.active_laws {
            lines.push(format!("  - {}: {}", shorten_type(law), value));
        }
    }

    lines.join("\n")
}

// === Player Missions ===

pub fn format_player_missions(missions: &types::PlayerMissions) -> String {
    let mut lines = Vec::new();

    lines.push("Player Missions".to_string());

    if let Some(note) = &missions.note {
        lines.push(format!("  Note: {}", note));
    }

    if missions.missions.is_empty() && missions.note.is_none() {
        lines.push("  No active missions".to_string());
    }

    lines.join("\n")
}

// === Player Resources ===

pub fn format_player_resources(resources: &types::PlayerResources) -> String {
    let mut lines = Vec::new();

    lines.push("Player Resources".to_string());

    if let Some(note) = &resources.note {
        lines.push(format!("  Note: {}", note));
    }

    if !resources.resources.is_empty() {
        lines.push("  Resources:".to_string());
        for (resource, amount) in &resources.resources {
            lines.push(format!("    {}: {}", shorten_type(resource), amount));
        }
    }

    lines.join("\n")
}

// === Utility ===

/// Shorten game type strings (e.g., "NATION_ROME" -> "Rome")
fn shorten_type(s: &str) -> String {
    s.split('_')
        .skip(1)
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first
                    .to_uppercase()
                    .chain(chars.map(|c| c.to_lowercase().next().unwrap_or(c)))
                    .collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
