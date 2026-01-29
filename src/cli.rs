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

/// Game commands matching OpenAPI spec (168 commands)
/// See: https://github.com/becked/OldWorldAPIEndpoint/blob/main/docs/openapi.yaml
#[derive(Subcommand)]
pub enum Action {
    // ===== Unit Movement (11) =====
    /// Move a unit to a target tile
    MoveUnit {
        /// Unit ID to move
        #[arg(long)]
        unit: i32,
        /// Target tile ID
        #[arg(long)]
        target: i32,
        /// Queue move instead of immediate
        #[arg(long)]
        queue: bool,
        /// March to target
        #[arg(long)]
        march: bool,
        /// Waypoint tile ID
        #[arg(long)]
        waypoint: Option<i32>,
    },

    /// Attack with a unit
    Attack {
        /// Unit ID to attack with
        #[arg(long)]
        unit: i32,
        /// Target tile ID to attack
        #[arg(long)]
        target: i32,
    },

    /// Fortify a unit
    Fortify {
        /// Unit ID to fortify
        #[arg(long)]
        unit: i32,
    },

    /// Pass a unit's turn
    Pass {
        /// Unit ID to pass
        #[arg(long)]
        unit: i32,
    },

    /// Skip a unit
    Skip {
        /// Unit ID to skip
        #[arg(long)]
        unit: i32,
    },

    /// Put a unit to sleep
    Sleep {
        /// Unit ID to put to sleep
        #[arg(long)]
        unit: i32,
    },

    /// Set a unit to sentry mode
    Sentry {
        /// Unit ID to set to sentry mode
        #[arg(long)]
        unit: i32,
    },

    /// Wake a unit
    Wake {
        /// Unit ID to wake
        #[arg(long)]
        unit: i32,
    },

    /// Heal a unit
    Heal {
        /// Unit ID to heal
        #[arg(long)]
        unit: i32,
        /// Auto-heal
        #[arg(long)]
        auto: bool,
    },

    /// March a unit
    March {
        /// Unit ID to march
        #[arg(long)]
        unit: i32,
    },

    /// Lock a unit
    Lock {
        /// Unit ID to lock
        #[arg(long)]
        unit: i32,
    },

    // ===== Unit Actions (6) =====
    /// Disband a unit
    Disband {
        /// Unit ID to disband
        #[arg(long)]
        unit: i32,
        /// Force disbanding
        #[arg(long)]
        force: bool,
    },

    /// Promote a unit
    Promote {
        /// Unit ID to promote
        #[arg(long)]
        unit: i32,
        /// Promotion type (e.g., PROMOTION_FIERCE)
        #[arg(long)]
        promotion: String,
    },

    /// Pillage a tile
    Pillage {
        /// Unit ID to pillage with
        #[arg(long)]
        unit: i32,
    },

    /// Burn a tile
    Burn {
        /// Unit ID to burn with
        #[arg(long)]
        unit: i32,
    },

    /// Upgrade a unit
    Upgrade {
        /// Unit ID to upgrade
        #[arg(long)]
        unit: i32,
        /// Unit type to upgrade to
        #[arg(long, name = "type")]
        unit_type: String,
        /// Buy goods for upgrade
        #[arg(long)]
        buy_goods: bool,
    },

    /// Spread religion with a unit
    SpreadReligion {
        /// Unit ID (missionary)
        #[arg(long)]
        unit: i32,
        /// Target city ID
        #[arg(long)]
        city: i32,
    },

    // ===== Worker Commands (3) =====
    /// Build an improvement
    BuildImprovement {
        /// Worker unit ID
        #[arg(long)]
        unit: i32,
        /// Improvement type to build
        #[arg(long, name = "type")]
        improvement_type: String,
        /// Tile ID to build on
        #[arg(long)]
        tile: i32,
        /// Buy goods
        #[arg(long)]
        buy_goods: bool,
        /// Queue the build
        #[arg(long)]
        queue: bool,
    },

    /// Upgrade an improvement
    UpgradeImprovement {
        /// Worker unit ID
        #[arg(long)]
        unit: i32,
        /// Buy goods
        #[arg(long)]
        buy_goods: bool,
    },

    /// Add a road
    AddRoad {
        /// Worker unit ID
        #[arg(long)]
        unit: i32,
        /// Tile ID to build road on
        #[arg(long)]
        tile: i32,
        /// Buy goods
        #[arg(long)]
        buy_goods: bool,
        /// Queue the build
        #[arg(long)]
        queue: bool,
    },

    // ===== City Foundation (2) =====
    /// Found a city with a settler
    FoundCity {
        /// Settler unit ID
        #[arg(long)]
        unit: i32,
        /// Family type for the city
        #[arg(long)]
        family: String,
        /// Nation type (optional)
        #[arg(long)]
        nation: Option<String>,
    },

    /// Join a city with a unit
    JoinCity {
        /// Unit ID to join
        #[arg(long)]
        unit: i32,
    },

    // ===== City Production (9) =====
    /// Generic build action in a city
    Build {
        /// City ID to build in
        #[arg(long)]
        city: i32,
        /// Unit type to build
        #[arg(long, name = "type")]
        unit_type: String,
        /// Buy goods
        #[arg(long)]
        buy_goods: bool,
        /// Add to front of queue
        #[arg(long)]
        first: bool,
    },

    /// Build a unit in a city
    BuildUnit {
        /// City ID to build in
        #[arg(long)]
        city: i32,
        /// Unit type to build
        #[arg(long, name = "type")]
        unit_type: String,
        /// Buy goods
        #[arg(long)]
        buy_goods: bool,
        /// Add to front of queue
        #[arg(long)]
        first: bool,
    },

    /// Build a project in a city
    BuildProject {
        /// City ID to build in
        #[arg(long)]
        city: i32,
        /// Project type to build
        #[arg(long, name = "type")]
        project_type: String,
        /// Buy goods
        #[arg(long)]
        buy_goods: bool,
        /// Add to front of queue
        #[arg(long)]
        first: bool,
        /// Repeat when complete
        #[arg(long)]
        repeat: bool,
    },

    /// Reorder build queue
    BuildQueue {
        /// City ID
        #[arg(long)]
        city: i32,
        /// Old slot position
        #[arg(long)]
        old_slot: i32,
        /// New slot position
        #[arg(long)]
        new_slot: i32,
    },

    /// Hurry production with civics
    HurryCivics {
        /// City ID
        #[arg(long)]
        city: i32,
    },

    /// Hurry production with training
    HurryTraining {
        /// City ID
        #[arg(long)]
        city: i32,
    },

    /// Hurry production with money
    HurryMoney {
        /// City ID
        #[arg(long)]
        city: i32,
    },

    /// Hurry production with population
    HurryPopulation {
        /// City ID
        #[arg(long)]
        city: i32,
    },

    /// Hurry production with orders
    HurryOrders {
        /// City ID
        #[arg(long)]
        city: i32,
    },

    // ===== Research & Decisions (5) =====
    /// Research a technology
    Research {
        /// Technology type to research (e.g., TECH_FORESTRY)
        #[arg(long)]
        tech: String,
    },

    /// Redraw available technologies
    RedrawTech,

    /// Set a target technology
    TargetTech {
        /// Technology type to target
        #[arg(long, name = "type")]
        tech_type: String,
    },

    /// Make a decision
    MakeDecision {
        /// Decision ID
        #[arg(long)]
        decision: i32,
        /// Choice index (0-based)
        #[arg(long)]
        choice: i32,
        /// Optional data parameter
        #[arg(long)]
        data: Option<i32>,
    },

