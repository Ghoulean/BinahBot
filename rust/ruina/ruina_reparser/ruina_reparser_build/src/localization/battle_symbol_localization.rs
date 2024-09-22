use std::collections::HashMap;

use roxmltree::{Document, Node};
use ruina_common::localizations::common::Locale;

use crate::{
    serde::{serialize_option_2, string_literal_serializer},
    xml::{get_nodes, get_unique_node, get_unique_node_text},
};

type BattleSymbolLocaleKey = String;
type BattleSymbolLocaleValue = String;

pub fn reserialize_battle_symbol_locales(
    document_strings: &HashMap<Locale, Vec<String>>,
) -> String {
    let battle_symbols: HashMap<Locale, HashMap<BattleSymbolLocaleKey, BattleSymbolLocaleValue>> =
        document_strings
            .iter()
            .map(|(x, y)| {
                (
                    x.clone(),
                    y.iter()
                        .flat_map(|document_string| {
                            process_battle_symbol_locale_file(document_string.as_str())
                        })
                        .collect::<HashMap<_, _>>(),
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

fn process_battle_symbol_locale_file(
    document_string: &str,
) -> HashMap<BattleSymbolLocaleKey, BattleSymbolLocaleValue> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "GiftTextRoot").unwrap();
    let battle_symbols = get_nodes(xml_root_node, "GiftText");

    battle_symbols
        .into_iter()
        .map(parse_battle_symbol_locale)
        .collect()
}

fn parse_battle_symbol_locale(node: Node) -> (BattleSymbolLocaleKey, BattleSymbolLocaleValue) {
    let id = node.attribute("ID").unwrap();
    let prefix = get_unique_node_text(node, "Prefix").unwrap();
    let postfix = get_unique_node_text(node, "Postfix").unwrap();
    let description = serialize_option_2(
        get_unique_node_text(node, "Desc"),
        string_literal_serializer,
    );
    let acquire_condition = serialize_option_2(
        get_unique_node_text(node, "AcquireCondition"),
        string_literal_serializer,
    );

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
