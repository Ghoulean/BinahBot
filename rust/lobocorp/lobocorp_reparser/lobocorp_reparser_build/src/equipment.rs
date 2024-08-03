use std::fs;
use std::i32;
use std::path::PathBuf;

use lobocorp_common::game_objects::common::DamageRange;
use lobocorp_common::game_objects::common::DamageType;
use lobocorp_common::game_objects::common::Defenses;
use lobocorp_common::game_objects::common::Resistance;
use lobocorp_common::game_objects::common::RiskLevel;
use lobocorp_common::game_objects::common::Stat;
use lobocorp_common::game_objects::common::StatBonus;
use lobocorp_common::game_objects::equipment::EquipRequirement;
use lobocorp_common::game_objects::equipment::EquipRequirementKey;
use lobocorp_common::game_objects::equipment::Slot;
use lobocorp_common::game_objects::equipment::WeaponAttackSpeed;
use lobocorp_common::game_objects::equipment::WeaponRange;
use roxmltree::Document;
use roxmltree::Node;

use crate::path::BASE_EQUIPMENT_PATH_STR;
use crate::xml::find_unique_node_with_attribute;
use crate::xml::get_first_node;
use crate::xml::get_nodes;
use crate::xml::get_unique_node;
use crate::xml::get_unique_node_text;

const DEBUG_IDS: &[u32] = &[
    // god weapon + armor
    177777, 277777,
    // rabbit team
    4,
    // leftover test equipment
    5, 11, 3, 200, 22,
    // special abno equipment from possessions and such
    101, 100018, 100105, 100046, 100036, 1021, 1022, 100041,
    // ordeals
    1200001, 
];

