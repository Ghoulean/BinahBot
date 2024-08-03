use std::collections::HashMap;
use std::fmt;

use lobocorp_common::game_objects::common::DamageType;
use lobocorp_common::game_objects::common::Defenses;
use lobocorp_common::game_objects::equipment::EquipRequirement;

use crate::abnormalities::ObtainEquipmentNumber;
use crate::abnormalities::PartialBreachingEntity;
use crate::abnormalities::PartialEncyclopediaInfo;
use crate::abnormalities::PartialNormalInfo;
use crate::abnormalities::PartialToolInfo;
use crate::equipment::AllEquipment;
use crate::equipment::PartialGift;
use crate::equipment::PartialSuit;
use crate::equipment::PartialWeapon;
use crate::list::ListEntry;

pub fn write_encyclopedia_info(partial_abnos: &HashMap<ListEntry, PartialEncyclopediaInfo>, partial_equipment: &AllEquipment) -> String {

    let hm: HashMap<u32, String> = partial_abnos.into_iter().map(|(entry, encyclopedia)| {
        (entry.id, match encyclopedia {
            PartialEncyclopediaInfo::Normal(x) => write_normal_info(entry, x, partial_equipment),
            PartialEncyclopediaInfo::Tool(x) => write_tool_info(entry, x),
            PartialEncyclopediaInfo::DontTouchMe => "EncyclopediaInfo::DontTouchMe".to_string(),
        })
    }).collect();

    let mut builder = phf_codegen::Map::new();
    for (key, value) in hm {
        builder.entry(key, &value);
    }
    format!(
        "static ENCYCLOPEDIA: phf::Map<u32, EncyclopediaInfo> = {};",
        builder.build()
    )
}

fn write_normal_info(list_entry: &ListEntry, partial_encyclopedia_info: &PartialNormalInfo, partial_equipment: &AllEquipment) -> String {
    let id = list_entry.id;
    let risk = &partial_encyclopedia_info.risk;
    let work_prob_instinct = partial_encyclopedia_info.work_probabilities.instinct;
    let work_prob_insight = partial_encyclopedia_info.work_probabilities.insight;
    let work_prob_attachment = partial_encyclopedia_info.work_probabilities.attachment;
    let work_prob_repression = partial_encyclopedia_info.work_probabilities.repression;
    let qliphoth_counter = serialize_option(&partial_encyclopedia_info.qliphoth_counter, display_serializer);
    let damage_type = &partial_encyclopedia_info.work_damage_type;
    let damage_range_min = partial_encyclopedia_info.work_damage_range.0;
    let damage_range_max = partial_encyclopedia_info.work_damage_range.1;
    let work_happiness_ranges = partial_encyclopedia_info.work_happiness_ranges;
    let work_speed = partial_encyclopedia_info.work_speed;
    let work_cooldown = partial_encyclopedia_info.work_cooldown;
    let max_probability_reduction_count = partial_encyclopedia_info.max_probability_reduction_count;
    let is_breachable = partial_encyclopedia_info.is_breachable;
    let defenses = serialize_option(&partial_encyclopedia_info.defenses.clone(), defenses_serializer);
    let observation_level_bonuses = write_vec(&partial_encyclopedia_info.observation_level_bonuses.clone().into_iter().map(|x| {
        format!("StatBonus(Stat::{:?}, {})", x.0, x.1)
    }).collect::<Vec<_>>());
    
    let equipment_ids = partial_encyclopedia_info.related_equipment.iter().map(|x| x.id).collect::<Vec<_>>();

    let weapon_entry = partial_equipment.weapons.iter().filter(|x| {
        equipment_ids.contains(&x.id)
    }).collect::<Vec<_>>();
    if weapon_entry.len() > 1 {
        panic!("abno has more than 1 weapon");
    }
    let weapon = weapon_entry.first().map(|x| write_weapon(partial_encyclopedia_info, x)).unwrap_or("None".to_string());
    
    let suit_entry = partial_equipment.suits.iter().filter(|x| {
        equipment_ids.contains(&x.id)
    }).collect::<Vec<_>>();
    if suit_entry.len() > 1 {
        panic!("abno has more than 1 suit");
    }
    let suit = suit_entry.first().map(|x| write_suit(partial_encyclopedia_info, x)).unwrap_or("None".to_string());

    let gifts = write_vec(&partial_equipment.gifts.iter().filter(|x| {
        equipment_ids.contains(&x.id)
    }).map(|x| {
        write_gift(partial_encyclopedia_info, x)
    }).collect::<Vec<_>>());

    let breaching_entities = write_vec(&partial_encyclopedia_info.breaching_entities.iter().map(|x| {
        write_breaching_entity(x)
    }).collect::<Vec<_>>());
    // todo: force-assign image to all abnos
    let image = partial_encyclopedia_info.image.clone().unwrap_or("".to_string());

    format!("EncyclopediaInfo::Normal(NormalInfo {{
        id: {id},
        risk: RiskLevel::{risk:?},
        work_probabilities: WorkProbabilities {{
            instinct: {work_prob_instinct:?},
            insight: {work_prob_insight:?},
            attachment: {work_prob_attachment:?},
            repression: {work_prob_repression:?}
        }},
        qliphoth_counter: {qliphoth_counter},
        work_damage_type: DamageType::{damage_type:?},
        work_damage_range: DamageRange({damage_range_min}, {damage_range_max}),
        work_happiness_ranges: {work_happiness_ranges:?},
        work_speed: {work_speed:?},
        work_cooldown: {work_cooldown},
        max_probability_reduction_count: {max_probability_reduction_count},
        is_breachable: {is_breachable},
        defenses: {defenses},
        observation_level_bonuses: {observation_level_bonuses},
        weapon: {weapon},
        suit: {suit},
        gifts: &{gifts},
        breaching_entities: &{breaching_entities},
        image: \"{image}\",
    }})")
}

