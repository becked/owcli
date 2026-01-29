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

- **src/client.rs** - Exposes `client::types::*` (progenitor-generated from OpenAPI spec)
- **src/output/table.rs** - Type-specific table formatters for 30+ response types using `tabled` crate

### Interactive Mode

- **src/repl/** - REPL with history and auto-completion via `rustyline`
- Completer provides static paths and dynamic player resource paths

## Configuration

Environment variables (also via `.env` file):
- `OWCLI_HOST` - API host (default: localhost)
- `OWCLI_PORT` - API port (default: 9877)

CLI flags: `--host`, `--port`, `--json` (raw JSON output instead of tables)

## Code Generation (Important)

This project uses [progenitor](https://github.com/oxidecomputer/progenitor) to generate Rust types and client code from the OpenAPI spec.

### Generated Code Location
- **Source**: `openapi.yaml` (local copy of upstream spec)
- **Generator**: `build.rs` runs progenitor at compile time
- **Output**: `target/debug/build/owcli-*/out/codegen.rs`
- **Access**: Via `crate::client::types::*`

### Rules for Working with API Types

**ALWAYS use generated types** for anything defined in the OpenAPI spec:
- Request/response structs (`GameCommand`, `CommandResult`, `BulkCommandResult`, etc.)
- Enum variants (`GameCommandAction`)
- Entity types (`Player`, `City`, `Unit`, `Tile`, etc.)

**NEVER hand-write structs** for things that exist in the spec. If you need a type that should come from the API:
1. Check if it exists in `client::types::*`
2. If missing, the `openapi.yaml` may need updating from upstream
3. Run `cargo build` to regenerate after spec changes

### Updating the Spec
```bash
curl -o openapi.yaml https://raw.githubusercontent.com/becked/OldWorldAPIEndpoint/main/docs/openapi.yaml
cargo build  # Regenerates types
```

## API Reference

This CLI implements the Old World API as specified in:
https://github.com/becked/OldWorldAPIEndpoint/blob/main/docs/openapi.yaml

The local `openapi.yaml` should be kept in sync with the upstream spec.

### Supported Commands (53 total)
- Unit Movement (11): moveUnit, attack, fortify, pass, skip, sleep, sentry, wake, heal, march, lock
- Unit Actions (6): disband, promote, pillage, burn, upgrade, spreadReligion
- Worker (3): buildImprovement, upgradeImprovement, addRoad
- City Foundation (2): foundCity, joinCity
- City Production (9): build, buildUnit, buildProject, buildQueue, hurryCivics, hurryTraining, hurryMoney, hurryPopulation, hurryOrders
- Research & Decisions (5): research, redrawTech, targetTech, makeDecision, removeDecision
- Diplomacy (9): declareWar, makePeace, declareTruce, declareWarTribe, makePeaceTribe, declareTruceTribe, giftCity, giftYield, allyTribe
- Character Management (7): assignGovernor, releaseGovernor, assignGeneral, releaseGeneral, assignAgent, releaseAgent, startMission
- Turn (1): endTurn

### Bulk Commands
Use `owcli bulk --file commands.json` to execute multiple commands.
JSON format: `[{"action": "moveUnit", "params": {"unitId": 1, "targetTileId": 100}}, ...]`

## Git Policy

- Do not commit unless specifically asked to
- Never push to remote
