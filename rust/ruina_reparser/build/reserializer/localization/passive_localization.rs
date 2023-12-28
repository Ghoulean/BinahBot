use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use roxmltree::{Document, Node};
use ruina_common::localizations::common::Locale;

use crate::reserializer::commons::paths::{localize_paths, read_xml_files_in_dir};
use crate::reserializer::commons::xml::{get_nodes, get_unique_node, get_unique_node_text};

fn passive_locale_paths() -> &'static Vec<(Locale, PathBuf)> {
    static PASSIVE_LOCALE_PATHS: OnceLock<Vec<(Locale, PathBuf)>> = OnceLock::new();
    PASSIVE_LOCALE_PATHS.get_or_init(|| {
        localize_paths()
            .clone()
            .iter()
            .map(|x| {
                (
                    x.0.clone(),
                    fs::canonicalize(x.1.as_path().join(PathBuf::from("PassiveDesc"))).unwrap(),
                )
            })
            .collect()
    })
}

pub fn reserialize_passive_locales() -> String {
    let passives: HashMap<Locale, HashMap<String, String>> = passive_locale_paths()
        .iter()
        .map(|x| {
            (
                x.0.clone(),
                read_xml_files_in_dir(&x.1)
                    .into_iter()
                    .map(|path_and_document_string| path_and_document_string.1)
                    .flat_map(|document_string| {
                        process_passive_locale_file(document_string.as_str())
                    })
                    .collect(),
            )
        })
        .collect();

    let mut builder = phf_codegen::Map::new();
    for (locale, map) in passives {
        let mut locale_builder = phf_codegen::Map::new();
        for (key, passive_locale) in map {
            locale_builder.entry(key, passive_locale.as_str());
        }
        let locale_builder_built = locale_builder.build();
        builder.entry(
            locale.to_string(),
            format!("{}", locale_builder_built).as_str(),
        );
    }

    format!(
        "static PASSIVE_LOCALES: phf::Map<&'static str, phf::Map<&str, PassiveLocale<'_>>> = {};",
        builder.build()
    )
}

fn process_passive_locale_file(document_string: &str) -> HashMap<String, String> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "PassiveDescRoot").unwrap();
    let passives = get_nodes(xml_root_node, "PassiveDesc");

    passives.into_iter().map(parse_passive_locale).collect()
}

fn parse_passive_locale(node: Node) -> (String, String) {
    let id = node.attribute("ID").unwrap();
    let name = get_unique_node_text(node, "Name").unwrap();
    let description = get_unique_node_text(node, "Desc").unwrap_or("");

    (
        String::from(id),
        format!(
            "PassiveLocale {{
            id: \"{id}\",
            name: r#\"{name}\"#,
            description: r#\"{description}\"#,
        }}"
        ),
    )
}