    /// Remove a decision
    RemoveDecision {
        /// Decision ID to remove
        #[arg(long)]
        decision: i32,
    },

    // ===== Diplomacy (9) =====
    /// Declare war on a player
    DeclareWar {
        /// Target player index
        #[arg(long)]
        player: i32,
    },

    /// Make peace with a player
    MakePeace {
        /// Target player index
        #[arg(long)]
        player: i32,
    },

    /// Declare a truce with a player
    DeclareTruce {
        /// Target player index
        #[arg(long)]
        player: i32,
    },

    /// Declare war on a tribe
    DeclareWarTribe {
        /// Tribe type
        #[arg(long)]
        tribe: String,
    },

    /// Make peace with a tribe
    MakePeaceTribe {
        /// Tribe type
        #[arg(long)]
        tribe: String,
    },

    /// Declare a truce with a tribe
    DeclareTruceTribe {
        /// Tribe type
        #[arg(long)]
        tribe: String,
    },

    /// Gift a city
    GiftCity {
        /// City ID to gift
        #[arg(long)]
        city: i32,
        /// Target player index
        #[arg(long)]
        player: i32,
    },

    /// Gift yield/resources
    GiftYield {
        /// Yield type to gift
        #[arg(long, name = "type")]
        yield_type: String,
        /// Target player index
        #[arg(long)]
        player: i32,
        /// Reverse (request instead of gift)
        #[arg(long)]
        reverse: bool,
    },

    /// Ally with a tribe
    AllyTribe {
        /// Tribe type
        #[arg(long)]
        tribe: String,
    },

    // ===== Character Management (7) =====
    /// Assign a governor to a city
    AssignGovernor {
        /// City ID to assign governor to
        #[arg(long)]
        city: i32,
        /// Character ID to assign
        #[arg(long)]
        character: i32,
    },

    /// Release a governor from a city
    ReleaseGovernor {
        /// City ID to release governor from
        #[arg(long)]
        city: i32,
    },

    /// Assign a general to a unit
    AssignGeneral {
        /// Unit ID to assign general to
        #[arg(long)]
        unit: i32,
        /// Character ID to assign
        #[arg(long)]
        character: i32,
    },

    /// Release a general from a unit
    ReleaseGeneral {
        /// Unit ID to release general from
        #[arg(long)]
        unit: i32,
    },

    /// Assign an agent to a city
    AssignAgent {
        /// City ID to assign agent to
        #[arg(long)]
        city: i32,
        /// Character ID to assign
        #[arg(long)]
        character: i32,
    },

    /// Release an agent from a city
    ReleaseAgent {
        /// City ID to release agent from
        #[arg(long)]
        city: i32,
    },

    /// Start a mission
    StartMission {
        /// Mission type
        #[arg(long)]
        mission: String,
        /// Character ID
        #[arg(long)]
        character: i32,
        /// Target (optional)
        #[arg(long)]
        target: Option<String>,
        /// Cancel existing mission
        #[arg(long)]
        cancel: bool,
    },

    // ===== Turn Control (1) =====
    /// End the current turn
    EndTurn {
        /// Force end turn even with pending actions
        #[arg(long)]
        force: bool,
    },

    // ===== Laws & Economy (7) =====
    /// Enact a law
    ChooseLaw {
        /// Law type to enact (e.g., LAW_SLAVERY)
        #[arg(long)]
        law: String,
    },

    /// Cancel an active law
    CancelLaw {
        /// Law type to cancel
        #[arg(long)]
        law: String,
    },

    /// Buy resources with money
    BuyYield {
        /// Yield type to buy (e.g., YIELD_TRAINING)
        #[arg(long, name = "type")]
        yield_type: String,
        /// Amount to buy
        #[arg(long)]
        amount: i32,
    },

    /// Sell resources for money
    SellYield {
        /// Yield type to sell
        #[arg(long, name = "type")]
        yield_type: String,
        /// Amount to sell
        #[arg(long)]
        amount: i32,
    },

    /// Convert orders to money
    ConvertOrders,

    /// Convert legitimacy
    ConvertLegitimacy,

    /// Convert orders to science
    ConvertOrdersToScience,

    // ===== Luxury Trading (5) =====
    /// Toggle luxury trading for a city
    TradeCityLuxury {
        /// City ID
        #[arg(long)]
        city: i32,
        /// Resource type (e.g., RESOURCE_WINE)
        #[arg(long)]
        resource: String,
        /// Enable trading
        #[arg(long)]
        enable: bool,
    },

    /// Toggle luxury trading for a family
    TradeFamilyLuxury {
        /// Family type
        #[arg(long)]
        family: String,
        /// Resource type
        #[arg(long)]
        resource: String,
        /// Enable trading
        #[arg(long)]
        enable: bool,
    },

    /// Toggle luxury trading with a tribe
    TradeTribeLuxury {
        /// Tribe type
        #[arg(long)]
        tribe: String,
        /// Resource type
        #[arg(long)]
        resource: String,
        /// Enable trading
        #[arg(long)]
        enable: bool,
    },

    /// Toggle luxury trading with another player
    TradePlayerLuxury {
        /// Target player index
        #[arg(long)]
        player: i32,
        /// Resource type
        #[arg(long)]
        resource: String,
        /// Enable trading
        #[arg(long)]
        enable: bool,
    },

    /// Send tribute to a player or tribe
    Tribute {
        /// Target player index (use this OR --to-tribe)
        #[arg(long)]
        to_player: Option<i32>,
        /// Target tribe type (use this OR --to-player)
        #[arg(long)]
        to_tribe: Option<String>,
        /// Yield type to send
        #[arg(long, name = "type")]
        yield_type: String,
        /// Amount to send
        #[arg(long)]
        amount: i32,
    },

    // ===== Unit Special Actions (20) =====
    /// Swap unit position with another unit
    Swap {
        /// Unit ID to swap
        #[arg(long)]
        unit: i32,
        /// Target unit ID to swap with
        #[arg(long)]
        target_unit: i32,
    },

    /// Execute unit's queued actions
    DoUnitQueue {
        /// Unit ID
        #[arg(long)]
        unit: i32,
    },

    /// Cancel unit's queued actions
    CancelUnitQueue {
        /// Unit ID
        #[arg(long)]
        unit: i32,
    },

    /// Set unit formation
    Formation {
        /// Unit ID
        #[arg(long)]
        unit: i32,
        /// Effect unit type (e.g., EFFECTUNIT_TESTUDO)
        #[arg(long)]
        effect_type: String,
    },

    /// Unlimber artillery unit
    Unlimber {
        /// Unit ID
        #[arg(long)]
        unit: i32,
    },

    /// Anchor ship
    Anchor {
        /// Unit ID
        #[arg(long)]
        unit: i32,
    },

    /// Repair a unit
    Repair {
        /// Unit ID
        #[arg(long)]
        unit: i32,
        /// Buy goods for repair
        #[arg(long)]
        buy_goods: bool,
    },

    /// Cancel improvement being built
    CancelImprovement {
        /// Unit ID
        #[arg(long)]
        unit: i32,
    },

    /// Remove vegetation from tile
    RemoveVegetation {
        /// Unit ID
        #[arg(long)]
        unit: i32,
    },

    /// Harvest resource with worker
    HarvestResource {
        /// Unit ID
        #[arg(long)]
        unit: i32,
    },

    /// Toggle unit automation
    UnitAutomate {
        /// Unit ID
        #[arg(long)]
        unit: i32,
        /// Enable automation
        #[arg(long)]
        enable: bool,
    },

