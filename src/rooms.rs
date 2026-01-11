use crate::{
    notes::{DevNote, Note},
    requirements::*,
    GameFlag,
};
use serde::Deserialize;
use serde_json::Map;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: RoomId,
    pub name: RoomName,
    pub area: Area,
    pub subarea: SubArea,
    pub subsubarea: Option<SubSubArea>,
    pub playable: bool,
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
    pub strats: Vec<Strat>,
    pub room_address: Option<RoomMemoryAddress>,
    pub room_environments: Option<Vec<RoomEnvironment>>,
    pub obstacles: Option<Vec<Obstacle>>,
    pub enemies: Option<Vec<Enemy>>,
    pub notables: Option<Vec<NotableStrat>>,
    pub note: Option<Note>,
    pub dev_note: Option<DevNote>,
}

#[derive(Deserialize, Debug)]
pub struct RoomId(u8);

#[derive(Deserialize, Debug)]
pub struct RoomName(String);

#[derive(Deserialize, Debug)]
pub struct NotableId(u8);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotableStrat {
    id: NotableId,
    name: StratName,
    wall_jump_avoid: Option<bool>,
    note: Note,
    dev_note: Option<DevNote>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Enemy {
    pub id: EnemyId,
    pub group_name: EnemyGroup,
    pub enemy_name: EnemyName,
    pub quantity: u8,
    pub home_nodes: Option<Vec<NodeId>>,
    pub between_nodes: Option<Vec<NodeId>>,
    pub spawn: Option<Vec<Requirement>>,
    pub stop_spawn: Option<Vec<Requirement>>,
    pub drop_requires: Option<Vec<Requirement>>,
    pub farm_cycles: Option<Vec<FarmCycle>>,
    pub note: Option<Note>,
    pub dev_note: Option<DevNote>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FarmCycle {
    pub name: String,
    pub cycle_frames: u8,
    pub requires: Option<Vec<Requirement>>,
    pub note: Option<Note>,
    pub dev_note: Option<DevNote>,
}

#[derive(Deserialize, Debug)]
pub struct EnemyId(String);

#[derive(Deserialize, Debug)]
pub struct EnemyGroup(String);

#[derive(Deserialize, Debug)]
pub struct EnemyName(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub from: NodeId,
    pub to: Option<Vec<LinkEnd>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinkEnd {
    pub id: NodeId,
    pub note: Option<Note>,
    pub dev_note: Option<DevNote>,
}

#[derive(Deserialize, Debug)]
pub enum Area {
    Ceres,
    Brinstar,
    Crateria,
    Norfair,
    LowerNorfair,
    Maridia,
    Tourain,
    WreckedShip,
}

#[derive(Deserialize, Debug)]
pub enum SubArea {
    Blue,
    Green,
    Pink,
    Red,
    Kraid,
    West,
    Central,
    East,
    Outer,
    Inner,
    Crocomire,
}

#[derive(Deserialize, Debug)]
pub enum SubSubArea {
    Green,
    Pink,
    Yellow,
}

#[derive(Deserialize, Debug)]
pub struct RoomMemoryAddress(String);

#[derive(Deserialize, Debug)]
pub struct RoomEnvironment {
    pub heated: Option<bool>,
    pub entrance_nodes: Option<Vec<NodeId>>,
    pub note: Option<Note>,
    pub dev_note: Option<DevNote>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: NodeId,
    pub name: NodeName,
    pub node_type: NodeType,
    pub node_sub_type: NodeSubType,
    pub node_address: Option<NodeMemoryAddress>,
    pub door_orientation: Option<DoorOrientation>,
    pub door_environments: Option<Vec<DoorEnvironment>>,
    pub use_implicit_door_unlocks: Option<bool>,
    pub interaction_requires: Option<Requirement>,
    pub spawn_at: Option<u8>,
    pub locks: Option<Vec<Lock>>,
    pub twin_door_addresses: Option<Vec<TwinDoorAddress>>,
    pub utility: Option<Utility>,
    pub viewable_nodes: Option<Vec<ViewableNode>>,
    pub yields: Option<Vec<Yield>>,
    pub note: Option<Note>,
    pub dev_note: Option<DevNote>,
}

#[derive(Deserialize, Debug)]
pub struct NodeId(u8);

#[derive(Deserialize, Debug)]
pub struct NodeName(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum NodeType {
    Door,
    Entrance,
    Exit,
    Event,
    Item,
    Junction,
    Utility,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum NodeSubType {
    Boss,
    Chozo,
    Flag,
    Hidden,
    Visible,
    Blue,
    ClosedWall,
    Doorway,
    Elevator,
    Eye,
    Gray,
    Green,
    Passage,
    Red,
    Sandpit,
    Vertical,
    Yellow,
    Junction,
    #[serde(rename = "g-mode")]
    GMode,
    Save,
    Missile,
    Energy,
    Ship,
    Map,
}

#[derive(Deserialize, Debug)]
pub struct NodeMemoryAddress(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DoorOrientation {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Deserialize, Debug)]
pub struct DoorEnvironment {
    pub physics: Physics,
    pub entrance_nodes: Option<Vec<NodeId>>,
    pub note: Option<Note>,
    pub dev_note: Option<DevNote>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Physics {
    Air,
    Water,
    Lava,
    Acid,
    Normal,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Lock {
    pub name: String,
    pub lock_type: LockType,
    pub unlock_strats: Vec<Strat>,
    pub lock: Option<Vec<Requirement>>,
    pub note: Option<Note>,
    pub dev_note: Option<DevNote>,
    pub yields: Option<Vec<Yield>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum LockType {
    BossFight,
    ColoredDoor,
    Cutscene,
    EscapeFunnel,
    GameFlag,
    KillEnemies,
    Permanent,
    TriggeredEvent,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Strat {
    pub name: StratName,
    pub requires: Vec<Requirement>,
    pub link: Option<(NodeId, NodeId)>,
    pub notable: Option<bool>,
    pub reusable_roomwide_notable: Option<StratName>,
    pub entrance_condition: Option<EntranceCondition>,
    pub exit_condition: Option<ExitCondition>,
    pub g_mode_regain_mobility: Option<GModeRegainMobility>,
    pub bypasses_door_shell: Option<bool>,
    pub unlocks_doors: Option<Vec<UnlocksDoor>>,
    pub clears_obstacles: Option<Vec<ObstacleId>>,
    pub resets_obstacles: Option<Vec<ObstacleId>>,
    pub collects_items: Option<Vec<NodeId>>,
    pub sets_flags: Option<Vec<GameFlag>>,
    pub flash_suit_checked: Option<bool>,
    pub failures: Option<Vec<Failure>>,
    pub note: Option<Note>,
    pub dev_note: Option<DevNote>,
}

#[derive(Deserialize, Debug)]
pub struct StratName(String);
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Obstacle {
    pub id: ObstacleId,
    pub name: String,
    pub obstacle_type: ObstacleType,
    pub note: Option<Note>,
    pub dev_note: Option<DevNote>,
}

#[derive(Deserialize, Debug)]
pub struct ObstacleId(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ObstacleType {
    Inanimate,
    Enemies,
    Abstract,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum EntranceCondition {
    ComeInNormally {},
    ComeInRunning(MovementConditions),
    ComeInJumping(MovementConditions),
    ComeInSpaceJumping(MovementConditions),
    ComeInShinecharging(Runway),
    ComeInGettingBlueSpeed(Runway),
    ComeInShinecharged {
        frames_required: u8,
    },
    ComeInShinechargedJumping {
        frames_required: u8,
    },
    ComeInWithSpark {
        position: Option<DoorSparkPosition>,
    },
    ComeInStutterShinecharging {
        min_tiles: u8,
    },
    ComeInWithBombBoost {},
    ComeInWithDoorStuckSetup {},
    ComeInSpeedballing {
        runway: Runway,
    },
    ComeInWithTemporaryBlue {
        direction: FacingThroughTransition,
    },
    ComeInBlueSpinning {
        unusable_tiles: u8,
        min_tiles: Option<u8>,
    },
    ComeInWithMockball {
        adjacent_min_tiles: Option<u8>,
        remote_and_landing_min_tiles: Option<Vec<u8>>,
    },
    ComeInWithSpringBallBounce {
        movement_type: SpringBallMovement,
        adjacent_min_tiles: Option<u8>,
        remote_and_landing_min_tiles: Option<Vec<u8>>,
    },
    ComeInWithStoredFallSpeed,
    ComeInWithRMode {},
    ComeInWithGMode {
        mode: GModeType,
        morphed: bool,
        mobility: Option<GModeMobility>,
    },
    ComeInWithWallJumpBelow {
        min_height: Option<u8>,
    },
    ComeInWithSpaceJumpBelow {},
    ComeInWithPlatformBelow {
        min_height: Option<u8>,
        max_height: Option<u8>,
        max_left_position: Option<u8>,
        min_right_position: Option<u8>,
    },
    ComeInWithGrappleTeleport {
        #[serde(rename = "blockPositions")]
        block_positions: Vec<(u8, u8)>,
    },
    ComesInThroughToilet(YesNoAny),
    DevNote(DevNote),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MovementConditions {
    pub speed_booster: TrueFalseAny,
    pub min_tiles: u8,
    pub max_tiles: Option<u8>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TrueFalseAny {
    True,
    False,
    Any,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum DoorSparkPosition {
    Top,
    Bottom,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Runway {
    pub length: u8,
    pub open_end: u8,
    pub gentle_up_tiles: Option<u8>,
    pub gentle_down_tiles: Option<u8>,
    pub steep_up_tiles: Option<u8>,
    pub steep_down_tiles: Option<u8>,
    pub min_tiles: Option<u8>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FacingThroughTransition {
    Left,
    Right,
    Any,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SpringBallMovement {
    Controlled,
    Uncontrolled,
    Any,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GModeType {
    Direct,
    Indirect,
    Any,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GModeMobility {
    Mobile,
    Immobile,
    Any,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum YesNoAny {
    No,
    Yes,
    Any,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ExitCondition {
    LeaveNormally {},
    LeaveWithRunway(Runway),
    LeaveShinecharged(SparkFramesRemaining),
    LeaveWithTemporaryBlue(FacingThroughTransition),
    LeaveWithSpark {
        position: Option<DoorSparkPosition>,
    },
    LeaveSpinning {
        remote_runway: Runway,
        blue: Option<YesNoAny>,
    },
    LeaveWithMockball {
        remote_runway: Runway,
        landing_runway: Runway,
        blue: Option<YesNoAny>,
    },
    LeaveWithSpringBallBounce {
        remote_runway: Runway,
        landing_runway: Runway,
        movement_type: SpringBallMovement,
        blue: Option<YesNoAny>,
    },
    LeaveSpaceJumping {
        remote_runway: Runway,
        blue: YesNoAny,
    },
    LeaveWithStoredFallSpeed {
        fall_speed_in_tiles: u8,
    },
    LeaveWithGModeSetup {
        knockback: Option<bool>,
    },
    LeaveWithGMode {
        morphed: bool,
    },
    LeaveWithDoorFrameBelow {
        height: u8,
    },
    LeaveWithPlatformBelow {
        height: u8,
        left_position: u8,
        right_position: u8,
    },
    LeaveWithGrappleTeleport {
        #[serde(alias = "blockPositions")]
        block_positions: Vec<(u8, u8)>,
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SparkFramesRemaining {
    Auto,
    #[serde(rename = "framesRemaining")]
    FramesRemaining(u8),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeaveWithPlatformBelow {
    pub height: u8,
    pub left_position: u8,
    pub right_position: u8,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GModeRegainMobility {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnlocksDoor {
    pub types: Vec<UnlockDoorType>,
    pub node_id: Option<NodeId>,
    pub requires: Option<Requirement>,
    pub use_implicit_requires: Option<bool>,
    pub note: Option<Note>,
    pub dev_note: Option<DevNote>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UnlockDoorType {
    Missiles,
    Super,
    Powerbomb,
    Gray,
    Ammo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Failure {
    pub name: String,
    pub leads_to_node: Option<NodeId>,
    pub cost: Option<Vec<Requirement>>,
    pub softlock: Option<bool>,
    pub note: Option<Note>,
    pub dev_note: Option<DevNote>,
}

#[derive(Deserialize, Debug)]
pub struct Yield(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TwinDoorAddress {
    pub room_address: String,
    pub door_address: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Utility {
    Save,
    Missile,
    Super,
    Powerbomb,
    Energy,
    Reserve,
    Map,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ViewableNode {
    pub id: NodeId,
    pub strats: Vec<Strat>,
}
