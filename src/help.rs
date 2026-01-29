//! Unified help system for owcli
//!
//! This module provides centralized help generation using:
//! - Clap's Subcommand trait for introspecting Action commands
//! - A static QueryInfo registry for query paths
//!
//! This ensures help text stays in sync with the implementation.

use clap::{Command, Subcommand};

use crate::cli::Action;

/// Information about a query path
pub struct QueryInfo {
    /// The path pattern (e.g., "player/<index>/units")
    pub path: &'static str,
    /// Description of what this query returns
    pub description: &'static str,
}

/// Central registry of all query paths - single source of truth
pub const QUERIES: &[QueryInfo] = &[
    // Simple endpoints
    QueryInfo {
        path: "state",
        description: "Full game snapshot",
    },
    QueryInfo {
        path: "config",
        description: "Game configuration",
    },
    QueryInfo {
        path: "players",
        description: "All players",
    },
    QueryInfo {
        path: "cities",
        description: "All cities",
    },
    QueryInfo {
        path: "characters",
        description: "All characters",
    },
    QueryInfo {
        path: "units",
        description: "All units",
    },
    QueryInfo {
        path: "map",
        description: "Map information",
    },
    QueryInfo {
        path: "tribes",
        description: "All tribes",
    },
    QueryInfo {
        path: "religions",
        description: "All religions",
    },
    // Player paths
    QueryInfo {
        path: "player/<index>",
        description: "Single player (0-indexed)",
    },
    QueryInfo {
        path: "player/<index>/units",
        description: "Player's units",
    },
    QueryInfo {
        path: "player/<index>/techs",
        description: "Player's technologies",
    },
    QueryInfo {
        path: "player/<index>/families",
        description: "Player's families",
    },
    QueryInfo {
        path: "player/<index>/religion",
        description: "Player's religion",
    },
    QueryInfo {
        path: "player/<index>/goals",
        description: "Player's goals",
    },
    QueryInfo {
        path: "player/<index>/decisions",
        description: "Player's pending decisions",
    },
    QueryInfo {
        path: "player/<index>/laws",
        description: "Player's laws",
    },
    QueryInfo {
        path: "player/<index>/missions",
        description: "Player's missions",
    },
    QueryInfo {
        path: "player/<index>/resources",
        description: "Player's resources",
    },
    // Single item lookups
    QueryInfo {
        path: "city/<id>",
        description: "Single city by ID",
    },
    QueryInfo {
        path: "character/<id>",
        description: "Single character by ID",
    },
    QueryInfo {
        path: "unit/<id>",
        description: "Single unit by ID",
    },
    QueryInfo {
        path: "tile/<id>",
        description: "Tile by ID",
    },
    QueryInfo {
        path: "tile/<x>/<y>",
        description: "Tile by coordinates",
    },
    QueryInfo {
        path: "tribe/<type>",
        description: "Single tribe by type",
    },
    // Diplomacy and events
    QueryInfo {
        path: "team-diplomacy",
        description: "Team diplomatic relations",
    },
    QueryInfo {
        path: "team-alliances",
        description: "Team alliances",
    },
    QueryInfo {
        path: "tribe-diplomacy",
        description: "Tribe diplomatic relations",
    },
    QueryInfo {
        path: "tribe-alliances",
        description: "Tribe alliances",
    },
    QueryInfo {
        path: "character-events",
        description: "Character events",
    },
    QueryInfo {
        path: "unit-events",
        description: "Unit events",
    },
    QueryInfo {
        path: "city-events",
        description: "City events",
    },
];

/// Player resource sub-paths for completion
pub const PLAYER_RESOURCES: &[&str] = &[
    "units",
    "techs",
    "families",
    "religion",
    "goals",
    "decisions",
    "laws",
    "missions",
    "resources",
];

/// Command categories for organized help display
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CommandCategory {
    // Original categories (9)
    UnitMovement,
    UnitActions,
    Worker,
    CityFoundation,
    CityProduction,
    Research,
    Diplomacy,
    Characters,
    TurnControl,
    // New categories (15)
    LawsEconomy,
    LuxuryTrading,
    UnitSpecialActions,
    AgentCaravan,
    ReligiousUnits,
    CharacterExtended,
    CityManagement,
    GoalsCommunication,
    GameState,
    DiplomacyExtended,
    EditorUnits,
    EditorCities,
    EditorTiles,
    EditorMapPlayer,
    EditorCharacters,
}

