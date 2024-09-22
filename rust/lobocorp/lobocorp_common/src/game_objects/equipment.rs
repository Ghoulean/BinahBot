use super::common::DamageRange;
use super::common::DamageType;
use super::common::Defenses;
use super::common::RiskLevel;
use super::common::StatBonus;

#[derive(Debug)]
pub struct Weapon<'a> {
    pub id: u32,
    pub name_id: &'a str,
    pub desc_id: Option<&'a str>,
    pub special_desc_id: Option<&'a str>,
    pub risk: RiskLevel,
    pub range: WeaponRange,
    pub attack_speed: WeaponAttackSpeed,
    pub damage_range: DamageRange,
    pub damage_type: WeaponDamageType,
    pub max_collectable_amount: i32,
    pub cost: Option<i32>,
    pub equip_requirements: &'a [EquipRequirement],
    pub observation_level: Option<i32>,
    pub image: &'a str,
}

#[derive(Debug)]
pub struct Suit<'a> {
    pub id: u32,
    pub name_id: &'a str,
    pub desc_id: Option<&'a str>,
    pub special_desc_id: Option<&'a str>,
    pub risk: RiskLevel,
    pub defenses: Defenses,
    pub max_collectable_amount: i32,
    pub cost: i32,
    pub equip_requirements: &'a [EquipRequirement],
    pub observation_level: i32,
    pub image: &'a str,
}

#[derive(Debug)]
pub struct Gift<'a> {
    pub id: u32,
    pub name_id: &'a str,
    pub desc_id: &'a str,
    pub slot: Slot,
    pub stat_bonuses: &'a [StatBonus],
    pub obtain_probability: Option<f64>,
    pub observation_level: Option<i32>,
    pub image: &'a str,
}

#[derive(Debug, Clone)]
pub struct WeaponRange(pub u32);

#[derive(Debug, Clone, PartialEq, strum_macros::Display)]
pub enum WeaponRangeCategories {
    VeryLong,
    Long,
    Medium,
    Short,
    VeryShort,
}

#[derive(Debug, Clone)]
pub struct EquipRequirement(pub EquipRequirementKey, pub i32);

#[derive(Debug, Clone, PartialEq, strum_macros::Display)]
pub enum EquipRequirementKey {
    AgentLevel,
    Fortitude,
    Prudence,
    Temperance,
    Justice,
}

#[derive(Debug, Clone, PartialEq, strum_macros::Display)]
pub enum Slot {
    Brooch,
    Cheek,
    Eye,
    Face,
    Hand1,
    Hand2,
    Hat,
    Helmet,
    LeftBack,
    Mouth1,
    Mouth2,
    Neckwear,
    RightBack,
    Special,
}

#[derive(Debug, Clone)]
pub struct WeaponAttackSpeed(pub f64);

#[derive(Debug, Clone)]
pub enum WeaponDamageType {
    Of(DamageType),
    All,
}

#[derive(Debug, Clone, PartialEq, strum_macros::Display)]
pub enum WeaponAttackSpeedCategories {
    VeryFast,
    Fast,
    Normal,
    Slow,
    VerySlow,
}

impl From<u32> for WeaponRangeCategories {
    fn from(value: u32) -> Self {
        if value <= 2 {
            WeaponRangeCategories::VeryShort
        } else if value == 3 {
            WeaponRangeCategories::Short
        } else if value == 4 {
            WeaponRangeCategories::Medium
        } else if value <= 15 {
            WeaponRangeCategories::Long
        } else {
            WeaponRangeCategories::VeryLong
        }
    }
}

impl From<&WeaponRange> for WeaponRangeCategories {
    fn from(value: &WeaponRange) -> Self {
        WeaponRangeCategories::from(value.0)
    }
}

impl From<WeaponRange> for WeaponRangeCategories {
    fn from(value: WeaponRange) -> Self {
        WeaponRangeCategories::from(&value)
    }
}

impl From<f64> for WeaponAttackSpeedCategories {
    fn from(value: f64) -> Self {
        if value <= 1.0 {
            WeaponAttackSpeedCategories::VeryFast
        } else if value < 2.0 {
            WeaponAttackSpeedCategories::Fast
        } else if value < 3.0 {
            WeaponAttackSpeedCategories::Normal
        } else if value < 5.0 {
            WeaponAttackSpeedCategories::Slow
        } else {
            WeaponAttackSpeedCategories::VerySlow
        }
    }
}

impl From<&WeaponAttackSpeed> for WeaponAttackSpeedCategories {
    fn from(value: &WeaponAttackSpeed) -> Self {
        WeaponAttackSpeedCategories::from(value.0)
    }
}

impl From<WeaponAttackSpeed> for WeaponAttackSpeedCategories {
    fn from(value: WeaponAttackSpeed) -> Self {
        WeaponAttackSpeedCategories::from(&value)
    }
}

impl TryFrom<&str> for EquipRequirementKey {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "b" => Ok(EquipRequirementKey::Temperance),
            "r" => Ok(EquipRequirementKey::Fortitude),
            "w" => Ok(EquipRequirementKey::Prudence),
            "p" => Ok(EquipRequirementKey::Justice),
            "level" => Ok(EquipRequirementKey::AgentLevel),
            _ => Err("invalid damage type".to_string()),
        }
    }
}
