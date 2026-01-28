use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "owcli")]
#[command(about = "CLI for the Old World game API")]
#[command(version)]
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
}

#[derive(Subcommand)]
pub enum Action {
    // Unit Movement
    /// Move a unit to a target tile
    MoveUnit {
        #[arg(long)]
        unit: i32,
        #[arg(long)]
        target: i32,
        #[arg(long)]
        queue: bool,
        #[arg(long)]
        force: bool,
    },

    /// Attack with a unit
    Attack {
        #[arg(long)]
        unit: i32,
        #[arg(long)]
        target: i32,
    },

    /// Fortify a unit
    Fortify {
        #[arg(long)]
        unit: i32,
    },

    /// Pass a unit's turn
    Pass {
        #[arg(long)]
        unit: i32,
    },

    /// Skip a unit
    Skip {
        #[arg(long)]
        unit: i32,
    },

    /// Put a unit to sleep
    Sleep {
        #[arg(long)]
        unit: i32,
    },

    /// Set a unit to sentry mode
    Sentry {
        #[arg(long)]
        unit: i32,
    },

    /// Wake a unit
    Wake {
        #[arg(long)]
        unit: i32,
    },

    /// Disband a unit
    Disband {
        #[arg(long)]
        unit: i32,
    },

    /// Heal a unit
    Heal {
        #[arg(long)]
        unit: i32,
    },

    /// March a unit
    March {
        #[arg(long)]
        unit: i32,
        #[arg(long)]
        target: i32,
    },

    /// Lock a unit
    Lock {
        #[arg(long)]
        unit: i32,
    },

    // Unit Special Actions
    /// Found a city with a settler
    FoundCity {
        #[arg(long)]
        unit: i32,
    },

    /// Join a city with a unit
    JoinCity {
        #[arg(long)]
        unit: i32,
        #[arg(long)]
        city: i32,
    },

    /// Build an improvement
    BuildImprovement {
        #[arg(long)]
        unit: i32,
        #[arg(long, name = "type")]
        improvement_type: String,
    },

    /// Add a road
    AddRoad {
        #[arg(long)]
        unit: i32,
    },

    /// Upgrade an improvement
    UpgradeImprovement {
        #[arg(long)]
        unit: i32,
    },

    /// Pillage a tile
    Pillage {
        #[arg(long)]
        unit: i32,
    },

    /// Burn a tile
    Burn {
        #[arg(long)]
        unit: i32,
    },

    /// Promote a unit
    Promote {
        #[arg(long)]
        unit: i32,
        #[arg(long)]
        promotion: String,
    },

    /// Upgrade a unit
    Upgrade {
        #[arg(long)]
        unit: i32,
    },

    /// Spread religion
    SpreadReligion {
        #[arg(long)]
        unit: i32,
    },

    // City Production
    /// Build a unit in a city
    BuildUnit {
        #[arg(long)]
        city: i32,
        #[arg(long, name = "type")]
        unit_type: String,
    },

    /// Build a project in a city
    BuildProject {
        #[arg(long)]
        city: i32,
        #[arg(long, name = "type")]
        project_type: String,
    },

    /// Add to build queue
    BuildQueue {
        #[arg(long)]
        city: i32,
        #[arg(long, name = "type")]
        build_type: String,
        #[arg(long)]
        item_type: String,
    },

    /// Hurry production with civics
    HurryCivics {
        #[arg(long)]
        city: i32,
    },

    /// Hurry production with training
    HurryTraining {
        #[arg(long)]
        city: i32,
    },

    /// Hurry production with money
    HurryMoney {
        #[arg(long)]
        city: i32,
    },

    /// Hurry production with population
    HurryPopulation {
        #[arg(long)]
        city: i32,
    },

    /// Hurry production with orders
    HurryOrders {
        #[arg(long)]
        city: i32,
    },

    // Research
    /// Research a technology
    Research {
        #[arg(long)]
        tech: String,
    },

    /// Redraw available technologies
    RedrawTech,

    /// Set a target technology
    TargetTech {
        #[arg(long)]
        tech: String,
    },

