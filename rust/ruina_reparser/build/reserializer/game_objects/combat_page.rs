use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use roxmltree::{Document, Node};
use ruina_common::game_objects::combat_page::{CombatRange, DieType};

use crate::reserializer::commons::paths::{game_obj_path, read_xml_files_in_dir};
use crate::reserializer::commons::serde::{
    get_chapter_from_str, get_rarity_from_str, serialize_option, serialize_option_debug,
    serialize_option_str, serialize_str_vector,
};
use crate::reserializer::commons::xml::{
    get_nodes, get_nodes_text, get_unique_node, get_unique_node_text,
};

fn combat_page_path() -> &'static PathBuf {
    static COMBAT_PAGE_PATH: OnceLock<PathBuf> = OnceLock::new();
    COMBAT_PAGE_PATH.get_or_init(|| {
        fs::canonicalize(game_obj_path().as_path().join(PathBuf::from("Card"))).unwrap()
    })
}

pub fn reserialize_combat_pages() -> String {
    let combat_pages: Vec<String> = read_xml_files_in_dir(combat_page_path())
        .into_iter()
        .map(|path_and_document_string| path_and_document_string.1)
        .flat_map(|document_string| process_combat_page_file(document_string.as_str()))
        .collect();
    format!(
        "pub const COMBAT_PAGES: [CombatPage; {}] = [{}];",
        combat_pages.len(),
        combat_pages.join(",")
    )
}

fn process_combat_page_file(document_string: &str) -> Vec<String> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "DiceCardXmlRoot").unwrap();
    let combat_node_list = get_nodes(xml_root_node, "Card");

    combat_node_list
        .into_iter()
        .map(|x| parse_combat_page(x))
        .collect()
}

fn parse_combat_page(combat_node: Node) -> String {
    let id = combat_node.attribute("ID").unwrap();
    let artwork = serialize_option_str(get_unique_node_text(combat_node, "Artwork"));
    let cost = get_unique_node(combat_node, "Spec")
        .unwrap()
        .attribute("Cost")
        .unwrap();
    let range = get_range_from_str(
        get_unique_node(combat_node, "Spec")
            .unwrap()
            .attribute("Range")
            .unwrap(),
    );
    let rarity = get_rarity_from_str(get_unique_node_text(combat_node, "Rarity").unwrap());
    let dice = parse_dice(get_unique_node(combat_node, "BehaviourList").unwrap());
    let keywords = serialize_str_vector(get_nodes_text(combat_node, "Keyword"));
    let options = serialize_str_vector(get_nodes_text(combat_node, "Option"));
    let script_id =
        serialize_option_str(get_unique_node_text(combat_node, "Script").filter(|x| !x.is_empty()));
    let skin_change = serialize_option_str(get_unique_node_text(combat_node, "SkinChange"));
    let map_change = serialize_option_str(get_unique_node_text(combat_node, "MapChange"));
    let chapter = serialize_option_debug(
        "Chapter",
        get_unique_node_text(combat_node, "Chapter").map(get_chapter_from_str),
    );
    let priority = serialize_option(get_unique_node_text(combat_node, "Priority"));

    format!(
        "CombatPage {{
        id: \"{id}\",
        artwork: {artwork},
        cost: {cost},
        range: CombatRange::{range:?},
        rarity: Rarity::{rarity:?},
        keywords: {keywords},
        options: {options},
        script_id: {script_id},
        skin_change: {skin_change},
        map_change: {map_change},
        chapter: {chapter},
        priority: {priority},
        dice: {dice}
    }}"
    )
}

fn parse_dice(dice_node: Node) -> String {
    let dice_str = get_nodes(dice_node, "Behaviour")
        .into_iter()
        .map(|x| parse_die(x))
        .collect::<Vec<String>>()
        .join(",");
    format!("&[{}]", dice_str)
}

fn parse_die(die_node: Node) -> String {
    let min = die_node.attribute("Min").unwrap();
    let max = die_node.attribute("Dice").unwrap();
    let die_type = die_node.attribute("Type").unwrap();
    let die_detail = die_node.attribute("Detail").unwrap_or("Slash");
    let die_type = get_die_type_from_strs(die_type, die_detail);
    let script = serialize_option_str(die_node.attribute("Script").filter(|x| !x.is_empty()));
    let actionscript =
        serialize_option_str(die_node.attribute("ActionScript").filter(|x| !x.is_empty()));
    let motion = die_node.attribute("Motion").unwrap();
    let effect_res = serialize_option_str(die_node.attribute("EffectRes"));

    format!(
        "Die{{
        min: {min},
        max: {max},
        die_type:DieType::{die_type:?},
        script: {script},
        actionscript: {actionscript},
        motion: \"{motion}\",
        effect_res: {effect_res}
    }}"
    )
}

fn get_range_from_str(str: &str) -> CombatRange {
    match str {
        "Near" => CombatRange::Melee,
        "Far" => CombatRange::Ranged,
        "FarAreaEach" => CombatRange::MassIndividual,
        "FarArea" => CombatRange::MassSummation,
        "Instance" => CombatRange::OnPlay,
        "Special" => CombatRange::Special,
        _ => panic!("unexpected missing/incorrect combat page range entry"),
    }
}

fn get_die_type_from_strs(type_tag: &str, detail_tag: &str) -> DieType {
    if type_tag == "Standby" {
        match detail_tag {
            "Slash" => DieType::CSlash,
            "Hit" => DieType::CBlunt,
            "Penetrate" => DieType::CPierce,
            "Guard" => DieType::CBlock,
            "Evasion" => DieType::CEvade,
            _ => panic!("unexpected missing/incorrect dice type entry"),
        }
    } else {
        match detail_tag {
            "Slash" => DieType::Slash,
            "Hit" => DieType::Blunt,
            "Penetrate" => DieType::Pierce,
            "Guard" => DieType::Block,
            "Evasion" => DieType::Evade,
            _ => panic!("unexpected missing/incorrect dice type entry"),
        }
    }
}
