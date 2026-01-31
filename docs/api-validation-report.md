# API Spec Validation Report

Generated: 2026-01-31 13:55:05

## Summary

- **Endpoints tested:** 32
- **Successful:** 32
- **Failed to fetch:** 0
- **Unique schema issues:** 6222

### Issues by Type

| Type | Count |
|------|-------|
| Type mismatch | 68 |
| Extra field (in response, not in spec) | 1689 |
| Missing required field | 0 |
| Missing optional field | 4025 |
| Enum value mismatch | 0 |
| Unexpected null | 440 |

## Type Mismatches (Critical)

These fields have incorrect types in the spec and will cause deserialization failures.

### Character

| Field | Spec Type | Actual Type | Example Value |
|-------|-----------|-------------|---------------|
| `characters.explorerEffectUnits` | string | array<empty> | `[]` |
| `characters.generalEffectUnits` | string | array<empty> | `[]` |
| `characters.explorerEffectUnits` | string | array<empty> | `[]` |
| `characters.generalEffectUnits` | string | array<empty> | `[]` |
| `characters.explorerEffectUnits` | string | array<empty> | `[]` |
| `characters.generalEffectUnits` | string | array<string> | `['EFFECTUNIT_HERO']` |
| `characters.explorerEffectUnits` | string | array<empty> | `[]` |
| `characters.generalEffectUnits` | string | array<string> | `['EFFECTUNIT_COMMANDER']` |
| `characters.explorerEffectUnits` | string | array<empty> | `[]` |
| `characters.generalEffectUnits` | string | array<string> | `['EFFECTUNIT_COMMANDER', 'EFFECTUNIT_TRA` |
| `characters.explorerEffectUnits` | string | array<string> | `['EFFECTUNIT_LEADER_EXPLORER']` |
| `characters.generalEffectUnits` | string | array<string> | `['EFFECTUNIT_LEADER_GENERAL']` |
| `characters.explorerEffectUnits` | string | array<empty> | `[]` |
| `characters.generalEffectUnits` | string | array<empty> | `[]` |
| `characters.explorerEffectUnits` | string | array<empty> | `[]` |
| `characters.generalEffectUnits` | string | array<empty> | `[]` |
| `characters.explorerEffectUnits` | string | array<empty> | `[]` |
| `characters.generalEffectUnits` | string | array<string> | `['EFFECTUNIT_HERO', 'EFFECTUNIT_TOUGH']` |
| `characters.explorerEffectUnits` | string | array<string> | `['EFFECTUNIT_LEADER_EXPLORER']` |
| `characters.generalEffectUnits` | string | array<string> | `['EFFECTUNIT_TACTICIAN', 'EFFECTUNIT_TAC` |
| `explorerEffectUnits` | string | array<empty> | `[]` |
| `generalEffectUnits` | string | array<empty> | `[]` |
| `explorerEffectUnits` | string | array<empty> | `[]` |
| `generalEffectUnits` | string | array<empty> | `[]` |
| `explorerEffectUnits` | string | array<empty> | `[]` |
| `generalEffectUnits` | string | array<empty> | `[]` |
| `explorerEffectUnits` | string | array<empty> | `[]` |
| `generalEffectUnits` | string | array<string> | `['EFFECTUNIT_HERO']` |
| `explorerEffectUnits` | string | array<empty> | `[]` |
| `generalEffectUnits` | string | array<string> | `['EFFECTUNIT_COMMANDER']` |
| `explorerEffectUnits` | string | array<empty> | `[]` |
| `generalEffectUnits` | string | array<string> | `['EFFECTUNIT_COMMANDER', 'EFFECTUNIT_TRA` |

### City

| Field | Spec Type | Actual Type | Example Value |
|-------|-----------|-------------|---------------|
| `activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_MARBLE', 'EFFECTCI` |
| `cities.activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_MARBLE', 'EFFECTCI` |
| `cities.activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_GOLD', 'EFFECTCITY` |
| `cities.activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_FUR', 'EFFECTCITY_` |
| `cities.activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_PIG', 'EFFECTCITY_` |
| `cities.activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_ORE', 'EFFECTCITY_` |
| `cities.activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_SALT', 'EFFECTCITY` |
| `cities.activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_SALT', 'EFFECTCITY` |
| `cities.activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_SALT', 'EFFECTCITY` |
| `cities.activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_FISH', 'EFFECTCITY` |
| `cities.activeEffectCity` | string | array<string> | `['EFFECTCITY_BASE', 'EFFECTCITY_FAMILY_S` |
| `activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_MARBLE', 'EFFECTCI` |
| `activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_GOLD', 'EFFECTCITY` |
| `activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_FUR', 'EFFECTCITY_` |
| `activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_PIG', 'EFFECTCITY_` |
| `activeEffectCity` | string | array<string> | `['EFFECTCITY_RESOURCE_ORE', 'EFFECTCITY_` |

### Player