impl CommandCategory {
    pub fn name(&self) -> &'static str {
        match self {
            Self::UnitMovement => "Unit Movement",
            Self::UnitActions => "Unit Actions",
            Self::Worker => "Worker Commands",
            Self::CityFoundation => "City Foundation",
            Self::CityProduction => "City Production",
            Self::Research => "Research & Decisions",
            Self::Diplomacy => "Diplomacy",
            Self::Characters => "Character Management",
            Self::TurnControl => "Turn Control",
            Self::LawsEconomy => "Laws & Economy",
            Self::LuxuryTrading => "Luxury Trading",
            Self::UnitSpecialActions => "Unit Special Actions",
            Self::AgentCaravan => "Agent & Caravan",
            Self::ReligiousUnits => "Religious Units",
            Self::CharacterExtended => "Character Management (Extended)",
            Self::CityManagement => "City Management",
            Self::GoalsCommunication => "Goals & Communication",
            Self::GameState => "Game State & Turn",
            Self::DiplomacyExtended => "Diplomacy (Extended)",
            Self::EditorUnits => "Editor/Debug - Units",
            Self::EditorCities => "Editor/Debug - Cities",
            Self::EditorTiles => "Editor/Debug - Tiles",
            Self::EditorMapPlayer => "Editor/Debug - Map & Player",
            Self::EditorCharacters => "Editor/Debug - Characters",
        }
    }

    /// Map command names to categories (matches OpenAPI spec)
    pub fn for_command(name: &str) -> Self {
        match name {
            // Unit Movement (11)
            "move-unit" | "attack" | "fortify" | "pass" | "skip" | "sleep" | "sentry" | "wake"
            | "heal" | "march" | "lock" => Self::UnitMovement,

            // Unit Actions (6)
            "disband" | "promote" | "pillage" | "burn" | "upgrade" | "spread-religion" => {
                Self::UnitActions
            }

            // Worker (3)
            "build-improvement" | "upgrade-improvement" | "add-road" => Self::Worker,

            // City Foundation (2)
            "found-city" | "join-city" => Self::CityFoundation,

            // City Production (9)
            "build" | "build-unit" | "build-project" | "build-queue" | "hurry-civics"
            | "hurry-training" | "hurry-money" | "hurry-population" | "hurry-orders" => {
                Self::CityProduction
            }

            // Research & Decisions (5)
            "research" | "redraw-tech" | "target-tech" | "make-decision" | "remove-decision" => {
                Self::Research
            }

            // Diplomacy (9)
            "declare-war" | "make-peace" | "declare-truce" | "declare-war-tribe"
            | "make-peace-tribe" | "declare-truce-tribe" | "gift-city" | "gift-yield"
            | "ally-tribe" => Self::Diplomacy,

            // Character Management (7)
            "assign-governor" | "release-governor" | "assign-general" | "release-general"
            | "assign-agent" | "release-agent" | "start-mission" => Self::Characters,

            // Turn Control (1)
            "end-turn" => Self::TurnControl,

            // Laws & Economy (7)
            "choose-law" | "cancel-law" | "buy-yield" | "sell-yield" | "convert-orders"
            | "convert-legitimacy" | "convert-orders-to-science" => Self::LawsEconomy,

            // Luxury Trading (5)
            "trade-city-luxury" | "trade-family-luxury" | "trade-tribe-luxury"
            | "trade-player-luxury" | "tribute" => Self::LuxuryTrading,

            // Unit Special Actions (20)
            "swap" | "do-unit-queue" | "cancel-unit-queue" | "formation" | "unlimber" | "anchor"
            | "repair" | "cancel-improvement" | "remove-vegetation" | "harvest-resource"
            | "unit-automate" | "add-urban" | "road-to" | "buy-tile" | "recruit-mercenary"
            | "hire-mercenary" | "gift-unit" | "launch-offensive" | "apply-effect-unit"
            | "select-unit" => Self::UnitSpecialActions,

            // Agent & Caravan (4)
            "create-agent-network" | "create-trade-outpost" | "caravan-mission-start"
            | "caravan-mission-cancel" => Self::AgentCaravan,

            // Religious Units (3)
            "purge-religion" | "spread-religion-tribe" | "establish-theology" => {
                Self::ReligiousUnits
            }

            // Character Management Extended (13)
            "character-name" | "add-character-trait" | "set-character-rating"
            | "set-character-experience" | "set-character-cognomen" | "set-character-nation"
            | "set-character-family" | "set-character-religion" | "set-character-courtier"
            | "set-character-council" | "player-leader" | "family-head" | "pin-character" => {
                Self::CharacterExtended
            }

            // City Management (8)
            "city-rename" | "city-automate" | "build-specialist" | "set-specialist"
            | "change-citizens" | "change-religion" | "change-family" | "change-family-seat" => {
                Self::CityManagement
            }

            // Goals & Communication (9)
            "abandon-ambition" | "add-player-goal" | "remove-player-goal" | "event-story"
            | "finish-goal" | "chat" | "ping" | "custom-reminder" | "clear-chat" => {
                Self::GoalsCommunication
            }

            // Game State & Turn (7)
            "extend-time" | "pause" | "undo" | "redo" | "replay-turn" | "ai-finish-turn"
            | "toggle-no-replay" => Self::GameState,

            // Diplomacy Extended (3)
            "team-alliance" | "tribe-invasion" | "victory-team" => Self::DiplomacyExtended,

            // Editor/Debug - Units (8)
            "create-unit" | "unit-name" | "set-unit-family" | "change-unit-owner"
            | "change-cooldown" | "change-damage" | "unit-increment-level"
            | "unit-change-promotion" => Self::EditorUnits,

            // Editor/Debug - Cities (8)
            "create-city" | "remove-city" | "city-owner" | "change-city-damage" | "change-culture"
            | "change-city-build-turns" | "change-city-discontent-level" | "change-project" => {
                Self::EditorCities
            }

            // Editor/Debug - Tiles (9)
            "set-terrain" | "set-terrain-height" | "set-vegetation" | "set-resource" | "set-road"
            | "set-improvement" | "set-tile-owner" | "set-city-site"
            | "improvement-build-turns" => Self::EditorTiles,

            // Editor/Debug - Map & Player (6)
            "map-reveal" | "map-unreveal" | "add-tech" | "add-yield" | "add-money" | "cheat" => {
                Self::EditorMapPlayer
            }

            // Editor/Debug - Characters (5)
            "make-character-dead" | "make-character-safe" | "new-character" | "add-character"
            | "tribe-leader" => Self::EditorCharacters,

            _ => Self::UnitMovement, // fallback
        }
    }

    /// All categories in display order
    pub fn all() -> &'static [Self] {
        &[
            Self::UnitMovement,
            Self::UnitActions,
            Self::Worker,
            Self::CityFoundation,
            Self::CityProduction,
            Self::Research,
            Self::Diplomacy,
            Self::Characters,
            Self::TurnControl,
            Self::LawsEconomy,
            Self::LuxuryTrading,
            Self::UnitSpecialActions,
            Self::AgentCaravan,
            Self::ReligiousUnits,
            Self::CharacterExtended,
            Self::CityManagement,
            Self::GoalsCommunication,
            Self::GameState,
            Self::DiplomacyExtended,
            Self::EditorUnits,
            Self::EditorCities,
            Self::EditorTiles,
            Self::EditorMapPlayer,
            Self::EditorCharacters,
        ]
    }
}

