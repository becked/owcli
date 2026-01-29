mod cli;
mod client;
mod commands;
mod config;
mod error;
mod output;
mod path_parser;
mod repl;

use clap::Parser;
use cli::{Cli, Commands};
use client::ApiClient;
use commands::{execute_action, execute_query};
use config::Config;
use output::{format_command_response, format_typed_output};

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

async fn handle_query(path: &str, config: &Config) -> error::Result<()> {
    let client = ApiClient::new(config)?;
    let result = execute_query(&client, path).await?;
    let output = format_typed_output(&result, config.json_output)?;
    println!("{}", output);
    Ok(())
}
