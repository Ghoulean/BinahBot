use crate::game_objects::common::Chapter;
use crate::game_objects::common::Rarity;

#[derive(Debug, PartialEq)]
pub enum KeyPageRange {
    Melee,
    Ranged,
    Hybrid,
}

#[derive(Debug, PartialEq)]
pub enum Resistance {
    Fatal,
    Weak,
    Normal,
    Endured,
    Ineffective,
    Immune,
}

pub struct KeyPageResists {
    pub hp_slash: Resistance,
    pub hp_pierce: Resistance,
    pub hp_blunt: Resistance,
    pub stagger_slash: Resistance,
    pub stagger_pierce: Resistance,
    pub stagger_blunt: Resistance,
}

pub struct KeyPage<'a> {
    pub id: &'a str,
    pub skin: Option<&'a str>,
    pub book_icon: Option<&'a str>, // todo: enum-ify
    pub hp: u16,
    pub stagger: u16,
    pub min_speed: u8,
    pub max_speed: u8,
    pub resists: KeyPageResists,
    pub base_speed_die: u8,
    pub starting_light: i8,
    pub base_light: u8,
    pub range: KeyPageRange,
    pub rarity: Rarity,
    pub episode_id: Option<&'a str>,
    pub passive_ids: &'a [&'a str],
    pub options: &'a [&'a str],
    pub chapter: Option<Chapter>,
    pub category: Option<&'a str>,
    pub only_card_ids: &'a [&'a str],
}