fn write_tool_info(list_entry: &ListEntry, partial_encyclopedia_info: &PartialToolInfo) -> String {
    let id = list_entry.id;
    let risk = &partial_encyclopedia_info.risk;
    let tool_type = &partial_encyclopedia_info.tool_type;
    // todo: force-assign an image to all tool abnos
    let binding = &"".to_string();
    let image = &partial_encyclopedia_info.image.as_ref().unwrap_or(binding);

    format!("EncyclopediaInfo::Tool(ToolInfo {{
        id: {id},
        risk: RiskLevel::{risk:?},
        tool_type: ToolType::{tool_type:?},
        image: \"{image}\"
    }})")
}

fn serialize_option<T>(
    option: &Option<T>,
    serializer: fn(&T) -> String
) -> String {
    option.as_ref().map(|x| format!("Some({})", serializer(&x))).unwrap_or("None".to_string())
}

// Numbers and boolean
fn display_serializer<T>(x: &T) -> String 
where T: fmt::Display,
{
    x.to_string()
}

fn str_serializer(x: &String) -> String {
    format!("\"{}\"", x)
}

fn defenses_serializer(x: &Defenses) -> String {
    let red = x.red.0;
    let white = x.white.0;
    let black = x.black.0;
    let pale = x.pale.0;
    format!("Defenses {{
        red: Resistance({red:?}),
        white: Resistance({white:?}),
        black: Resistance({black:?}),
        pale: Resistance({pale:?})
    }}")
}

fn write_weapon(info: &PartialNormalInfo, weapon: &PartialWeapon) -> String {
    let equipment_entry = info.related_equipment.iter()
        .find(|x| x.id == weapon.id)
        .expect("couldn't refind equipment entry");

    let id = weapon.id;
    let name_id = weapon.name_id.as_ref().expect("no name id");
    let desc_id = serialize_option(&weapon.desc_id, str_serializer);
    let special_desc_id = serialize_option(&weapon.special_desc_id, str_serializer);
    let risk = &weapon.risk;
    let range = &weapon.range.0;
    let attack_speed = &weapon.attack_speed.0;
    let damage_range_min = &weapon.damage_range.0;
    let damage_range_max = &weapon.damage_range.1;
    let damage_type = &weapon.damage_type;
    let max_collectable_amount = weapon.max_collectable_amount;
    let cost = match equipment_entry.obtain_number {
        ObtainEquipmentNumber::Cost(x) => x,
        ObtainEquipmentNumber::Probability(_) => unreachable!(),
    };
    let equip_requirements = write_vec(&weapon.equip_requirements.iter().map(|x| {
        format!("EquipRequirement(EquipRequirementKey::{:?}, {})", x.0, x.1)
    }).collect::<Vec<_>>());
    let observation_level = equipment_entry.observation_level;
    // todo: force image
    let image = weapon.image.clone().unwrap_or("".to_string());

    format!("Some(Weapon {{
        id: {id},
        name_id: \"{name_id}\",
        desc_id: {desc_id},
        special_desc_id: {special_desc_id},
        risk: RiskLevel::{risk:?},
        range: WeaponRange({range}),
        attack_speed: WeaponAttackSpeed({attack_speed:?}),
        damage_range: DamageRange({damage_range_min}, {damage_range_max}),
        damage_type: DamageType::{damage_type:?},
        max_collectable_amount: {max_collectable_amount},
        cost: {cost},
        equip_requirements: &{equip_requirements},
        observation_level: {observation_level},
        image: \"{image}\",
    }})")
}

