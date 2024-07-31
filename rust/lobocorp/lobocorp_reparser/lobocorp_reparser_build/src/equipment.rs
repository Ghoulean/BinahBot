use std::fs;
use std::path::PathBuf;

use lobocorp_common::game_objects::common::DamageRange;
use lobocorp_common::game_objects::common::DamageType;
use lobocorp_common::game_objects::common::RiskLevel;
use lobocorp_common::game_objects::equipment::EquipRequirement;
use lobocorp_common::game_objects::equipment::WeaponAttackSpeed;
use lobocorp_common::game_objects::equipment::WeaponRange;
use roxmltree::Document;
use roxmltree::Node;

use crate::path::BASE_EQUIPMENT_PATH_STR;
use crate::xml::get_nodes;
use crate::xml::get_unique_node;
use crate::xml::get_unique_node_text;

#[derive(Debug)]
pub struct PartialWeapon<'a> {
    pub id: u32,
    pub name_id: &'a str,
    pub desc_id: Option<&'a str>,
    pub special_desc_id: Option<&'a str>,
    pub risk: RiskLevel,
    pub range: WeaponRange,
    pub attack_speed: WeaponAttackSpeed,
    pub damage_range: DamageRange,
    pub damage_type: DamageType,
    pub max_collectable_amount: i32,
    pub equip_requirements: &'a [EquipRequirement],
    pub image: Option<&'a str>,
}

#[derive(Debug)]
pub struct PartialSuit {
}

#[derive(Debug)]
pub struct PartialGift {
}

#[derive(Debug)]
pub struct AllEquipment<'a> {
    weapon_strs: Vec<PartialWeapon<'a>>,
    suit_strs: Vec<PartialSuit>,
    gift_strs: Vec<PartialGift>
}

pub fn load_equipment<'a>() -> AllEquipment<'a> {
    let binding = PathBuf::from(BASE_EQUIPMENT_PATH_STR);
    let abno_list_str = fs::read_to_string(binding.as_path()).expect("cannot read BaseList.xml");

    let doc: Document = Document::parse(&abno_list_str).expect("failed parsing BaseList.xml");

    let equipment_list_node = get_unique_node(&doc.root(), "equipment_list").expect("couldn't find equipment_list");

    let mut weapon_strs = Vec::new();
    let mut suit_strs = Vec::new();
    let mut gift_strs = Vec::new();

    get_nodes(&equipment_list_node, "equipment").iter().for_each(|x| {
        match x.attribute("type").expect("no type on equipment") {
            "weapon" => weapon_strs.push(parse_weapon(x)),
            "armor" => suit_strs.push(parse_suit(x)),
            "special" => gift_strs.push(parse_gift(x)),
            _ => panic!()
        }
    });

    todo!()
}

fn parse_weapon<'a>(node: &'a Node) -> PartialWeapon<'a> {
    let id: u32 = node.attribute("id").expect("no id").parse().expect("id parse failure");
    let name_id = get_unique_node_text(node, "name").expect("no name id");
    let desc_id = get_unique_node_text(node, "desc").ok();
    let special_desc_id = get_unique_node_text(node, "specialDesc").ok();
    let risk = RiskLevel::try_from(
        get_unique_node_text(node, "grade").expect("no grade").parse::<i32>().expect("couldn't parse grade")
    ).expect("couldn't get risk level");
    let range = WeaponRange(get_unique_node_text(node, "range").expect("no range").parse::<u32>().expect("couldn't parse weapon range"));
    let attack_speed = WeaponAttackSpeed(get_unique_node_text(node, "attackSpeed").expect("no range").parse::<f64>().expect("couldn't parse weapon attack speed"));

    PartialWeapon {
        id, name_id, special_desc_id, risk, range, attack_speed,
        pub damage_range: [i32; 2],
        pub damage_type: DamageType,
        pub max_collectable_amount: i32,
        pub equip_requirements: &'a [EquipRequirement],
        pub image: Option<&'a str>,
    }
}

fn parse_suit(node: &Node) -> PartialSuit {
    todo!()
}

fn parse_gift(node: &Node) -> PartialGift {
    todo!()
}