/// Build a Command with all Action subcommands for introspection
fn get_action_command() -> Command {
    Action::augment_subcommands(Command::new("action"))
}

/// Get all command names from Clap introspection
pub fn get_command_names() -> Vec<String> {
    get_action_command()
        .get_subcommands()
        .map(|c| c.get_name().to_string())
        .collect()
}

/// Get static query completions (paths without parameters or with trailing slash)
pub fn get_query_completions() -> Vec<&'static str> {
    let mut completions = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for info in QUERIES {
        if !info.path.contains('<') {
            // Static path - use as-is
            if seen.insert(info.path) {
                completions.push(info.path);
            }
        } else {
            // Parameterized path - extract the prefix with trailing slash
            if let Some(slash_pos) = info.path.find('/') {
                let prefix = &info.path[..=slash_pos];
                if seen.insert(prefix) {
                    // We need to return a 'static str, so match against known prefixes
                    match prefix {
                        "player/" => completions.push("player/"),
                        "city/" => completions.push("city/"),
                        "character/" => completions.push("character/"),
                        "unit/" => completions.push("unit/"),
                        "tile/" => completions.push("tile/"),
                        "tribe/" => completions.push("tribe/"),
                        _ => {}
                    }
                }
            }
        }
    }

    completions
}

/// Get player resource completions
pub fn get_player_resource_completions() -> &'static [&'static str] {
    PLAYER_RESOURCES
}

