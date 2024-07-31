use super::common::DamageRange;
use super::common::DamageType;
use super::common::Defenses;
use super::common::RiskLevel;
use super::common::StatBonus;
use super::equipment::Gift;
use super::equipment::Suit;
use super::equipment::Weapon;

#[derive(Debug)]
pub struct EncyclopediaInfo<'a> {
    pub id: u32,
    pub risk: RiskLevel,
    pub work_probabilities: WorkProbabilities,
    pub qliphoth_counter: i32,
    pub work_damage_type: DamageType,
    pub work_damage_range: DamageRange,
    pub work_happiness_ranges: [i32; 3], // <= for :(, <= for :|, <= for :)
    pub work_speed: f64,
    pub work_cooldown: i32,
    pub is_breachable: bool,
    pub defenses: Option<Defenses>, // can be None even if is_breachable == true
    pub observation_level_bonuses: [StatBonus; 4],
    pub weapon: Option<Weapon<'a>>,
    pub suit: Option<Suit<'a>>,
    pub gifts: Vec<Gift<'a>>,
    pub breaching_entities: Vec<BreachingEntity<'a>>, // may include self, if is_breachable == true
    pub image: &'a str,
}

#[derive(Debug)]
pub struct BreachingEntity<'a> {
    pub id: &'a str,
    pub hp: i32,
    pub speed: i32,
    pub defenses: Defenses,
    pub risk_level: RiskLevel
}

#[derive(Debug)]
pub struct WorkProbabilities {
    pub instinct: [f64; 5],
    pub insight: [f64; 5],
    pub attachment: [f64; 5],
    pub repression: [f64; 5],
}
