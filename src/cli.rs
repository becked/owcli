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
    #[arg(long, global = true, env = "OWCLI_JSON")]
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

    /// Fetch all tiles (or use --offset/--limit for manual pagination)
    Tiles {
        /// Offset for pagination (enables manual pagination mode)
        #[arg(long)]
        offset: Option<u32>,

        /// Limit for pagination (enables manual pagination mode)
        #[arg(long)]
        limit: Option<u32>,
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

    /// Render a hex map of the game world
    Map,
}

// Action enum and to_game_command() impl are generated from openapi.yaml
// Regenerate with: cargo run --bin gen > src/cli_generated.rs
include!("cli_generated.rs");
