use std::collections::HashMap;

use roxmltree::{Document, Node};
use ruina_common::localizations::common::Locale;

use crate::{
    serde::str_array_serializer,
    xml::{get_nodes, get_nodes_text, get_unique_node, get_unique_node_text},
};

type AbnoPageLocaleKey = String;
type AbnoPageLocaleValue = String;

pub fn reserialize_abno_locales(document_strings: &HashMap<Locale, Vec<String>>) -> String {
    let abnos: HashMap<Locale, HashMap<AbnoPageLocaleKey, AbnoPageLocaleValue>> = document_strings
        .iter()
        .map(|(x, y)| {
            (
                x.clone(),
                y.iter()
                    .flat_map(|document_string| process_abno_locale_file(document_string.as_str()))
                    .collect::<HashMap<_, _>>(),
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

fn process_abno_locale_file(document_string: &str) -> HashMap<AbnoPageLocaleKey, AbnoPageLocaleValue> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "AbnormalityCardsRoot").unwrap();
    let sephirah_list = get_nodes(xml_root_node, "Sephirah");

    sephirah_list.into_iter().flat_map(parse_sephirah).collect()
}

fn parse_sephirah(node: Node) -> HashMap<AbnoPageLocaleKey, AbnoPageLocaleValue> {
    let floor = node.attribute("SephirahType").unwrap();
    let abno_list = get_nodes(node, "AbnormalityCard");

    abno_list
        .into_iter()
        .map(|x| parse_abno_locale(x, floor))
        .collect()
}

fn parse_abno_locale(node: Node, floor: &str) -> (AbnoPageLocaleKey, AbnoPageLocaleValue) {
    let internal_name = node.attribute("ID").unwrap();
    let abnormality = get_unique_node_text(node, "Abnormality").unwrap_or("");
    let card_name = get_unique_node_text(node, "CardName").unwrap();
    let description = get_unique_node_text(node, "AbilityDesc").unwrap();
    let flavor_text = get_unique_node_text(node, "FlaborText").unwrap_or("");
    let dialogues_node = get_unique_node(node, "Dialogue").unwrap();
    let dialogues = str_array_serializer(&get_nodes_text(dialogues_node, "Dialogue"));

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
