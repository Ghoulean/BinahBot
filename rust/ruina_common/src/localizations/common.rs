use std::str::FromStr;

use unic_langid::LanguageIdentifier;

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

impl From<&LanguageIdentifier> for Locale {
    fn from(value: &LanguageIdentifier) -> Self {
        match value.to_string().as_str() {
            "zh-CN" => Locale::Chinese,
            "zh-TW" => Locale::TraditionalChinese,
            _ => Locale::from_str(value.language.as_str()).unwrap_or(Locale::English)
        }
    }
}

impl From<LanguageIdentifier> for Locale {
    fn from(value: LanguageIdentifier) -> Self {
        Locale::from(&value)
    }
}