use std::str::FromStr;

use unic_langid::langid;
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
    #[strum(serialize = "cn_tr")]
    ChineseTraditional,
    #[strum(serialize = "ru")]
    Russian,
    #[strum(serialize = "bg")]
    Bulgarian,
    #[strum(serialize = "es")]
    Spanish,
    #[strum(serialize = "fr")]
    French,
    #[strum(serialize = "pt_br")]
    PortugueseBrazil,
    #[strum(serialize = "pt_pt")]
    PortuguesePortugal,
}

impl From<&LanguageIdentifier> for Locale {
    fn from(value: &LanguageIdentifier) -> Self {
        match value.to_string().as_str() {
            "es-US" => Locale::Spanish,
            "fr-FR" => Locale::French,
            "ko" => Locale::Korean,
            "ja" => Locale::Japanese,
            "pt-BR" => Locale::PortugueseBrazil,
            "pt-PT" => Locale::PortuguesePortugal,
            "ru-RU" => Locale::Russian,
            "zh-CN" => Locale::Chinese,
            "zh-TW" => Locale::ChineseTraditional,
            _ => Locale::from_str(value.language.as_str()).unwrap_or(Locale::English)
        }
    }
}

impl From<LanguageIdentifier> for Locale {
    fn from(value: LanguageIdentifier) -> Self {
        Locale::from(&value)
    }
}

impl From<&Locale> for LanguageIdentifier {
    fn from(value: &Locale) -> Self {
        match value {
            Locale::English => langid!("en-US"),
            Locale::Korean => langid!("ko"),
            Locale::Japanese => langid!("ja"),
            Locale::Chinese => langid!("zh-CN"),
            Locale::ChineseTraditional => langid!("zh-TW"),
            Locale::Russian => langid!("ru-RU"),
            Locale::Bulgarian => langid!("bg"),
            Locale::Spanish => langid!("es-US"),
            Locale::French => langid!("fr-FR"),
            Locale::PortugueseBrazil => langid!("pt-BR"),
            Locale::PortuguesePortugal => langid!("pt-PT"),
        }
    }
}

impl From<Locale> for LanguageIdentifier {
    fn from(value: Locale) -> Self {
        LanguageIdentifier::from(&value)
    }
}