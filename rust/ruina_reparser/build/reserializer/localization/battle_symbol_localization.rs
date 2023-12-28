use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use roxmltree::{Document, Node};
use ruina_common::localizations::common::Locale;

use crate::reserializer::commons::paths::{localize_paths, read_xml_files_in_dir};
use crate::reserializer::commons::serde::serialize_option_str;
use crate::reserializer::commons::xml::{get_nodes, get_unique_node, get_unique_node_text};

fn battle_symbol_locale_paths() -> &'static Vec<(Locale, PathBuf)> {
    static BATTLE_SYMBOL_LOCALE_PATHS: OnceLock<Vec<(Locale, PathBuf)>> = OnceLock::new();
    BATTLE_SYMBOL_LOCALE_PATHS.get_or_init(|| {
        localize_paths()
            .clone()
            .iter()
            .map(|x| {
                (
                    x.0.clone(),
                    fs::canonicalize(x.1.as_path().join(PathBuf::from("GiftTexts"))).unwrap(),
                )
            })
            .collect()
    })
}

pub fn reserialize_battle_symbol_locales() -> String {
    let battle_symbols: HashMap<Locale, HashMap<String, String>> = battle_symbol_locale_paths()
        .iter()
        .map(|x| {
            (
                x.0.clone(),
                read_xml_files_in_dir(&x.1)
                    .into_iter()
                    .map(|path_and_document_string| path_and_document_string.1)
                    .flat_map(|document_string| {
                        process_battle_symbol_locale_file(document_string.as_str())
                    })
                    .collect(),
            )
        })
        .collect();

    let mut builder = phf_codegen::Map::new();
    for (locale, map) in battle_symbols {
        let mut locale_builder = phf_codegen::Map::new();
        for (key, battle_symbol_locale) in map {
            locale_builder.entry(key, battle_symbol_locale.as_str());
        }
        let locale_builder_built = locale_builder.build();
        builder.entry(
            locale.to_string(),
            format!("{}", locale_builder_built).as_str(),
        );
    }

    format!("static BATTLE_SYMBOL_LOCALES: phf::Map<&'static str, phf::Map<&str, BattleSymbolLocale<'_>>> = {};", builder.build())
}

fn process_battle_symbol_locale_file(document_string: &str) -> HashMap<String, String> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "GiftTextRoot").unwrap();
    let battle_symbols = get_nodes(xml_root_node, "GiftText");

    battle_symbols
        .into_iter()
        .map(parse_battle_symbol_locale)
        .collect()
}

fn parse_battle_symbol_locale(node: Node) -> (String, String) {
    let id = node.attribute("ID").unwrap();
    let prefix = get_unique_node_text(node, "Prefix").unwrap();
    let postfix = get_unique_node_text(node, "Postfix").unwrap();
    let description = serialize_option_str(get_unique_node_text(node, "Desc"));
    let acquire_condition = serialize_option_str(get_unique_node_text(node, "AcquireCondition"));

    (
        String::from(id),
        format!(
            "BattleSymbolLocale {{
            internal_name: \"{id}\",
            prefix: r#\"{prefix}\"#,
            postfix: r#\"{postfix}\"#,
            description: {description},
            acquire_condition: {acquire_condition}
        }}"
        ),
    )
}