| Field | Spec Type | Actual Type | Example Value |
|-------|-----------|-------------|---------------|
| `families` | string | array<string> | `['FAMILY_FABIUS', 'FAMILY_CLAUDIUS', 'FA` |
| `firstPopup` | string | object | `{'mzText': '<color=#e3c08c><link="HELP_L` |
| `players.families` | string | array<string> | `['FAMILY_FABIUS', 'FAMILY_CLAUDIUS', 'FA` |
| `players.firstPopup` | string | object | `{'mzText': '<color=#e3c08c><link="HELP_L` |
| `players.families` | string | array<string> | `['FAMILY_KUSSARAN', 'FAMILY_NENASSAN', '` |
| `players.families` | string | array<string> | `['FAMILY_SARGONID', 'FAMILY_TUDIYA', 'FA` |
| `players.families` | string | array<string> | `['FAMILY_IRTJET', 'FAMILY_WAWAT', 'FAMIL` |
| `players.families` | string | array<string> | `['FAMILY_BARCID', 'FAMILY_MAGONID', 'FAM` |
| `families` | string | array<string> | `['FAMILY_FABIUS', 'FAMILY_CLAUDIUS', 'FA` |
| `firstPopup` | string | object | `{'mzText': '<color=#e3c08c><link="HELP_L` |
| `families` | string | array<string> | `['FAMILY_KUSSARAN', 'FAMILY_NENASSAN', '` |
| `families` | string | array<string> | `['FAMILY_SARGONID', 'FAMILY_TUDIYA', 'FA` |
| `families` | string | array<string> | `['FAMILY_IRTJET', 'FAMILY_WAWAT', 'FAMIL` |
| `families` | string | array<string> | `['FAMILY_BARCID', 'FAMILY_MAGONID', 'FAM` |

### Unit

| Field | Spec Type | Actual Type | Example Value |
|-------|-----------|-------------|---------------|
| `effectUnits` | string | array<string> | `['EFFECTUNIT_RANGED', 'EFFECTUNIT_MOUNTE` |
| `effectUnits` | string | array<string> | `['EFFECTUNIT_RANGED', 'EFFECTUNIT_MOUNTE` |
| `effectUnits` | string | array<string> | `['EFFECTUNIT_RANGED', 'EFFECTUNIT_MOUNTE` |
| `effectUnits` | string | array<string> | `['EFFECTUNIT_RANGED', 'EFFECTUNIT_MOUNTE` |
| `effectUnits` | string | array<string> | `['EFFECTUNIT_RANGED', 'EFFECTUNIT_MOUNTE` |
| `effectUnits` | string | array<string> | `['EFFECTUNIT_RANGED', 'EFFECTUNIT_MOUNTE` |

## Extra Fields (Spec Missing)

These fields appear in API responses but are not defined in the spec.

### Character

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `changeJobExtraOpinions` | object | `{'JOB_AMBASSADOR': 40, 'JOB_CHANCELLOR':` |
| `characters.changeJobExtraOpinions` | object | `{'JOB_AMBASSADOR': 40, 'JOB_CHANCELLOR':` |
| `characters.cognomenMinValues` | object | `{'COGNOMEN_WARRIOR': 950, 'COGNOMEN_BRAV` |
| `characters.hasCouncilPrereqs` | object | `{'COUNCIL_SPYMASTER': True}` |
| `characters.hasTraitReplacess` | object | `{'TRAIT_INSPIRING': True, 'TRAIT_ROMANTI` |
| `characters.isTraits` | object | `{'TRAIT_SCHEMER_ARCHETYPE': True, 'TRAIT` |
| `characters.jobOpinions` | object | `{'JOB_AMBASSADOR': 40, 'JOB_CHANCELLOR':` |
| `characters.makeCouncilExtraOpinions` | object | `{'COUNCIL_AMBASSADOR': 40, 'COUNCIL_CHAN` |
| `characters.ratings.RATING_CHARISMA` | integer | `-2` |
| `characters.ratings.RATING_DISCIPLINE` | integer | `0` |
| `characters.ratings.RATING_WISDOM` | integer | `4` |
| `characters.traitTurnLengths` | object | `{'TRAIT_SCHEMER_ARCHETYPE': 17, 'TRAIT_B` |
| `characters.traitTurns` | object | `{'TRAIT_SCHEMER_ARCHETYPE': 1, 'TRAIT_BI` |
| `characters.tribeEthnicitys` | object | `{'TRIBE_GAULS': 100}` |
| `characters.yieldRateCourtiers` | object | `{'YIELD_SCIENCE': 15}` |
| `characters.yieldRateLeaderSpouses` | object | `{'YIELD_SCIENCE': 25}` |
| `characters.yieldRateLeaders` | object | `{'YIELD_SCIENCE': 50}` |
| `characters.yieldRateSuccessors` | object | `{'YIELD_SCIENCE': 25}` |
| `characters.ratings.RATING_COURAGE` | integer | `-2` |
| `characters.familyOpinionCouncils` | object | `{'COUNCIL_SPYMASTER': 5}` |
| `characters.isJobs` | object | `{'JOB_GOVERNOR': True}` |
| `characters.nationEthnicitys` | object | `{'NATION_ROME': 100}` |
| `cognomenMinValues` | object | `{'COGNOMEN_WARRIOR': 950, 'COGNOMEN_BRAV` |
| `hasCouncilPrereqs` | object | `{'COUNCIL_SPYMASTER': True}` |
| `hasTraitReplacess` | object | `{'TRAIT_INSPIRING': True, 'TRAIT_ROMANTI` |
| `isTraits` | object | `{'TRAIT_SCHEMER_ARCHETYPE': True, 'TRAIT` |
| `jobOpinions` | object | `{'JOB_AMBASSADOR': 40, 'JOB_CHANCELLOR':` |
| `makeCouncilExtraOpinions` | object | `{'COUNCIL_AMBASSADOR': 40, 'COUNCIL_CHAN` |
| `ratings.RATING_CHARISMA` | integer | `-2` |
| `ratings.RATING_DISCIPLINE` | integer | `0` |
| `ratings.RATING_WISDOM` | integer | `4` |
| `traitTurnLengths` | object | `{'TRAIT_SCHEMER_ARCHETYPE': 17, 'TRAIT_B` |
| `traitTurns` | object | `{'TRAIT_SCHEMER_ARCHETYPE': 1, 'TRAIT_BI` |
| `tribeEthnicitys` | object | `{'TRIBE_GAULS': 100}` |
| `yieldRateCourtiers` | object | `{'YIELD_SCIENCE': 15}` |
| `yieldRateLeaderSpouses` | object | `{'YIELD_SCIENCE': 25}` |
| `yieldRateLeaders` | object | `{'YIELD_SCIENCE': 50}` |
| `yieldRateSuccessors` | object | `{'YIELD_SCIENCE': 25}` |
| `ratings.RATING_COURAGE` | integer | `-2` |
| `familyOpinionCouncils` | object | `{'COUNCIL_SPYMASTER': 5}` |

