use std::collections::HashMap;

use roxmltree::{Document, Node};
use ruina_common::game_objects::common::{Collectability, PageType};
use ruina_common::game_objects::key_page::{KeyPageRange, Resistance};

use crate::game_objects::common::CollectabilityMap;
use crate::game_objects::common::ParserProps;
use crate::serde::{
    chapter_enum_serializer, get_rarity_from_str, serialize_option_2, str_array_serializer, string_literal_serializer
};
use crate::xml::get_nodes_text;
use crate::xml::{get_nodes, get_unique_node, get_unique_node_text};

use super::common::{from_chapter_map, ChapterMap};

type KeyPageKey = String;
type KeyPageValue = String;

pub fn reserialize_key_pages(parser_props: &ParserProps) -> String {
    let key_pages: HashMap<_, _> = parser_props.document_strings
        .iter()
        .flat_map(|document_string| process_key_page_file(
            document_string.as_str(),
            parser_props.collectability_map,
            parser_props.chapter_map,
        ))
        .collect();

    let mut builder = phf_codegen::Map::new();
    for (id, key_page_entry) in key_pages {
        builder.entry(id.clone(), &key_page_entry);
    }
    format!(
        "static KEY_PAGES: phf::Map<&'static str, KeyPage> = {};",
        builder.build()
    )
}

fn process_key_page_file(
    document_string: &str,
    collectability_map: &CollectabilityMap,
    chapter_map: &ChapterMap,
) -> HashMap<KeyPageKey, KeyPageValue> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "BookXmlRoot").unwrap();
    let key_page_node_list = get_nodes(xml_root_node, "Book");

    key_page_node_list
        .into_iter()
        .map(|x| parse_key_page(x, collectability_map, chapter_map))
        .collect()
}

fn parse_key_page(
    key_node: Node,
    collectability_map: &CollectabilityMap,
    chapter_map: &ChapterMap,
) -> (KeyPageKey, KeyPageValue) {
    let id = key_node.attribute("ID").unwrap();
    let text_id = serialize_option_2(get_unique_node_text(key_node, "TextId"), string_literal_serializer);
    let equip_effect_node = get_unique_node(key_node, "EquipEffect").unwrap();
    let skin = serialize_option_2(get_unique_node_text(key_node, "CharacterSkin"), string_literal_serializer);
    let hp = get_unique_node_text(equip_effect_node, "HP").unwrap();
    let stagger = get_unique_node_text(equip_effect_node, "Break").unwrap();
    let min_speed = get_unique_node_text(equip_effect_node, "SpeedMin").unwrap();
    let max_speed = get_unique_node_text(equip_effect_node, "Speed").unwrap();
    let book_icon = serialize_option_2(get_unique_node_text(key_node, "BookIcon"), string_literal_serializer);
    let rarity = get_rarity_from_str(get_unique_node_text(key_node, "Rarity").unwrap());
    let base_speed_die = get_unique_node_text(key_node, "SpeedDiceNum").unwrap_or("1");
    let starting_light = get_unique_node_text(key_node, "StartPlayPoint").unwrap_or("3");
    let base_light = get_unique_node_text(key_node, "MaxPlayPoint").unwrap_or("3");
    let range =
        get_key_page_range_from_str(get_unique_node_text(key_node, "RangeType").unwrap_or("Melee"));
    let episode_id = serialize_option_2(get_unique_node_text(key_node, "Episode"), string_literal_serializer);
    let passive_ids = str_array_serializer(&get_nodes_text(equip_effect_node, "Passive"));
    let options = str_array_serializer(&get_nodes_text(key_node, "Option"));
    let chapter = serialize_option_2(
        from_chapter_map(&id, &PageType::KeyPage, chapter_map),
        chapter_enum_serializer
    );
    let category = serialize_option_2(get_unique_node_text(key_node, "Category"), string_literal_serializer);
    let only_card_ids = str_array_serializer(&get_nodes_text(equip_effect_node, "OnlyCard"));

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

    let is_collectable = collectability_map.collectable.key_pages.iter().any(|x| x == id);
    let is_enemy_only = collectability_map.enemy_only.key_pages.iter().any(|x| x == id);

    let collectability = if is_collectable {
        Collectability::Collectable
    } else if is_enemy_only {
        Collectability::EnemyOnly
    } else {
        unreachable!()
    };

    (
        id.to_string(),
        format!(
            "KeyPage {{
        text_id: {text_id},
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
        only_card_ids: {only_card_ids},
        collectability: Collectability::{collectability:?},
    }}"
        ),
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
