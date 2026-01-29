use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "owcli")]
#[command(about = "CLI for the Old World game API")]
#[command(version)]
#[command(disable_help_subcommand = true)]
pub struct Cli {
    /// API host address
    #[arg(long, global = true, env = "OWCLI_HOST", default_value = "localhost")]
    pub host: String,

    /// API port
    #[arg(long, global = true, env = "OWCLI_PORT", default_value = "9877")]
    pub port: u16,

    /// Output raw JSON instead of tables
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,

    /// API path to query (e.g., player/0/units, cities, tile/5/12)
    #[arg(trailing_var_arg = true)]
    pub path: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Execute a game command
    Command {
        #[command(subcommand)]
        action: Action,
    },

    /// Fetch paginated tiles
    Tiles {
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,

        /// Limit for pagination (max 1000)
        #[arg(long, default_value = "100")]
        limit: u32,
    },

    /// Execute multiple commands from a file or stdin
    Bulk {
        /// Path to JSON file containing commands (use - for stdin)
        #[arg(long, default_value = "-")]
        file: String,

        /// Continue executing even if a command fails
        #[arg(long)]
        continue_on_error: bool,
    },

    /// Show detailed help for commands and queries
    Help {
        /// Topic to get help on (e.g., 'commands', 'queries', 'command declare-war')
        #[arg(trailing_var_arg = true)]
        topic: Vec<String>,
    },
}

/// Game commands matching OpenAPI spec (53 commands)
/// See: https://github.com/becked/OldWorldAPIEndpoint/blob/main/docs/openapi.yaml
#[derive(Subcommand)]
pub enum Action {
    // ===== Unit Movement (11) =====
    /// Move a unit to a target tile
    MoveUnit {
        /// Unit ID to move
        #[arg(long)]
        unit: i32,
        /// Target tile ID
        #[arg(long)]
        target: i32,
        /// Queue move instead of immediate
        #[arg(long)]
        queue: bool,
        /// March to target
        #[arg(long)]
        march: bool,
        /// Waypoint tile ID
        #[arg(long)]
        waypoint: Option<i32>,
    },

    /// Attack with a unit
    Attack {
        /// Unit ID to attack with
        #[arg(long)]
        unit: i32,
        /// Target tile ID to attack
        #[arg(long)]
        target: i32,
    },

    /// Fortify a unit
    Fortify {
        /// Unit ID to fortify
        #[arg(long)]
        unit: i32,
    },

    /// Pass a unit's turn
    Pass {
        /// Unit ID to pass
        #[arg(long)]
        unit: i32,
    },

    /// Skip a unit
    Skip {
        /// Unit ID to skip
        #[arg(long)]
        unit: i32,
    },

    /// Put a unit to sleep
    Sleep {
        /// Unit ID to put to sleep
        #[arg(long)]
        unit: i32,
    },

    /// Set a unit to sentry mode
    Sentry {
        /// Unit ID to set to sentry mode
        #[arg(long)]
        unit: i32,
    },

    /// Wake a unit
    Wake {
        /// Unit ID to wake
        #[arg(long)]
        unit: i32,
    },

    /// Heal a unit
    Heal {
        /// Unit ID to heal
        #[arg(long)]
        unit: i32,
        /// Auto-heal
        #[arg(long)]
        auto: bool,
    },

    /// March a unit
    March {
        /// Unit ID to march
        #[arg(long)]
        unit: i32,
    },

    /// Lock a unit
    Lock {
        /// Unit ID to lock
        #[arg(long)]
        unit: i32,
    },

    // ===== Unit Actions (6) =====
    /// Disband a unit
    Disband {
        /// Unit ID to disband
        #[arg(long)]
        unit: i32,
        /// Force disbanding
        #[arg(long)]
        force: bool,
    },

    /// Promote a unit
    Promote {
        /// Unit ID to promote
        #[arg(long)]
        unit: i32,
        /// Promotion type (e.g., PROMOTION_FIERCE)
        #[arg(long)]
        promotion: String,
    },

    /// Pillage a tile
    Pillage {
        /// Unit ID to pillage with
        #[arg(long)]
        unit: i32,
    },

    /// Burn a tile
    Burn {
        /// Unit ID to burn with
        #[arg(long)]
        unit: i32,
    },

