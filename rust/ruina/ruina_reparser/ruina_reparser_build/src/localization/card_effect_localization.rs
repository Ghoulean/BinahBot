use std::collections::HashMap;

use roxmltree::{Document, Node};
use ruina_common::localizations::common::Locale;

use crate::{
    serde::str_array_serializer,
    xml::{get_nodes, get_nodes_text, get_unique_node},
};

type CardEffectLocaleKey = String;
type CardEffectLocaleValue = String;

pub fn reserialize_card_effect_locales(document_strings: &HashMap<Locale, Vec<String>>) -> String {
    let card_effect: HashMap<Locale, HashMap<CardEffectLocaleKey, CardEffectLocaleValue>> =
        document_strings
            .iter()
            .map(|(x, y)| {
                (
                    x.clone(),
                    y.iter()
                        .flat_map(|document_string| {
                            process_card_effect_locale_file(document_string.as_str())
                        })
                        .collect::<HashMap<_, _>>(),
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

fn process_card_effect_locale_file(
    document_string: &str,
) -> HashMap<CardEffectLocaleKey, CardEffectLocaleValue> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "BattleCardAbilityDescRoot").unwrap();
    let card_effect_list = get_nodes(xml_root_node, "BattleCardAbility");

    card_effect_list
        .into_iter()
        .map(parse_card_effect)
        .collect()
}

fn parse_card_effect(node: Node) -> (CardEffectLocaleKey, CardEffectLocaleValue) {
    let id = node.attribute("ID").unwrap();
    let desc = str_array_serializer(&get_nodes_text(node, "Desc"));

    let key = String::from(id);
    let card_effect_locale = format!(
        "CardEffectLocale {{
        id: \"{id}\",
        desc: {desc}
    }}"
    );

    (key, card_effect_locale)
}