    /// Add urban tile
    AddUrban {
        /// Unit ID
        #[arg(long)]
        unit: i32,
        /// Tile ID
        #[arg(long)]
        tile: i32,
    },

    /// Build road to tiles
    RoadTo {
        /// Unit ID
        #[arg(long)]
        unit: i32,
        /// Target tile ID
        #[arg(long)]
        target: i32,
    },

    /// Buy a tile for a city
    BuyTile {
        /// City ID
        #[arg(long)]
        city: i32,
        /// Tile ID to buy
        #[arg(long)]
        tile: i32,
    },

    /// Recruit mercenary
    RecruitMercenary {
        /// Unit type to recruit
        #[arg(long, name = "type")]
        unit_type: String,
        /// Tile ID for placement
        #[arg(long)]
        tile: i32,
    },

    /// Hire mercenary
    HireMercenary {
        /// Unit ID
        #[arg(long)]
        unit: i32,
    },

    /// Gift unit to another player
    GiftUnit {
        /// Unit ID to gift
        #[arg(long)]
        unit: i32,
        /// Target player index
        #[arg(long)]
        player: i32,
    },

    /// Launch offensive with general
    LaunchOffensive {
        /// Unit ID
        #[arg(long)]
        unit: i32,
    },

    /// Apply effect to unit
    ApplyEffectUnit {
        /// Unit ID
        #[arg(long)]
        unit: i32,
        /// Effect type
        #[arg(long)]
        effect: String,
    },

    /// Select a unit
    SelectUnit {
        /// Unit ID
        #[arg(long)]
        unit: i32,
    },

    // ===== Agent & Caravan (4) =====
    /// Create agent network in city
    CreateAgentNetwork {
        /// Agent unit ID
        #[arg(long)]
        unit: i32,
        /// Target city ID
        #[arg(long)]
        city: i32,
    },

    /// Create trade outpost on tile
    CreateTradeOutpost {
        /// Caravan unit ID
        #[arg(long)]
        unit: i32,
        /// Tile ID
        #[arg(long)]
        tile: i32,
    },

    /// Start caravan mission
    CaravanMissionStart {
        /// Caravan unit ID
        #[arg(long)]
        unit: i32,
        /// Target city ID
        #[arg(long)]
        target: i32,
    },

    /// Cancel caravan mission
    CaravanMissionCancel {
        /// Caravan unit ID
        #[arg(long)]
        unit: i32,
    },

    // ===== Religious Units (3) =====
    /// Purge religion from city
    PurgeReligion {
        /// Religious unit ID
        #[arg(long)]
        unit: i32,
        /// Target city ID
        #[arg(long)]
        city: i32,
    },

    /// Spread religion to tribe
    SpreadReligionTribe {
        /// Religious unit ID
        #[arg(long)]
        unit: i32,
        /// Target tribe type
        #[arg(long)]
        tribe: String,
    },

    /// Establish theology
    EstablishTheology {
        /// Religious unit ID
        #[arg(long)]
        unit: i32,
        /// Theology type (e.g., THEOLOGY_CLERGY)
        #[arg(long)]
        theology: String,
    },

    // ===== Character Management Extended (13) =====
    /// Set character name
    CharacterName {
        /// Character ID
        #[arg(long)]
        character: i32,
        /// New name
        #[arg(long)]
        name: String,
    },

    /// Add or remove character trait
    AddCharacterTrait {
        /// Character ID
        #[arg(long)]
        character: i32,
        /// Trait type (e.g., TRAIT_WARRIOR)
        #[arg(long, name = "trait")]
        trait_type: String,
        /// Remove instead of add
        #[arg(long)]
        remove: bool,
    },

    /// Set character rating value
    SetCharacterRating {
        /// Character ID
        #[arg(long)]
        character: i32,
        /// Rating type (e.g., RATING_COURAGE)
        #[arg(long)]
        rating: String,
        /// New value
        #[arg(long)]
        value: i32,
    },

    /// Set character XP
    SetCharacterExperience {
        /// Character ID
        #[arg(long)]
        character: i32,
        /// XP amount
        #[arg(long)]
        xp: i32,
    },

    /// Set character cognomen/title
    SetCharacterCognomen {
        /// Character ID
        #[arg(long)]
        character: i32,
        /// Cognomen type (e.g., COGNOMEN_GREAT)
        #[arg(long)]
        cognomen: String,
    },

    /// Set character nation
    SetCharacterNation {
        /// Character ID
        #[arg(long)]
        character: i32,
        /// Nation type
        #[arg(long)]
        nation: String,
    },

    /// Set character family
    SetCharacterFamily {
        /// Character ID
        #[arg(long)]
        character: i32,
        /// Family type
        #[arg(long)]
        family: String,
    },

    /// Set character religion
    SetCharacterReligion {
        /// Character ID
        #[arg(long)]
        character: i32,
        /// Religion type
        #[arg(long)]
        religion: String,
    },

    /// Set character courtier role
    SetCharacterCourtier {
        /// Character ID
        #[arg(long)]
        character: i32,
        /// Courtier type
        #[arg(long)]
        courtier: String,
    },

    /// Set character council position
    SetCharacterCouncil {
        /// Character ID
        #[arg(long)]
        character: i32,
        /// Council type
        #[arg(long)]
        council: String,
    },

    /// Set player leader
    PlayerLeader {
        /// Player index
        #[arg(long)]
        player: i32,
        /// Character ID to make leader
        #[arg(long)]
        character: i32,
    },

    /// Set family head
    FamilyHead {
        /// Family type
        #[arg(long)]
        family: String,
        /// Character ID to make head
        #[arg(long)]
        character: i32,
    },

    /// Pin character in UI
    PinCharacter {
        /// Character ID
        #[arg(long)]
        character: i32,
        /// Pin (true) or unpin (false)
        #[arg(long)]
        pin: bool,
    },

    // ===== City Management (8) =====
    /// Rename a city
    CityRename {
        /// City ID
        #[arg(long)]
        city: i32,
        /// New name
        #[arg(long)]
        name: String,
    },

    /// Toggle city automation
    CityAutomate {
        /// City ID
        #[arg(long)]
        city: i32,
        /// Enable automation
        #[arg(long)]
        enable: bool,
    },

    /// Build specialist on tile
    BuildSpecialist {
        /// Tile ID
        #[arg(long)]
        tile: i32,
        /// Specialist type (e.g., SPECIALIST_SAGE)
        #[arg(long, name = "type")]
        specialist_type: String,
    },

    /// Set tile specialist
    SetSpecialist {
        /// Tile ID
        #[arg(long)]
        tile: i32,
        /// Specialist type
        #[arg(long, name = "type")]
        specialist_type: String,
    },

    /// Change city citizen count
    ChangeCitizens {
        /// City ID
        #[arg(long)]
        city: i32,
        /// Delta (positive to add, negative to remove)
        #[arg(long)]
        delta: i32,
    },

    /// Add/remove religion in city
    ChangeReligion {
        /// City ID
        #[arg(long)]
        city: i32,
        /// Religion type
        #[arg(long)]
        religion: String,
        /// Add (true) or remove (false)
        #[arg(long)]
        add: bool,
    },

    /// Change city family
    ChangeFamily {
        /// City ID
        #[arg(long)]
        city: i32,
        /// Family type
        #[arg(long)]
        family: String,
    },

    /// Change family seat city
    ChangeFamilySeat {
        /// Family type
        #[arg(long)]
        family: String,
        /// City ID for new seat
        #[arg(long)]
        city: i32,
    },