    /// Make a decision
    MakeDecision {
        #[arg(long)]
        decision: i32,
        #[arg(long)]
        choice: i32,
    },

    /// Remove a decision
    RemoveDecision {
        #[arg(long)]
        decision: i32,
    },

    // Diplomacy
    /// Declare war on a player
    DeclareWar {
        #[arg(long)]
        player: i32,
    },

    /// Make peace with a player
    MakePeace {
        #[arg(long)]
        player: i32,
    },

    /// Declare a truce with a player
    DeclareTruce {
        #[arg(long)]
        player: i32,
    },

    /// Declare war on a tribe
    DeclareWarTribe {
        #[arg(long)]
        tribe: String,
    },

    /// Make peace with a tribe
    MakePeaceTribe {
        #[arg(long)]
        tribe: String,
    },

    /// Declare a truce with a tribe
    DeclareTruceTribe {
        #[arg(long)]
        tribe: String,
    },

    /// Gift a city
    GiftCity {
        #[arg(long)]
        city: i32,
        #[arg(long)]
        player: i32,
    },

    /// Gift yield/resources
    GiftYield {
        #[arg(long)]
        player: i32,
        #[arg(long, name = "type")]
        yield_type: String,
        #[arg(long)]
        amount: i32,
    },

    /// Ally with a tribe
    AllyTribe {
        #[arg(long)]
        tribe: String,
    },

    // Characters
    /// Assign a governor to a city
    AssignGovernor {
        #[arg(long)]
        character: i32,
        #[arg(long)]
        city: i32,
    },

    /// Release a governor from a city
    ReleaseGovernor {
        #[arg(long)]
        city: i32,
    },

    /// Assign a general to a unit
    AssignGeneral {
        #[arg(long)]
        character: i32,
        #[arg(long)]
        unit: i32,
    },

    /// Release a general from a unit
    ReleaseGeneral {
        #[arg(long)]
        unit: i32,
    },

    /// Assign an agent
    AssignAgent {
        #[arg(long)]
        character: i32,
    },

    /// Release an agent
    ReleaseAgent {
        #[arg(long)]
        character: i32,
    },

    /// Start a mission
    StartMission {
        #[arg(long)]
        character: i32,
        #[arg(long)]
        mission: String,
    },

    // Turn Control
    /// End the current turn
    EndTurn,
}

