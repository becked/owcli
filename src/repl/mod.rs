mod completer;

use crate::cli::Action;
use crate::client::ApiClient;
use crate::commands::{execute_action, execute_query};
use crate::config::Config;
use crate::output::{format_command_response, format_typed_output};
use completer::OwcliCompleter;
use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Config as RlConfig, EditMode, Editor};
use tokio::runtime::Runtime;

pub fn run_repl(config: &Config) -> crate::error::Result<()> {
    let rt = Runtime::new().map_err(|e| crate::error::OwcliError::Other(e.to_string()))?;
    let client = ApiClient::new(config)?;

    let rl_config = RlConfig::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

    let completer = OwcliCompleter::new();
    let mut rl = Editor::with_config(rl_config)?;
    rl.set_helper(Some(completer));

    println!("Old World CLI - Interactive Mode");
    println!("Type 'help' for available commands, 'exit' to quit\n");

    loop {
        let readline = rl.readline("owcli> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line)?;

                match process_repl_line(&client, line, &rt) {
                    ReplResult::Continue(output) => {
                        if let Some(text) = output {
                            println!("{}\n", text);
                        }
                    }
                    ReplResult::Exit => {
                        println!("Goodbye!");
                        break;
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

enum ReplResult {
    Continue(Option<String>),
    Exit,
}

fn process_repl_line(client: &ApiClient, line: &str, rt: &Runtime) -> ReplResult {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return ReplResult::Continue(None);
    }

    match parts[0] {
        "exit" | "quit" | "q" => ReplResult::Exit,

        "help" | "?" => ReplResult::Continue(Some(get_help_text())),

        "command" | "cmd" => {
            if parts.len() < 2 {
                return ReplResult::Continue(Some(
                    "Usage: command <action> [--param value ...]\nUse 'help commands' for available actions".to_string()
                ));
            }
            match parse_and_execute_command(client, &parts[1..], rt) {
                Ok(output) => ReplResult::Continue(Some(output)),
                Err(e) => ReplResult::Continue(Some(format!("Error: {}", e))),
            }
        }

        "tiles" => {
            let (offset, limit) = parse_tiles_args(&parts[1..]);
            match rt.block_on(crate::commands::query::execute_tiles_query(client, offset, limit)) {
                Ok(result) => {
                    let output = format_typed_output(&result, false)
                        .unwrap_or_else(|e| format!("Format error: {}", e));
                    ReplResult::Continue(Some(output))
                }
                Err(e) => ReplResult::Continue(Some(format!("Error: {}", e))),
            }
        }

        _ => {
            match rt.block_on(execute_query(client, parts[0])) {
                Ok(result) => {
                    let output = format_typed_output(&result, false)
                        .unwrap_or_else(|e| format!("Format error: {}", e));
                    ReplResult::Continue(Some(output))
                }
                Err(e) => ReplResult::Continue(Some(format!("Error: {}", e))),
            }
        }
    }
}

fn parse_tiles_args(args: &[&str]) -> (u32, u32) {
    let mut offset = 0u32;
    let mut limit = 100u32;

    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "--offset" if i + 1 < args.len() => {
                offset = args[i + 1].parse().unwrap_or(0);
                i += 2;
            }
            "--limit" if i + 1 < args.len() => {
                limit = args[i + 1].parse().unwrap_or(100);
                i += 2;
            }
            _ => i += 1,
        }
    }

    (offset, limit.min(1000))
}

fn parse_and_execute_command(client: &ApiClient, args: &[&str], rt: &Runtime) -> crate::error::Result<String> {
    if args.is_empty() {
        return Ok("No action specified".to_string());
    }

    let action = parse_action_from_args(args)?;
    let response = rt.block_on(execute_action(client, &action))?;

    Ok(format_command_response(
        response.success,
        response.error.as_deref(),
        response.request_id.as_deref(),
        false,
    ))
}

fn parse_action_from_args(args: &[&str]) -> crate::error::Result<Action> {
    use crate::error::OwcliError;

    let action_name = args[0];
    let params = &args[1..];

    let get_param = |key: &str| -> Option<&str> {
        for i in 0..params.len() {
            if params[i] == format!("--{}", key) && i + 1 < params.len() {
                return Some(params[i + 1]);
            }
        }
        None
    };

    let get_int_param = |key: &str| -> crate::error::Result<i32> {
        get_param(key)
            .ok_or_else(|| OwcliError::Other(format!("Missing required parameter: --{}", key)))?
            .parse()
            .map_err(|_| OwcliError::Other(format!("Invalid integer for --{}", key)))
    };

    let get_string_param = |key: &str| -> crate::error::Result<String> {
        get_param(key)
            .ok_or_else(|| OwcliError::Other(format!("Missing required parameter: --{}", key)))
            .map(|s| s.to_string())
    };

    let has_flag = |key: &str| -> bool {
        params.contains(&format!("--{}", key).as_str())
    };

    match action_name {
        // Unit Movement
        "move-unit" => Ok(Action::MoveUnit {
            unit: get_int_param("unit")?,
            target: get_int_param("target")?,
            queue: has_flag("queue"),
            force: has_flag("force"),
        }),
        "attack" => Ok(Action::Attack {
            unit: get_int_param("unit")?,
            target: get_int_param("target")?,
        }),
        "fortify" => Ok(Action::Fortify { unit: get_int_param("unit")? }),
        "pass" => Ok(Action::Pass { unit: get_int_param("unit")? }),
        "skip" => Ok(Action::Skip { unit: get_int_param("unit")? }),
        "sleep" => Ok(Action::Sleep { unit: get_int_param("unit")? }),
        "sentry" => Ok(Action::Sentry { unit: get_int_param("unit")? }),
        "wake" => Ok(Action::Wake { unit: get_int_param("unit")? }),
        "disband" => Ok(Action::Disband { unit: get_int_param("unit")? }),
        "heal" => Ok(Action::Heal { unit: get_int_param("unit")? }),
        "march" => Ok(Action::March {
            unit: get_int_param("unit")?,
            target: get_int_param("target")?,
        }),
        "lock" => Ok(Action::Lock { unit: get_int_param("unit")? }),

        // Unit Special
        "found-city" => Ok(Action::FoundCity { unit: get_int_param("unit")? }),
        "join-city" => Ok(Action::JoinCity {
            unit: get_int_param("unit")?,
            city: get_int_param("city")?,
        }),
        "build-improvement" => Ok(Action::BuildImprovement {
            unit: get_int_param("unit")?,
            improvement_type: get_string_param("type")?,
        }),
        "add-road" => Ok(Action::AddRoad { unit: get_int_param("unit")? }),
        "upgrade-improvement" => Ok(Action::UpgradeImprovement { unit: get_int_param("unit")? }),
        "pillage" => Ok(Action::Pillage { unit: get_int_param("unit")? }),
        "burn" => Ok(Action::Burn { unit: get_int_param("unit")? }),
        "promote" => Ok(Action::Promote {
            unit: get_int_param("unit")?,
            promotion: get_string_param("promotion")?,
        }),
        "upgrade" => Ok(Action::Upgrade { unit: get_int_param("unit")? }),
        "spread-religion" => Ok(Action::SpreadReligion { unit: get_int_param("unit")? }),

        // City Production
        "build-unit" => Ok(Action::BuildUnit {
            city: get_int_param("city")?,
            unit_type: get_string_param("type")?,
        }),
        "build-project" => Ok(Action::BuildProject {
            city: get_int_param("city")?,
            project_type: get_string_param("type")?,
        }),
        "build-queue" => Ok(Action::BuildQueue {
            city: get_int_param("city")?,
            build_type: get_string_param("build-type")?,
            item_type: get_string_param("item-type")?,
        }),
        "hurry-civics" => Ok(Action::HurryCivics { city: get_int_param("city")? }),
        "hurry-training" => Ok(Action::HurryTraining { city: get_int_param("city")? }),
        "hurry-money" => Ok(Action::HurryMoney { city: get_int_param("city")? }),
        "hurry-population" => Ok(Action::HurryPopulation { city: get_int_param("city")? }),
        "hurry-orders" => Ok(Action::HurryOrders { city: get_int_param("city")? }),

        // Research
        "research" => Ok(Action::Research { tech: get_string_param("tech")? }),
        "redraw-tech" => Ok(Action::RedrawTech),
        "target-tech" => Ok(Action::TargetTech { tech: get_string_param("tech")? }),
        "make-decision" => Ok(Action::MakeDecision {
            decision: get_int_param("decision")?,
            choice: get_int_param("choice")?,
        }),
        "remove-decision" => Ok(Action::RemoveDecision { decision: get_int_param("decision")? }),

        // Diplomacy
        "declare-war" => Ok(Action::DeclareWar { player: get_int_param("player")? }),
        "make-peace" => Ok(Action::MakePeace { player: get_int_param("player")? }),
        "declare-truce" => Ok(Action::DeclareTruce { player: get_int_param("player")? }),
        "declare-war-tribe" => Ok(Action::DeclareWarTribe { tribe: get_string_param("tribe")? }),
        "make-peace-tribe" => Ok(Action::MakePeaceTribe { tribe: get_string_param("tribe")? }),
        "declare-truce-tribe" => Ok(Action::DeclareTruceTribe { tribe: get_string_param("tribe")? }),
        "gift-city" => Ok(Action::GiftCity {
            city: get_int_param("city")?,
            player: get_int_param("player")?,
        }),
        "gift-yield" => Ok(Action::GiftYield {
            player: get_int_param("player")?,
            yield_type: get_string_param("type")?,
            amount: get_int_param("amount")?,
        }),
        "ally-tribe" => Ok(Action::AllyTribe { tribe: get_string_param("tribe")? }),

        // Characters
        "assign-governor" => Ok(Action::AssignGovernor {
            character: get_int_param("character")?,
            city: get_int_param("city")?,
        }),
        "release-governor" => Ok(Action::ReleaseGovernor { city: get_int_param("city")? }),
        "assign-general" => Ok(Action::AssignGeneral {
            character: get_int_param("character")?,
            unit: get_int_param("unit")?,
        }),
        "release-general" => Ok(Action::ReleaseGeneral { unit: get_int_param("unit")? }),
        "assign-agent" => Ok(Action::AssignAgent { character: get_int_param("character")? }),
        "release-agent" => Ok(Action::ReleaseAgent { character: get_int_param("character")? }),
        "start-mission" => Ok(Action::StartMission {
            character: get_int_param("character")?,
            mission: get_string_param("mission")?,
        }),

        // Turn
        "end-turn" => Ok(Action::EndTurn),

        _ => Err(OwcliError::Other(format!("Unknown action: {}", action_name))),
    }
}

fn get_help_text() -> String {
    r#"Available Commands:

QUERIES (type the path directly):
  state                     Full game snapshot
  config                    Game configuration
  players                   All players
  player/<index>            Single player (0-indexed)
  player/<index>/units      Player's units
  player/<index>/techs      Player's technologies
  player/<index>/families   Player's families
  cities                    All cities
  city/<id>                 Single city
  characters                All characters
  character/<id>            Single character
  units                     All units
  unit/<id>                 Single unit
  tiles [--offset N --limit M]  Paginated tiles
  tile/<id>                 Tile by ID
  tile/<x>/<y>              Tile by coordinates
  tribes                    All tribes
  tribe/<type>              Single tribe
  religions                 All religions
  team-diplomacy            Team relations
  tribe-diplomacy           Tribe relations
  character-events          Character events
  unit-events               Unit events
  city-events               City events

COMMANDS (prefix with 'command'):
  command end-turn
  command move-unit --unit <id> --target <tile_id> [--queue] [--force]
  command attack --unit <id> --target <tile_id>
  command fortify --unit <id>
  command research --tech <tech_type>
  command build-unit --city <id> --type <unit_type>
  ... and many more (use 'help commands' for full list)

OTHER:
  help, ?                   Show this help
  exit, quit, q             Exit interactive mode"#.to_string()
}