    // ===== Goals & Communication (9) =====
    /// Abandon an ambition
    AbandonAmbition {
        /// Ambition type (optional)
        #[arg(long)]
        ambition: Option<String>,
    },

    /// Add a goal to player
    AddPlayerGoal {
        /// Player index
        #[arg(long)]
        player: i32,
        /// Goal type
        #[arg(long)]
        goal: String,
    },

    /// Remove player goal
    RemovePlayerGoal {
        /// Player index
        #[arg(long)]
        player: i32,
        /// Goal type
        #[arg(long)]
        goal: String,
    },

    /// Trigger event story
    EventStory {
        /// Event type
        #[arg(long)]
        event: String,
    },

    /// Complete or fail a goal
    FinishGoal {
        /// Goal type
        #[arg(long)]
        goal: String,
        /// Success (true) or failure (false)
        #[arg(long)]
        success: bool,
    },

    /// Send chat message
    Chat {
        /// Message text
        #[arg(long)]
        message: String,
    },

    /// Ping a map location
    Ping {
        /// Tile ID to ping
        #[arg(long)]
        tile: i32,
    },

    /// Create custom reminder
    CustomReminder {
        /// Reminder text
        #[arg(long)]
        text: String,
        /// Turn number for reminder
        #[arg(long)]
        turn: i32,
    },

    /// Clear chat history
    ClearChat,

    // ===== Game State & Turn (7) =====
    /// Extend game time
    ExtendTime {
        /// Minutes to extend
        #[arg(long)]
        minutes: i32,
    },

    /// Pause the game
    Pause {
        /// Enable pause
        #[arg(long)]
        enable: bool,
    },

    /// Undo last action
    Undo,

    /// Redo undone action
    Redo,

    /// Replay previous turns
    ReplayTurn {
        /// Turn number to replay from
        #[arg(long)]
        turn: i32,
    },

    /// Let AI finish turn
    AiFinishTurn,

    /// Toggle turn replay setting
    ToggleNoReplay,

    // ===== Diplomacy Extended (3) =====
    /// Create team alliance
    TeamAlliance {
        /// Team number
        #[arg(long)]
        team: i32,
        /// Ally team number
        #[arg(long)]
        ally_team: i32,
        /// Enable alliance
        #[arg(long)]
        enable: bool,
    },

    /// Trigger tribe invasion
    TribeInvasion {
        /// Tribe type
        #[arg(long)]
        tribe: String,
    },

    /// Set victory for team
    VictoryTeam {
        /// Team number
        #[arg(long)]
        team: i32,
    },

    // ===== Editor/Debug - Units (8) =====
    /// Create a unit at a tile
    CreateUnit {
        /// Tile ID for placement
        #[arg(long)]
        tile: i32,
        /// Unit type to create
        #[arg(long, name = "type")]
        unit_type: String,
        /// Player index (owner)
        #[arg(long)]
        player: i32,
    },

    /// Set unit name
    UnitName {
        /// Unit ID
        #[arg(long)]
        unit: i32,
        /// New name
        #[arg(long)]
        name: String,
    },

    /// Change unit's family
    SetUnitFamily {
        /// Unit ID
        #[arg(long)]
        unit: i32,
        /// Family type
        #[arg(long)]
        family: String,
    },

    /// Transfer unit to player/tribe
    ChangeUnitOwner {
        /// Unit ID
        #[arg(long)]
        unit: i32,
        /// New player owner (use this OR --tribe)
        #[arg(long)]
        player: Option<i32>,
        /// New tribe owner (use this OR --player)
        #[arg(long)]
        tribe: Option<String>,
    },

    /// Modify unit cooldown
    ChangeCooldown {
        /// Unit ID
        #[arg(long)]
        unit: i32,
        /// Delta to add
        #[arg(long)]
        delta: i32,
    },

    /// Modify unit damage
    ChangeDamage {
        /// Unit ID
        #[arg(long)]
        unit: i32,
        /// Delta to add
        #[arg(long)]
        delta: i32,
    },

    /// Increase unit level
    UnitIncrementLevel {
        /// Unit ID
        #[arg(long)]
        unit: i32,
    },

    /// Add/remove promotion
    UnitChangePromotion {
        /// Unit ID
        #[arg(long)]
        unit: i32,
        /// Promotion type
        #[arg(long)]
        promotion: String,
        /// Remove instead of add
        #[arg(long)]
        remove: bool,
    },

    // ===== Editor/Debug - Cities (8) =====
    /// Create a city at a tile
    CreateCity {
        /// Tile ID for placement
        #[arg(long)]
        tile: i32,
        /// Player index (owner)
        #[arg(long)]
        player: i32,
        /// Family type
        #[arg(long)]
        family: String,
    },

    /// Remove a city
    RemoveCity {
        /// City ID
        #[arg(long)]
        city: i32,
    },

    /// Change city owner
    CityOwner {
        /// City ID
        #[arg(long)]
        city: i32,
        /// New player owner
        #[arg(long)]
        player: i32,
    },

    /// Modify city damage
    ChangeCityDamage {
        /// City ID
        #[arg(long)]
        city: i32,
        /// Delta to add
        #[arg(long)]
        delta: i32,
    },

    /// Grow/shrink city culture
    ChangeCulture {
        /// City ID
        #[arg(long)]
        city: i32,
        /// Delta to add
        #[arg(long)]
        delta: i32,
    },

    /// Modify build progress
    ChangeCityBuildTurns {
        /// City ID
        #[arg(long)]
        city: i32,
        /// Delta to add
        #[arg(long)]
        delta: i32,
    },

    /// Modify discontent level
    ChangeCityDiscontentLevel {
        /// City ID
        #[arg(long)]
        city: i32,
        /// Delta to add
        #[arg(long)]
        delta: i32,
    },

    /// Modify project progress
    ChangeProject {
        /// City ID
        #[arg(long)]
        city: i32,
        /// Project type
        #[arg(long)]
        project: String,
        /// Delta to add
        #[arg(long)]
        delta: i32,
    },

    // ===== Editor/Debug - Tiles (9) =====
    /// Set tile terrain type
    SetTerrain {
        /// Tile ID
        #[arg(long)]
        tile: i32,
        /// Terrain type
        #[arg(long, name = "type")]
        terrain_type: String,
    },

    /// Set tile height
    SetTerrainHeight {
        /// Tile ID
        #[arg(long)]
        tile: i32,
        /// Height type
        #[arg(long)]
        height: String,
    },

    /// Set tile vegetation
    SetVegetation {
        /// Tile ID
        #[arg(long)]
        tile: i32,
        /// Vegetation type
        #[arg(long, name = "type")]
        vegetation_type: String,
    },

    /// Set tile resource
    SetResource {
        /// Tile ID
        #[arg(long)]
        tile: i32,
        /// Resource type
        #[arg(long, name = "type")]
        resource_type: String,
    },

    /// Add/remove road
    SetRoad {
        /// Tile ID
        #[arg(long)]
        tile: i32,
        /// Enable road
        #[arg(long)]
        enable: bool,
    },

    /// Set tile improvement
    SetImprovement {
        /// Tile ID
        #[arg(long)]
        tile: i32,
        /// Improvement type
        #[arg(long, name = "type")]
        improvement_type: String,
    },

    /// Set tile owner
    SetTileOwner {
        /// Tile ID
        #[arg(long)]
        tile: i32,
        /// Player index (optional)
        #[arg(long)]
        player: Option<i32>,
        /// City ID (optional)
        #[arg(long)]
        city: Option<i32>,
    },

