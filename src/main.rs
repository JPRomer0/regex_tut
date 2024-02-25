use regex::Regex;
use regex::RegexBuilder;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
// use serde_json::Value::Array;
// use serde_json::Value::Bool;
// use serde_json::Value::Null;
// use serde_json::Value::Number;
// use serde_json::Value::Object;
// use serde_json::Value::String as Str;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use strum::IntoEnumIterator;
// use std::time::SystemTime;
// use std::time::UNIX_EPOCH;

// contains all the links from which json data from the game can be extracted
const PUBLIC_EXPORT: &'static str = "https://origin.warframe.com/PublicExport/index_en.txt.lzma";
// The first part of the url where the json files with info about warframes items reside
const LINK_BASE: &'static str = "http://content.warframe.com/PublicExport/Manifest/";
// Where I save the links
const LINKS_SAVE_FILE: &'static str = "links.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    update_links()?;
    for db in Database::iter() {
        update_database(db)?
    }

    // deserializing weapons
    let path: PathBuf = std::env::current_dir()?;
    let mut weapons_path: PathBuf = path.clone();
    weapons_path.push("weapons.txt");
    let weapons_file_content: String = fs::read_to_string(&weapons_path)?;
    // into value first to circumvent shit json (duplicate keys Error)
    let weapons_val: Value = serde_json::from_str(&weapons_file_content)?;
    let weapons: Weapons = serde_json::from_value(weapons_val)?;

    // deserializing warframes
    let mut warframes_path: PathBuf = path.clone();
    warframes_path.push("warframes.txt");
    let warframes_file_content: String = fs::read_to_string(&warframes_path)?;

    let warframes_val: Value = serde_json::from_str(&warframes_file_content)?;
    let warframes: Warframes = serde_json::from_value(warframes_val)?;

    // deserializing relic_arcane
    let mut relic_arcane_path: PathBuf = path.clone();
    relic_arcane_path.push("relic_arcane.txt");
    let relic_arcane_file_content: String = fs::read_to_string(&relic_arcane_path)?;

    let relic_arcane_val: RelicArcanes = serde_json::from_str(&relic_arcane_file_content)?;

    // let regex: Regex = Regex::new(r"Arcane").unwrap();

    for something in relic_arcane_val.export_relic_arcane.iter() {
        if something.description.is_some() {
            println!("{}", something.name);
                println!("{:?}", something.description);
            std::thread::sleep(std::time::Duration::from_millis(30));
        }
    }

    Ok(())
}

// fn which_variant(value: &Value) -> u8 {
//     match value {
//         Null => 0,
//         Bool(_) => 1,
//         Number(_) => 2,
//         Str(_) => 3,
//         Array(_) => 4,
//         Object(_) => 5,
//     }
// }    

// fn is_melee_weapon(value: &Value) -> bool {
//     let Some(obj) = value.as_object() else {
//         return false;
//     };
//     if let Some(Number(num)) = obj.get("slot") {
//         let is_five = num.as_u64() == Some(5);
//         if !is_five {
//             return false;
//         };
//     }

//     // if let Some(String(string)) = obj.get("")
//     true
// }

// fn get_values_that_fulfill_condition(
//     original_value: &Value,
//     condition: impl Fn(&Value) -> bool,
// ) -> Vec<&Value> {
//     let mut unchecked_values: Vec<&Value> = Vec::new();
//     let mut matching_values: Vec<&Value> = Vec::new();
//     unchecked_values.push(original_value);

