use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use roxmltree::{Document, Node};
use ruina_common::localizations::common::Locale;

use crate::reserializer::commons::paths::{localize_paths, read_xml_files_in_dir};
use crate::reserializer::commons::serde::serialize_option_str;
use crate::reserializer::commons::xml::{get_nodes, get_unique_node, get_unique_node_text};

fn combat_page_locale_paths() -> &'static Vec<(Locale, PathBuf)> {
    static COMBAT_PAGE_LOCALE_PATHS: OnceLock<Vec<(Locale, PathBuf)>> = OnceLock::new();
    COMBAT_PAGE_LOCALE_PATHS.get_or_init(|| {
        localize_paths()
            .clone()
            .iter()
            .map(|x| {
                (
                    x.0.clone(),
                    fs::canonicalize(x.1.as_path().join(PathBuf::from("BattlesCards"))).unwrap(),
                )
            })
            .collect()
    })
}

pub fn reserialize_combat_page_locales() -> String {
    let combat_pages: HashMap<Locale, HashMap<String, String>> = combat_page_locale_paths()
        .iter()
        .map(|x| {
            (
                x.0.clone(),
                read_xml_files_in_dir(&x.1)
                    .into_iter()
                    .map(|path_and_document_string| path_and_document_string.1)
                    .flat_map(|document_string| {
                        process_combat_page_locale_file(document_string.as_str())
                    })
                    .collect(),
            )
        })
        .collect();

    let mut builder = phf_codegen::Map::new();
    for (locale, map) in combat_pages {
        let mut locale_builder = phf_codegen::Map::new();
        for (key, combat_page_locale) in map {
            locale_builder.entry(key, combat_page_locale.as_str());
        }
        let locale_builder_built = locale_builder.build();
        builder.entry(
            locale.to_string(),
            format!("{}", locale_builder_built).as_str(),
        );
    }

    format!("static COMBAT_PAGE_LOCALES: phf::Map<&'static str, phf::Map<&str, CombatPageLocale<'_>>> = {};", builder.build())
}

fn process_combat_page_locale_file(document_string: &str) -> HashMap<String, String> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "BattleCardDescRoot").unwrap();
    let card_desc_list = get_unique_node(xml_root_node, "cardDescList").unwrap();
    let combat_pages = get_nodes(card_desc_list, "BattleCardDesc");

    combat_pages
        .into_iter()
        .map(parse_combat_page_locale)
        .collect()
}

fn parse_combat_page_locale(node: Node) -> (String, String) {
    let id = node.attribute("ID").unwrap();
    let name = get_unique_node_text(node, "LocalizedName").unwrap_or("");
    let card_effect = serialize_option_str(get_unique_node_text(node, "AbilityDesc"));

    (
        String::from(id),
        format!(
            "CombatPageLocale {{
            id: \"{id}\",
            name: r#\"{name}\"#,
            card_effect: {card_effect},
        }}"
        ),
    )
}