### City

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `activeImprovementClassCounts` | object | `{'IMPROVEMENTCLASS_MINE': 1, 'IMPROVEMEN` |
| `activeImprovementCounts` | object | `{'IMPROVEMENT_MINE': 1, 'IMPROVEMENT_QUA` |
| `baseYieldNets` | object | `{'YIELD_GROWTH': 170, 'YIELD_CIVICS': 17` |
| `cities.activeImprovementClassCounts` | object | `{'IMPROVEMENTCLASS_MINE': 1, 'IMPROVEMEN` |
| `cities.activeImprovementCounts` | object | `{'IMPROVEMENT_MINE': 1, 'IMPROVEMENT_QUA` |
| `cities.baseYieldNets` | object | `{'YIELD_GROWTH': 170, 'YIELD_CIVICS': 17` |
| `cities.happinessLevelYieldModifiers` | object | `{'YIELD_MAINTENANCE': 5}` |
| `cities.improvementClassCounts` | object | `{'IMPROVEMENTCLASS_MINE': 1, 'IMPROVEMEN` |
| `cities.improvementCounts` | object | `{'IMPROVEMENT_MINE': 1, 'IMPROVEMENT_QUA` |
| `cities.isReligionHolyCitys` | object | `{'RELIGION_PAGAN_ROME': True}` |
| `cities.isReligionSpreadEligibles` | object | `{'RELIGION_ZOROASTRIANISM': True, 'RELIG` |
| `cities.isReligions` | object | `{'RELIGION_PAGAN_ROME': True}` |
| `cities.isUnitSpawnPossibles` | object | `{'UNIT_SETTLER': True, 'UNIT_WORKER': Tr` |
| `cities.isYieldBuildCurrents` | object | `{'YIELD_GROWTH': True}` |
| `cities.makeGovernorCosts` | object | `{'YIELD_CIVICS': 100, 'YIELD_ORDERS': 2}` |
| `cities.specialistCostModifiers` | object | `{'SPECIALIST_FARMER': -25, 'SPECIALIST_M` |
| `cities.territoryTileCounts` | object | `{'TERRAIN_WATER': 2, 'TERRAIN_URBAN': 7,` |
| `cities.unitProductionCounts` | object | `{'UNIT_SETTLER': 3}` |
| `cities.yieldLevels` | object | `{'YIELD_GROWTH': 0, 'YIELD_CIVICS': 0, '` |
| `cities.yieldProgresss` | object | `{'YIELD_CULTURE': 620, 'YIELD_HAPPINESS'` |
| `cities.yieldThresholdWholes` | object | `{'YIELD_GROWTH': 100, 'YIELD_CULTURE': 1` |
| `cities.yieldThresholds` | object | `{'YIELD_GROWTH': 1000, 'YIELD_CULTURE': ` |
| `cities.yieldTurnsLefts` | object | `{'YIELD_GROWTH': 0, 'YIELD_CIVICS': 1, '` |
| `cities.improvementModifiers` | object | `{'IMPROVEMENT_NETS': 100}` |
| `cities.nextTurnOverflows` | object | `{'YIELD_CIVICS': 20}` |
| `cities.yieldOverflows` | object | `{'YIELD_GROWTH': 2, 'YIELD_CIVICS': 20}` |
| `cities.buildUnitLevelss` | object | `{'UNIT_SLINGER': 1, 'UNIT_ARCHER': 1, 'U` |
| `cities.hasProjects` | object | `{'PROJECT_TREASURY_1': True}` |
| `cities.projectCounts` | object | `{'PROJECT_TREASURY_1': 1}` |
| `cities.militaryUnitFamilyCounts` | object | `{'FAMILY_SARGONID': 2, 'FAMILY_TUDIYA': ` |
| `cities.improvementClassModifiers` | object | `{'IMPROVEMENTCLASS_SHRINE': 50}` |
| `happinessLevelYieldModifiers` | object | `{'YIELD_MAINTENANCE': 5}` |
| `improvementClassCounts` | object | `{'IMPROVEMENTCLASS_MINE': 1, 'IMPROVEMEN` |
| `improvementCounts` | object | `{'IMPROVEMENT_MINE': 1, 'IMPROVEMENT_QUA` |
| `isReligionHolyCitys` | object | `{'RELIGION_PAGAN_ROME': True}` |
| `isReligionSpreadEligibles` | object | `{'RELIGION_ZOROASTRIANISM': True, 'RELIG` |
| `isReligions` | object | `{'RELIGION_PAGAN_ROME': True}` |
| `isUnitSpawnPossibles` | object | `{'UNIT_SETTLER': True, 'UNIT_WORKER': Tr` |
| `isYieldBuildCurrents` | object | `{'YIELD_GROWTH': True}` |
| `makeGovernorCosts` | object | `{'YIELD_CIVICS': 100, 'YIELD_ORDERS': 2}` |
| `specialistCostModifiers` | object | `{'SPECIALIST_FARMER': -25, 'SPECIALIST_M` |
| `territoryTileCounts` | object | `{'TERRAIN_WATER': 2, 'TERRAIN_URBAN': 7,` |
| `unitProductionCounts` | object | `{'UNIT_SETTLER': 3}` |
| `yieldLevels` | object | `{'YIELD_GROWTH': 0, 'YIELD_CIVICS': 0, '` |
| `yieldProgresss` | object | `{'YIELD_CULTURE': 620, 'YIELD_HAPPINESS'` |
| `yieldThresholdWholes` | object | `{'YIELD_GROWTH': 100, 'YIELD_CULTURE': 1` |
| `yieldThresholds` | object | `{'YIELD_GROWTH': 1000, 'YIELD_CULTURE': ` |
| `yieldTurnsLefts` | object | `{'YIELD_GROWTH': 0, 'YIELD_CIVICS': 1, '` |
| `improvementModifiers` | object | `{'IMPROVEMENT_NETS': 100}` |
| `nextTurnOverflows` | object | `{'YIELD_CIVICS': 20}` |
| `yieldOverflows` | object | `{'YIELD_GROWTH': 2, 'YIELD_CIVICS': 20}` |
| `buildUnitLevelss` | object | `{'UNIT_SLINGER': 1, 'UNIT_ARCHER': 1, 'U` |
| `hasProjects` | object | `{'PROJECT_TREASURY_1': True}` |
| `projectCounts` | object | `{'PROJECT_TREASURY_1': 1}` |

