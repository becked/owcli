mod cli;
mod client;
mod commands;
mod config;
mod error;
mod help;
mod output;
mod path_parser;
mod repl;

use std::io::Read;

use clap::Parser;
use cli::{Cli, Commands};
use client::{command_succeeded, ApiClient};
use commands::{execute_action, execute_query};
use config::Config;
use output::{format_bulk_response, format_command_response, format_typed_output};

fn main() {
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();
    let config = Config::new(cli.host.clone(), cli.port, cli.json);

    let result = if cli.command.is_none() && cli.path.is_empty() {
        repl::run_repl(&config)
    } else {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
        rt.block_on(run(cli, config))
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run(cli: Cli, config: Config) -> error::Result<()> {
    if let Some(command) = cli.command {
        return handle_command(command, &config).await;
    }

    if !cli.path.is_empty() {
        let path = cli.path.join("/");
        return handle_query(&path, &config).await;
    }

    Ok(())
}

async fn handle_command(command: Commands, config: &Config) -> error::Result<()> {
    match command {
        Commands::Tiles { offset, limit } => {
            let client = ApiClient::new(config)?;
            let result = commands::query::execute_tiles_query(&client, offset, limit).await?;
            let output = format_typed_output(&result, config.json_output)?;
            println!("{}", output);
            Ok(())
        }

        Commands::Command { action } => {
            let client = ApiClient::new(config)?;
            let response = execute_action(&client, &action).await?;
            let succeeded = command_succeeded(&response);
            let output = format_command_response(
                succeeded,
                response.error.as_deref(),
                response.request_id.as_deref(),
                config.json_output,
            );
            println!("{}", output);

            if !succeeded {
                std::process::exit(1);
            }
            Ok(())
        }

        Commands::Bulk {
            file,
            continue_on_error,
        } => {
            use client::types::{BulkCommand, GameCommand, GameCommandAction};

            let input = if file == "-" {
                let mut buf = String::new();
                std::io::stdin().read_to_string(&mut buf)?;
                buf
            } else {
                std::fs::read_to_string(&file)?
            };

            // Parse JSON input into GameCommand structs
            let raw_commands: Vec<serde_json::Value> = serde_json::from_str(&input)?;
            let game_commands: Vec<GameCommand> = raw_commands
                .into_iter()
                .filter_map(|cmd| {
                    let action_str = cmd["action"].as_str()?;
                    let action: GameCommandAction = serde_json::from_value(
                        serde_json::Value::String(action_str.to_string())
                    ).ok()?;
                    let params = cmd.get("params")
                        .and_then(|p| p.as_object())
                        .cloned()
                        .unwrap_or_default();
                    Some(GameCommand {
                        action,
                        params,
                        request_id: None,
                    })
                })
                .collect();

            let bulk = BulkCommand {
                commands: game_commands,
                request_id: None,
                stop_on_error: !continue_on_error,
            };

            let client = ApiClient::new(config)?;
            let response = client.execute_bulk_commands(&bulk).await?;

            let all_succeeded = response.all_succeeded.unwrap_or(true);
            let output = format_bulk_response(&response, config.json_output);
            println!("{}", output);

            if !all_succeeded {
                std::process::exit(1);
            }
            Ok(())
        }

        Commands::Help { topic } => {
            use crate::help::{
                generate_command_help, generate_commands_list, generate_overview_help,
                generate_queries_list,
            };

            // Handle: help, help commands, help queries, help <cmd>, help command <cmd>
            let output = match topic.as_slice() {
                [] => generate_overview_help(),
                [t] if t == "queries" => generate_queries_list(),
                [t] if t == "commands" => generate_commands_list(),
                [t] => generate_command_help(t).unwrap_or_else(|| {
                    format!(
                        "Unknown command: '{}'. Use 'help commands' for available commands.",
                        t
                    )
                }),
                // Handle "help command <cmd>" syntax
                [first, cmd] if first == "command" || first == "cmd" => {
                    generate_command_help(cmd).unwrap_or_else(|| {
                        format!(
                            "Unknown command: '{}'. Use 'help commands' for available commands.",
                            cmd
                        )
                    })
                }
                _ => {
                    let joined = topic.join(" ");
                    format!(
                        "Unknown help topic: '{}'. Use 'help commands' for available commands.",
                        joined
                    )
                }
            };
            println!("{}", output);
            Ok(())
        }
    }
}

async fn handle_query(path: &str, config: &Config) -> error::Result<()> {
    let client = ApiClient::new(config)?;
    let result = execute_query(&client, path).await?;
    let output = format_typed_output(&result, config.json_output)?;
    println!("{}", output);
    Ok(())
}
