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

