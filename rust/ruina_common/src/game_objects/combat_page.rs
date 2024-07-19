use crate::game_objects::common::Chapter;
use crate::game_objects::common::Collectability;
use crate::game_objects::common::Rarity;

#[derive(Debug, Clone, PartialEq, strum_macros::Display)]
pub enum DieType {
    Slash,
    Pierce,
    Blunt,
    Block,
    Evade,
    CSlash,
    CPierce,
    CBlunt,
    CBlock,
    CEvade,
}

#[derive(Clone, Debug)]
pub struct Die<'a> {
    pub min: u16,
    pub max: u16,
    pub die_type: DieType,
    pub script: Option<&'a str>,
    pub actionscript: Option<&'a str>,
    pub motion: &'a str,
    pub effect_res: Option<&'a str>,
}

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum CombatRange {
    Melee,
    Ranged,
    Special,
    OnPlay,
    MassIndividual,
    MassSummation,
}

#[derive(Debug)]
pub struct CombatPage<'a> {
    pub id: &'a str,
    pub artwork: Option<&'a str>,
    pub cost: u8,
    pub range: CombatRange,
    pub rarity: Rarity,
    pub dice: &'a [Die<'a>],
    pub keywords: &'a [&'a str],
    pub options: &'a [&'a str],
    pub script_id: Option<&'a str>,
    pub skin_change: Option<&'a str>,
    pub map_change: Option<&'a str>,
    pub chapter: Option<Chapter>,
    pub priority: Option<i16>,
    pub collectability: Collectability,
}
