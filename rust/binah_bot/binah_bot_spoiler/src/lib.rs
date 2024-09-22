use toml::Table;

pub fn build_spoiler_channel_hashmap() -> String {
    let toml = include_str!("./channel_config.toml");
    let toml = toml.parse::<Table>().unwrap();

    let mut builder = phf_codegen::Map::new();
    for (key, val) in toml {
        if val.as_str().filter(|x| *x != "None").is_none() {
            continue;
        }
        builder.entry(
            format!("{}", key),
            &format!(
                "Chapter::{}",
                val.as_str().expect("couldn't get val as string")
            ),
        );
    }
    format!(
        "static SPOILER_CONFIG: phf::Map<&'static str, Chapter> = {};",
        builder.build()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let _does_not_crash = build_spoiler_channel_hashmap();
    }
}