    /// Set city site marker
    SetCitySite {
        /// Tile ID
        #[arg(long)]
        tile: i32,
        /// Enable city site
        #[arg(long)]
        enable: bool,
    },

    /// Set improvement build turns
    ImprovementBuildTurns {
        /// Tile ID
        #[arg(long)]
        tile: i32,
        /// Turns remaining
        #[arg(long)]
        turns: i32,
    },

    // ===== Editor/Debug - Map & Player (6) =====
    /// Reveal entire map
    MapReveal {
        /// Team index (-1 for all teams)
        #[arg(long, default_value = "-1")]
        team: i32,
    },

    /// Hide entire map
    MapUnreveal {
        /// Team index (-1 for all teams)
        #[arg(long, default_value = "-1")]
        team: i32,
    },

    /// Grant technology to player
    AddTech {
        /// Player index
        #[arg(long)]
        player: i32,
        /// Technology type
        #[arg(long)]
        tech: String,
    },

    /// Add yield to player stockpile
    AddYield {
        /// Player index
        #[arg(long)]
        player: i32,
        /// Yield type
        #[arg(long, name = "type")]
        yield_type: String,
        /// Amount to add
        #[arg(long)]
        amount: i32,
    },

    /// Add money to player
    AddMoney {
        /// Player index
        #[arg(long)]
        player: i32,
        /// Amount to add
        #[arg(long)]
        amount: i32,
    },

    /// Execute cheat hotkey
    Cheat {
        /// Cheat key
        #[arg(long)]
        key: String,
    },

    // ===== Editor/Debug - Characters (5) =====
    /// Kill a character
    MakeCharacterDead {
        /// Character ID
        #[arg(long)]
        character: i32,
    },

    /// Make character immune to death
    MakeCharacterSafe {
        /// Character ID
        #[arg(long)]
        character: i32,
        /// Enable safety
        #[arg(long)]
        enable: bool,
    },

    /// Create random character
    NewCharacter {
        /// Player index
        #[arg(long)]
        player: i32,
    },

    /// Add specific character type
    AddCharacter {
        /// Player index
        #[arg(long)]
        player: i32,
        /// Character type
        #[arg(long, name = "type")]
        character_type: String,
    },

    /// Set tribe leader
    TribeLeader {
        /// Tribe type
        #[arg(long)]
        tribe: String,
        /// Character ID
        #[arg(long)]
        character: i32,
    },
}