### PaginatedTiles

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `pagination.hasMore` | boolean | `True` |
| `pagination.limit` | integer | `10` |
| `pagination.offset` | integer | `0` |
| `pagination.total` | integer | `5476` |

### Player

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `activeImprovementClassCounts` | object | `{'IMPROVEMENTCLASS_MINE': 1, 'IMPROVEMEN` |
| `activeImprovementCounts` | object | `{'IMPROVEMENT_MINE': 1, 'IMPROVEMENT_QUA` |
| `adoptReligionCosts` | object | `{'RELIGION_ZOROASTRIANISM': 400, 'RELIGI` |
| `effectCityRebelProbs` | object | `{'EFFECTCITY_OPINIONFAMILY_FURIOUS': 15,` |
| `effectPlayerCounts` | object | `{'EFFECTPLAYER_BASE': 1, 'EFFECTPLAYER_D` |
| `familyControls` | object | `{'FAMILY_FABIUS': 1, 'FAMILY_JULIUS': 2}` |
| `familyHeadIDs` | object | `{'FAMILY_FABIUS': 52, 'FAMILY_CLAUDIUS':` |
| `familyOpinionRates` | object | `{'FAMILY_FABIUS': 57, 'FAMILY_JULIUS': 1` |
| `familySeatCityIDs` | object | `{'FAMILY_FABIUS': 13, 'FAMILY_CLAUDIUS':` |
| `familyTurnsNoLeaders` | object | `{'FAMILY_FABIUS': 13, 'FAMILY_CLAUDIUS':` |
| `giftYieldQuantitys` | object | `{'YIELD_CIVICS': 100, 'YIELD_TRAINING': ` |
| `goalStartedCounts` | object | `{'GOAL_SIX_QUARRIES': 1}` |
| `hasFamilyHeads` | object | `{'FAMILY_FABIUS': True, 'FAMILY_CLAUDIUS` |
| `hasFamilyOpinions` | object | `{'FAMILY_FABIUS': True, 'FAMILY_CLAUDIUS` |
| `hasFamilySeatCitys` | object | `{'FAMILY_FABIUS': True, 'FAMILY_CLAUDIUS` |
| `hasHeightClaimeds` | object | `{'HEIGHT_COAST': True, 'HEIGHT_FLAT': Tr` |
| `hasReligionOpinionHistorys` | object | `{'RELIGION_PAGAN_ROME': True, 'RELIGION_` |
| `hasReligions` | object | `{'RELIGION_PAGAN_ROME': True}` |
| `hasTerrainClaimeds` | object | `{'TERRAIN_WATER': True, 'TERRAIN_URBAN':` |
| `hasTraitCharacters` | object | `{'TRAIT_HERO_ARCHETYPE': True, 'TRAIT_CO` |
| `hasVegetationClaimeds` | object | `{'VEGETATION_TREES': True, 'VEGETATION_S` |
| `improvementClassCounts` | object | `{'IMPROVEMENTCLASS_MINE': 2, 'IMPROVEMEN` |
| `improvementCounts` | object | `{'IMPROVEMENT_MINE': 2, 'IMPROVEMENT_QUA` |
| `improvementLawsRequireds` | object | `{'IMPROVEMENT_GARRISON_1': 1, 'IMPROVEME` |
| `isActiveLaws` | object | `{'LAW_PRIMOGENITURE': True, 'LAW_FREEDOM` |
| `isCityYieldBuildCurrents` | object | `{'YIELD_GROWTH': True}` |
| `isCouncilUnlocks` | object | `{'COUNCIL_GRAND_VIZIER': True}` |
| `isFamilyStarteds` | object | `{'FAMILY_FABIUS': True, 'FAMILY_CLAUDIUS` |
| `isHideUnitUnlocks` | object | `{'UNIT_SCOUT': True}` |
| `isImprovementUnlockeds` | object | `{'IMPROVEMENT_ANCIENT_RUINS': True, 'IMP` |
| `isLatestUpgrades` | object | `{'UNIT_SCOUT': True, 'UNIT_CARAVAN': Tru` |
| `isMinTreatyTurnss` | object | `{'TRIBE_VANDALS': True}` |
| `isNationImprovements` | object | `{'IMPROVEMENT_ANCIENT_RUINS': True, 'IMP` |
| `isTechAcquireds` | object | `{'TECH_IRONWORKING': True, 'TECH_STONECU` |
| `isTechAnyDecks` | object | `{'TECH_STONECUTTING_BONUS_STONE': True, ` |
| `isTechAvailables` | object | `{'TECH_ARISTOCRACY': True, 'TECH_FORESTR` |
| `isTechHides` | object | `{'TECH_STONECUTTING_BONUS_STONE': True, ` |
| `isTechLockeds` | object | `{'TECH_EVENT_RESOURCE_EBONY_BONUS': True` |
| `isTechPassedCurrents` | object | `{'TECH_TRAPPING': True, 'TECH_ADMINISTRA` |
| `isTechPasseds` | object | `{'TECH_TRAPPING': True, 'TECH_ADMINISTRA` |
| `isUnitUnlockeds` | object | `{'UNIT_SETTLER': True, 'UNIT_WORKER': Tr` |
| `isWorldOrOurPaganReligions` | object | `{'RELIGION_ZOROASTRIANISM': True, 'RELIG` |
| `players.activeImprovementClassCounts` | object | `{'IMPROVEMENTCLASS_MINE': 1, 'IMPROVEMEN` |
| `players.activeImprovementCounts` | object | `{'IMPROVEMENT_MINE': 1, 'IMPROVEMENT_QUA` |
| `players.adoptReligionCosts` | object | `{'RELIGION_ZOROASTRIANISM': 400, 'RELIGI` |
| `players.effectCityRebelProbs` | object | `{'EFFECTCITY_OPINIONFAMILY_FURIOUS': 15,` |
| `players.effectPlayerCounts` | object | `{'EFFECTPLAYER_BASE': 1, 'EFFECTPLAYER_D` |
| `players.familyControls` | object | `{'FAMILY_FABIUS': 1, 'FAMILY_JULIUS': 2}` |
| `players.familyHeadIDs` | object | `{'FAMILY_FABIUS': 52, 'FAMILY_CLAUDIUS':` |
| `players.familyOpinionRates` | object | `{'FAMILY_FABIUS': 57, 'FAMILY_JULIUS': 1` |
| `players.familySeatCityIDs` | object | `{'FAMILY_FABIUS': 13, 'FAMILY_CLAUDIUS':` |
| `players.familyTurnsNoLeaders` | object | `{'FAMILY_FABIUS': 13, 'FAMILY_CLAUDIUS':` |
| `players.giftYieldQuantitys` | object | `{'YIELD_CIVICS': 100, 'YIELD_TRAINING': ` |
| `players.goalStartedCounts` | object | `{'GOAL_SIX_QUARRIES': 1}` |
| `players.hasFamilyHeads` | object | `{'FAMILY_FABIUS': True, 'FAMILY_CLAUDIUS` |
| `players.hasFamilyOpinions` | object | `{'FAMILY_FABIUS': True, 'FAMILY_CLAUDIUS` |
| `players.hasFamilySeatCitys` | object | `{'FAMILY_FABIUS': True, 'FAMILY_CLAUDIUS` |
| `players.hasHeightClaimeds` | object | `{'HEIGHT_COAST': True, 'HEIGHT_FLAT': Tr` |
| `players.hasReligionOpinionHistorys` | object | `{'RELIGION_PAGAN_ROME': True, 'RELIGION_` |
| `players.hasReligions` | object | `{'RELIGION_PAGAN_ROME': True}` |
| `players.hasTerrainClaimeds` | object | `{'TERRAIN_WATER': True, 'TERRAIN_URBAN':` |
| `players.hasTraitCharacters` | object | `{'TRAIT_HERO_ARCHETYPE': True, 'TRAIT_CO` |
| `players.hasVegetationClaimeds` | object | `{'VEGETATION_TREES': True, 'VEGETATION_S` |
| `players.improvementClassCounts` | object | `{'IMPROVEMENTCLASS_MINE': 2, 'IMPROVEMEN` |
| `players.improvementCounts` | object | `{'IMPROVEMENT_MINE': 2, 'IMPROVEMENT_QUA` |
| `players.improvementLawsRequireds` | object | `{'IMPROVEMENT_GARRISON_1': 1, 'IMPROVEME` |
| `players.isActiveLaws` | object | `{'LAW_PRIMOGENITURE': True, 'LAW_FREEDOM` |
| `players.isCityYieldBuildCurrents` | object | `{'YIELD_GROWTH': True}` |
| `players.isCouncilUnlocks` | object | `{'COUNCIL_GRAND_VIZIER': True}` |
| `players.isFamilyStarteds` | object | `{'FAMILY_FABIUS': True, 'FAMILY_CLAUDIUS` |
| `players.isHideUnitUnlocks` | object | `{'UNIT_SCOUT': True}` |
| `players.isImprovementUnlockeds` | object | `{'IMPROVEMENT_ANCIENT_RUINS': True, 'IMP` |
| `players.isLatestUpgrades` | object | `{'UNIT_SCOUT': True, 'UNIT_CARAVAN': Tru` |
| `players.isMinTreatyTurnss` | object | `{'TRIBE_VANDALS': True}` |
| `players.isNationImprovements` | object | `{'IMPROVEMENT_ANCIENT_RUINS': True, 'IMP` |
| `players.isTechAcquireds` | object | `{'TECH_IRONWORKING': True, 'TECH_STONECU` |
| `players.isTechAnyDecks` | object | `{'TECH_STONECUTTING_BONUS_STONE': True, ` |
| `players.isTechAvailables` | object | `{'TECH_ARISTOCRACY': True, 'TECH_FORESTR` |
| `players.isTechHides` | object | `{'TECH_STONECUTTING_BONUS_STONE': True, ` |
| `players.isTechLockeds` | object | `{'TECH_EVENT_RESOURCE_EBONY_BONUS': True` |
| `players.isTechPassedCurrents` | object | `{'TECH_TRAPPING': True, 'TECH_ADMINISTRA` |
| `players.isTechPasseds` | object | `{'TECH_TRAPPING': True, 'TECH_ADMINISTRA` |
| `players.isUnitUnlockeds` | object | `{'UNIT_SETTLER': True, 'UNIT_WORKER': Tr` |
| `players.isWorldOrOurPaganReligions` | object | `{'RELIGION_ZOROASTRIANISM': True, 'RELIG` |
| `players.religionCounts` | object | `{'RELIGION_PAGAN_ROME': 1}` |
| `players.religionOpinionRates` | object | `{'RELIGION_PAGAN_ROME': 42, 'RELIGION_PA` |
| `players.resourceRevealeds` | object | `{'RESOURCE_MARBLE': 3, 'RESOURCE_SALT': ` |
| `players.startLawCosts` | object | `{'LAW_PRIMOGENITURE': 400, 'LAW_ULTIMOGE` |
| `players.techCostWholes` | object | `{'TECH_IRONWORKING': 80, 'TECH_STONECUTT` |
| `players.techCosts` | object | `{'TECH_IRONWORKING': 800, 'TECH_STONECUT` |
| `players.techProgresss` | object | `{'TECH_DIVINATION': 810, 'TECH_LABOR_FOR` |
| `players.techTurnDiscovereds` | object | `{'TECH_IRONWORKING': 1, 'TECH_STONECUTTI` |
| `players.unitsProducedTurns` | object | `{'UNIT_SETTLER': 4, 'UNIT_WORKER': 1, 'U` |
| `players.unitsProduceds` | object | `{'UNIT_SETTLER': 4, 'UNIT_WORKER': 1, 'U` |
| `players.yieldStockpileWholes` | object | `{'YIELD_GROWTH': 0, 'YIELD_CIVICS': 562,` |
| `players.yieldStockpiles` | object | `{'YIELD_GROWTH': 0, 'YIELD_CIVICS': 5620` |
| `players.yieldTotals` | object | `{'YIELD_GROWTH': 5840, 'YIELD_CIVICS': 8` |
| `players.isTechTrasheds` | object | `{'TECH_ADMINISTRATION_BONUS_WORKER': Tru` |
| `players.missionStartedTurns` | object | `{'MISSION_INFLUENCE': 6}` |
| `players.projectsProduceds` | object | `{'PROJECT_TREASURY_1': 1, 'PROJECT_FESTI` |
| `players.isFamilyReligions` | object | `{'FAMILY_WAWAT': True}` |
| `players.councilCharacters` | object | `{'COUNCIL_AMBASSADOR': 47}` |
| `players.hasCouncilCharacters` | object | `{'COUNCIL_AMBASSADOR': True}` |
| `religionCounts` | object | `{'RELIGION_PAGAN_ROME': 1}` |
| `religionOpinionRates` | object | `{'RELIGION_PAGAN_ROME': 42, 'RELIGION_PA` |
| `resourceRevealeds` | object | `{'RESOURCE_MARBLE': 3, 'RESOURCE_SALT': ` |
| `startLawCosts` | object | `{'LAW_PRIMOGENITURE': 400, 'LAW_ULTIMOGE` |
| `techCostWholes` | object | `{'TECH_IRONWORKING': 80, 'TECH_STONECUTT` |
| `techCosts` | object | `{'TECH_IRONWORKING': 800, 'TECH_STONECUT` |
| `techProgresss` | object | `{'TECH_DIVINATION': 810, 'TECH_LABOR_FOR` |
| `techTurnDiscovereds` | object | `{'TECH_IRONWORKING': 1, 'TECH_STONECUTTI` |
| `unitsProducedTurns` | object | `{'UNIT_SETTLER': 4, 'UNIT_WORKER': 1, 'U` |
| `unitsProduceds` | object | `{'UNIT_SETTLER': 4, 'UNIT_WORKER': 1, 'U` |
| `yieldStockpileWholes` | object | `{'YIELD_GROWTH': 0, 'YIELD_CIVICS': 562,` |
| `yieldStockpiles` | object | `{'YIELD_GROWTH': 0, 'YIELD_CIVICS': 5620` |
| `yieldTotals` | object | `{'YIELD_GROWTH': 5840, 'YIELD_CIVICS': 8` |
| `isTechTrasheds` | object | `{'TECH_ADMINISTRATION_BONUS_WORKER': Tru` |
| `missionStartedTurns` | object | `{'MISSION_INFLUENCE': 6}` |
| `projectsProduceds` | object | `{'PROJECT_TREASURY_1': 1, 'PROJECT_FESTI` |
| `isFamilyReligions` | object | `{'FAMILY_WAWAT': True}` |
| `councilCharacters` | object | `{'COUNCIL_AMBASSADOR': 47}` |
| `hasCouncilCharacters` | object | `{'COUNCIL_AMBASSADOR': True}` |

