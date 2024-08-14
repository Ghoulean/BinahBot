use std::fmt;
use std::str::FromStr;

use super::abno_page::AbnoPage;
use super::battle_symbol::BattleSymbol;
use super::combat_page::CombatPage;
use super::key_page::KeyPage;
use super::passive::Passive;

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum Rarity {
    Paperback,
    Hardcover,
    Limited,
    ObjetDArt,
}

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum Floor {
    Malkuth,
    Yesod,
    Netzach,
    Hod,
    Tiphereth,
    Gebura,
    Chesed,
    Binah,
    Hokma,
    Keter,
    None,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, strum_macros::Display)]
pub enum Chapter {
    Canard,
    UrbanMyth,
    UrbanLegend,
    UrbanPlague,
    UrbanNightmare,
    StarOfTheCity,
    ImpuritasCivitatis,
}

#[derive(Clone, Debug, Eq, PartialEq, strum_macros::Display)]
pub enum Collectability {
    Collectable,
    Obtainable,
    EnemyOnly
}

#[derive(Clone, Eq, Hash, PartialEq, Debug, strum_macros::EnumIter)]
pub enum PageType {
    AbnoPage,
    BattleSymbol,
    CombatPage,
    KeyPage,
    Passive,
}

#[derive(Debug)]
pub enum Page<'a> {
    Abno(&'a AbnoPage<'a>),
    BattleSymbol(&'a BattleSymbol<'a>),
    CombatPage(&'a CombatPage<'a>),
    KeyPage(&'a KeyPage<'a>),
    Passive(&'a Passive<'a>)
}

impl fmt::Display for PageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PageType::AbnoPage => write!(f, "a#"),
            PageType::BattleSymbol => write!(f, "b#"),
            PageType::CombatPage => write!(f, "c#"),
            PageType::KeyPage => write!(f, "k#"),
            PageType::Passive => write!(f, "p#"),
        }
    }
}

impl FromStr for PageType {
    type Err = Box<dyn std::error::Error>;

    fn from_str(pagetype_str: &str) -> Result<Self, Self::Err> {
        match pagetype_str {
            "a#" => Ok(PageType::AbnoPage),
            "b#" => Ok(PageType::BattleSymbol),
            "c#" => Ok(PageType::CombatPage),
            "k#" => Ok(PageType::KeyPage),
            "p#" => Ok(PageType::Passive),
            _ => Err("unrecognized PageType")?,
        }
    }
}