/// Generate the main help overview
pub fn generate_overview_help() -> String {
    let mut output = String::from("Old World CLI\n\n");

    // Queries section
    output.push_str("QUERIES (type the path directly):\n");
    for info in QUERIES.iter().take(20) {
        // Show first 20 queries
        output.push_str(&format!("  {:<28} {}\n", info.path, info.description));
    }
    output.push_str("  ...use 'help queries' for full list\n");

    // Commands section grouped by category
    output.push_str("\nCOMMANDS (prefix with 'command'):\n");

    let cmd = get_action_command();
    for category in CommandCategory::all() {
        output.push_str(&format!("\n  {}:\n", category.name()));

        for subcmd in cmd.get_subcommands() {
            let name = subcmd.get_name();
            if CommandCategory::for_command(name) == *category {
                let about = subcmd
                    .get_about()
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                output.push_str(&format!("    {:<22} {}\n", name, about));
            }
        }
    }

    output.push_str("\nOTHER:\n");
    output.push_str("  help, ?                     Show this help\n");
    output.push_str("  help <command>              Show command details\n");
    output.push_str("  help queries                List all query paths\n");
    output.push_str("  help commands               List all commands\n");
    output.push_str("  exit, quit, q               Exit interactive mode\n");

    output
}

/// Generate detailed help for a specific command
pub fn generate_command_help(command_name: &str) -> Option<String> {
    let cmd = get_action_command();

    let subcmd = cmd
        .get_subcommands()
        .find(|c| c.get_name() == command_name)?;

    let mut output = String::new();

    // Command name and description
    output.push_str(&format!("{}\n", subcmd.get_name()));
    if let Some(about) = subcmd.get_about() {
        output.push_str(&format!("{}\n", about));
    }

    // Collect arguments
    let args: Vec<_> = subcmd
        .get_arguments()
        .filter(|a| !a.is_positional() && a.get_id() != "help" && a.get_id() != "version")
        .collect();

    if !args.is_empty() {
        output.push_str("\nArguments:\n");
        for arg in &args {
            let name = arg.get_id().as_str();
            let required = arg.is_required_set();
            let help = arg.get_help().map(|h| h.to_string()).unwrap_or_default();

            let req_marker = if required { " (required)" } else { "" };
            output.push_str(&format!("  --{}{}\n", name, req_marker));
            if !help.is_empty() {
                output.push_str(&format!("      {}\n", help));
            }
        }

        // Usage example
        output.push_str(&format!("\nUsage: command {}", subcmd.get_name()));
        for arg in &args {
            let name = arg.get_id().as_str();
            if arg.is_required_set() {
                output.push_str(&format!(" --{} <{}>", name, name.to_uppercase()));
            } else {
                output.push_str(&format!(" [--{}]", name));
            }
        }
        output.push('\n');
    }

    Some(output)
}

/// Generate list of all commands
pub fn generate_commands_list() -> String {
    let mut output = String::from("All Commands:\n\n");

    let cmd = get_action_command();
    for subcmd in cmd.get_subcommands() {
        let about = subcmd
            .get_about()
            .map(|s| s.to_string())
            .unwrap_or_default();
        output.push_str(&format!("  {:<24} {}\n", subcmd.get_name(), about));
    }

    output
}

/// Generate list of all queries
pub fn generate_queries_list() -> String {
    let mut output = String::from("All Query Paths:\n\n");

    for info in QUERIES {
        output.push_str(&format!("  {:<28} {}\n", info.path, info.description));
    }

    output.push_str("\nSpecial:\n");
    output.push_str("  tiles                       Fetch all tiles (auto-paginated)\n");
    output.push_str("  tiles --offset N --limit M  Manual pagination (max 1000 per page)\n");

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_names() {
        let names = get_command_names();
        assert!(names.contains(&"move-unit".to_string()));
        assert!(names.contains(&"end-turn".to_string()));
        assert!(names.contains(&"declare-war".to_string()));
        assert_eq!(names.len(), 168); // 168 commands per OpenAPI spec
    }

    #[test]
    fn test_get_query_completions() {
        let completions = get_query_completions();
        assert!(completions.contains(&"state"));
        assert!(completions.contains(&"players"));
        assert!(completions.contains(&"player/"));
    }

    #[test]
    fn test_generate_command_help() {
        let help = generate_command_help("move-unit");
        assert!(help.is_some());
        let help = help.unwrap();
        assert!(help.contains("move-unit"));
        assert!(help.contains("--unit"));
        assert!(help.contains("--target"));
    }

    #[test]
    fn test_generate_command_help_unknown() {
        let help = generate_command_help("unknown-command");
        assert!(help.is_none());
    }
}
