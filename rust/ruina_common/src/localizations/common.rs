#[derive(Clone, Debug, Eq, Hash, PartialEq, strum_macros::Display, strum_macros::EnumIter, strum_macros::EnumString)]
pub enum Locale {
    #[strum(serialize = "kr")]
    Korean,
    #[strum(serialize = "en")]
    English,
    #[strum(serialize = "jp")]
    Japanese,
    #[strum(serialize = "cn")]
    Chinese,
    #[strum(serialize = "trcn")]
    TraditionalChinese,
}
