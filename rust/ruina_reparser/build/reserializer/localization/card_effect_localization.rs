use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use roxmltree::{Document, Node};
use ruina_common::localizations::common::Locale;

use crate::reserializer::commons::paths::{localize_paths, read_xml_files_in_dir};
use crate::reserializer::commons::serde::serialize_str_vector;
use crate::reserializer::commons::xml::{get_nodes, get_nodes_text, get_unique_node};

fn card_effect_locale_paths() -> &'static Vec<(Locale, PathBuf)> {
    static CARD_EFFECT_LOCALE_PATHS: OnceLock<Vec<(Locale, PathBuf)>> = OnceLock::new();
    CARD_EFFECT_LOCALE_PATHS.get_or_init(|| {
        localize_paths()
            .clone()
            .iter()
            .map(|x| {
                (
                    x.0.clone(),
                    fs::canonicalize(x.1.as_path().join(PathBuf::from("BattleCardAbilities")))
                        .unwrap(),
                )
            })
            .collect()
    })
}

pub fn reserialize_card_effect_locales() -> String {
    let card_effect: HashMap<Locale, HashMap<String, String>> = card_effect_locale_paths()
        .iter()
        .map(|x| {
            (
                x.0.clone(),
                read_xml_files_in_dir(&x.1)
                    .into_iter()
                    .map(|path_and_document_string| path_and_document_string.1)
                    .flat_map(|document_string| {
                        process_card_effect_locale_file(document_string.as_str())
                    })
                    .collect(),
            )
        })
        .collect();

    let mut builder = phf_codegen::Map::new();
    for (locale, map) in card_effect {
        let mut locale_builder = phf_codegen::Map::new();
        for (key, card_effect_locale) in map {
            locale_builder.entry(key, card_effect_locale.as_str());
        }
        let locale_builder_built = locale_builder.build();
        builder.entry(
            locale.to_string(),
            format!("{}", locale_builder_built).as_str(),
        );
    }
    format!("static CARD_EFFECT_LOCALES: phf::Map<&'static str, phf::Map<&str, CardEffectLocale<'_>>> = {};", builder.build())
}

fn process_card_effect_locale_file(document_string: &str) -> HashMap<String, String> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "BattleCardAbilityDescRoot").unwrap();
    let card_effect_list = get_nodes(xml_root_node, "BattleCardAbility");

    card_effect_list
        .into_iter()
        .map(parse_card_effect)
        .collect()
}

fn parse_card_effect(node: Node) -> (String, String) {
    let id = node.attribute("ID").unwrap();
    let desc = serialize_str_vector(get_nodes_text(node, "Desc"));

    let key = String::from(id);
    let card_effect_locale = format!(
        "CardEffectLocale {{
        id: \"{id}\",
        desc: {desc}
    }}"
    );

    (key, card_effect_locale)
}
