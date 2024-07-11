use std::collections::HashMap;

use roxmltree::{Document, Node};
use ruina_common::localizations::common::Locale;

use crate::{
    serde::{serialize_option_2, string_literal_serializer},
    xml::{get_nodes, get_unique_node, get_unique_node_text},
};

type CombatPageLocaleKey = String;
type CombatPageLocaleValue = String;

pub fn reserialize_combat_page_locales(document_strings: &HashMap<Locale, Vec<String>>) -> String {
    let combat_pages: HashMap<Locale, HashMap<CombatPageLocaleKey, CombatPageLocaleValue>> = document_strings
        .iter()
        .map(|(x, y)| {
            (
                x.clone(),
                y.iter()
                    .flat_map(|document_string| process_combat_page_locale_file(document_string.as_str()))
                    .collect::<HashMap<_, _>>(),
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

fn process_combat_page_locale_file(document_string: &str) -> HashMap<CombatPageLocaleKey, CombatPageLocaleValue> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "BattleCardDescRoot").unwrap();
    let card_desc_list = get_unique_node(xml_root_node, "cardDescList").unwrap();
    let combat_pages = get_nodes(card_desc_list, "BattleCardDesc");

    combat_pages
        .into_iter()
        .map(parse_combat_page_locale)
        .collect()
}

fn parse_combat_page_locale(node: Node) -> (CombatPageLocaleKey, CombatPageLocaleValue) {
    let id = node.attribute("ID").unwrap();
    let name = get_unique_node_text(node, "LocalizedName").unwrap_or("");
    let card_effect = serialize_option_2(get_unique_node_text(node, "AbilityDesc"), string_literal_serializer);
    let dice_desc_overrides = get_nodes(node, "Behaviour");

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
