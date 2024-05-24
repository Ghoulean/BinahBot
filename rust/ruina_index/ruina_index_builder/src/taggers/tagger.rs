use std::fmt;

// TODO:
// Currently need to maintain two different variants of some
// of these types and impls due to String vs &str difference

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Tag(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypedId(pub PageType, pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum PageType {
    AbnoPageId,
    BattleSymbolId,
    CombatPageId,
    KeyPageId,
    PassiveId,
}

pub trait Tagger {
    fn get_typed_id(&self) -> TypedId;
    fn generate_tags(&self) -> Vec<Tag>;
}

impl fmt::Display for PageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PageType::AbnoPageId => write!(f, "a#"),
            PageType::BattleSymbolId => write!(f, "b#"),
            PageType::CombatPageId => write!(f, "c#"),
            PageType::KeyPageId => write!(f, "k#"),
            PageType::PassiveId => write!(f, "p#"),
        }
    }
}

impl fmt::Display for TypedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}{}", self.0, self.1))
    }
}