#[derive(Debug, Clone)]
pub struct PartialWeapon {
    pub id: u32,
    pub name_id: Option<String>,
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

#[derive(Debug, Clone)]
pub struct PartialSuit {
    pub id: u32,
    pub name_id: Option<String>,
    pub desc_id: Option<String>,
    pub special_desc_id: Option<String>,
    pub armor_id: String,
    pub defenses: Defenses,
    pub risk: RiskLevel,
    pub equip_requirements: Vec<EquipRequirement>,
    pub max_collectable_amount: i32,
    pub image: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PartialGift {
    pub id: u32,
    pub name_id: Option<String>,
    pub desc_id: Option<String>,
    pub slot: Slot,
    pub stat_bonuses: Vec<StatBonus>,
    pub image: Option<String>,
}

#[derive(Debug)]
pub struct AllEquipment {
    weapons: Vec<PartialWeapon>,
    suits: Vec<PartialSuit>,
    gifts: Vec<PartialGift>
}

pub fn load_equipment() -> AllEquipment {
    let binding = PathBuf::from(BASE_EQUIPMENT_PATH_STR);
    let abno_list_str = fs::read_to_string(binding.as_path()).expect("cannot read BaseList.xml");

    let doc: Document = Document::parse(&abno_list_str).expect("failed parsing BaseList.xml");

    let equipment_list_node = get_unique_node(&doc.root(), "equipment_list").expect("couldn't find equipment_list");

    let mut weapons = Vec::new();
    let mut suits = Vec::new();
    let mut gifts = Vec::new();

    get_nodes(&equipment_list_node, "equipment").iter().for_each(|x| {
        let id: u32 = x.attribute("id").and_then(|y| y.parse::<u32>().ok()).expect("id parse failure");
        if DEBUG_IDS.contains(&id) {
            return;
        }

        match x.attribute("type").expect("no type on equipment") {
            "weapon" => weapons.push(parse_weapon(x)),
            "armor" => suits.push(parse_suit(x)),
            "special" => gifts.push(parse_gift(x)),
            _ => panic!()
        };
    });

    AllEquipment {
        weapons, suits, gifts
    }
}

fn parse_weapon(node: &Node) -> PartialWeapon {
    let id: u32 = node.attribute("id").expect("no id").parse().expect("id parse failure");
    let name_id = get_unique_node_text(node, "name").map(|x| x.to_string()).ok();
    let desc_id = get_unique_node_text(node, "desc").map(|x| x.to_string()).ok();
    let special_desc_id = get_unique_node_text(node, "specialDesc").map(|x| x.to_string()).ok();
    let risk = RiskLevel::try_from(
        get_unique_node_text(node, "grade").expect("no grade").parse::<i32>().expect("couldn't parse grade")
    ).expect("couldn't get risk level");
    let range = WeaponRange(get_unique_node_text(node, "range").expect("no range").parse::<u32>().expect("couldn't parse weapon range"));
    let attack_speed = get_unique_node_text(node, "attackSpeed").ok()
        .and_then(|x| x.parse::<f64>().ok())
        .map(|x| WeaponAttackSpeed(x))
        .expect("couldn't get attack speed");

    // this is not unique
    let damage_node = get_first_node(node, "damage").expect("no damage node");
    let damage_range = DamageRange(
        damage_node.attribute("min").and_then(|x| x.trim().parse().ok()).expect("couldn't get min damage"),
        damage_node.attribute("max").and_then(|x| x.trim().parse().ok()).expect("couldn't get max damage")
    );
    let damage_type = damage_node.attribute("type").and_then(|x| DamageType::try_from(x).ok()).expect("couldn't get damage type");

    let max_collectable_amount = get_unique_node_text(node, "maxNum").ok()
        .and_then(|x| x.parse::<i32>().ok())
        .unwrap_or(i32::MAX);

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
    let id: u32 = node.attribute("id").expect("no id").parse().expect("id parse failure");
    let name_id = get_unique_node_text(node, "name").map(|x| x.to_string()).ok();
    let desc_id = get_unique_node_text(node, "desc").map(|x| x.to_string()).ok();
    let special_desc_id = get_unique_node_text(node, "specialDesc").map(|x| x.to_string()).ok();
    let armor_id = get_unique_node_text(node, "armorId").map(|x| x.to_string()).expect("couldn't find armorId");

    let defense_node = get_unique_node(node, "defense").expect("no defense node");
    let defenses = Defenses {
        red: get_defense_val(&defense_node, "R"),
        white: get_defense_val(&defense_node, "W"),
        black: get_defense_val(&defense_node, "B"),
        pale: get_defense_val(&defense_node, "P"),
    };

    let risk = get_unique_node_text(node, "grade").ok()
        .and_then(|x| x.parse::<i32>().ok())
        .and_then(|x| RiskLevel::try_from(x).ok())
        .unwrap_or(RiskLevel::Zayin);

    let max_collectable_amount = get_unique_node_text(node, "maxNum").ok()
        .and_then(|x| x.parse::<i32>().ok())
        .unwrap_or(i32::MAX);

    let equip_requirements = get_nodes(node, "require").iter().map(|x| {
        let key = x.attribute("type").and_then(|x| EquipRequirementKey::try_from(x).ok()).expect("no type attribute on requirement");
        let val = x.text().and_then(|x| x.parse::<i32>().ok()).expect("couldn't get requirement val");
        EquipRequirement(key, val)
    }).collect::<Vec<_>>();

    let image = None;

    PartialSuit {
        id, name_id, desc_id, special_desc_id, armor_id, defenses,
        risk, max_collectable_amount, equip_requirements, image,
    }
}

pub fn get_defense_val(defense_node: &Node, str_code: &str) -> Resistance {
    Resistance(
        find_unique_node_with_attribute(&defense_node, "type", str_code).ok()
            .and_then(|x| x.text())
            .and_then(|x| x.parse::<f64>().ok())
            .expect("couldn't parse defense val")
    )
}

fn parse_gift(node: &Node) -> PartialGift {
    let id: u32 = node.attribute("id").expect("no id").parse().expect("id parse failure");
    let name_id = get_unique_node_text(node, "name").map(|x| x.to_string()).ok();
    let desc_id = get_unique_node_text(node, "desc").map(|x| x.to_string()).ok();

    let slot = get_gift_slot(
        get_unique_node_text(node, "attachPos").expect("couldn't find attachPos"),
        get_unique_node_text(node, "attachType").expect("couldn't find attachType")
    ).expect("couldn't get gift slot");

    let binding: Vec<StatBonus> = Vec::new();
    let bonus_node = get_unique_node(node, "bonus");
    let stat_bonuses = bonus_node.map(|x| {
        x.descendants().filter(|n| n.is_element() && n.tag_name().name() != "bonus").map(|x| {
            let key = Stat::try_from(x.tag_name().name()).expect("couldn't recognize tag name");
            let val = x.text().and_then(|x| x.parse::<i32>().ok()).expect("couldn't get val");
            StatBonus(key, val)
        }).collect::<Vec<_>>()
    }).unwrap_or(binding);

    let image = get_unique_node_text(node, "sprite").map(|x| x.to_string()).ok();

    PartialGift {
        id, name_id, desc_id, slot, stat_bonuses, image
    }
}

fn get_gift_slot(attach_pos: &str, attach_type: &str) -> Result<Slot, String>{
    Ok(match (attach_pos.to_lowercase().as_str(), attach_type.to_lowercase().as_str()) {
        ("body_up", _) => Slot::Brooch,
        ("right_cheek", _) => Slot::Cheek,
        ("eye", _) => Slot::Eye,
        ("face", _) => Slot::Face,
        ("right_hand", "add") => Slot::Hand1,
        ("right_hand", _) => Slot::Hand2,
        ("head", _) => Slot::Hat,
        ("hair", _) => Slot::Helmet,
        ("back2", _) => Slot::LeftBack,
        ("mouth", "add") => Slot::Mouth1,
        ("mouth", _) => Slot::Mouth2,
        ("ribborn", _) => Slot::Neckwear,
        ("back", _) => Slot::RightBack,
        ("headback", _) => Slot::Special,
        _ => return Err("invalid slot type".to_string())
    })
}