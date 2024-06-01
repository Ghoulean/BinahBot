use std::collections::HashMap;

use roxmltree::{Document, Node};

use crate::serde::{display_serializer, get_rarity_from_str, rarity_enum_serializer, serialize_option_2};
use crate::xml::{get_nodes, get_unique_node, get_unique_node_text};

type PassiveKey = String;
type PassiveValue = String;

pub fn reserialize_passives(document_strings: &[String]) -> String {
    let passives: HashMap<_, _> = document_strings
        .iter()
        .flat_map(|document_string| process_passive_file(document_string.as_str()))
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

fn process_passive_file(document_string: &str) -> HashMap<PassiveKey, PassiveValue> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "PassiveXmlRoot").unwrap();
    let passive_node_list = get_nodes(xml_root_node, "Passive");

    passive_node_list
        .into_iter()
        .map(|x| parse_passive(x))
        .collect()
}

fn parse_passive(passive_node: Node) -> (PassiveKey, PassiveValue) {
    let id = passive_node.attribute("ID").unwrap();
    let cost = serialize_option_2(get_unique_node_text(passive_node, "Cost"), display_serializer);
    let rarity = serialize_option_2(
        get_unique_node_text(passive_node, "Rarity").map(get_rarity_from_str),
        rarity_enum_serializer
    );
    let hidden =
        serialize_option_2(
            get_unique_node_text(passive_node, "IsHide").map(|x| x == "true"), display_serializer
        );
    let transferable =
        serialize_option_2(get_unique_node_text(passive_node, "CanGivePassive").map(|x| x == "true"), display_serializer);
    let inner_type = serialize_option_2(get_unique_node_text(passive_node, "InnerType"), display_serializer);

    (
        id.to_string(),
        format!(
            "Passive {{
        id: \"{id}\",
        cost: {cost},
        rarity: {rarity},
        hidden: {hidden},
        transferable: {transferable},
        inner_type: {inner_type}
    }}"
        ),
    )
}