impl Action {
    /// Convert action to API action name and parameters
    pub fn to_api_params(&self) -> (&'static str, serde_json::Value) {
        use serde_json::json;

        match self {
            // Unit Movement
            Action::MoveUnit { unit, target, queue, force } => (
                "moveUnit",
                json!({ "unitId": unit, "targetTileId": target, "queue": queue, "force": force }),
            ),
            Action::Attack { unit, target } => (
                "attack",
                json!({ "unitId": unit, "targetTileId": target }),
            ),
            Action::Fortify { unit } => ("fortify", json!({ "unitId": unit })),
            Action::Pass { unit } => ("pass", json!({ "unitId": unit })),
            Action::Skip { unit } => ("skip", json!({ "unitId": unit })),
            Action::Sleep { unit } => ("sleep", json!({ "unitId": unit })),
            Action::Sentry { unit } => ("sentry", json!({ "unitId": unit })),
            Action::Wake { unit } => ("wake", json!({ "unitId": unit })),
            Action::Disband { unit } => ("disband", json!({ "unitId": unit })),
            Action::Heal { unit } => ("heal", json!({ "unitId": unit })),
            Action::March { unit, target } => (
                "march",
                json!({ "unitId": unit, "targetTileId": target }),
            ),
            Action::Lock { unit } => ("lock", json!({ "unitId": unit })),

            // Unit Special Actions
            Action::FoundCity { unit } => ("foundCity", json!({ "unitId": unit })),
            Action::JoinCity { unit, city } => (
                "joinCity",
                json!({ "unitId": unit, "cityId": city }),
            ),
            Action::BuildImprovement { unit, improvement_type } => (
                "buildImprovement",
                json!({ "unitId": unit, "improvementType": improvement_type }),
            ),
            Action::AddRoad { unit } => ("addRoad", json!({ "unitId": unit })),
            Action::UpgradeImprovement { unit } => (
                "upgradeImprovement",
                json!({ "unitId": unit }),
            ),
            Action::Pillage { unit } => ("pillage", json!({ "unitId": unit })),
            Action::Burn { unit } => ("burn", json!({ "unitId": unit })),
            Action::Promote { unit, promotion } => (
                "promote",
                json!({ "unitId": unit, "promotionType": promotion }),
            ),
            Action::Upgrade { unit } => ("upgrade", json!({ "unitId": unit })),
            Action::SpreadReligion { unit } => ("spreadReligion", json!({ "unitId": unit })),

            // City Production
            Action::BuildUnit { city, unit_type } => (
                "buildUnit",
                json!({ "cityId": city, "unitType": unit_type }),
            ),
            Action::BuildProject { city, project_type } => (
                "buildProject",
                json!({ "cityId": city, "projectType": project_type }),
            ),
            Action::BuildQueue { city, build_type, item_type } => (
                "buildQueue",
                json!({ "cityId": city, "buildType": build_type, "itemType": item_type }),
            ),
            Action::HurryCivics { city } => ("hurryCivics", json!({ "cityId": city })),
            Action::HurryTraining { city } => ("hurryTraining", json!({ "cityId": city })),
            Action::HurryMoney { city } => ("hurryMoney", json!({ "cityId": city })),
            Action::HurryPopulation { city } => ("hurryPopulation", json!({ "cityId": city })),
            Action::HurryOrders { city } => ("hurryOrders", json!({ "cityId": city })),

            // Research
            Action::Research { tech } => ("research", json!({ "techType": tech })),
            Action::RedrawTech => ("redrawTech", json!({})),
            Action::TargetTech { tech } => ("targetTech", json!({ "techType": tech })),
            Action::MakeDecision { decision, choice } => (
                "makeDecision",
                json!({ "decisionId": decision, "choiceIndex": choice }),
            ),
            Action::RemoveDecision { decision } => (
                "removeDecision",
                json!({ "decisionId": decision }),
            ),

            // Diplomacy
            Action::DeclareWar { player } => ("declareWar", json!({ "targetPlayer": player })),
            Action::MakePeace { player } => ("makePeace", json!({ "targetPlayer": player })),
            Action::DeclareTruce { player } => ("declareTruce", json!({ "targetPlayer": player })),
            Action::DeclareWarTribe { tribe } => (
                "declareWarTribe",
                json!({ "tribeType": tribe }),
            ),
            Action::MakePeaceTribe { tribe } => (
                "makePeaceTribe",
                json!({ "tribeType": tribe }),
            ),
            Action::DeclareTruceTribe { tribe } => (
                "declareTruceTribe",
                json!({ "tribeType": tribe }),
            ),
            Action::GiftCity { city, player } => (
                "giftCity",
                json!({ "cityId": city, "targetPlayer": player }),
            ),
            Action::GiftYield { player, yield_type, amount } => (
                "giftYield",
                json!({ "targetPlayer": player, "yieldType": yield_type, "amount": amount }),
            ),
            Action::AllyTribe { tribe } => ("allyTribe", json!({ "tribeType": tribe })),

            // Characters
            Action::AssignGovernor { character, city } => (
                "assignGovernor",
                json!({ "characterId": character, "cityId": city }),
            ),
            Action::ReleaseGovernor { city } => (
                "releaseGovernor",
                json!({ "cityId": city }),
            ),
            Action::AssignGeneral { character, unit } => (
                "assignGeneral",
                json!({ "characterId": character, "unitId": unit }),
            ),
            Action::ReleaseGeneral { unit } => (
                "releaseGeneral",
                json!({ "unitId": unit }),
            ),
            Action::AssignAgent { character } => (
                "assignAgent",
                json!({ "characterId": character }),
            ),
            Action::ReleaseAgent { character } => (
                "releaseAgent",
                json!({ "characterId": character }),
            ),
            Action::StartMission { character, mission } => (
                "startMission",
                json!({ "characterId": character, "missionType": mission }),
            ),

            // Turn Control
            Action::EndTurn => ("endTurn", json!({})),
        }
    }
}