### PlayerFamilies

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `families.family` | string | `FAMILY_FABIUS` |
| `families.headId` | integer | `52` |
| `families.opinion` | string | `OPINIONFAMILY_CAUTIOUS` |
| `families.opinionRate` | integer | `57` |
| `families.seatCityId` | integer | `13` |

### PlayerGoals

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `goals.finished` | boolean | `False` |
| `goals.id` | integer | `0` |
| `goals.isLegacy` | boolean | `False` |
| `goals.isQuest` | boolean | `False` |
| `goals.maxTurns` | integer | `0` |
| `goals.turn` | integer | `12` |
| `goals.type` | string | `GOAL_SIX_QUARRIES` |

### PlayerLaws

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `activeLaws.LAWCLASS_ORDER` | string | `LAW_PRIMOGENITURE` |
| `activeLaws.LAWCLASS_SLAVERY_FREEDOM` | string | `LAW_FREEDOM` |

### PlayerReligion

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `religionCounts.RELIGION_PAGAN_ROME` | integer | `1` |

### PlayerResources

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `revealed.RESOURCE_CAMEL` | integer | `1` |
| `revealed.RESOURCE_CITRUS` | integer | `1` |
| `revealed.RESOURCE_CRAB` | integer | `1` |
| `revealed.RESOURCE_GAME` | integer | `5` |
| `revealed.RESOURCE_GOAT` | integer | `2` |
| `revealed.RESOURCE_GOLD` | integer | `1` |
| `revealed.RESOURCE_HORSE` | integer | `3` |
| `revealed.RESOURCE_INCENSE` | integer | `1` |
| `revealed.RESOURCE_LAVENDER` | integer | `1` |
| `revealed.RESOURCE_MARBLE` | integer | `3` |
| `revealed.RESOURCE_SALT` | integer | `1` |
| `revealed.RESOURCE_SHEEP` | integer | `1` |
| `revealed.RESOURCE_SILVER` | integer | `2` |
| `revealed.RESOURCE_SORGHUM` | integer | `2` |

