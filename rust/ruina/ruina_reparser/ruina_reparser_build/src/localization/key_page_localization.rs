use std::collections::HashMap;

use roxmltree::{Document, Node};
use ruina_common::localizations::common::Locale;

use crate::{
    serde::str_array_serializer,
    xml::{get_nodes, get_nodes_text, get_unique_node, get_unique_node_text},
};

type KeyPageLocaleKey = String;
type KeyPageLocaleValue = String;

pub fn reserialize_key_page_locales(document_strings: &HashMap<Locale, Vec<String>>) -> String {
    let key_pages: HashMap<Locale, HashMap<KeyPageLocaleKey, KeyPageLocaleValue>> =
        document_strings
            .iter()
            .map(|(x, y)| {
                (
                    x.clone(),
                    y.iter()
                        .flat_map(|document_string| {
                            process_key_page_locale_file(document_string.as_str())
                        })
                        .collect::<HashMap<_, _>>(),
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

fn process_key_page_locale_file(
    document_string: &str,
) -> HashMap<KeyPageLocaleKey, KeyPageLocaleValue> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "BookDescRoot").unwrap();
    let book_desc_list = get_unique_node(xml_root_node, "bookDescList").unwrap();
    let key_pages = get_nodes(book_desc_list, "BookDesc");

    key_pages.into_iter().map(parse_key_page_locale).collect()
}

fn parse_key_page_locale(node: Node) -> (KeyPageLocaleKey, KeyPageLocaleValue) {
    let text_id = node.attribute("BookID").unwrap();
    let name = get_unique_node_text(node, "BookName").unwrap_or("");
    let text_list = get_unique_node(node, "TextList").unwrap();
    let description = str_array_serializer(&get_nodes_text(text_list, "Desc"));

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