    /// Upgrade a unit
    Upgrade {
        /// Unit ID to upgrade
        #[arg(long)]
        unit: i32,
        /// Unit type to upgrade to
        #[arg(long, name = "type")]
        unit_type: String,
        /// Buy goods for upgrade
        #[arg(long)]
        buy_goods: bool,
    },

    /// Spread religion with a unit
    SpreadReligion {
        /// Unit ID (missionary)
        #[arg(long)]
        unit: i32,
        /// Target city ID
        #[arg(long)]
        city: i32,
    },

    // ===== Worker Commands (3) =====
    /// Build an improvement
    BuildImprovement {
        /// Worker unit ID
        #[arg(long)]
        unit: i32,
        /// Improvement type to build
        #[arg(long, name = "type")]
        improvement_type: String,
        /// Tile ID to build on
        #[arg(long)]
        tile: i32,
        /// Buy goods
        #[arg(long)]
        buy_goods: bool,
        /// Queue the build
        #[arg(long)]
        queue: bool,
    },

    /// Upgrade an improvement
    UpgradeImprovement {
        /// Worker unit ID
        #[arg(long)]
        unit: i32,
        /// Buy goods
        #[arg(long)]
        buy_goods: bool,
    },

    /// Add a road
    AddRoad {
        /// Worker unit ID
        #[arg(long)]
        unit: i32,
        /// Tile ID to build road on
        #[arg(long)]
        tile: i32,
        /// Buy goods
        #[arg(long)]
        buy_goods: bool,
        /// Queue the build
        #[arg(long)]
        queue: bool,
    },

    // ===== City Foundation (2) =====
    /// Found a city with a settler
    FoundCity {
        /// Settler unit ID
        #[arg(long)]
        unit: i32,
        /// Family type for the city
        #[arg(long)]
        family: String,
        /// Nation type (optional)
        #[arg(long)]
        nation: Option<String>,
    },

    /// Join a city with a unit
    JoinCity {
        /// Unit ID to join
        #[arg(long)]
        unit: i32,
    },

    // ===== City Production (9) =====
    /// Generic build action in a city
    Build {
        /// City ID to build in
        #[arg(long)]
        city: i32,
        /// Unit type to build
        #[arg(long, name = "type")]
        unit_type: String,
        /// Buy goods
        #[arg(long)]
        buy_goods: bool,
        /// Add to front of queue
        #[arg(long)]
        first: bool,
    },

    /// Build a unit in a city
    BuildUnit {
        /// City ID to build in
        #[arg(long)]
        city: i32,
        /// Unit type to build
        #[arg(long, name = "type")]
        unit_type: String,
        /// Buy goods
        #[arg(long)]
        buy_goods: bool,
        /// Add to front of queue
        #[arg(long)]
        first: bool,
    },

    /// Build a project in a city
    BuildProject {
        /// City ID to build in
        #[arg(long)]
        city: i32,
        /// Project type to build
        #[arg(long, name = "type")]
        project_type: String,
        /// Buy goods
        #[arg(long)]
        buy_goods: bool,
        /// Add to front of queue
        #[arg(long)]
        first: bool,
        /// Repeat when complete
        #[arg(long)]
        repeat: bool,
    },

    /// Reorder build queue
    BuildQueue {
        /// City ID
        #[arg(long)]
        city: i32,
        /// Old slot position
        #[arg(long)]
        old_slot: i32,
        /// New slot position
        #[arg(long)]
        new_slot: i32,
    },

    /// Hurry production with civics
    HurryCivics {
        /// City ID
        #[arg(long)]
        city: i32,
    },

    /// Hurry production with training
    HurryTraining {
        /// City ID
        #[arg(long)]
        city: i32,
    },

    /// Hurry production with money
    HurryMoney {
        /// City ID
        #[arg(long)]
        city: i32,
    },

    /// Hurry production with population
    HurryPopulation {
        /// City ID
        #[arg(long)]
        city: i32,
    },

    /// Hurry production with orders
    HurryOrders {
        /// City ID
        #[arg(long)]
        city: i32,
    },

    // ===== Research & Decisions (5) =====
    /// Research a technology
    Research {
        /// Technology type to research (e.g., TECH_FORESTRY)
        #[arg(long)]
        tech: String,
    },

    /// Redraw available technologies
    RedrawTech,

    /// Set a target technology
    TargetTech {
        /// Technology type to target
        #[arg(long, name = "type")]
        tech_type: String,
    },