### PlayerTechs

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `progress.TECH_ARISTOCRACY` | integer | `1034` |
| `progress.TECH_DIVINATION` | integer | `810` |
| `progress.TECH_LABOR_FORCE` | integer | `1315` |

### TeamDiplomacy

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `teamDiplomacy.conflictNumTurns` | integer | `18` |
| `teamDiplomacy.diplomacyBlockTurn` | integer | `0` |
| `teamDiplomacy.diplomacyBlockTurns` | integer | `0` |
| `teamDiplomacy.diplomacyNumTurns` | integer | `18` |
| `teamDiplomacy.diplomacyTurn` | integer | `0` |
| `conflictNumTurns` | integer | `18` |
| `diplomacyBlockTurn` | integer | `0` |
| `diplomacyBlockTurns` | integer | `0` |
| `diplomacyNumTurns` | integer | `18` |
| `diplomacyTurn` | integer | `0` |

### Tile

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `isImprovementBorderSpreads` | object | `{'IMPROVEMENT_ANCIENT_RUINS': True, 'IMP` |
| `isResourceValids` | object | `{'RESOURCE_DYE': True, 'RESOURCE_PEARL':` |
| `isSpecialistCostCitizens` | object | `{'SPECIALIST_FARMER': True, 'SPECIALIST_` |
| `tiles.isImprovementBorderSpreads` | object | `{'IMPROVEMENT_ANCIENT_RUINS': True, 'IMP` |
| `tiles.isResourceValids` | object | `{'RESOURCE_DYE': True, 'RESOURCE_PEARL':` |
| `tiles.isSpecialistCostCitizens` | object | `{'SPECIALIST_FARMER': True, 'SPECIALIST_` |

