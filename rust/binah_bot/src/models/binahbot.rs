use serde::Deserialize;
use serde::Serialize;

use ruina_common::localizations::common::Locale;
use ruina_common::game_objects::common::Rarity;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordSecrets {
    pub application_id: String,
    pub auth_token: String,
    pub public_key: String
}

pub struct BinahBotEnvironment {
    pub discord_secrets: DiscordSecrets,
    pub discord_client_id: String,
    pub s3_bucket_name: String,
}

#[derive(Clone, Debug, strum::Display, strum_macros::EnumString)]
pub enum BinahBotLocale {
    #[strum(serialize = "en-US")]
    EnglishUS,
    #[strum(serialize = "ko")]
    Korean,
    #[strum(serialize = "ja")]
    Japanese,
    #[strum(serialize = "zh-CN")]
    ChineseChina,
    #[strum(serialize = "zh-TW")]
    ChineseTaiwan,
    Other
}

#[derive(Clone, Debug)]
#[repr(i32)]
pub enum DiscordEmbedColors {
    Default = 0x000000,
    AwakeningAbnoPage = 0x40ce78,
    BreakdownAbnoPage = 0xd14141,
    PaperbackRarity = 0x7bd671,
    HardcoverRarity = 0x305fba,
    LimitedRarity = 0x6b26bf,
    ObjetDArtRarity = 0xebbe00,
}

impl From<Locale> for BinahBotLocale {
    fn from(value: Locale) -> Self {
        match value {
            Locale::English => BinahBotLocale::EnglishUS,
            Locale::Korean => BinahBotLocale::Korean,
            Locale::Japanese => BinahBotLocale::Japanese,
            Locale::Chinese => BinahBotLocale::ChineseChina,
            Locale::TraditionalChinese => BinahBotLocale::ChineseTaiwan,
        }
    }
}

impl From<BinahBotLocale> for Locale {
    fn from(value: BinahBotLocale) -> Self {
        match value {
            BinahBotLocale::EnglishUS => Locale::English,
            BinahBotLocale::Korean => Locale::Korean,
            BinahBotLocale::Japanese => Locale::Japanese,
            BinahBotLocale::ChineseChina => Locale::Chinese,
            BinahBotLocale::ChineseTaiwan => Locale::TraditionalChinese,
            _ => Locale::English
        }
    }
}

impl From<&Rarity> for DiscordEmbedColors {
    fn from(value: &Rarity) -> Self {
        match value {
            Rarity::Paperback => DiscordEmbedColors::PaperbackRarity,
            Rarity::Hardcover => DiscordEmbedColors::HardcoverRarity,
            Rarity::Limited => DiscordEmbedColors::LimitedRarity,
            Rarity::ObjetDArt => DiscordEmbedColors::ObjetDArtRarity
        }
    }
}