fn write_suit(info: &PartialNormalInfo, suit: &PartialSuit) -> String {
    let equipment_entry = info.related_equipment.iter()
        .find(|x| x.id == suit.id)
        .expect("couldn't refind equipment entry");

    let id = suit.id;
    let name_id = suit.name_id.as_ref().expect("no name id");
    let desc_id = serialize_option(&suit.desc_id, str_serializer);
    let special_desc_id = serialize_option(&suit.special_desc_id, str_serializer);
    let risk = &suit.risk;
    let defenses = defenses_serializer(&suit.defenses);
    let max_collectable_amount = suit.max_collectable_amount;
    let cost = match equipment_entry.obtain_number {
        ObtainEquipmentNumber::Cost(x) => x,
        ObtainEquipmentNumber::Probability(_) => unreachable!(),
    };
    let equip_requirements = write_vec(&suit.equip_requirements.iter().map(|x| {
        format!("EquipRequirement(EquipRequirementKey::{:?}, {})", x.0, x.1)
    }).collect::<Vec<_>>());
    let observation_level = equipment_entry.observation_level;
    // todo: force image
    let image = suit.image.clone().unwrap_or("".to_string());

    format!("Some(Suit {{
        id: {id},
        name_id: \"{name_id}\",
        desc_id: {desc_id},
        special_desc_id: {special_desc_id},
        risk: RiskLevel::{risk:?},
        defenses: {defenses},
        max_collectable_amount: {max_collectable_amount},
        cost: {cost},
        equip_requirements: &{equip_requirements},
        observation_level: {observation_level},
        image: \"{image}\",
    }})")
}

fn write_gift(info: &PartialNormalInfo, gift: &PartialGift) -> String {
    let equipment_entry = info.related_equipment.iter()
        .find(|x| x.id == gift.id)
        .expect("couldn't refind equipment entry");

    let id = gift.id;
    let name_id = gift.name_id.as_ref().expect("no name id");
    let desc_id = gift.desc_id.as_ref().expect("no desc id");
    let slot = &gift.slot;
    let stat_bonuses = write_vec(&gift.stat_bonuses.iter().map(|x| {
        format!("StatBonus(Stat::{:?}, {})", x.0, x.1)
    }).collect::<Vec<_>>());
    let prob = match equipment_entry.obtain_number {
        ObtainEquipmentNumber::Probability(x) => x,
        ObtainEquipmentNumber::Cost(_) => unreachable!(),
    };
    let observation_level = equipment_entry.observation_level;
    // todo: force image
    let image = gift.image.clone().unwrap_or("".to_string());

    format!("Gift {{
        id: {id},
        name_id: \"{name_id}\",
        desc_id: \"{desc_id}\",
        slot: Slot::{slot:?},
        stat_bonuses: &{stat_bonuses},
        obtain_probability: {prob:?},
        observation_level: {observation_level},
        image: \"{image}\",
    }}")
}

fn write_breaching_entity(info: &PartialBreachingEntity) -> String {
    let id = &info.id;
    let hp = info.hp;
    let speed = info.speed;
    let defenses = defenses_serializer(&info.defenses);
    let damage_type = &info.damage_type;
    let risk_level = &info.risk_level;

    format!("BreachingEntity {{
        id: \"{id}\",
        hp: {hp},
        speed: {speed},
        defenses: {defenses},
        damage_type: DamageType::{damage_type:?},
        risk_level: RiskLevel::{risk_level:?}
    }}")
}

fn write_vec(v: &[String]) -> String {
    format!("[{}]", v.join(","))
}