### Tribe

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `allyTeam` | null | `None` |
| `cityIds` | array<empty> | `[]` |
| `hasPlayerAlly` | boolean | `False` |
| `numCities` | integer | `0` |
| `numTribeImprovements` | integer | `0` |
| `numUnits` | integer | `0` |
| `settlementTileIds` | array<empty> | `[]` |
| `strength` | integer | `1` |
| `tribes.allyTeam` | null | `None` |
| `tribes.cityIds` | array<empty> | `[]` |
| `tribes.hasPlayerAlly` | boolean | `False` |
| `tribes.numCities` | integer | `0` |
| `tribes.numTribeImprovements` | integer | `0` |
| `tribes.numUnits` | integer | `0` |
| `tribes.settlementTileIds` | array<empty> | `[]` |
| `tribes.strength` | integer | `1` |

### TribeDiplomacy

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `tribeDiplomacy.conflictNumTurns` | integer | `2` |
| `tribeDiplomacy.conflictTurn` | integer | `16` |
| `tribeDiplomacy.diplomacyBlockTurn` | integer | `24` |
| `tribeDiplomacy.diplomacyBlockTurns` | integer | `6` |
| `tribeDiplomacy.diplomacyNumTurns` | integer | `2` |
| `tribeDiplomacy.diplomacyTurn` | integer | `16` |
| `tribeDiplomacy.toTeam` | integer | `0` |
| `tribeDiplomacy.tribe` | string | `TRIBE_VANDALS` |
| `tribeDiplomacy.warScore` | integer | `0` |
| `tribeDiplomacy.warState` | string | `WARSTATE_NEUTRAL` |
| `conflictNumTurns` | integer | `2` |
| `conflictTurn` | integer | `16` |
| `diplomacyBlockTurn` | integer | `24` |
| `diplomacyBlockTurns` | integer | `6` |
| `diplomacyNumTurns` | integer | `2` |
| `diplomacyTurn` | integer | `16` |
| `toTeam` | integer | `0` |
| `tribe` | string | `TRIBE_VANDALS` |
| `warScore` | integer | `0` |
| `warState` | string | `WARSTATE_NEUTRAL` |

