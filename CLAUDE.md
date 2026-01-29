# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
cargo build              # Build debug version
cargo build --release    # Build release version
cargo test               # Run all tests
cargo test test_name     # Run a single test by name
cargo clippy             # Run linter
cargo fmt                # Format code
```

## Architecture Overview

This is a Rust CLI tool (`owcli`) for interacting with the Old World game API. It supports both direct command-line invocation and an interactive REPL mode.

### Core Components

- **src/main.rs** - Entry point, routes between direct CLI and REPL modes
- **src/cli.rs** - Clap CLI definition with 60+ game actions as subcommands
- **src/client.rs** - HTTP client (blocking reqwest) for API communication; POST requests to `/command` endpoint with UUID-based request IDs
- **src/config.rs** - Configuration from env vars (`OWCLI_HOST`, `OWCLI_PORT`) and CLI flags
- **src/path_parser.rs** - Parses and validates query paths (e.g., `player/0/units`, `tile/5/12`)

### Command Flow

1. **Queries** (`src/commands/query.rs`) - GET-style paths like `players`, `cities`, `player/0/units`
2. **Actions** (`src/commands/actions.rs`) - Game commands like `end-turn`, `move-unit`, `build-unit`

### Data & Output

- **src/models/** - Serde structs for game entities (Player, City, Unit, Character, Tile, etc.)
- **src/output/table.rs** - Type-specific table formatters for 30+ response types using `tabled` crate

### Interactive Mode

- **src/repl/** - REPL with history and auto-completion via `rustyline`
- Completer provides static paths and dynamic player resource paths

## Configuration

Environment variables (also via `.env` file):
- `OWCLI_HOST` - API host (default: localhost)
- `OWCLI_PORT` - API port (default: 9877)

CLI flags: `--host`, `--port`, `--json` (raw JSON output instead of tables)

## Git Policy

- Do not commit unless specifically asked to
- Never push to remote