impl Action {
    /// Convert CLI action to generated GameCommand type
    pub fn to_game_command(&self) -> crate::client::types::GameCommand {
        use crate::client::types::{GameCommand, GameCommandAction};
        use serde_json::json;

        // Helper to convert json Value to Map
        fn to_map(v: serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
            match v {
                serde_json::Value::Object(m) => m,
                _ => serde_json::Map::new(),
            }
        }

        let (action, params) = match self {
            // ===== Unit Movement =====
            Action::MoveUnit { unit, target, queue, march, waypoint } => {
                let mut p = json!({ "unitId": unit, "targetTileId": target });
                if *queue { p["queue"] = json!(true); }
                if *march { p["march"] = json!(true); }
                if let Some(wp) = waypoint { p["waypointTileId"] = json!(wp); }
                (GameCommandAction::MoveUnit, p)
            }
            Action::Attack { unit, target } =>
                (GameCommandAction::Attack, json!({ "unitId": unit, "targetTileId": target })),
            Action::Fortify { unit } => (GameCommandAction::Fortify, json!({ "unitId": unit })),
            Action::Pass { unit } => (GameCommandAction::Pass, json!({ "unitId": unit })),
            Action::Skip { unit } => (GameCommandAction::Skip, json!({ "unitId": unit })),
            Action::Sleep { unit } => (GameCommandAction::Sleep, json!({ "unitId": unit })),
            Action::Sentry { unit } => (GameCommandAction::Sentry, json!({ "unitId": unit })),
            Action::Wake { unit } => (GameCommandAction::Wake, json!({ "unitId": unit })),
            Action::Heal { unit, auto } => {
                let mut p = json!({ "unitId": unit });
                if *auto { p["auto"] = json!(true); }
                (GameCommandAction::Heal, p)
            }
            Action::March { unit } => (GameCommandAction::March, json!({ "unitId": unit })),
            Action::Lock { unit } => (GameCommandAction::Lock, json!({ "unitId": unit })),

            // ===== Unit Actions =====
            Action::Disband { unit, force } => {
                let mut p = json!({ "unitId": unit });
                if *force { p["force"] = json!(true); }
                (GameCommandAction::Disband, p)
            }
            Action::Promote { unit, promotion } =>
                (GameCommandAction::Promote, json!({ "unitId": unit, "promotion": promotion })),
            Action::Pillage { unit } => (GameCommandAction::Pillage, json!({ "unitId": unit })),
            Action::Burn { unit } => (GameCommandAction::Burn, json!({ "unitId": unit })),
            Action::Upgrade { unit, unit_type, buy_goods } => {
                let mut p = json!({ "unitId": unit, "unitType": unit_type });
                if *buy_goods { p["buyGoods"] = json!(true); }
                (GameCommandAction::Upgrade, p)
            }
            Action::SpreadReligion { unit, city } =>
                (GameCommandAction::SpreadReligion, json!({ "unitId": unit, "cityId": city })),

            // ===== Worker Commands =====
            Action::BuildImprovement { unit, improvement_type, tile, buy_goods, queue } => {
                let mut p = json!({ "unitId": unit, "improvementType": improvement_type, "tileId": tile });
                if *buy_goods { p["buyGoods"] = json!(true); }
                if *queue { p["queue"] = json!(true); }
                (GameCommandAction::BuildImprovement, p)
            }
            Action::UpgradeImprovement { unit, buy_goods } => {
                let mut p = json!({ "unitId": unit });
                if *buy_goods { p["buyGoods"] = json!(true); }
                (GameCommandAction::UpgradeImprovement, p)
            }
            Action::AddRoad { unit, tile, buy_goods, queue } => {
                let mut p = json!({ "unitId": unit, "tileId": tile });
                if *buy_goods { p["buyGoods"] = json!(true); }
                if *queue { p["queue"] = json!(true); }
                (GameCommandAction::AddRoad, p)
            }

            // ===== City Foundation =====
            Action::FoundCity { unit, family, nation } => {
                let mut p = json!({ "unitId": unit, "familyType": family });
                if let Some(n) = nation { p["nationType"] = json!(n); }
                (GameCommandAction::FoundCity, p)
            }
            Action::JoinCity { unit } => (GameCommandAction::JoinCity, json!({ "unitId": unit })),

            // ===== City Production =====
            Action::Build { city, unit_type, buy_goods, first } => {
                let mut p = json!({ "cityId": city, "unitType": unit_type });
                if *buy_goods { p["buyGoods"] = json!(true); }
                if *first { p["first"] = json!(true); }
                (GameCommandAction::Build, p)
            }
            Action::BuildUnit { city, unit_type, buy_goods, first } => {
                let mut p = json!({ "cityId": city, "unitType": unit_type });
                if *buy_goods { p["buyGoods"] = json!(true); }
                if *first { p["first"] = json!(true); }
                (GameCommandAction::BuildUnit, p)
            }
            Action::BuildProject { city, project_type, buy_goods, first, repeat } => {
                let mut p = json!({ "cityId": city, "projectType": project_type });
                if *buy_goods { p["buyGoods"] = json!(true); }
                if *first { p["first"] = json!(true); }
                if *repeat { p["repeat"] = json!(true); }
                (GameCommandAction::BuildProject, p)
            }
            Action::BuildQueue { city, old_slot, new_slot } =>
                (GameCommandAction::BuildQueue, json!({ "cityId": city, "oldSlot": old_slot, "newSlot": new_slot })),
            Action::HurryCivics { city } => (GameCommandAction::HurryCivics, json!({ "cityId": city })),
            Action::HurryTraining { city } => (GameCommandAction::HurryTraining, json!({ "cityId": city })),
            Action::HurryMoney { city } => (GameCommandAction::HurryMoney, json!({ "cityId": city })),
            Action::HurryPopulation { city } => (GameCommandAction::HurryPopulation, json!({ "cityId": city })),
            Action::HurryOrders { city } => (GameCommandAction::HurryOrders, json!({ "cityId": city })),

            // ===== Research & Decisions =====
            Action::Research { tech } => (GameCommandAction::Research, json!({ "tech": tech })),
            Action::RedrawTech => (GameCommandAction::RedrawTech, json!({})),
            Action::TargetTech { tech_type } => (GameCommandAction::TargetTech, json!({ "techType": tech_type })),
            Action::MakeDecision { decision, choice, data } => {
                let mut p = json!({ "decisionId": decision, "choiceIndex": choice });
                if let Some(d) = data { p["data"] = json!(d); }
                (GameCommandAction::MakeDecision, p)
            }
            Action::RemoveDecision { decision } =>
                (GameCommandAction::RemoveDecision, json!({ "decisionId": decision })),

            // ===== Diplomacy =====
            Action::DeclareWar { player } => (GameCommandAction::DeclareWar, json!({ "targetPlayer": player })),
            Action::MakePeace { player } => (GameCommandAction::MakePeace, json!({ "targetPlayer": player })),
            Action::DeclareTruce { player } => (GameCommandAction::DeclareTruce, json!({ "targetPlayer": player })),
            Action::DeclareWarTribe { tribe } => (GameCommandAction::DeclareWarTribe, json!({ "tribeType": tribe })),
            Action::MakePeaceTribe { tribe } => (GameCommandAction::MakePeaceTribe, json!({ "tribeType": tribe })),
            Action::DeclareTruceTribe { tribe } => (GameCommandAction::DeclareTruceTribe, json!({ "tribeType": tribe })),
            Action::GiftCity { city, player } =>
                (GameCommandAction::GiftCity, json!({ "cityId": city, "targetPlayer": player })),
            Action::GiftYield { yield_type, player, reverse } => {
                let mut p = json!({ "yieldType": yield_type, "targetPlayer": player });
                if *reverse { p["reverse"] = json!(true); }
                (GameCommandAction::GiftYield, p)
            }
            Action::AllyTribe { tribe } => (GameCommandAction::AllyTribe, json!({ "tribeType": tribe })),

            // ===== Character Management =====
            Action::AssignGovernor { city, character } =>
                (GameCommandAction::AssignGovernor, json!({ "cityId": city, "characterId": character })),
            Action::ReleaseGovernor { city } => (GameCommandAction::ReleaseGovernor, json!({ "cityId": city })),
            Action::AssignGeneral { unit, character } =>
                (GameCommandAction::AssignGeneral, json!({ "unitId": unit, "characterId": character })),
            Action::ReleaseGeneral { unit } => (GameCommandAction::ReleaseGeneral, json!({ "unitId": unit })),
            Action::AssignAgent { city, character } =>
                (GameCommandAction::AssignAgent, json!({ "cityId": city, "characterId": character })),
            Action::ReleaseAgent { city } => (GameCommandAction::ReleaseAgent, json!({ "cityId": city })),
            Action::StartMission { mission, character, target, cancel } => {
                let mut p = json!({ "missionType": mission, "characterId": character });
                if let Some(t) = target { p["target"] = json!(t); }
                if *cancel { p["cancel"] = json!(true); }
                (GameCommandAction::StartMission, p)
            }

            // ===== Turn Control =====
            Action::EndTurn { force } => {
                let p = if *force { json!({ "force": true }) } else { json!({}) };
                (GameCommandAction::EndTurn, p)
            }

            // ===== Laws & Economy =====
            Action::ChooseLaw { law } => (GameCommandAction::ChooseLaw, json!({ "lawType": law })),
            Action::CancelLaw { law } => (GameCommandAction::CancelLaw, json!({ "lawType": law })),
            Action::BuyYield { yield_type, amount } =>
                (GameCommandAction::BuyYield, json!({ "yieldType": yield_type, "amount": amount })),
            Action::SellYield { yield_type, amount } =>
                (GameCommandAction::SellYield, json!({ "yieldType": yield_type, "amount": amount })),
            Action::ConvertOrders => (GameCommandAction::ConvertOrders, json!({})),
            Action::ConvertLegitimacy => (GameCommandAction::ConvertLegitimacy, json!({})),
            Action::ConvertOrdersToScience => (GameCommandAction::ConvertOrdersToScience, json!({})),

            // ===== Luxury Trading =====
            Action::TradeCityLuxury { city, resource, enable } => {
                let mut p = json!({ "cityId": city, "resourceType": resource });
                if *enable { p["enable"] = json!(true); }
                (GameCommandAction::TradeCityLuxury, p)
            }
            Action::TradeFamilyLuxury { family, resource, enable } => {
                let mut p = json!({ "familyType": family, "resourceType": resource });
                if *enable { p["enable"] = json!(true); }
                (GameCommandAction::TradeFamilyLuxury, p)
            }
            Action::TradeTribeLuxury { tribe, resource, enable } => {
                let mut p = json!({ "tribeType": tribe, "resourceType": resource });
                if *enable { p["enable"] = json!(true); }
                (GameCommandAction::TradeTribeLuxury, p)
            }
            Action::TradePlayerLuxury { player, resource, enable } => {
                let mut p = json!({ "targetPlayer": player, "resourceType": resource });
                if *enable { p["enable"] = json!(true); }
                (GameCommandAction::TradePlayerLuxury, p)
            }
            Action::Tribute { to_player, to_tribe, yield_type, amount } => {
                let mut p = json!({ "yieldType": yield_type, "amount": amount });
                if let Some(player) = to_player { p["toPlayer"] = json!(player); }
                if let Some(tribe) = to_tribe { p["toTribe"] = json!(tribe); }
                (GameCommandAction::Tribute, p)
            }

            // ===== Unit Special Actions =====
            Action::Swap { unit, target_unit } =>
                (GameCommandAction::Swap, json!({ "unitId": unit, "targetUnitId": target_unit })),
            Action::DoUnitQueue { unit } => (GameCommandAction::DoUnitQueue, json!({ "unitId": unit })),
            Action::CancelUnitQueue { unit } => (GameCommandAction::CancelUnitQueue, json!({ "unitId": unit })),
            Action::Formation { unit, effect_type } =>
                (GameCommandAction::Formation, json!({ "unitId": unit, "effectUnitType": effect_type })),
            Action::Unlimber { unit } => (GameCommandAction::Unlimber, json!({ "unitId": unit })),
            Action::Anchor { unit } => (GameCommandAction::Anchor, json!({ "unitId": unit })),
            Action::Repair { unit, buy_goods } => {
                let mut p = json!({ "unitId": unit });
                if *buy_goods { p["buyGoods"] = json!(true); }
                (GameCommandAction::Repair, p)
            }
            Action::CancelImprovement { unit } => (GameCommandAction::CancelImprovement, json!({ "unitId": unit })),
            Action::RemoveVegetation { unit } => (GameCommandAction::RemoveVegetation, json!({ "unitId": unit })),
            Action::HarvestResource { unit } => (GameCommandAction::HarvestResource, json!({ "unitId": unit })),
            Action::UnitAutomate { unit, enable } => {
                let mut p = json!({ "unitId": unit });
                if *enable { p["enable"] = json!(true); }
                (GameCommandAction::UnitAutomate, p)
            }
            Action::AddUrban { unit, tile } =>
                (GameCommandAction::AddUrban, json!({ "unitId": unit, "tileId": tile })),
            Action::RoadTo { unit, target } =>
                (GameCommandAction::RoadTo, json!({ "unitId": unit, "targetTileId": target })),
            Action::BuyTile { city, tile } =>
                (GameCommandAction::BuyTile, json!({ "cityId": city, "tileId": tile })),
            Action::RecruitMercenary { unit_type, tile } =>
                (GameCommandAction::RecruitMercenary, json!({ "unitType": unit_type, "tileId": tile })),
            Action::HireMercenary { unit } => (GameCommandAction::HireMercenary, json!({ "unitId": unit })),
            Action::GiftUnit { unit, player } =>
                (GameCommandAction::GiftUnit, json!({ "unitId": unit, "targetPlayer": player })),
            Action::LaunchOffensive { unit } => (GameCommandAction::LaunchOffensive, json!({ "unitId": unit })),
            Action::ApplyEffectUnit { unit, effect } =>
                (GameCommandAction::ApplyEffectUnit, json!({ "unitId": unit, "effectType": effect })),
            Action::SelectUnit { unit } => (GameCommandAction::SelectUnit, json!({ "unitId": unit })),

            // ===== Agent & Caravan =====
            Action::CreateAgentNetwork { unit, city } =>
                (GameCommandAction::CreateAgentNetwork, json!({ "unitId": unit, "cityId": city })),
            Action::CreateTradeOutpost { unit, tile } =>
                (GameCommandAction::CreateTradeOutpost, json!({ "unitId": unit, "tileId": tile })),
            Action::CaravanMissionStart { unit, target } =>
                (GameCommandAction::CaravanMissionStart, json!({ "unitId": unit, "targetCityId": target })),
            Action::CaravanMissionCancel { unit } =>
                (GameCommandAction::CaravanMissionCancel, json!({ "unitId": unit })),

            // ===== Religious Units =====
            Action::PurgeReligion { unit, city } =>
                (GameCommandAction::PurgeReligion, json!({ "unitId": unit, "cityId": city })),
            Action::SpreadReligionTribe { unit, tribe } =>
                (GameCommandAction::SpreadReligionTribe, json!({ "unitId": unit, "tribeType": tribe })),
            Action::EstablishTheology { unit, theology } =>
                (GameCommandAction::EstablishTheology, json!({ "unitId": unit, "theologyType": theology })),

            // ===== Character Management Extended =====
            Action::CharacterName { character, name } =>
                (GameCommandAction::CharacterName, json!({ "characterId": character, "name": name })),
            Action::AddCharacterTrait { character, trait_type, remove } => {
                let mut p = json!({ "characterId": character, "traitType": trait_type });
                if *remove { p["remove"] = json!(true); }
                (GameCommandAction::AddCharacterTrait, p)
            }
            Action::SetCharacterRating { character, rating, value } =>
                (GameCommandAction::SetCharacterRating, json!({ "characterId": character, "ratingType": rating, "value": value })),
            Action::SetCharacterExperience { character, xp } =>
                (GameCommandAction::SetCharacterExperience, json!({ "characterId": character, "xp": xp })),
            Action::SetCharacterCognomen { character, cognomen } =>
                (GameCommandAction::SetCharacterCognomen, json!({ "characterId": character, "cognomenType": cognomen })),
            Action::SetCharacterNation { character, nation } =>
                (GameCommandAction::SetCharacterNation, json!({ "characterId": character, "nationType": nation })),
            Action::SetCharacterFamily { character, family } =>
                (GameCommandAction::SetCharacterFamily, json!({ "characterId": character, "familyType": family })),
            Action::SetCharacterReligion { character, religion } =>
                (GameCommandAction::SetCharacterReligion, json!({ "characterId": character, "religionType": religion })),
            Action::SetCharacterCourtier { character, courtier } =>
                (GameCommandAction::SetCharacterCourtier, json!({ "characterId": character, "courtierType": courtier })),
            Action::SetCharacterCouncil { character, council } =>
                (GameCommandAction::SetCharacterCouncil, json!({ "characterId": character, "councilType": council })),
            Action::PlayerLeader { player, character } =>
                (GameCommandAction::PlayerLeader, json!({ "playerIndex": player, "characterId": character })),
            Action::FamilyHead { family, character } =>
                (GameCommandAction::FamilyHead, json!({ "familyType": family, "characterId": character })),
            Action::PinCharacter { character, pin } => {
                let mut p = json!({ "characterId": character });
                if *pin { p["pin"] = json!(true); }
                (GameCommandAction::PinCharacter, p)
            }

            // ===== City Management =====
            Action::CityRename { city, name } =>
                (GameCommandAction::CityRename, json!({ "cityId": city, "name": name })),
            Action::CityAutomate { city, enable } => {
                let mut p = json!({ "cityId": city });
                if *enable { p["enable"] = json!(true); }
                (GameCommandAction::CityAutomate, p)
            }
            Action::BuildSpecialist { tile, specialist_type } =>
                (GameCommandAction::BuildSpecialist, json!({ "tileId": tile, "specialistType": specialist_type })),
            Action::SetSpecialist { tile, specialist_type } =>
                (GameCommandAction::SetSpecialist, json!({ "tileId": tile, "specialistType": specialist_type })),
            Action::ChangeCitizens { city, delta } =>
                (GameCommandAction::ChangeCitizens, json!({ "cityId": city, "delta": delta })),
            Action::ChangeReligion { city, religion, add } => {
                let mut p = json!({ "cityId": city, "religionType": religion });
                if *add { p["add"] = json!(true); }
                (GameCommandAction::ChangeReligion, p)
            }
            Action::ChangeFamily { city, family } =>
                (GameCommandAction::ChangeFamily, json!({ "cityId": city, "familyType": family })),
            Action::ChangeFamilySeat { family, city } =>
                (GameCommandAction::ChangeFamilySeat, json!({ "familyType": family, "cityId": city })),

            // ===== Goals & Communication =====
            Action::AbandonAmbition { ambition } => {
                let mut p = json!({});
                if let Some(a) = ambition { p["ambitionType"] = json!(a); }
                (GameCommandAction::AbandonAmbition, p)
            }
            Action::AddPlayerGoal { player, goal } =>
                (GameCommandAction::AddPlayerGoal, json!({ "playerIndex": player, "goalType": goal })),
            Action::RemovePlayerGoal { player, goal } =>
                (GameCommandAction::RemovePlayerGoal, json!({ "playerIndex": player, "goalType": goal })),
            Action::EventStory { event } =>
                (GameCommandAction::EventStory, json!({ "eventType": event })),
            Action::FinishGoal { goal, success } => {
                let mut p = json!({ "goalType": goal });
                if *success { p["success"] = json!(true); }
                (GameCommandAction::FinishGoal, p)
            }
            Action::Chat { message } =>
                (GameCommandAction::Chat, json!({ "message": message })),
            Action::Ping { tile } =>
                (GameCommandAction::Ping, json!({ "tileId": tile })),
            Action::CustomReminder { text, turn } =>
                (GameCommandAction::CustomReminder, json!({ "text": text, "turn": turn })),
            Action::ClearChat => (GameCommandAction::ClearChat, json!({})),

            // ===== Game State & Turn =====
            Action::ExtendTime { minutes } =>
                (GameCommandAction::ExtendTime, json!({ "minutes": minutes })),
            Action::Pause { enable } => {
                let mut p = json!({});
                if *enable { p["enable"] = json!(true); }
                (GameCommandAction::Pause, p)
            }
            Action::Undo => (GameCommandAction::Undo, json!({})),
            Action::Redo => (GameCommandAction::Redo, json!({})),
            Action::ReplayTurn { turn } =>
                (GameCommandAction::ReplayTurn, json!({ "turn": turn })),
            Action::AiFinishTurn => (GameCommandAction::AiFinishTurn, json!({})),
            Action::ToggleNoReplay => (GameCommandAction::ToggleNoReplay, json!({})),

            // ===== Diplomacy Extended =====
            Action::TeamAlliance { team, ally_team, enable } => {
                let mut p = json!({ "team": team, "allyTeam": ally_team });
                if *enable { p["enable"] = json!(true); }
                (GameCommandAction::TeamAlliance, p)
            }
            Action::TribeInvasion { tribe } =>
                (GameCommandAction::TribeInvasion, json!({ "tribeType": tribe })),
            Action::VictoryTeam { team } =>
                (GameCommandAction::VictoryTeam, json!({ "team": team })),

            // ===== Editor/Debug - Units =====
            Action::CreateUnit { tile, unit_type, player } =>
                (GameCommandAction::CreateUnit, json!({ "tileId": tile, "unitType": unit_type, "playerType": player })),
            Action::UnitName { unit, name } =>
                (GameCommandAction::UnitName, json!({ "unitId": unit, "name": name })),
            Action::SetUnitFamily { unit, family } =>
                (GameCommandAction::SetUnitFamily, json!({ "unitId": unit, "familyType": family })),
            Action::ChangeUnitOwner { unit, player, tribe } => {
                let mut p = json!({ "unitId": unit });
                if let Some(pl) = player { p["playerType"] = json!(pl); }
                if let Some(tr) = tribe { p["tribeType"] = json!(tr); }
                (GameCommandAction::ChangeUnitOwner, p)
            }
            Action::ChangeCooldown { unit, delta } =>
                (GameCommandAction::ChangeCooldown, json!({ "unitId": unit, "delta": delta })),
            Action::ChangeDamage { unit, delta } =>
                (GameCommandAction::ChangeDamage, json!({ "unitId": unit, "delta": delta })),
            Action::UnitIncrementLevel { unit } =>
                (GameCommandAction::UnitIncrementLevel, json!({ "unitId": unit })),
            Action::UnitChangePromotion { unit, promotion, remove } => {
                let mut p = json!({ "unitId": unit, "promotionType": promotion });
                if *remove { p["remove"] = json!(true); }
                (GameCommandAction::UnitChangePromotion, p)
            }

            // ===== Editor/Debug - Cities =====
            Action::CreateCity { tile, player, family } =>
                (GameCommandAction::CreateCity, json!({ "tileId": tile, "playerType": player, "familyType": family })),
            Action::RemoveCity { city } =>
                (GameCommandAction::RemoveCity, json!({ "cityId": city })),
            Action::CityOwner { city, player } =>
                (GameCommandAction::CityOwner, json!({ "cityId": city, "playerType": player })),
            Action::ChangeCityDamage { city, delta } =>
                (GameCommandAction::ChangeCityDamage, json!({ "cityId": city, "delta": delta })),
            Action::ChangeCulture { city, delta } =>
                (GameCommandAction::ChangeCulture, json!({ "cityId": city, "delta": delta })),
            Action::ChangeCityBuildTurns { city, delta } =>
                (GameCommandAction::ChangeCityBuildTurns, json!({ "cityId": city, "delta": delta })),
            Action::ChangeCityDiscontentLevel { city, delta } =>
                (GameCommandAction::ChangeCityDiscontentLevel, json!({ "cityId": city, "delta": delta })),
            Action::ChangeProject { city, project, delta } =>
                (GameCommandAction::ChangeProject, json!({ "cityId": city, "projectType": project, "delta": delta })),

            // ===== Editor/Debug - Tiles =====
            Action::SetTerrain { tile, terrain_type } =>
                (GameCommandAction::SetTerrain, json!({ "tileId": tile, "terrainType": terrain_type })),
            Action::SetTerrainHeight { tile, height } =>
                (GameCommandAction::SetTerrainHeight, json!({ "tileId": tile, "heightType": height })),
            Action::SetVegetation { tile, vegetation_type } =>
                (GameCommandAction::SetVegetation, json!({ "tileId": tile, "vegetationType": vegetation_type })),
            Action::SetResource { tile, resource_type } =>
                (GameCommandAction::SetResource, json!({ "tileId": tile, "resourceType": resource_type })),
            Action::SetRoad { tile, enable } => {
                let mut p = json!({ "tileId": tile });
                if *enable { p["enable"] = json!(true); }
                (GameCommandAction::SetRoad, p)
            }
            Action::SetImprovement { tile, improvement_type } =>
                (GameCommandAction::SetImprovement, json!({ "tileId": tile, "improvementType": improvement_type })),
            Action::SetTileOwner { tile, player, city } => {
                let mut p = json!({ "tileId": tile });
                if let Some(pl) = player { p["playerType"] = json!(pl); }
                if let Some(c) = city { p["cityId"] = json!(c); }
                (GameCommandAction::SetTileOwner, p)
            }
            Action::SetCitySite { tile, enable } => {
                let mut p = json!({ "tileId": tile });
                if *enable { p["enable"] = json!(true); }
                (GameCommandAction::SetCitySite, p)
            }
            Action::ImprovementBuildTurns { tile, turns } =>
                (GameCommandAction::ImprovementBuildTurns, json!({ "tileId": tile, "turns": turns })),

            // ===== Editor/Debug - Map & Player =====
            Action::MapReveal { team } =>
                (GameCommandAction::MapReveal, json!({ "teamType": team })),
            Action::MapUnreveal { team } =>
                (GameCommandAction::MapUnreveal, json!({ "teamType": team })),
            Action::AddTech { player, tech } =>
                (GameCommandAction::AddTech, json!({ "playerType": player, "techType": tech })),
            Action::AddYield { player, yield_type, amount } =>
                (GameCommandAction::AddYield, json!({ "playerType": player, "yieldType": yield_type, "amount": amount })),
            Action::AddMoney { player, amount } =>
                (GameCommandAction::AddMoney, json!({ "playerType": player, "amount": amount })),
            Action::Cheat { key } =>
                (GameCommandAction::Cheat, json!({ "key": key })),

            // ===== Editor/Debug - Characters =====
            Action::MakeCharacterDead { character } =>
                (GameCommandAction::MakeCharacterDead, json!({ "characterId": character })),
            Action::MakeCharacterSafe { character, enable } => {
                let mut p = json!({ "characterId": character });
                if *enable { p["enable"] = json!(true); }
                (GameCommandAction::MakeCharacterSafe, p)
            }
            Action::NewCharacter { player } =>
                (GameCommandAction::NewCharacter, json!({ "playerType": player })),
            Action::AddCharacter { player, character_type } =>
                (GameCommandAction::AddCharacter, json!({ "playerType": player, "characterType": character_type })),
            Action::TribeLeader { tribe, character } =>
                (GameCommandAction::TribeLeader, json!({ "tribeType": tribe, "characterId": character })),
        };

        GameCommand {
            action,
            params: to_map(params),
            request_id: None,
        }
    }
}
