use regex::Captures;
use regex::Regex;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The first part of the url where the json files with info about warframes items reside
    let base_url = String::from("http://content.warframe.com/PublicExport/Manifest/");
    
    // From here I get the second part and last part of the URLs with the information
    let url_public_export =
        String::from("https://origin.warframe.com/PublicExport/index_en.txt.lzma");

    // I get a "Response" type from the page that contains info on the HTTP transaction
    let page_info = reqwest::blocking::get(url_public_export)?;//TODO: Error handling

    // I get the body formatted as bytes
    let body_bytes = page_info.bytes()?; //TODO: Error handling

    let mut decompressor = lzma::read(&body_bytes[..])?; //TODO: Error handling
    let mut decompressed_data = Vec::new();

    // Here the data gets decompressed
    let maybe_err = decompressor.read_to_end(&mut decompressed_data)?; //TODO: Error handling
                                                                               

    let decoded_text = String::from_utf8(decompressed_data)?; //TODO: Error handling

    // I put all the urls where I´m going to extract the json files from in the urls String
    // One url per line
    let urls: String = decoded_text
        .lines()
        .map(|line| format!("{}{}\n", &base_url, &line))
        .collect();
    println!("{:?}", urls);
    let weapons_pattern = RegexBuilder::new(r"^http.*Weapons.*$")
        .multi_line(true)
        .build()
        .unwrap();
    let weapons_url = url_match(&weapons_pattern, &urls).expect("No url for weapons found");
    // let weapons_url =  String::from("http://content.warframe.com/PublicExport/Manifest/ExportWeapons_en.json!00_7IpQBg-3w3MSysJSS0CSnw");
    println!("{}", &weapons_url);
    let reqwest_weapons_json_unformatted = reqwest::blocking::get(&weapons_url)?; //TODO: Error handling
    let mut weapons_json_raw: String = reqwest_weapons_json_unformatted
        .text()
        .expect("Weapons url might have failed to fullfil GET reqwest");
    // remove whitespace characters
    weapons_json_raw.retain(|char| !char.is_whitespace());
    let obj: serde_json::Value =
        serde_json::from_str(&weapons_json_raw).expect("failed to convert to json object");
    // println!("{:?}", obj);
    let weapons_json_formatted: String =
        serde_json::to_string_pretty(&obj).expect("couldn´t pretty print");
    //print!("{}", weapons_json_formatted);

    // Some more editing to make the json file more readable
    let replace_me = RegexBuilder::new(r"(\[[0-9,\s]*\],)")
        .dot_matches_new_line(true)
        .build()
        .unwrap();
    let more_formatting = replace_me.replace_all(&weapons_json_formatted, |caps: &Captures| {
        delete_whitespace(&caps[1])
    });
    print!("{}", more_formatting);
    Ok(())
}
#[derive(Serialize, Deserialize)]
struct URL(String);

impl URL {
    fn new(url: &str) -> URL {
        URL(url.to_string())
    }
}

#[derive(Serialize, Deserialize)]
struct WhenLastGET {
    url: URL,
    seconds: u64,
}

impl WhenLastGET {
    fn new(url: &str, seconds: u64) -> WhenLastGET {
        WhenLastGET {
            url: URL(url.to_string()),
            seconds,
        }
    }
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
    }
    let possible_path: Result<PathBuf, io::Error> = std::env::current_dir();
    if let Ok(path) = possible_path {
        println!("YAY, path: {:?}", path);
    }
}

fn open_no_symlink<P: AsRef<Path>>(path: P) -> Result<fs::File, io::Error> {
    let path = path.as_ref();
    if fs::symlink_metadata(path)?.is_symlink() {
        Err(io::Error::other("symlink not allowed"))
    } else {
        fs::File::open(path)
    }
}
