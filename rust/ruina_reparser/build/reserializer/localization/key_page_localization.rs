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

fn key_page_locale_paths() -> &'static Vec<(Locale, PathBuf)> {
    static KEY_PAGE_LOCALE_PATHS: OnceLock<Vec<(Locale, PathBuf)>> = OnceLock::new();
    KEY_PAGE_LOCALE_PATHS.get_or_init(|| {
        localize_paths()
            .clone()
            .iter()
            .map(|x| {
                (
                    x.0.clone(),
                    fs::canonicalize(x.1.as_path().join(PathBuf::from("Books"))).unwrap(),
                )
            })
            .collect()
    })
}

pub fn reserialize_key_page_locales() -> String {
    let key_pages: HashMap<Locale, HashMap<String, String>> = key_page_locale_paths()
        .iter()
        .map(|x| {
            (
                x.0.clone(),
                read_xml_files_in_dir(&x.1)
                    .into_iter()
                    .map(|path_and_document_string| path_and_document_string.1)
                    .flat_map(|document_string| {
                        process_key_page_locale_file(document_string.as_str())
                    })
                    .collect(),
            )
        })
        .collect();

    let mut builder = phf_codegen::Map::new();
    for (locale, map) in key_pages {
        let mut locale_builder = phf_codegen::Map::new();
        for (key, key_page_locale) in map {
            locale_builder.entry(key, key_page_locale.as_str());
        }
        let locale_builder_built = locale_builder.build();
        builder.entry(
            locale.to_string(),
            format!("{}", locale_builder_built).as_str(),
        );
    }

    format!(
        "static KEY_PAGE_LOCALES: phf::Map<&'static str, phf::Map<&str, KeyPageLocale<'_>>> = {};",
        builder.build()
    )
}

fn process_key_page_locale_file(document_string: &str) -> HashMap<String, String> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "BookDescRoot").unwrap();
    let book_desc_list = get_unique_node(xml_root_node, "bookDescList").unwrap();
    let key_pages = get_nodes(book_desc_list, "BookDesc");

    key_pages.into_iter().map(parse_key_page_locale).collect()
}

fn parse_key_page_locale(node: Node) -> (String, String) {
    let text_id = node.attribute("BookID").unwrap();
    let name = get_unique_node_text(node, "BookName").unwrap_or("");
    let text_list = get_unique_node(node, "TextList").unwrap();
    let description = serialize_str_vector(get_nodes_text(text_list, "Desc"));

    (
        String::from(text_id),
        format!(
            "KeyPageLocale {{
            text_id: \"{text_id}\",
            name: r#\"{name}\"#,
            description: {description},
        }}"
        ),
    )
}
