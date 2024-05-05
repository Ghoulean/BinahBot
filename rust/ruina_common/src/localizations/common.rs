use strum_macros::{Display, EnumIter};

#[derive(Clone, Debug, Display, EnumIter, Eq, Hash, PartialEq)]
pub enum Locale {
    Korean,
    English,
    Japanese,
    Chinese,
    TraditionalChinese,
}
