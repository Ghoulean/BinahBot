use std::fs::read_to_string;

use toml::Table;

pub fn build_spoiler_channel_hashmap() -> String {
    let toml = read_to_string("channel_config.toml").unwrap_or("".to_string()).parse::<Table>().unwrap();
 
    let mut builder = phf_codegen::Map::new();
    for entry in toml {
        if entry.1.as_str().filter(|x| *x == "None").is_none() {
            continue;
        }
        builder.entry(format!("\"{}\"", entry.0), &format!("Chapter::{:?}", entry.1.as_str()));
    }
    format!(
        "static SPOILER_CONFIG: phf::Map<&'static str, Chapter> = {};",
        builder.build()
    )
}