    /// Make a decision
    MakeDecision {
        /// Decision ID
        #[arg(long)]
        decision: i32,
        /// Choice index (0-based)
        #[arg(long)]
        choice: i32,
        /// Optional data parameter
        #[arg(long)]
        data: Option<i32>,
    },

    /// Remove a decision
    RemoveDecision {
        /// Decision ID to remove
        #[arg(long)]
        decision: i32,
    },

    // ===== Diplomacy (9) =====
    /// Declare war on a player
    DeclareWar {
        /// Target player index
        #[arg(long)]
        player: i32,
    },

    /// Make peace with a player
    MakePeace {
        /// Target player index
        #[arg(long)]
        player: i32,
    },

    /// Declare a truce with a player
    DeclareTruce {
        /// Target player index
        #[arg(long)]
        player: i32,
    },

    /// Declare war on a tribe
    DeclareWarTribe {
        /// Tribe type
        #[arg(long)]
        tribe: String,
    },

    /// Make peace with a tribe
    MakePeaceTribe {
        /// Tribe type
        #[arg(long)]
        tribe: String,
    },

    /// Declare a truce with a tribe
    DeclareTruceTribe {
        /// Tribe type
        #[arg(long)]
        tribe: String,
    },

    /// Gift a city
    GiftCity {
        /// City ID to gift
        #[arg(long)]
        city: i32,
        /// Target player index
        #[arg(long)]
        player: i32,
    },

    /// Gift yield/resources
    GiftYield {
        /// Yield type to gift
        #[arg(long, name = "type")]
        yield_type: String,
        /// Target player index
        #[arg(long)]
        player: i32,
        /// Reverse (request instead of gift)
        #[arg(long)]
        reverse: bool,
    },

    /// Ally with a tribe
    AllyTribe {
        /// Tribe type
        #[arg(long)]
        tribe: String,
    },

    // ===== Character Management (7) =====
    /// Assign a governor to a city
    AssignGovernor {
        /// City ID to assign governor to
        #[arg(long)]
        city: i32,
        /// Character ID to assign
        #[arg(long)]
        character: i32,
    },

    /// Release a governor from a city
    ReleaseGovernor {
        /// City ID to release governor from
        #[arg(long)]
        city: i32,
    },

    /// Assign a general to a unit
    AssignGeneral {
        /// Unit ID to assign general to
        #[arg(long)]
        unit: i32,
        /// Character ID to assign
        #[arg(long)]
        character: i32,
    },

    /// Release a general from a unit
    ReleaseGeneral {
        /// Unit ID to release general from
        #[arg(long)]
        unit: i32,
    },

    /// Assign an agent to a city
    AssignAgent {
        /// City ID to assign agent to
        #[arg(long)]
        city: i32,
        /// Character ID to assign
        #[arg(long)]
        character: i32,
    },

    /// Release an agent from a city
    ReleaseAgent {
        /// City ID to release agent from
        #[arg(long)]
        city: i32,
    },

    /// Start a mission
    StartMission {
        /// Mission type
        #[arg(long)]
        mission: String,
        /// Character ID
        #[arg(long)]
        character: i32,
        /// Target (optional)
        #[arg(long)]
        target: Option<String>,
        /// Cancel existing mission
        #[arg(long)]
        cancel: bool,
    },

    // ===== Turn Control (1) =====
    /// End the current turn
    EndTurn {
        /// Force end turn even with pending actions
        #[arg(long)]
        force: bool,
    },
}

