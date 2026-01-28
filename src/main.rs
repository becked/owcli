mod cli;
mod client;
mod commands;
mod config;
mod error;
mod models;
mod output;
mod path_parser;
mod repl;

use clap::Parser;
use cli::{Cli, Commands};
use client::ApiClient;
use commands::{execute_action, execute_query};
use config::Config;
use output::{format_command_response, format_output};

fn main() {
    // Load .env file if present
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();
    let config = Config::new(cli.host.clone(), cli.port, cli.json);

    if let Err(e) = run(cli, config) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli, config: Config) -> error::Result<()> {
    // Handle subcommands first
    if let Some(command) = cli.command {
        return handle_command(command, &config);
    }

    // If no subcommand but path provided, execute as query
    if !cli.path.is_empty() {
        let path = cli.path.join("/");
        return handle_query(&path, &config);
    }

    // No command and no path - launch interactive mode
    repl::run_repl(&config)
}

fn handle_command(command: Commands, config: &Config) -> error::Result<()> {
    match command {
        Commands::Tiles { offset, limit } => {
            let client = ApiClient::new(config)?;
            let result = commands::query::execute_tiles_query(&client, offset, limit)?;
            let output = format_output(&result.data, &result.path.endpoint_type, config.json_output)?;
            println!("{}", output);
            Ok(())
        }

        Commands::Command { action } => {
            let client = ApiClient::new(config)?;
            let response = execute_action(&client, &action)?;
            let output = format_command_response(
                response.success,
                response.error.as_deref(),
                response.request_id.as_deref(),
                config.json_output,
            );
            println!("{}", output);

            if !response.success {
                std::process::exit(1);
            }
            Ok(())
        }
    }
}

fn handle_query(path: &str, config: &Config) -> error::Result<()> {
    let client = ApiClient::new(config)?;
    let result = execute_query(&client, path)?;
    let output = format_output(&result.data, &result.path.endpoint_type, config.json_output)?;
    println!("{}", output);
    Ok(())
}
