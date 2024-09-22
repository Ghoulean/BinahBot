use std::collections::HashMap;

use roxmltree::{Document, Node};
use ruina_common::game_objects::combat_page::{CombatRange, DieType};
use ruina_common::game_objects::common::{Collectability, PageType};

use crate::game_objects::common::CollectabilityMap;
use crate::game_objects::common::ParserProps;
use crate::serde::{
    chapter_enum_serializer, display_serializer, get_rarity_from_str, serialize_option_2,
    str_array_serializer, string_literal_serializer,
};
use crate::xml::get_nodes_text;
use crate::xml::{get_nodes, get_unique_node, get_unique_node_text};

use super::common::{from_chapter_map, ChapterMap};

type CombatPageKey = String;
type CombatPageValue = String;

pub fn reserialize_combat_pages(parser_props: &ParserProps) -> String {
    let combat_pages: HashMap<_, _> = parser_props
        .document_strings
        .iter()
        .flat_map(|document_string| {
            process_combat_page_file(
                document_string.as_str(),
                parser_props.collectability_map,
                parser_props.chapter_map,
            )
        })
        .collect();

    let mut builder = phf_codegen::Map::new();
    for (id, combat_page_entry) in combat_pages {
        builder.entry(id.clone(), &combat_page_entry);
    }
    format!(
        "static COMBAT_PAGES: phf::Map<&'static str, CombatPage> = {};",
        builder.build()
    )
}

fn process_combat_page_file(
    document_string: &str,
    collectability_map: &CollectabilityMap,
    chapter_map: &ChapterMap,
) -> HashMap<CombatPageKey, CombatPageValue> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "DiceCardXmlRoot").unwrap();
    let combat_node_list = get_nodes(xml_root_node, "Card");

    combat_node_list
        .into_iter()
        .map(|x| parse_combat_page(x, collectability_map, chapter_map))
        .collect()
}

fn parse_combat_page(
    combat_node: Node,
    collectability_map: &CollectabilityMap,
    chapter_map: &ChapterMap,
) -> (CombatPageKey, CombatPageValue) {
    let id = combat_node.attribute("ID").unwrap();
    let artwork = serialize_option_2(
        get_unique_node_text(combat_node, "Artwork"),
        string_literal_serializer,
    );
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
    let keywords = str_array_serializer(&get_nodes_text(combat_node, "Keyword"));
    let options = str_array_serializer(&get_nodes_text(combat_node, "Option"));
    let script_id = serialize_option_2(
        get_unique_node_text(combat_node, "Script").filter(|x| !x.is_empty()),
        string_literal_serializer,
    );
    let skin_change = serialize_option_2(
        get_unique_node_text(combat_node, "SkinChange"),
        string_literal_serializer,
    );
    let map_change = serialize_option_2(
        get_unique_node_text(combat_node, "MapChange"),
        string_literal_serializer,
    );
    let chapter = serialize_option_2(
        from_chapter_map(&id, &PageType::CombatPage, chapter_map),
        chapter_enum_serializer,
    );
    let priority = serialize_option_2(
        get_unique_node_text(combat_node, "Priority"),
        display_serializer,
    );
    let is_collectable = collectability_map
        .collectable
        .combat_pages
        .iter()
        .any(|x| x == id);
    let is_obtainable = collectability_map
        .obtainable
        .combat_pages
        .iter()
        .any(|x| x == id);
    let is_enemy_only = collectability_map
        .enemy_only
        .combat_pages
        .iter()
        .any(|x| x == id);

    let collectability = if is_collectable {
        Collectability::Collectable
    } else if is_obtainable {
        Collectability::Obtainable
    } else if is_enemy_only {
        Collectability::EnemyOnly
    } else {
        unreachable!()
    };

    (
        id.to_string(),
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
        dice: {dice},
        collectability: Collectability::{collectability:?}
    }}"
        ),
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
    let script = serialize_option_2(
        die_node.attribute("Script").filter(|x| !x.is_empty()),
        string_literal_serializer,
    );
    let actionscript = serialize_option_2(
        die_node.attribute("ActionScript").filter(|x| !x.is_empty()),
        string_literal_serializer,
    );
    let motion = die_node.attribute("Motion").unwrap();
    let effect_res = serialize_option_2(die_node.attribute("EffectRes"), string_literal_serializer);

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
