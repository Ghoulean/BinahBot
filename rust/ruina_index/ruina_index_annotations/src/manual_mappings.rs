use std::collections::HashMap;
use std::str::FromStr;

use fluent_templates::Loader;
use fluent_templates::StaticLoader;
use ruina_common::localizations::common::Locale;
use ruina_identifier::PageType;
use ruina_identifier::TypedId;
use serde::Deserialize;

use crate::AnnotationMapping;

#[derive(Deserialize, Debug)]
pub struct TomlData {
    pagetype: String,
    id: String,
    localization_id: String
}

pub fn parse_manual_mappings<'a>(locales: &'static StaticLoader, toml_data: &'a [TomlData]) -> AnnotationMapping<'a> {
    let mut map = HashMap::new();
    
    toml_data.iter().for_each(|x| {
        let mut typed_id_map = HashMap::new();

        locales.locales().for_each(|y| {
            typed_id_map.insert(Locale::from(y), locales.lookup(&y, &x.localization_id));
        });

        map.insert(
            TypedId(
                PageType::from_str(&x.pagetype).expect("bad pagetype found in toml"),
                x.id.clone()
            ), 
            typed_id_map
        );
    });

    map
}
