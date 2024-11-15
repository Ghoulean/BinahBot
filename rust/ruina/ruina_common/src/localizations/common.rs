use std::str::FromStr;

use unic_langid::langid;
use unic_langid::LanguageIdentifier;

use super::abno_page_locale::AbnoPageLocale;
use super::battle_symbol_locale::BattleSymbolLocale;
use super::card_effect_locale::CardEffectLocale;
use super::combat_page_locale::CombatPageLocale;
use super::key_page_locale::KeyPageLocale;
use super::passive_locale::PassiveLocale;

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    strum_macros::Display,
    strum_macros::EnumIter,
    strum_macros::EnumString,
)]
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
        match value.language.as_str() {
            "zh" => match value.region {
                Some(x) => match x.as_str() {
                    "TW" => Locale::TraditionalChinese,
                    _ => Locale::Chinese,
                },
                None => Locale::Chinese,
            },
            "ko" => Locale::Korean,
            "ja" => Locale::Japanese,
            _ => Locale::from_str(value.language.as_str()).unwrap_or(Locale::English),
        }
    }
}

impl From<LanguageIdentifier> for Locale {
    fn from(value: LanguageIdentifier) -> Self {
        Locale::from(&value)
    }
}

// Avoid encoding region if not necessary (for our use case)
impl From<&Locale> for LanguageIdentifier {
    fn from(value: &Locale) -> Self {
        match value {
            Locale::English => langid!("en-US"),
            Locale::Korean => langid!("ko"),
            Locale::Japanese => langid!("ja"),
            Locale::Chinese => langid!("zh-CN"),
            Locale::TraditionalChinese => langid!("zh-TW"),
        }
    }
}

impl From<Locale> for LanguageIdentifier {
    fn from(value: Locale) -> Self {
        LanguageIdentifier::from(&value)
    }
}

#[derive(Debug)]
pub enum PageLocale<'a> {
    Abno(&'a AbnoPageLocale<'a>),
    BattleSymbol(&'a BattleSymbolLocale<'a>),
    CardEffect(&'a CardEffectLocale<'a>),
    CombatPage(&'a CombatPageLocale<'a>),
    KeyPage(&'a KeyPageLocale<'a>),
    Passive(&'a PassiveLocale<'a>),
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_language_conversion() {
        assert_eq!(Locale::English, langid!("en").into());
        assert_eq!(Locale::English, langid!("en-US").into());
        assert_eq!(Locale::English, langid!("en-UK").into());
        assert_eq!(Locale::Korean, langid!("ko").into());
        assert_eq!(Locale::Korean, langid!("ko-KR").into());
        assert_eq!(Locale::Japanese, langid!("ja").into());
        assert_eq!(Locale::Japanese, langid!("ja-JP").into());
        assert_eq!(Locale::Chinese, langid!("zh").into());
        assert_eq!(Locale::Chinese, langid!("zh-CN").into());
        assert_eq!(Locale::TraditionalChinese, langid!("zh-TW").into());
    }
}