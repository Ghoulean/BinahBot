use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use roxmltree::{Document, Node};
use ruina_common::game_objects::battle_symbol::BattleSymbolSlot;

use crate::reserializer::commons::paths::{game_obj_path, read_xml_files_in_dir};
use crate::reserializer::commons::serde::serialize_option;
use crate::reserializer::commons::xml::{get_nodes, get_unique_node, get_unique_node_text};

fn battle_symbol_path() -> &'static PathBuf {
    static BATTLE_SYMBOL_PATH: OnceLock<PathBuf> = OnceLock::new();
    BATTLE_SYMBOL_PATH.get_or_init(|| {
        fs::canonicalize(game_obj_path().as_path().join(PathBuf::from("GiftInfo"))).unwrap()
    })
}

pub fn reserialize_battle_symbols() -> String {
    let battle_symbols: Vec<String> = read_xml_files_in_dir(battle_symbol_path())
        .into_iter()
        .map(|path_and_document_string| path_and_document_string.1)
        .flat_map(|document_string| process_battle_symbol_file(document_string.as_str()))
        .collect();
    format!(
        "pub const BATTLE_SYMBOLS: [BattleSymbol; {}] = [{}];",
        battle_symbols.len(),
        battle_symbols.join(",")
    )
}

fn process_battle_symbol_file(document_string: &str) -> Vec<String> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "GiftXmlRoot").unwrap();
    let battle_symbol_node_list = get_nodes(xml_root_node, "Gift");

    battle_symbol_node_list
        .into_iter()
        .map(|x| parse_battle_symbol(x))
        .collect()
}

fn parse_battle_symbol(battle_symbol_node: Node) -> String {
    let id = battle_symbol_node.attribute("ID").unwrap();
    // Unsure why empty string results in exclusion from the XML tree, possibly limitation with XML library
    let internal_name = get_unique_node_text(battle_symbol_node, "Name").unwrap_or("");
    let resource = get_unique_node_text(battle_symbol_node, "Resource").unwrap_or("");
    let slot = get_battle_symbol_slot_from_str(
        get_unique_node_text(battle_symbol_node, "Position").unwrap(),
    );
    let hidden = get_unique_node_text(battle_symbol_node, "NoAppear")
        .map(|x| x == "true")
        .unwrap_or(false);
    let count = serialize_option(get_unique_node_text(battle_symbol_node, "Count"));

    format!(
        "BattleSymbol {{
        id: \"{id}\",
        internal_name: \"{internal_name}\",
        resource: \"{resource}\",
        slot: BattleSymbolSlot::{slot:?},
        hidden: {hidden},
        count: {count}
    }}"
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
