use fluent_templates::StaticLoader;
use ruina::ruina_common::game_objects::common::Chapter;
use serde::Deserialize;
use serde::Serialize;

use lobocorp::lobocorp_common::localizations::common::Locale as LobocorpLocale;
use ruina::ruina_common::game_objects::combat_page::DieType;
use ruina::ruina_common::game_objects::common::Rarity;
use ruina::ruina_common::localizations::common::Locale as RuinaLocale;
use unic_langid::LanguageIdentifier;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscordSecrets {
    pub application_id: String,
    pub auth_token: String,
    pub public_key: String,
    pub bot_token: String,
}

pub struct Emojis {
    pub slash: Option<String>,
    pub pierce: Option<String>,
    pub blunt: Option<String>,
    pub block: Option<String>,
    pub evade: Option<String>,
    pub c_slash: Option<String>,
    pub c_pierce: Option<String>,
    pub c_blunt: Option<String>,
    pub c_block: Option<String>,
    pub c_evade: Option<String>,
    pub instinct: Option<String>,
    pub insight: Option<String>,
    pub attachment: Option<String>,
    pub repression: Option<String>,
    pub agent: Option<String>,
    pub red_damage: Option<String>,
    pub white_damage: Option<String>,
    pub black_damage: Option<String>,
    pub pale_damage: Option<String>,
    pub good_mood: Option<String>,
    pub normal_mood: Option<String>,
    pub bad_mood: Option<String>,
    pub risk_zayin: Option<String>,
    pub risk_teth: Option<String>,
    pub risk_he: Option<String>,
    pub risk_waw: Option<String>,
    pub risk_aleph: Option<String>,
}

pub struct BinahBotEnvironment {
    pub discord_secrets: DiscordSecrets,
    pub s3_bucket_name: String,
    pub emojis: Emojis,
    pub locales: &'static StaticLoader,
    pub ddb_table_name: String,
    pub ddb_interaction_ttl_table_name: String,
    pub thumbnail_lambda_name: String,
    pub spoiler_config: &'static phf::Map<&'static str, Chapter>,
    pub ddb_client: Option<aws_sdk_dynamodb::Client>,
    pub lambda_client: Option<aws_sdk_lambda::Client>,
    pub reqwest_client: Option<reqwest::Client>,
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
    Other,
}

#[derive(Clone, Debug)]
#[repr(i32)]
pub enum DiscordEmbedColors {
    Default = 0x0d0011,

    AwakeningAbnoPage = 0x40ce78,
    BreakdownAbnoPage = 0xd14141,

    PaperbackRarity = 0x7bd671,
    HardcoverRarity = 0x305fba,
    LimitedRarity = 0x6b26bf,
    ObjetDArtRarity = 0xebbe00,

    Zayin = 0x22f800,
    Teth = 0x1aa0ff,
    He = 0xfef900,
    Waw = 0x7b2ff3,
    Aleph = 0xfe0000,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionTtl {
    pub interaction_id: String,
    pub ttl: u64,
    pub token: String,
    pub original_user_id: String,
}

impl From<&RuinaLocale> for BinahBotLocale {
    fn from(value: &RuinaLocale) -> Self {
        match value {
            RuinaLocale::English => BinahBotLocale::EnglishUS,
            RuinaLocale::Korean => BinahBotLocale::Korean,
            RuinaLocale::Japanese => BinahBotLocale::Japanese,
            RuinaLocale::Chinese => BinahBotLocale::ChineseChina,
            RuinaLocale::TraditionalChinese => BinahBotLocale::ChineseTaiwan,
        }
    }
}

impl From<RuinaLocale> for BinahBotLocale {
    fn from(value: RuinaLocale) -> Self {
        BinahBotLocale::from(&value)
    }
}

impl From<&BinahBotLocale> for RuinaLocale {
    fn from(value: &BinahBotLocale) -> Self {
        match value {
            BinahBotLocale::EnglishUS => RuinaLocale::English,
            BinahBotLocale::Korean => RuinaLocale::Korean,
            BinahBotLocale::Japanese => RuinaLocale::Japanese,
            BinahBotLocale::ChineseChina => RuinaLocale::Chinese,
            BinahBotLocale::ChineseTaiwan => RuinaLocale::TraditionalChinese,
            _ => RuinaLocale::English,
        }
    }
}

impl From<BinahBotLocale> for RuinaLocale {
    fn from(value: BinahBotLocale) -> Self {
        RuinaLocale::from(&value)
    }
}

impl From<BinahBotLocale> for unic_langid::LanguageIdentifier {
    fn from(value: BinahBotLocale) -> Self {
        LanguageIdentifier::from(&value)
    }
}

impl From<&BinahBotLocale> for unic_langid::LanguageIdentifier {
    fn from(value: &BinahBotLocale) -> Self {
        value.to_string().parse().unwrap()
    }
}

impl From<&LobocorpLocale> for BinahBotLocale {
    fn from(value: &LobocorpLocale) -> Self {
        match value {
            LobocorpLocale::English => BinahBotLocale::EnglishUS,
            LobocorpLocale::Korean => BinahBotLocale::Korean,
            LobocorpLocale::Japanese => BinahBotLocale::Japanese,
            LobocorpLocale::Chinese => BinahBotLocale::ChineseChina,
            LobocorpLocale::ChineseTraditional => BinahBotLocale::ChineseTaiwan,
            _ => BinahBotLocale::Other,
        }
    }
}

impl From<LobocorpLocale> for BinahBotLocale {
    fn from(value: LobocorpLocale) -> Self {
        BinahBotLocale::from(&value)
    }
}

impl From<&BinahBotLocale> for LobocorpLocale {
    fn from(value: &BinahBotLocale) -> Self {
        match value {
            BinahBotLocale::EnglishUS => LobocorpLocale::English,
            BinahBotLocale::Korean => LobocorpLocale::Korean,
            BinahBotLocale::Japanese => LobocorpLocale::Japanese,
            BinahBotLocale::ChineseChina => LobocorpLocale::Chinese,
            BinahBotLocale::ChineseTaiwan => LobocorpLocale::ChineseTraditional,
            _ => LobocorpLocale::English,
        }
    }
}

impl From<BinahBotLocale> for LobocorpLocale {
    fn from(value: BinahBotLocale) -> Self {
        LobocorpLocale::from(&value)
    }
}

impl From<&Rarity> for DiscordEmbedColors {
    fn from(value: &Rarity) -> Self {
        match value {
            Rarity::Paperback => DiscordEmbedColors::PaperbackRarity,
            Rarity::Hardcover => DiscordEmbedColors::HardcoverRarity,
            Rarity::Limited => DiscordEmbedColors::LimitedRarity,
            Rarity::ObjetDArt => DiscordEmbedColors::ObjetDArtRarity,
        }
    }
}

pub fn get_dietype_emoji<'a>(emojis: &'a Emojis, die_type: &'a DieType) -> String {
    let emoji_match = match die_type {
        DieType::Slash => emojis.slash.as_ref(),
        DieType::Pierce => emojis.pierce.as_ref(),
        DieType::Blunt => emojis.blunt.as_ref(),
        DieType::Block => emojis.block.as_ref(),
        DieType::Evade => emojis.evade.as_ref(),
        DieType::CSlash => emojis.c_slash.as_ref(),
        DieType::CPierce => emojis.c_pierce.as_ref(),
        DieType::CBlunt => emojis.c_blunt.as_ref(),
        DieType::CBlock => emojis.c_block.as_ref(),
        DieType::CEvade => emojis.c_evade.as_ref(),
    };
    emoji_match.unwrap_or(&die_type.to_string()).to_string()
}
