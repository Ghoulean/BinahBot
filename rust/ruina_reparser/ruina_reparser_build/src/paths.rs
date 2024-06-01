use std::fs::{self, DirEntry};
use std::io::{self};
use std::path::PathBuf;

use ruina_common::localizations::common::Locale;

pub static BASEMOD_STATIC_INFO: &str = "./BaseMod/StaticInfo";
pub static BASEMOD_LOCALIZE: &str = "./BaseMod/Localize";

pub static ABNO_PAGE_PATH_STR: &str = "./BaseMod/StaticInfo/EmotionCard";
pub static BATTLE_SYMBOL_PATH_STR: &str = "./BaseMod/StaticInfo/GiftInfo";
pub static COMBAT_PAGE_PATH_STR: &str = "./BaseMod/StaticInfo/Card";
pub static KEY_PAGE_PATH_STR: &str = "./BaseMod/StaticInfo/EquipPage";
pub static PASSIVE_PATH_STR: &str = "./BaseMod/StaticInfo/PassiveList";

pub static EN_LOCALE_PATH_STR: &str = "./BaseMod/Localize/en";
pub static KR_LOCALE_PATH_STR: &str = "./BaseMod/Localize/kr";
pub static JP_LOCALE_PATH_STR: &str = "./BaseMod/Localize/jp";
pub static CN_LOCALE_PATH_STR: &str = "./BaseMod/Localize/cn";
pub static TRCN_LOCALE_PATH_STR: &str = "./BaseMod/Localize/trcn";

pub static ABNO_LOCALIZE_DIR: &str = "AbnormalityCards";
pub static BATTLE_SYMBOL_LOCALIZE_DIR: &str = "GiftTexts";
pub static CARD_EFFECT_LOCALIZE_DIR: &str = "BattleCardAbilities";
pub static COMBAT_PAGE_LOCALIZE_DIR: &str = "BattlesCards";
pub static KEY_PAGE_LOCALIZE_DIR: &str = "Books";
pub static PASSIVE_LOCALIZE_DIR: &str = "PassiveDesc";

pub static MOST_PATHS: &[&str] = &[
    BASEMOD_STATIC_INFO,
    BASEMOD_LOCALIZE,
    ABNO_PAGE_PATH_STR,
    BATTLE_SYMBOL_PATH_STR,
    COMBAT_PAGE_PATH_STR,
    KEY_PAGE_PATH_STR,
    PASSIVE_PATH_STR,
    EN_LOCALE_PATH_STR,
    KR_LOCALE_PATH_STR,
    JP_LOCALE_PATH_STR,
    CN_LOCALE_PATH_STR,
    TRCN_LOCALE_PATH_STR
];

pub static LOCALE_PAGE_PATHS: &[&str] = &[
    ABNO_LOCALIZE_DIR,
    BATTLE_SYMBOL_LOCALIZE_DIR,
    CARD_EFFECT_LOCALIZE_DIR,
    COMBAT_PAGE_LOCALIZE_DIR,
    KEY_PAGE_LOCALIZE_DIR,
    PASSIVE_LOCALIZE_DIR
];

pub fn get_locale_path(locale: &Locale) -> PathBuf {
    let str = match locale {
        Locale::English => EN_LOCALE_PATH_STR,
        Locale::Korean => KR_LOCALE_PATH_STR,
        Locale::Japanese => JP_LOCALE_PATH_STR,
        Locale::Chinese => CN_LOCALE_PATH_STR,
        Locale::TraditionalChinese => TRCN_LOCALE_PATH_STR
    };
    PathBuf::from(str)
}

pub fn read_xml_files_in_dir(dir: &PathBuf) -> Vec<(PathBuf, String)> {
    let pathbufs: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .filter(|x: &Result<DirEntry, io::Error>| is_xml_file(x.as_ref().unwrap()))
        .map(|x: Result<DirEntry, io::Error>| x.unwrap().path())
        .collect();
    let raw_file_text: Vec<_> = pathbufs
        .clone()
        .iter()
        .map(|x| fs::read_to_string(x.as_path()).unwrap())
        .collect();
    Iterator::zip(pathbufs.into_iter(), raw_file_text).collect()
}

fn is_xml_file(dir_entry: &DirEntry) -> bool {
    dir_entry.path().is_file() && dir_entry.path().extension().unwrap() == "xml"
}