//     while let Some(value) = unchecked_values.pop() {
//         if condition(value) {
//             matching_values.push(value)
//         } else {
//             match value {
//                 Value::Array(array) => {
//                     unchecked_values.extend(array.iter());
//                 }
//                 Value::Object(obj) => {
//                     unchecked_values.extend(obj.values());
//                 }
//                 _ => {}
//             }
//         }
//     }
//     matching_values
// }

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct RelicArcanes {
    export_relic_arcane: Vec<RelicOrArcane>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct RelicOrArcane {
    codex_secret: bool,
    exclude_from_codex: Option<bool>,
    description: Option<String>,
    name: String,
    relic_rewards: Option<Vec<RelicRewards>>,
    rarity: Option<String>,
    level_stats: Option<Vec<LevelStats>>,
    unique_name: String,
}


// struct Arcane {
    
// }

struct Relic {
    codex_secret: bool,
    exclude_from_codex: Option<bool>,
    description: Option<String>,
    name: String,
    relic_rewards: Vec<RelicRewards>,
    rarity: Option<String>,
    unique_name: String,
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

// #[derive(Deserialize, Serialize, Debug)]
// #[serde(deny_unknown_fields, rename_all = "camelCase")]
// struct Relic {
//     codex_secret: bool,
//     exclude_from_codex: Option<bool>,
//     description: Option<String>,
//     name: String,
//     relic_rewards: Value,
//     rarity: Option<String>,
//     level_stats: Option<Vec<Value>>,
//     unique_name: String,

// }

// #[derive(Deserialize, Serialize, Debug)]
// #[serde(deny_unknown_fields, rename_all = "camelCase")]
// enum RelicOrNot {
//     Relic(Relic),
//     Not(Value),
// }

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct Weapons {
    export_railjack_weapons: Vec<RailjackWeapon>,
    export_weapons: Vec<Weapon>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct RailjackWeapon {
    accuracy: f64,
    codex_secret: bool,
    critical_chance: f64,
    critical_multiplier: f64,
    damage_per_shot: Vec<f64>,
    description: String,
    exclude_from_codex: bool,
    fire_rate: f64,
    magazine_size: u32,
    mastery_req: u32,
    multishot: u16,
    name: String,
    noise: String,
    omega_attenuation: f64,
    proc_chance: f64,
    product_category: String,
    reload_time: f64,
    slot: u32,
    total_damage: f64,
    trigger: String,
    unique_name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Weapon {
    accuracy: Option<f64>,
    blocking_angle: Option<u32>,
    codex_secret: bool,
    combo_duration: Option<u32>,
    critical_chance: f64,
    critical_multiplier: f64,
    damage_per_shot: Vec<f64>,
    description: String,
    exclude_from_codex: Option<bool>,
    fire_rate: f64,
    follow_through: Option<f64>,
    heavy_attack_damage: Option<u32>,
    heavy_slam_attack: Option<u32>,
    heavy_slam_radial_damage: Option<u32>,
    heavy_slam_radius: Option<u32>,
    magazine_size: Option<u32>,
    mastery_req: u32,
    max_level_cap: Option<u32>,
    multishot: Option<u16>,
    name: String,
    noise: Option<String>,
    omega_attenuation: f64,
    prime_omega_attenuation: Option<f64>,
    proc_chance: f64,
    product_category: String,
    range: Option<f64>,
    slam_attack: Option<u32>,
    slam_radial_damage: Option<u32>,
    slam_radius: Option<u32>,
    slide_attack: Option<u32>,
    reload_time: Option<f64>,
    sentinel: Option<bool>,
    slot: Option<u32>,
    total_damage: f64,
    trigger: Option<String>,
    unique_name: String,
    wind_up: Option<f64>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "PascalCase")]
struct Warframes {
    export_warframes: Vec<Warframe>,
    export_abilities: Vec<Ability>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Warframe {
    abilities: Vec<Ability>,
    armor: u32,
    codex_secret: bool,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    exalted: Option<Vec<String>>,
    health: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    long_description: Option<String>,
    mastery_req: u32,
    name: String,
    parent_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    passive_description: Option<String>,
    power: u32,
    product_category: String,
    shield: u32,
    sprint_speed: serde_json::Number,
    stamina: u32,
    unique_name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Ability {
    ability_unique_name: String,
    ability_name: String,
    description: String,
}

// struct ProgramData {
//     // date = seconds since unix epoch till given time
//     date_last_time_program_run: u64,
//     date_last_public_export_update: u64,
//     links_up_to_date: bool,
//     json_weapons_up_to_date: bool,
//     json_warframes_up_to_date: bool,
// }

// CLI should work like:
// wfdb (WarFrame DataBase)
// wfdb --update-links
// Any error should be reported, this should update all weapon links
// Should not update if last update was 24 hours ago
// and tell to use next command if you still want to update anyways
// wfdb --update-links
// wfdb --update-links --verbose
// wfdb --update-database <given-database-here>
// should report if already up to date or otherwise try to update
#[derive(strum_macros::EnumIter)]
enum Database {
    Customs,
    Drones,
    Flavour,
    FusionBundles,
    Gear,
    Keys,
    Recipes,
    Regions,
    RelicArcane,
    Resources,
    Sentinels,
    SortieRewards,
    Upgrades,
    Warframes,
    Weapons,
    Manifest,
}

// struct WeaponsSchema {
//     uniqueName: String,
//     name: String,
//     description: String,
//     codexSecret: bool,
//     parentName: String,
//     excludeFromCodex: bool,
// }

fn update_links() -> Result<(), Box<dyn std::error::Error>> {
    // I get a "Response" type from the page that contains info on the HTTP transaction
    let page_info = reqwest::blocking::get(PUBLIC_EXPORT)?;

    // I get the body formatted as bytes
    let body_bytes = page_info.bytes()?;

    let mut decompressor = lzma::read(body_bytes.as_ref())?;

    let mut decompressed_data = Vec::new();

    // Here the data gets decompressed
    let maybe_err = decompressor.read_to_end(&mut decompressed_data)?;

    let decoded_text = String::from_utf8(decompressed_data)?;

    // I put all the urls where I´m going to extract the json files from in the urls String
    // One url per line
    let urls: String = decoded_text
        .lines()
        .map(|line| format!("{}{}\n", LINK_BASE, &line))
        .collect();

    let mut path = std::env::current_dir()?;
    path.push(LINKS_SAVE_FILE);
    println!("{:?}", &path);
    let mut file = open_no_symlink(path)?;
    file.write_all(urls.as_bytes())?;
    Ok(())
}

fn update_database(db: Database) -> Result<(), Box<dyn std::error::Error>> {
    // println!("{:?}", urls);
    let db_regex = RegexBuilder::new(match db {
        Database::Customs => r"^http.*Customs.*$",
        Database::Drones => r"^http.*Drones.*$",
        Database::Flavour => r"^http.*Flavour.*$",
        Database::FusionBundles => r"^http.*FusionBundles.*$",
        Database::Gear => r"^http.*Gear.*$",
        Database::Keys => r"^http.*Keys.*$",
        Database::Recipes => r"^http.*Recipes.*$",
        Database::Regions => r"^http.*Regions.*$",
        Database::RelicArcane => r"^http.*RelicArcane.*$",
        Database::Resources => r"^http.*Resources.*$",
        Database::Sentinels => r"^http.*Sentinels.*$",
        Database::SortieRewards => r"^http.*SortieRewards.*$",
        Database::Upgrades => r"^http.*Upgrades.*$",
        Database::Warframes => r"^http.*Warframes.*$",
        Database::Weapons => r"^http.*Weapons.*$",
        Database::Manifest => r"^http.*Manifest.*$",
    })
    .multi_line(true)
    .build()?;

    let mut path = std::env::current_dir()?;
    path.push(LINKS_SAVE_FILE);

    let mut file = open_no_symlink(path)?;
    let mut urls = String::new();
    file.read_to_string(&mut urls)?;

    let db_url = url_match(&db_regex, &urls).expect("No URL found");

    let request_db_json = reqwest::blocking::get(&db_url)?; //TODO: Error handling

    let db_json_raw: String = request_db_json.text()?;

    // cleaning up so it's able to be parsed
    let db_json = db_json_raw.replace("\r\n", "\\n");

    let write_to = match db {
        Database::Customs => "customs.txt",
        Database::Drones => "drones.txt",
        Database::Flavour => "flavour.txt",
        Database::FusionBundles => "fusion_bundles.txt",
        Database::Gear => "gear.txt",
        Database::Keys => "keys.txt",
        Database::Recipes => "recipes.txt",
        Database::Regions => "regions.txt",
        Database::RelicArcane => "relic_arcane.txt",
        Database::Resources => "resources.txt",
        Database::Sentinels => "sentinels.txt",
        Database::SortieRewards => "sortie_rewards.txt",
        Database::Upgrades => "upgrades.txt",
        Database::Warframes => "warframes.txt",
        Database::Weapons => "weapons.txt",
        Database::Manifest => "manifest.txt",
    };

    let mut file = open_no_symlink(write_to)?;
    file.set_len(0)?;
    file.write_all(db_json.as_bytes())?;

    Ok(())
}

fn url_match(regex: &Regex, urls: &str) -> Option<String> {
    match regex.find(&urls) {
        Some(first_match) => Some(first_match.as_str().to_owned()),
        None => None,
    }
}

// fn borrador() {
//     let secs_since_unix_epoch = SystemTime::now().duration_since(UNIX_EPOCH);
//     if let Ok(seconds) = secs_since_unix_epoch {
//         println!("YAY, seconds: {:?}", seconds);
//         println!("Year: {}", 1970 + seconds.as_secs() / (365 * 24 * 60 * 60));
//     }
//     let possible_path: Result<PathBuf, io::Error> = std::env::current_dir();
//     if let Ok(path) = possible_path {
//         println!("YAY, path: {:?}", path);
//     }
// }

fn open_no_symlink<P: AsRef<Path>>(path: P) -> io::Result<fs::File> {
    let path = path.as_ref();
    if !path.exists() {
        fs::File::create(&path)?;
    }
    if fs::symlink_metadata(path)?.is_symlink() {
        Err(io::Error::other("symlink not allowed"))
    } else {
        fs::OpenOptions::new().write(true).read(true).open(path)
    }
}
