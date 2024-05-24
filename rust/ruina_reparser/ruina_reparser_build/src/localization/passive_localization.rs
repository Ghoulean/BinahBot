use std::collections::HashMap;

use roxmltree::{Document, Node};
use ruina_common::localizations::common::Locale;

use crate::xml::{get_nodes, get_unique_node, get_unique_node_text};

type PassiveLocaleKey = String;
type PassiveLocaleValue = String;

pub fn reserialize_passive_locales(document_strings: &HashMap<Locale, Vec<String>>) -> String {
    let passives: HashMap<Locale, HashMap<PassiveLocaleKey, PassiveLocaleValue>> = document_strings
        .iter()
        .map(|(x, y)| {
            (
                x.clone(),
                y.into_iter()
                    .flat_map(|document_string| process_passive_locale_file(document_string.as_str()))
                    .collect::<HashMap<_, _>>(),
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

fn process_passive_locale_file(document_string: &str) -> HashMap<PassiveLocaleKey, PassiveLocaleValue> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "PassiveDescRoot").unwrap();
    let passives = get_nodes(xml_root_node, "PassiveDesc");

    passives.into_iter().map(parse_passive_locale).collect()
}

fn parse_passive_locale(node: Node) -> (PassiveLocaleKey, PassiveLocaleValue) {
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
