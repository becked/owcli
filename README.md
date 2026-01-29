# owcli

A command-line interface for the [Old World](https://oldworldgame.com) game API. Query game state, execute commands, and visualize the map from your terminal.

## Installation

```bash
cargo build --release
```

The binary will be at `target/release/owcli`.

## Configuration

Set the API server address via environment variables or CLI flags:

```bash
# Environment variables (or .env file)
export OWCLI_HOST=localhost
export OWCLI_PORT=9877

# Or use CLI flags
owcli --host localhost --port 9877 players
```

## Usage

### Interactive Mode (REPL)

Run `owcli` without arguments to enter interactive mode:

```
$ owcli
owcli> players
owcli> player/0/units
owcli> command move-unit --unit 1 --target 100
owcli> help commands
owcli> exit
```

Features tab completion for commands, queries, and player resource paths.

### Direct CLI Mode

Execute a single query or command:

```bash
# Queries
owcli players                    # List all players
owcli player/0                   # Get player 0's details
owcli player/0/units             # Get player 0's units
owcli cities                     # List all cities
owcli city/5                     # Get city by ID
owcli tile/10/20                 # Get tile at coordinates (10, 20)
owcli state                      # Full game state

# Commands
owcli command move-unit --unit 1 --target 100
owcli command end-turn
owcli command build-unit --city 3 --unit-type Archer
```

### Map View

Render a colored minimap of the game world:

```bash
owcli map
```

### JSON Output

Use `--json` for raw JSON output (useful for scripting):

```bash
owcli --json player/0 | jq '.player_name'
```

## Queries

Query game state with path-style syntax:

| Path | Description |
|------|-------------|
| `state` | Full game snapshot |
| `players` | All players |
| `player/<index>` | Specific player |
| `player/<index>/units` | Player's units |
| `player/<index>/cities` | Player's cities |
| `player/<index>/techs` | Player's technologies |
| `player/<index>/families` | Player's families |
| `player/<index>/decisions` | Pending decisions |
| `cities` | All cities |
| `city/<id>` | City by ID |
| `units` | All units |
| `unit/<id>` | Unit by ID |
| `characters` | All characters |
| `character/<id>` | Character by ID |
| `tiles` | All map tiles |
| `tile/<id>` | Tile by ID |
| `tile/<x>/<y>` | Tile by coordinates |
| `tribes` | All tribes |
| `religions` | All religions |
| `map` | Map data |

Run `owcli help queries` for the complete list.

## Commands

Execute game actions with `owcli command <action>`:

**Unit Movement**
- `move-unit --unit <id> --target <tile-id>` - Move a unit
- `attack --unit <id> --target <tile-id>` - Attack
- `fortify --unit <id>` - Fortify unit
- `pass --unit <id>` - Pass unit's turn
- `skip --unit <id>` - Skip unit
- `sleep --unit <id>` - Put unit to sleep
- `wake --unit <id>` - Wake unit

**City Production**
- `build-unit --city <id> --unit-type <type>` - Build a unit
- `build-project --city <id> --project <type>` - Build a project
- `hurry-civics --city <id>` - Rush with civics
- `hurry-money --city <id>` - Rush with money

**Diplomacy**
- `declare-war --player <id>` - Declare war
- `make-peace --player <id>` - Make peace
- `declare-truce --player <id>` - Declare truce

**Other**
- `end-turn` - End your turn
- `research --tech <type>` - Research technology
- `make-decision --decision <id> --choice <index>` - Make a decision

Run `owcli help commands` for all 53 available commands.

## Bulk Commands

Execute multiple commands in a single request:

```bash
owcli bulk --file commands.json
```

JSON format:
```json
[
  {"action": "moveUnit", "params": {"unitId": 1, "targetTileId": 100}},
  {"action": "moveUnit", "params": {"unitId": 2, "targetTileId": 101}},
  {"action": "endTurn", "params": {}}
]
```

Options:
- `--continue-on-error` - Don't stop on first failure
- `--file -` - Read from stdin

## Help

```bash
owcli help              # Overview
owcli help queries      # List all query paths
owcli help commands     # List all game commands
owcli help move-unit    # Help for specific command
```

## Requirements

Requires the [Old World API Endpoint](https://github.com/becked/OldWorldAPIEndpoint) mod to be running with your game.

## License

MIT
