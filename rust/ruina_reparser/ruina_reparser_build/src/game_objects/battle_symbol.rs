use std::collections::HashMap;

use roxmltree::{Document, Node};
use ruina_common::game_objects::battle_symbol::BattleSymbolSlot;

use crate::serde::{display_serializer, serialize_option_2};
use crate::xml::{get_nodes, get_unique_node, get_unique_node_text};

type BattleSymbolKey = String;
type BattleSymbolValue = String;

pub fn reserialize_battle_symbols(document_strings: &[String]) -> String {
    let battle_symbols: HashMap<_, _> = document_strings
        .iter()
        .flat_map(|document_string| process_battle_symbol_file(document_string))
        .collect();

    let mut builder = phf_codegen::Map::new();
    for (id, battle_symbol_entry) in battle_symbols {
        builder.entry(id.clone(), &battle_symbol_entry);
    }
    format!(
        "static BATTLE_SYMBOLS: phf::Map<&'static str, BattleSymbol> = {};",
        builder.build()
    )
}

fn process_battle_symbol_file(document_string: &str) -> HashMap<BattleSymbolKey, BattleSymbolValue> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "GiftXmlRoot").unwrap();
    let battle_symbol_node_list = get_nodes(xml_root_node, "Gift");

    battle_symbol_node_list
        .into_iter()
        .map(|x| parse_battle_symbol(x))
        .collect()
}

fn parse_battle_symbol(battle_symbol_node: Node) -> (BattleSymbolKey, BattleSymbolValue) {
    let id = battle_symbol_node.attribute("ID").unwrap();
    // Unsure why empty string results in exclusion from the generated XML tree,
    // possibly a limitation with XML library
    let internal_name = get_unique_node_text(battle_symbol_node, "Name").unwrap_or("");

    // TODO - change this to optional
    let resource = get_unique_node_text(battle_symbol_node, "Resource").unwrap_or("");
    let slot = get_battle_symbol_slot_from_str(
        get_unique_node_text(battle_symbol_node, "Position").unwrap(),
    );
    let hidden = get_unique_node_text(battle_symbol_node, "NoAppear")
        .map(|x| x == "true")
        .unwrap_or(false);
    let count = serialize_option_2(
        get_unique_node_text(battle_symbol_node, "Count"),
        display_serializer
    );

    (
        internal_name.to_string(),
        format!(
            "BattleSymbol {{
        id: \"{id}\",
        internal_name: \"{internal_name}\",
        resource: \"{resource}\",
        slot: BattleSymbolSlot::{slot:?},
        hidden: {hidden},
        count: {count}
    }}"
        ),
    )
}

fn get_battle_symbol_slot_from_str(str: &str) -> BattleSymbolSlot {
    match str {
        "Eye" => BattleSymbolSlot::Eye,
        "Nose" => BattleSymbolSlot::Nose,
        "Cheek" => BattleSymbolSlot::Cheek,
        "Mouth" => BattleSymbolSlot::Mouth,
        "Ear" => BattleSymbolSlot::Ear,
        "HairAccessory" => BattleSymbolSlot::Headwear1,
        "Hood" => BattleSymbolSlot::Headwear2,
        "Mask" => BattleSymbolSlot::Headwear3,
        "Helmet" => BattleSymbolSlot::Headwear4,
        "None" => BattleSymbolSlot::None,
        _ => panic!("unexpected missing/incorrect battle symbol slot entry"),
    }
}
