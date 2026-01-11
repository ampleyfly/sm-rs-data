use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StartingResource {
    resource: Resource,
    max_amount: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpgradeItem {
    name: ItemName,
    data: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExpansionItem {
    name: ItemName,
    data: String,
    resource: Resource,
    resource_amount: u16,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Resource {
    #[default]
    RegularEnergy,
    ReserveEnergy,
    Missile,
    Super,
    PowerBomb,
}

#[derive(Deserialize, Debug, Clone)]
pub enum ItemName {
    PowerBeam,
    PowerSuit,
    Morph,
    Bombs,
    Charge,
    HiJump,
    Spazer,
    Varia,
    SpeedBooster,
    Ice,
    Grapple,
    Wave,
    XRayScope,
    Gravity,
    SpaceJump,
    SpringBall,
    Plasma,
    ScrewAttack,
    ETank,
    Super,
    Missile,
    ReserveTank,
    PowerBomb,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum GameFlag {
    f_AnimalsSaved,
    f_BeatSuperMetroid,
    f_DefeatedBombTorizo,
    f_DefeatedBotwoon,
    f_DefeatedCeresRidley,
    f_DefeatedCrocomire,
    f_DefeatedDraygon,
    f_DefeatedGoldenTorizo,
    f_DefeatedKraid,
    f_DefeatedMotherBrain,
    f_DefeatedPhantoon,
    f_DefeatedRidley,
    f_DefeatedSporeSpawn,
    f_KilledMetroidRoom1,
    f_KilledMetroidRoom2,
    f_KilledMetroidRoom3,
    f_KilledMetroidRoom4,
    f_KilledZebetites1,
    f_KilledZebetites2,
    f_KilledZebetites3,
    f_KilledZebetites4,
    f_MaridiaTubeBroken,
    f_ShaktoolDoneDigging,
    f_TourianOpen,
    f_UsedAcidChozoStatue,
    f_ZebesAwake,
    f_ZebesSetAblaze,
    f_MotherBrainGlassBroken,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Items {
    starting_room: String,
    starting_node: u16,
    starting_items: Vec<ItemName>,
    starting_flags: Vec<String>,
    starting_locks: Vec<String>,
    starting_resources: Vec<StartingResource>,
    implicit_items: Vec<ItemName>,
    upgrade_items: Vec<UpgradeItem>,
    expansion_items: Vec<ExpansionItem>,
    game_flags: Vec<GameFlag>,
}
