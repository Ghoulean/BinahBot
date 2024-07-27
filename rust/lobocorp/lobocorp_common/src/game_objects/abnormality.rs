use super::common::DamageType;
use super::common::Defenses;
use super::common::RiskLevel;
use super::common::StatBonus;

#[derive(Debug)]
pub struct Abnormality<'a> {
    pub id: &'a str,
    pub name_id: &'a str,
    pub risk: RiskLevel,
    pub work_probabilities: WorkProbabilities,
    pub qliphoth_counter: i32,
    pub work_damage_type: DamageType,
    pub work_damage_range: [i32; 2],
    pub work_happiness_ranges: [i32; 3], // <= for :(, <= for :|, <= for :)
    pub work_speed: f64,
    pub work_cooldown: i32,
    pub is_breaching: bool,
    pub breaching_stats: Option<BreachingStats<'a>>,
    pub observation_level_bonuses: [StatBonus; 4],
    pub weapon_id: Option<&'a str>,
    pub armor_id: Option<&'a str>,
    pub gift_ids: &'a [&'a str],
    pub image: &'a str,
}

#[derive(Debug)]
pub struct WorkProbabilities {
    instinct: [f64; 5],
    insight: [f64; 5],
    attachment: [f64; 5],
    repression: [f64; 5],
}

#[derive(Debug)]
pub struct BreachingStats<'a> {
    pub hp: &'a [i32],
    pub speed: i32,
    pub defenses: &'a Defenses
}