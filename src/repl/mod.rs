mod completer;

use crate::cli::{Cli, Commands};
use crate::client::{command_succeeded, ApiClient};
use crate::commands::{execute_action, execute_query};
use crate::config::Config;
use crate::help::{
    generate_command_help, generate_commands_list, generate_overview_help, generate_queries_list,
};
use crate::output::{format_command_response, format_typed_output};
use clap::Parser;
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

        "help" | "?" => {
            if parts.len() == 1 {
                ReplResult::Continue(Some(generate_overview_help()))
            } else {
                match parts[1] {
                    "queries" => ReplResult::Continue(Some(generate_queries_list())),
                    "commands" => ReplResult::Continue(Some(generate_commands_list())),
                    cmd => {
                        if let Some(help) = generate_command_help(cmd) {
                            ReplResult::Continue(Some(help))
                        } else {
                            ReplResult::Continue(Some(format!(
                                "Unknown command: '{}'. Use 'help commands' for available commands.",
                                cmd
                            )))
                        }
                    }
                }
            }
        }

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
            let result = match (offset, limit) {
                (Some(o), Some(l)) => {
                    rt.block_on(crate::commands::query::execute_tiles_query(client, o, l))
                }
                (Some(o), None) => {
                    rt.block_on(crate::commands::query::execute_tiles_query(client, o, 100))
                }
                (None, Some(l)) => {
                    rt.block_on(crate::commands::query::execute_tiles_query(client, 0, l))
                }
                (None, None) => rt.block_on(crate::commands::query::execute_all_tiles_query(client)),
            };
            match result {
                Ok(result) => {
                    let output = format_typed_output(&result, false)
                        .unwrap_or_else(|e| format!("Format error: {}", e));
                    ReplResult::Continue(Some(output))
                }
                Err(e) => ReplResult::Continue(Some(format!("Error: {}", e))),
            }
        }

        _ => match rt.block_on(execute_query(client, parts[0])) {
            Ok(result) => {
                let output = format_typed_output(&result, false)
                    .unwrap_or_else(|e| format!("Format error: {}", e));
                ReplResult::Continue(Some(output))
            }
            Err(e) => ReplResult::Continue(Some(format!("Error: {}", e))),
        },
    }
}

fn parse_tiles_args(args: &[&str]) -> (Option<u32>, Option<u32>) {
    let mut offset: Option<u32> = None;
    let mut limit: Option<u32> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i] {
            "--offset" if i + 1 < args.len() => {
                offset = args[i + 1].parse().ok();
                i += 2;
            }
            "--limit" if i + 1 < args.len() => {
                limit = args[i + 1].parse().ok().map(|l: u32| l.min(1000));
                i += 2;
            }
            _ => i += 1,
        }
    }

    (offset, limit)
}

fn parse_and_execute_command(
    client: &ApiClient,
    args: &[&str],
    rt: &Runtime,
) -> crate::error::Result<String> {
    if args.is_empty() {
        return Ok("No action specified. Use 'help commands' for available commands.".to_string());
    }

    // Build argument vector for Clap parsing
    let mut clap_args: Vec<&str> = vec!["owcli", "command"];
    clap_args.extend(args);

    // Use Clap to parse the action - this ensures REPL parsing matches CLI parsing
    match Cli::try_parse_from(&clap_args) {
        Ok(cli) => {
            if let Some(Commands::Command { action }) = cli.command {
                let response = rt.block_on(execute_action(client, &action))?;
                Ok(format_command_response(
                    command_succeeded(&response),
                    response.error.as_deref(),
                    response.request_id.as_deref(),
                    false,
                ))
            } else {
                Ok("Failed to parse command".to_string())
            }
        }
        Err(e) => {
            // Clap provides good error messages including suggestions
            Ok(e.to_string())
        }
    }
}