impl Action {
    /// Convert CLI action to generated GameCommand type
    pub fn to_game_command(&self) -> crate::client::types::GameCommand {
        use crate::client::types::{GameCommand, GameCommandAction};
        use serde_json::json;

        // Helper to convert json Value to Map
        fn to_map(v: serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
            match v {
                serde_json::Value::Object(m) => m,
                _ => serde_json::Map::new(),
            }
        }

        let (action, params) = match self {
            // ===== Unit Movement =====
            Action::MoveUnit { unit, target, queue, march, waypoint } => {
                let mut p = json!({ "unitId": unit, "targetTileId": target });
                if *queue { p["queue"] = json!(true); }
                if *march { p["march"] = json!(true); }
                if let Some(wp) = waypoint { p["waypointTileId"] = json!(wp); }
                (GameCommandAction::MoveUnit, p)
            }
            Action::Attack { unit, target } =>
                (GameCommandAction::Attack, json!({ "unitId": unit, "targetTileId": target })),
            Action::Fortify { unit } => (GameCommandAction::Fortify, json!({ "unitId": unit })),
            Action::Pass { unit } => (GameCommandAction::Pass, json!({ "unitId": unit })),
            Action::Skip { unit } => (GameCommandAction::Skip, json!({ "unitId": unit })),
            Action::Sleep { unit } => (GameCommandAction::Sleep, json!({ "unitId": unit })),
            Action::Sentry { unit } => (GameCommandAction::Sentry, json!({ "unitId": unit })),
            Action::Wake { unit } => (GameCommandAction::Wake, json!({ "unitId": unit })),
            Action::Heal { unit, auto } => {
                let mut p = json!({ "unitId": unit });
                if *auto { p["auto"] = json!(true); }
                (GameCommandAction::Heal, p)
            }
            Action::March { unit } => (GameCommandAction::March, json!({ "unitId": unit })),
            Action::Lock { unit } => (GameCommandAction::Lock, json!({ "unitId": unit })),

            // ===== Unit Actions =====
            Action::Disband { unit, force } => {
                let mut p = json!({ "unitId": unit });
                if *force { p["force"] = json!(true); }
                (GameCommandAction::Disband, p)
            }
            Action::Promote { unit, promotion } =>
                (GameCommandAction::Promote, json!({ "unitId": unit, "promotion": promotion })),
            Action::Pillage { unit } => (GameCommandAction::Pillage, json!({ "unitId": unit })),
            Action::Burn { unit } => (GameCommandAction::Burn, json!({ "unitId": unit })),
            Action::Upgrade { unit, unit_type, buy_goods } => {
                let mut p = json!({ "unitId": unit, "unitType": unit_type });
                if *buy_goods { p["buyGoods"] = json!(true); }
                (GameCommandAction::Upgrade, p)
            }
            Action::SpreadReligion { unit, city } =>
                (GameCommandAction::SpreadReligion, json!({ "unitId": unit, "cityId": city })),

            // ===== Worker Commands =====
            Action::BuildImprovement { unit, improvement_type, tile, buy_goods, queue } => {
                let mut p = json!({ "unitId": unit, "improvementType": improvement_type, "tileId": tile });
                if *buy_goods { p["buyGoods"] = json!(true); }
                if *queue { p["queue"] = json!(true); }
                (GameCommandAction::BuildImprovement, p)
            }
            Action::UpgradeImprovement { unit, buy_goods } => {
                let mut p = json!({ "unitId": unit });
                if *buy_goods { p["buyGoods"] = json!(true); }
                (GameCommandAction::UpgradeImprovement, p)
            }
            Action::AddRoad { unit, tile, buy_goods, queue } => {
                let mut p = json!({ "unitId": unit, "tileId": tile });
                if *buy_goods { p["buyGoods"] = json!(true); }
                if *queue { p["queue"] = json!(true); }
                (GameCommandAction::AddRoad, p)
            }

            // ===== City Foundation =====
            Action::FoundCity { unit, family, nation } => {
                let mut p = json!({ "unitId": unit, "familyType": family });
                if let Some(n) = nation { p["nationType"] = json!(n); }
                (GameCommandAction::FoundCity, p)
            }
            Action::JoinCity { unit } => (GameCommandAction::JoinCity, json!({ "unitId": unit })),

            // ===== City Production =====
            Action::Build { city, unit_type, buy_goods, first } => {
                let mut p = json!({ "cityId": city, "unitType": unit_type });
                if *buy_goods { p["buyGoods"] = json!(true); }
                if *first { p["first"] = json!(true); }
                (GameCommandAction::Build, p)
            }
            Action::BuildUnit { city, unit_type, buy_goods, first } => {
                let mut p = json!({ "cityId": city, "unitType": unit_type });
                if *buy_goods { p["buyGoods"] = json!(true); }
                if *first { p["first"] = json!(true); }
                (GameCommandAction::BuildUnit, p)
            }
            Action::BuildProject { city, project_type, buy_goods, first, repeat } => {
                let mut p = json!({ "cityId": city, "projectType": project_type });
                if *buy_goods { p["buyGoods"] = json!(true); }
                if *first { p["first"] = json!(true); }
                if *repeat { p["repeat"] = json!(true); }
                (GameCommandAction::BuildProject, p)
            }
            Action::BuildQueue { city, old_slot, new_slot } =>
                (GameCommandAction::BuildQueue, json!({ "cityId": city, "oldSlot": old_slot, "newSlot": new_slot })),
            Action::HurryCivics { city } => (GameCommandAction::HurryCivics, json!({ "cityId": city })),
            Action::HurryTraining { city } => (GameCommandAction::HurryTraining, json!({ "cityId": city })),
            Action::HurryMoney { city } => (GameCommandAction::HurryMoney, json!({ "cityId": city })),
            Action::HurryPopulation { city } => (GameCommandAction::HurryPopulation, json!({ "cityId": city })),
            Action::HurryOrders { city } => (GameCommandAction::HurryOrders, json!({ "cityId": city })),

            // ===== Research & Decisions =====
            Action::Research { tech } => (GameCommandAction::Research, json!({ "tech": tech })),
            Action::RedrawTech => (GameCommandAction::RedrawTech, json!({})),
            Action::TargetTech { tech_type } => (GameCommandAction::TargetTech, json!({ "techType": tech_type })),
            Action::MakeDecision { decision, choice, data } => {
                let mut p = json!({ "decisionId": decision, "choiceIndex": choice });
                if let Some(d) = data { p["data"] = json!(d); }
                (GameCommandAction::MakeDecision, p)
            }
            Action::RemoveDecision { decision } =>
                (GameCommandAction::RemoveDecision, json!({ "decisionId": decision })),

            // ===== Diplomacy =====
            Action::DeclareWar { player } => (GameCommandAction::DeclareWar, json!({ "targetPlayer": player })),
            Action::MakePeace { player } => (GameCommandAction::MakePeace, json!({ "targetPlayer": player })),
            Action::DeclareTruce { player } => (GameCommandAction::DeclareTruce, json!({ "targetPlayer": player })),
            Action::DeclareWarTribe { tribe } => (GameCommandAction::DeclareWarTribe, json!({ "tribeType": tribe })),
            Action::MakePeaceTribe { tribe } => (GameCommandAction::MakePeaceTribe, json!({ "tribeType": tribe })),
            Action::DeclareTruceTribe { tribe } => (GameCommandAction::DeclareTruceTribe, json!({ "tribeType": tribe })),
            Action::GiftCity { city, player } =>
                (GameCommandAction::GiftCity, json!({ "cityId": city, "targetPlayer": player })),
            Action::GiftYield { yield_type, player, reverse } => {
                let mut p = json!({ "yieldType": yield_type, "targetPlayer": player });
                if *reverse { p["reverse"] = json!(true); }
                (GameCommandAction::GiftYield, p)
            }
            Action::AllyTribe { tribe } => (GameCommandAction::AllyTribe, json!({ "tribeType": tribe })),

            // ===== Character Management =====
            Action::AssignGovernor { city, character } =>
                (GameCommandAction::AssignGovernor, json!({ "cityId": city, "characterId": character })),
            Action::ReleaseGovernor { city } => (GameCommandAction::ReleaseGovernor, json!({ "cityId": city })),
            Action::AssignGeneral { unit, character } =>
                (GameCommandAction::AssignGeneral, json!({ "unitId": unit, "characterId": character })),
            Action::ReleaseGeneral { unit } => (GameCommandAction::ReleaseGeneral, json!({ "unitId": unit })),
            Action::AssignAgent { city, character } =>
                (GameCommandAction::AssignAgent, json!({ "cityId": city, "characterId": character })),
            Action::ReleaseAgent { city } => (GameCommandAction::ReleaseAgent, json!({ "cityId": city })),
            Action::StartMission { mission, character, target, cancel } => {
                let mut p = json!({ "missionType": mission, "characterId": character });
                if let Some(t) = target { p["target"] = json!(t); }
                if *cancel { p["cancel"] = json!(true); }
                (GameCommandAction::StartMission, p)
            }

            // ===== Turn Control =====
            Action::EndTurn { force } => {
                let p = if *force { json!({ "force": true }) } else { json!({}) };
                (GameCommandAction::EndTurn, p)
            }
        };

        GameCommand {
            action,
            params: to_map(params),
            request_id: None,
        }
    }
}
