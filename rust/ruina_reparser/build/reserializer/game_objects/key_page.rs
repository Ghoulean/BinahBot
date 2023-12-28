use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use roxmltree::{Document, Node};
use ruina_common::game_objects::key_page::{KeyPageRange, Resistance};

use crate::reserializer::commons::paths::{game_obj_path, read_xml_files_in_dir};
use crate::reserializer::commons::serde::{
    get_chapter_from_str, get_rarity_from_str, serialize_option_debug, serialize_option_str,
    serialize_str_vector,
};
use crate::reserializer::commons::xml::{
    get_nodes, get_nodes_text, get_unique_node, get_unique_node_text,
};

fn key_page_path() -> &'static PathBuf {
    static KEY_PAGE_PATH: OnceLock<PathBuf> = OnceLock::new();
    KEY_PAGE_PATH.get_or_init(|| {
        fs::canonicalize(game_obj_path().as_path().join(PathBuf::from("EquipPage"))).unwrap()
    })
}

pub fn reserialize_key_pages() -> String {
    let key_pages: Vec<String> = read_xml_files_in_dir(key_page_path())
        .into_iter()
        .map(|path_and_document_string| path_and_document_string.1)
        .flat_map(|document_string| process_key_page_file(document_string.as_str()))
        .collect();
    format!(
        "pub const KEY_PAGES: [KeyPage; {}] = [{}];",
        key_pages.len(),
        key_pages.join(",")
    )
}

fn process_key_page_file(document_string: &str) -> Vec<String> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "BookXmlRoot").unwrap();
    let key_page_node_list = get_nodes(xml_root_node, "Book");

    key_page_node_list
        .into_iter()
        .map(|x| parse_key_page(x))
        .collect()
}

fn parse_key_page(key_node: Node) -> String {
    let id = key_node.attribute("ID").unwrap();
    let equip_effect_node = get_unique_node(key_node, "EquipEffect").unwrap();
    let skin = serialize_option_str(get_unique_node_text(key_node, "CharacterSkin"));
    let hp = get_unique_node_text(equip_effect_node, "HP").unwrap();
    let stagger = get_unique_node_text(equip_effect_node, "Break").unwrap();
    let min_speed = get_unique_node_text(equip_effect_node, "SpeedMin").unwrap();
    let max_speed = get_unique_node_text(equip_effect_node, "Speed").unwrap();
    let book_icon = serialize_option_str(get_unique_node_text(key_node, "BookIcon"));
    let rarity = get_rarity_from_str(get_unique_node_text(key_node, "Rarity").unwrap());
    let base_speed_die = get_unique_node_text(key_node, "SpeedDiceNum").unwrap_or("1");
    let starting_light = get_unique_node_text(key_node, "StartPlayPoint").unwrap_or("3");
    let base_light = get_unique_node_text(key_node, "MaxPlayPoint").unwrap_or("3");
    let range =
        get_key_page_range_from_str(get_unique_node_text(key_node, "RangeType").unwrap_or("Melee"));
    let episode_id = serialize_option_str(get_unique_node_text(key_node, "Episode"));
    let passive_ids = serialize_str_vector(get_nodes_text(equip_effect_node, "Passive"));
    let options = serialize_str_vector(get_nodes_text(key_node, "Option"));
    let chapter = serialize_option_debug(
        "Chapter",
        get_unique_node_text(key_node, "Chapter").map(get_chapter_from_str),
    );
    let category = serialize_option_str(get_unique_node_text(key_node, "Category"));
    let only_card_ids = serialize_str_vector(get_nodes_text(equip_effect_node, "OnlyCard"));

    let hp_slash_resist = get_resistance_from_str(
        get_unique_node_text(equip_effect_node, "SResist").unwrap_or("Normal"),
    );
    let hp_pierce_resist = get_resistance_from_str(
        get_unique_node_text(equip_effect_node, "PResist").unwrap_or("Normal"),
    );
    let hp_blunt_resist = get_resistance_from_str(
        get_unique_node_text(equip_effect_node, "HResist").unwrap_or("Normal"),
    );
    let stagger_slash_resist = get_resistance_from_str(
        get_unique_node_text(equip_effect_node, "SBResist").unwrap_or("Normal"),
    );
    let stagger_pierce_resist = get_resistance_from_str(
        get_unique_node_text(equip_effect_node, "PBResist").unwrap_or("Normal"),
    );
    let stagger_blunt_resist = get_resistance_from_str(
        get_unique_node_text(equip_effect_node, "HBResist").unwrap_or("Normal"),
    );

    format!(
        "KeyPage {{
        id: \"{id}\",
        skin: {skin},
        book_icon: {book_icon},
        hp: {hp},
        stagger: {stagger},
        min_speed: {min_speed},
        max_speed: {max_speed},
        resists: KeyPageResists {{
            hp_slash: Resistance::{hp_slash_resist:?},
            hp_pierce: Resistance::{hp_pierce_resist:?},
            hp_blunt: Resistance::{hp_blunt_resist:?},
            stagger_slash: Resistance::{stagger_slash_resist:?},
            stagger_pierce: Resistance::{stagger_pierce_resist:?},
            stagger_blunt: Resistance::{stagger_blunt_resist:?}
        }},
        base_speed_die: {base_speed_die},
        starting_light: {starting_light},
        base_light: {base_light},
        range: KeyPageRange::{range:?},
        rarity: Rarity::{rarity:?},
        episode_id: {episode_id},
        passive_ids: {passive_ids},
        options: {options},
        chapter: {chapter},
        category: {category},
        only_card_ids: {only_card_ids}
    }}"
    )
}

fn get_key_page_range_from_str(str: &str) -> KeyPageRange {
    match str {
        "Melee" => KeyPageRange::Melee,
        "Range" => KeyPageRange::Ranged,
        "Hybrid" => KeyPageRange::Hybrid,
        _ => panic!("unexpected missing/incorrect KeyPageRange entry: {}", str),
    }
}

fn get_resistance_from_str(str: &str) -> Resistance {
    match str {
        "Weak" => Resistance::Fatal,
        "Vulnerable" => Resistance::Weak,
        "Normal" => Resistance::Normal,
        "Endure" => Resistance::Endured,
        "Resist" => Resistance::Ineffective,
        "Immune" => Resistance::Immune,
        _ => panic!("unexpected missing/incorrect Resistance entry: {}", str),
    }
}
