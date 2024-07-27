use super::common::DamageType;
use super::common::Defenses;
use super::common::RiskLevel;
use super::common::StatBonus;

#[derive(Debug)]
pub struct Weapon<'a> {
    pub id: &'a str,
    pub name_id: &'a str,
    pub desc_id: &'a str,
    pub risk: RiskLevel,
    pub range: WeaponRange,
    pub attack_speed: WeaponAttackSpeed,
    pub damage_range: [i32; 2],
    pub damage_type: DamageType,
    pub max_collectable_amount: i32,
    pub cost: i32,
    pub special_info_id: &'a str,
    pub special_ability_id: &'a str,
    pub equip_requirements: &'a [EquipRequirement],
    pub observation_level: Option<i32>,
    pub image: &'a str,
}

#[derive(Debug)]
pub struct Suit<'a> {
    pub id: &'a str,
    pub name_id: &'a str,
    pub desc_id: &'a str,
    pub defenses: Defenses,
    pub max_collectable_amount: i32,
    pub cost: i32,
    pub special_info_id: &'a str,
    pub special_ability_id: &'a str,
    pub equip_requirements: &'a [EquipRequirement],
    pub observation_level: Option<i32>,
    pub image: &'a str,
}

#[derive(Debug)]
pub struct Gift<'a> {
    pub id: &'a str,
    pub name_id: &'a str,
    pub desc_id: &'a str,
    pub slot: Slot,
    pub stat_bonuses: &'a [StatBonus],
    pub obtain_probability: Option<f64>,
    pub observation_level: Option<i32>
}

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum WeaponRange {
    VeryLong,
    Long,
    Medium,
    Short,
    VeryShort,
}

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum WeaponAttackSpeed {
    VeryFast,
    Fast,
    Normal,
    Slow,
    VerySlow,
}

#[derive(Debug)]
pub struct EquipRequirement(EquipRequirementKey, i32);

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum EquipRequirementKey {
    AgentLevel, Fortitude, Prudence, Temperance, Justice, AllStats
}

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum Slot {
    Brooch, Cheek, Eye, Face, Hand1, Hand2, Hat, Helmet, LeftBack, Mouth1, Mouth2, Neckwear, RightBack, Special
}