### Unit

| Field | Actual Type | Example Value |
|-------|-------------|---------------|
| `effectUnitCounts` | object | `{'EFFECTUNIT_RANGED': 1, 'EFFECTUNIT_MOU` |
| `hasEffectUnits` | object | `{'EFFECTUNIT_RANGED': True, 'EFFECTUNIT_` |
| `tradeOutpostCosts` | object | `{'TRIBE_REBELS': 500, 'TRIBE_ANARCHY': 5` |
| `hasPromotions` | object | `{'PROMOTION_STRIKE1': True}` |
| `isGeneralEffectUnits` | object | `{'EFFECTUNIT_HIGHLANDER': True, 'EFFECTU` |

## Endpoints Tested

| Endpoint | Schema | Status |
|----------|--------|--------|
| `/state` | GameState | 2766 issues |
| `/config` | GameConfig | ok |
| `/players` | Player | 471 issues |
| `/cities` | City | 334 issues |
| `/characters` | Character | 728 issues |
| `/units` | Unit | 470 issues |
| `/tribes` | Tribe | 40 issues |
| `/religions` | Religion | ok |
| `/map` | MapMetadata | ok |
| `/tiles?limit=10` | PaginatedTiles | 774 issues |
| `/team-diplomacy` | TeamDiplomacy | 25 issues |
| `/team-alliances` | TeamAlliance | ok |
| `/tribe-diplomacy` | TribeDiplomacy | 60 issues |
| `/tribe-alliances` | TribeAlliance | ok |
| `/character-events` | CharacterEvent | ok |
| `/unit-events` | UnitEvent | ok |
| `/city-events` | CityEvent | ok |
| `/player/0` | Player | 95 issues |
| `/player/0/units` | Unit | 440 issues |
| `/player/0/techs` | PlayerTechs | 3 issues |
| `/player/0/families` | PlayerFamilies | 15 issues |
| `/player/0/religion` | PlayerReligion | 1 issues |
| `/player/0/goals` | PlayerGoals | 7 issues |
| `/player/0/decisions` | PlayerDecisions | ok |
| `/player/0/laws` | PlayerLaws | 2 issues |
| `/player/0/missions` | PlayerMissions | ok |
| `/player/0/resources` | PlayerResources | 14 issues |
| `/city/0` | City | 63 issues |
| `/character/0` | Character | 146 issues |
| `/unit/8` | Unit | 94 issues |
| `/tile/0` | Tile | 77 issues |
| `/tribe/TRIBE_REBELS` | Tribe | 8 issues |
