//  use regex::Captures;
use regex::Regex;
use regex::RegexBuilder;
use strum::IntoEnumIterator;
//use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io};

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
    Ok(())
}

struct ProgramData {
    // date = seconds since unix epoch till given time
    date_last_time_program_run: u64,
    date_last_public_export_update: u64,
    links_up_to_date: bool,
    json_weapons_up_to_date: bool,
    json_warframes_up_to_date: bool,
}

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

    let mut db_json: String = request_db_json.text()?;

    // remove whitespace characters
    db_json.retain(|char| !char.is_whitespace());

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

fn delete_whitespace(string: &str) -> String {
    let mut output = string.to_string();
    output.retain(|c| !c.is_whitespace());
    output
}

fn url_match(regex: &Regex, urls: &str) -> Option<String> {
    match regex.find(&urls) {
        Some(first_match) => Some(first_match.as_str().to_owned()),
        None => None,
    }
}

fn borrador() {
    let secs_since_unix_epoch = SystemTime::now().duration_since(UNIX_EPOCH);
    if let Ok(seconds) = secs_since_unix_epoch {
        println!("YAY, seconds: {:?}", seconds);
        println!("Year: {}", 1970 + seconds.as_secs() / (365 * 24 * 60 * 60));
    }
    let possible_path: Result<PathBuf, io::Error> = std::env::current_dir();
    if let Ok(path) = possible_path {
        println!("YAY, path: {:?}", path);
    }
}

fn open_no_symlink<P: AsRef<Path>>(path: P) -> io::Result<fs::File> {
    let path = path.as_ref();
    if !path.exists() {
        fs::File::create(&path)?;
    }
    if fs::symlink_metadata(path)?.is_symlink() {
        Err(io::Error::other("symlink not allowed"))
    } else {
        fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(path)
    }
}
