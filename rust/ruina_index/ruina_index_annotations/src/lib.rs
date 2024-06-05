mod manual_mappings;

use std::collections::HashMap;
use manual_mappings::parse_manual_mappings;
use manual_mappings::TomlData;
use ruina_common::localizations::common::Locale;
use ruina_identifier::TypedId;
use toml::from_str;

fluent_templates::static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

pub type AnnotationMapping<'a> = HashMap<TypedId, HashMap<Locale, String>>;

pub fn precompute_annotations_map<'a>() -> AnnotationMapping<'a> {
    let manual_annotation_toml_map: HashMap<String, Vec<TomlData>> = from_str(
        include_str!("../manual/annotations.toml")
    ).unwrap();
    let manual_annotations_toml = manual_annotation_toml_map.get("entry").expect("couldn't find \"entry\" from toml");

    let manual_mappings = parse_manual_mappings(&LOCALES, &manual_annotations_toml);

    manual_mappings
}

pub fn precompute_disambiguations_map<'a>() -> AnnotationMapping<'a> {
    let mut manual_disambiguation_toml_map: HashMap<String, Vec<TomlData>> = from_str(
        include_str!("../manual/disambiguations.toml")
    ).unwrap();
    let manual_disambiguation_toml = manual_disambiguation_toml_map.remove("entry").take().expect("couldn't find \"entry\" from toml");

    let manual_mappings = parse_manual_mappings(&LOCALES, &manual_disambiguation_toml);

    manual_mappings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let _does_not_crash = precompute_annotations_map();
        let _does_not_crash_2 = precompute_disambiguations_map();
    }
}