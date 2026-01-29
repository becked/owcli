# API Spec Validation Report

Generated: 2026-01-28 22:23:10

## Summary

- **Endpoints tested:** 31
- **Successful:** 30
- **Failed to fetch:** 1
- **With schema issues:** 0
- **Total issues found:** 0

## Connection Errors

- `/unit/0`: 404 Not Found

## No Schema Discrepancies Found

## Endpoints Tested

| Endpoint | Schema | Status | Issues |
|----------|--------|--------|--------|
| `/state` | GameState | ok | 0 |
| `/config` | GameConfig | ok | 0 |
| `/players` | Player | ok | 0 |
| `/player/0` | Player | ok | 0 |
| `/player/0/units` | Unit | ok | 0 |
| `/player/0/techs` | PlayerTechs | ok | 0 |
| `/player/0/families` | PlayerFamilies | ok | 0 |
| `/player/0/religion` | PlayerReligion | ok | 0 |
| `/player/0/goals` | PlayerGoals | ok | 0 |
| `/player/0/decisions` | PlayerDecisions | ok | 0 |
| `/player/0/laws` | PlayerLaws | ok | 0 |
| `/player/0/missions` | PlayerMissions | ok | 0 |
| `/player/0/resources` | PlayerResources | ok | 0 |
| `/cities` | City | ok | 0 |
| `/city/0` | City | ok | 0 |
| `/characters` | Character | ok | 0 |
| `/character/0` | Character | ok | 0 |
| `/units` | Unit | ok | 0 |
| `/unit/0` | Unit | error: 404 Not Found | 0 |
| `/tribes` | Tribe | ok | 0 |
| `/religions` | Religion | ok | 0 |
| `/map` | MapMetadata | ok | 0 |
| `/tiles?limit=10` | TilesPaginated | ok | 0 |
| `/tile/0` | Tile | ok | 0 |
| `/team-diplomacy` | TeamDiplomacy | ok | 0 |
| `/team-alliances` | TeamAlliance | ok | 0 |
| `/tribe-diplomacy` | TribeDiplomacy | ok | 0 |
| `/tribe-alliances` | TribeAlliance | ok | 0 |
| `/character-events` | CharacterEvent | ok | 0 |
| `/unit-events` | UnitEvent | ok | 0 |
| `/city-events` | CityEvent | ok | 0 |
