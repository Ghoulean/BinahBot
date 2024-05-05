use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use roxmltree::{Document, Node};
use ruina_common::localizations::common::Locale;

use crate::reserializer::commons::paths::{localize_paths, read_xml_files_in_dir};
use crate::reserializer::commons::serde::serialize_str_vector;
use crate::reserializer::commons::xml::{
    get_nodes, get_nodes_text, get_unique_node, get_unique_node_text,
};

fn abno_locale_paths() -> &'static Vec<(Locale, PathBuf)> {
    static ABNO_LOCALE_PATHS: OnceLock<Vec<(Locale, PathBuf)>> = OnceLock::new();
    ABNO_LOCALE_PATHS.get_or_init(|| {
        localize_paths()
            .clone()
            .iter()
            .map(|x| {
                (
                    x.0.clone(),
                    fs::canonicalize(x.1.as_path().join(PathBuf::from("AbnormalityCards")))
                        .unwrap(),
                )
            })
            .collect()
    })
}

pub fn reserialize_abno_locales() -> String {
    let abnos: HashMap<Locale, HashMap<String, String>> = abno_locale_paths()
        .iter()
        .map(|x| {
            (
                x.0.clone(),
                read_xml_files_in_dir(&x.1)
                    .into_iter()
                    .map(|path_and_document_string| path_and_document_string.1)
                    .flat_map(|document_string| process_abno_locale_file(document_string.as_str()))
                    .collect(),
            )
        })
        .collect();

    let mut builder = phf_codegen::Map::new();
    for (locale, map) in abnos {
        let mut locale_builder = phf_codegen::Map::new();
        for (key, abno_locale) in map {
            locale_builder.entry(key, abno_locale.as_str());
        }
        let locale_builder_built = locale_builder.build();
        builder.entry(
            locale.to_string(),
            format!("{}", locale_builder_built).as_str(),
        );
    }
    format!(
        "static ABNO_PAGE_LOCALES: phf::Map<&'static str, phf::Map<&str, AbnoPageLocale<'_>>> = {};",
        builder.build()
    )
}

fn process_abno_locale_file(document_string: &str) -> HashMap<String, String> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "AbnormalityCardsRoot").unwrap();
    let sephirah_list = get_nodes(xml_root_node, "Sephirah");

    sephirah_list.into_iter().flat_map(parse_sephirah).collect()
}

fn parse_sephirah(node: Node) -> HashMap<String, String> {
    let floor = node.attribute("SephirahType").unwrap();
    let abno_list = get_nodes(node, "AbnormalityCard");

    abno_list
        .into_iter()
        .map(|x| parse_abno_locale(x, floor))
        .collect()
}

fn parse_abno_locale(node: Node, floor: &str) -> (String, String) {
    let internal_name = node.attribute("ID").unwrap();
    let abnormality = get_unique_node_text(node, "Abnormality").unwrap_or("");
    let card_name = get_unique_node_text(node, "CardName").unwrap();
    let description = get_unique_node_text(node, "AbilityDesc").unwrap();
    let flavor_text = get_unique_node_text(node, "FlaborText").unwrap_or("");
    let dialogues_node = get_unique_node(node, "Dialogue").unwrap();
    let dialogues = serialize_str_vector(get_nodes_text(dialogues_node, "Dialogue"));

    (
        internal_name.to_string(),
        format!(
            "AbnoPageLocale {{
        internal_name: \"{internal_name}\",
        floor: \"{floor}\",
        abnormality: \"{abnormality}\",
        card_name: \"{card_name}\",
        description: r#\"{description}\"#,
        flavor_text: r#\"{flavor_text}\"#,
        dialogues: {dialogues}
    }}"
        ),
    )
}
