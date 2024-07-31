use std::fs;
use std::path::PathBuf;

use lobocorp_common::game_objects::common::DamageRange;
use lobocorp_common::game_objects::common::DamageType;
use lobocorp_common::game_objects::common::RiskLevel;
use lobocorp_common::game_objects::equipment::EquipRequirement;
use lobocorp_common::game_objects::equipment::EquipRequirementKey;
use lobocorp_common::game_objects::equipment::WeaponAttackSpeed;
use lobocorp_common::game_objects::equipment::WeaponRange;
use roxmltree::Document;
use roxmltree::Node;

use crate::path::BASE_EQUIPMENT_PATH_STR;
use crate::xml::get_first_node;
use crate::xml::get_nodes;
use crate::xml::get_unique_node;
use crate::xml::get_unique_node_text;

#[derive(Debug)]
pub struct PartialWeapon {
    pub id: u32,
    pub name_id: String,
    pub desc_id: Option<String>,
    pub special_desc_id: Option<String>,
    pub risk: RiskLevel,
    pub range: WeaponRange,
    pub attack_speed: WeaponAttackSpeed,
    pub damage_range: DamageRange,
    pub damage_type: DamageType,
    pub max_collectable_amount: i32,
    pub equip_requirements: Vec<EquipRequirement>,
    pub image: Option<String>,
}

#[derive(Debug)]
pub struct PartialSuit {
}

#[derive(Debug)]
pub struct PartialGift {
}

#[derive(Debug)]
pub struct AllEquipment {
    weapon_strs: Vec<PartialWeapon>,
    suit_strs: Vec<PartialSuit>,
    gift_strs: Vec<PartialGift>
}

pub fn load_equipment() -> AllEquipment {
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

fn parse_weapon(node: &Node) -> PartialWeapon {
    let id: u32 = node.attribute("id").expect("no id").parse().expect("id parse failure");
    let name_id = get_unique_node_text(node, "name").expect("no name id").to_string();
    let desc_id = get_unique_node_text(node, "desc").map(|x| x.to_string()).ok();
    let special_desc_id = get_unique_node_text(node, "specialDesc").map(|x| x.to_string()).ok();
    let risk = RiskLevel::try_from(
        get_unique_node_text(node, "grade").expect("no grade").parse::<i32>().expect("couldn't parse grade")
    ).expect("couldn't get risk level");
    let range = WeaponRange(get_unique_node_text(node, "range").expect("no range").parse::<u32>().expect("couldn't parse weapon range"));
    let attack_speed = WeaponAttackSpeed(get_unique_node_text(node, "attackSpeed").expect("no range").parse::<f64>().expect("couldn't parse weapon attack speed"));

    // this is not unique
    let damage_node = get_first_node(node, "damage").expect("no damage node");
    let damage_range = DamageRange(
        damage_node.attribute("min").and_then(|x| x.parse().ok()).expect("couldn't get min damage"),
        damage_node.attribute("max").and_then(|x| x.parse().ok()).expect("couldn't get max damage")
    );
    let damage_type = damage_node.attribute("type").and_then(|x| DamageType::try_from(x).ok()).expect("couldn't get damage type");

    let max_collectable_amount = get_unique_node_text(node, "maxNum").expect("no maxNum").parse::<i32>().expect("couldn't parse maxNum");

    let equip_requirements = get_nodes(node, "require").iter().map(|x| {
        let key = x.attribute("type").and_then(|x| EquipRequirementKey::try_from(x).ok()).expect("no type attribute on requirement");
        let val = x.text().and_then(|x| x.parse::<i32>().ok()).expect("couldn't get requirement val");
        EquipRequirement(key, val)
    }).collect::<Vec<_>>();

    let image = get_unique_node_text(node, "sprite").map(|x| x.to_string()).ok();

    PartialWeapon {
        id, name_id, desc_id, special_desc_id, risk, range, attack_speed,
        damage_range, damage_type, max_collectable_amount, equip_requirements, image,
    }
}

fn parse_suit(node: &Node) -> PartialSuit {
    todo!()
}

fn parse_gift(node: &Node) -> PartialGift {
    todo!()
}

