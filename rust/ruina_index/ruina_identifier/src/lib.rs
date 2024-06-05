use std::fmt;

pub mod identifiers;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypedId(pub PageType, pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq, strum_macros::EnumString)]
pub enum PageType {
    AbnoPage,
    BattleSymbol,
    CombatPage,
    KeyPage,
    Passive,
}

pub trait Identifier {
    fn get_typed_id(&self) -> TypedId;
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

impl fmt::Display for TypedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}{}", self.0, self.1))
    }
}
