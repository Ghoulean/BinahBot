use std::collections::HashMap;

use roxmltree::{Document, Node};
use ruina_common::game_objects::common::Collectability;
use ruina_common::game_objects::common::PageType;

use crate::game_objects::common::CollectabilityMap;
use crate::game_objects::common::ParserProps;
use crate::serde::chapter_enum_serializer;
use crate::serde::{
    display_serializer, get_rarity_from_str, rarity_enum_serializer, serialize_option_2,
};
use crate::xml::{get_nodes, get_unique_node, get_unique_node_text};

use super::common::from_chapter_map;
use super::common::ChapterMap;

type PassiveKey = String;
type PassiveValue = String;

pub fn reserialize_passives(parser_props: &ParserProps) -> String {
    let passives: HashMap<_, _> = parser_props
        .document_strings
        .iter()
        .flat_map(|document_string| {
            process_passive_file(
                document_string.as_str(),
                parser_props.collectability_map,
                parser_props.chapter_map,
            )
        })
        .collect();

    let mut builder = phf_codegen::Map::new();
    for (id, passive_entry) in passives {
        builder.entry(id.clone(), &passive_entry);
    }
    format!(
        "static PASSIVES: phf::Map<&'static str, Passive> = {};",
        builder.build()
    )
}

fn process_passive_file(
    document_string: &str,
    collectability_map: &CollectabilityMap,
    chapter_map: &ChapterMap,
) -> HashMap<PassiveKey, PassiveValue> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "PassiveXmlRoot").unwrap();
    let passive_node_list = get_nodes(xml_root_node, "Passive");

    // Manually adding these missing passives
    let missing_passives = vec![
        "505321", "605214", "805513", "505501", "305551", "705211", "605412", "805315", "9055121",
        "605432", "705212", "605222", "905522", "505561", "505214", "305512", "9055113", "305511",
        "9055122", "805213", "605411", "705112", "605115", "505212", "405611", "704024", "405621",
        "705521", "505323", "505213", "605221", "605435", "705313", "805221", "805223", "605414",
        "705121", "605413", "705124", "505311", "505531", "805512", "305521", "305541", "605120",
        "505314", "705111", "704027", "605521", "704025", "105512", "805212", "805211", "705519",
        "705514", "705321", "205421", "805520", "1005313", "805516", "705515", "805514", "605122",
        "105312", "505215", "405521", "170335", "605311", "405613", "305513", "505504", "704032",
        "505502", "805521", "105313", "705517", "505211", "605433", "605111", "705413", "405612",
        "221003", "805515", "405511", "605421", "605301", "605212", "605434", "9055131", "705312",
        "705322", "805312", "605211", "805321", "605112", "705513", "605322", "505521", "705516",
        "705511", "221002", "605532", "905523", "605511", "505631", "705421", "605331", "1309022",
        "605223", "705531", "505222", "105314", "705422", "705215", "105514", "605512", "705412",
        "805224", "505505", "705411", "505103", "605114", "605224", "405512", "605302", "105112",
        "905502", "705214", "605332", "9055133", "705213", "9055112", "605531", "605213", "705518",
        "250522", "105513", "305510", "605522", "1005315", "1005314", "605312", "605121", "705122",
        "705510", "805511", "805518", "305531", "705311", "805311", "805519", "505541", "805313",
        "9055132", "505503", "505611", "305571", "170200", "505621", "505322", "705216", "505312",
        "505223", "805316", "705123", "605341", "9055111", "505324", "505500", "605113", "605215",
        "705423", "704026", "505221", "505313", "704031", "805222", "1005316", "405614", "805314",
        "505641", "1309023", "221004", "505511", "605431", "805517", "605321", "505331", "705512",
        "505600", "300001", "305561", "704041",
    ];

    let passives_in_xml = passive_node_list
        .iter()
        .map(|x| parse_passive(*x, collectability_map, chapter_map));

    let missing_passives = missing_passives
        .iter()
        .map(|x| default_passive_settings(x, collectability_map, chapter_map));

    passives_in_xml.chain(missing_passives).collect()
}

fn parse_passive(
    passive_node: Node,
    collectability_map: &CollectabilityMap,
    chapter_map: &ChapterMap,
) -> (PassiveKey, PassiveValue) {
    let id = passive_node.attribute("ID").unwrap();
    let cost = serialize_option_2(
        get_unique_node_text(passive_node, "Cost"),
        display_serializer,
    );
    let rarity = serialize_option_2(
        get_unique_node_text(passive_node, "Rarity").map(get_rarity_from_str),
        rarity_enum_serializer,
    );
    let hidden = serialize_option_2(
        get_unique_node_text(passive_node, "IsHide").map(|x| x == "true"),
        display_serializer,
    );
    let transferable = serialize_option_2(
        get_unique_node_text(passive_node, "CanGivePassive").map(|x| x == "true"),
        display_serializer,
    );
    let inner_type = serialize_option_2(
        get_unique_node_text(passive_node, "InnerType"),
        display_serializer,
    );

    let chapter = serialize_option_2(
        from_chapter_map(&id, &PageType::Passive, chapter_map),
        chapter_enum_serializer,
    );

    let is_collectable = collectability_map
        .collectable
        .passives
        .iter()
        .any(|x| x == id);

    let collectability = if is_collectable {
        Collectability::Collectable
    } else {
        Collectability::EnemyOnly
    };

    (
        id.to_string(),
        format!(
            "Passive {{
        id: \"{id}\",
        cost: {cost},
        rarity: {rarity},
        hidden: {hidden},
        transferable: {transferable},
        inner_type: {inner_type},
        collectability: Collectability::{collectability:?},
        chapter: {chapter},
    }}"
        ),
    )
}

fn default_passive_settings(
    id: &str,
    collectability_map: &CollectabilityMap,
    chapter_map: &ChapterMap,
) -> (PassiveKey, PassiveValue) {
    let chapter = serialize_option_2(
        from_chapter_map(&id, &PageType::Passive, chapter_map),
        chapter_enum_serializer,
    );

    let is_collectable = collectability_map
        .collectable
        .passives
        .iter()
        .any(|x| x == id);

    let collectability = if is_collectable {
        Collectability::Collectable
    } else {
        Collectability::EnemyOnly
    };

    (
        id.to_string(),
        format!(
            "Passive {{
        id: \"{id}\",
        cost: None,
        rarity: None,
        hidden: None,
        transferable: None,
        inner_type: None,
        collectability: Collectability::{collectability:?},
        chapter: {chapter},
    }}"
        ),
    )
}
