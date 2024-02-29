
use serde::{Deserialize, Serialize};
use super::common_structs::*;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum RelicOrNot {
    Relic(Relic),
    Arcane(Arcane),
    CosmeticEnhancer(CosmeticEnhancer),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Arcane {
    codex_secret: bool,
    exclude_from_codex: Option<bool>, // only 1 is excluded
    name: String,
    rarity: Option<String>,
    level_stats: Vec<LevelStats>,
    unique_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Relic {
    codex_secret: bool,
    description: String,
    name: String,
    relic_rewards: Vec<RelicRewards>,
    unique_name: ItemID, 
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct LevelStats {
    stats: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct RelicRewards {
    item_count: u16,
    rarity: String,
    reward_name: String,
    tier: u8,
}

//useless info
#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct CosmeticEnhancer {
    codex_secret: bool,
    exclude_from_codex: bool,
    name: String,
    unique_name: String,
}
