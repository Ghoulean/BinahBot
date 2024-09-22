use super::common::DamageRange;
use super::common::DamageType;
use super::common::Defenses;
use super::common::RiskLevel;
use super::common::StatBonus;
use super::equipment::Gift;
use super::equipment::Suit;
use super::equipment::Weapon;

#[derive(Debug)]
pub enum EncyclopediaInfo<'a> {
    Normal(NormalInfo<'a>),
    Tool(ToolInfo<'a>),
    DontTouchMe(DontTouchMeInfo<'a>),
}

#[derive(Debug)]
pub struct NormalInfo<'a> {
    pub id: u32,
    pub risk: RiskLevel,
    pub work_probabilities: WorkProbabilities,
    pub qliphoth_counter: Option<i32>,
    pub work_damage_type: DamageType,
    pub work_damage_range: DamageRange,
    pub work_happiness_ranges: [i32; 3], // <= for :(, <= for :|, <= for :)
    pub work_speed: f64,
    pub work_cooldown: i32,
    pub max_probability_reduction_count: i32,
    pub is_breachable: bool,
    pub defenses: Option<Defenses>, // can be None even if is_breachable == true
    pub observation_level_bonuses: [StatBonus; 4],
    pub weapon: Option<Weapon<'a>>,
    pub suit: Option<Suit<'a>>,
    pub gifts: &'a [Gift<'a>],
    pub breaching_entities: &'a [BreachingEntity<'a>], // includes self if is_breachable == true
    pub image: &'a str,
}

#[derive(Debug)]
pub struct ToolInfo<'a> {
    pub id: u32,
    pub risk: RiskLevel,
    pub tool_type: ToolType,
    pub breaching_entities: &'a [BreachingEntity<'a>],
    pub image: &'a str,
}

#[derive(Debug)]
pub struct DontTouchMeInfo<'a> {
    pub id: u32,
    pub risk: RiskLevel,
    pub image: &'a str,
}

#[derive(Debug)]
pub struct BreachingEntity<'a> {
    pub id: &'a str,
    pub hp: i32,
    pub speed: f64,
    pub defenses: Defenses,
    pub damage_type: DamageType,
    pub risk_level: RiskLevel,
}

#[derive(Debug)]
pub struct WorkProbabilities {
    pub instinct: [f64; 5],
    pub insight: [f64; 5],
    pub attachment: [f64; 5],
    pub repression: [f64; 5],
}

#[derive(Debug)]
pub enum ToolType {
    Equippable,
    SustainedUse,
    SingleUse,
}

impl TryFrom<&str> for ToolType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value.to_lowercase().trim() {
            "equip" => ToolType::Equippable,
            "channel" => ToolType::SustainedUse,
            "oneshot" => ToolType::SingleUse,
            _ => return Err("invalid tool type".to_string()),
        })
    }
}
