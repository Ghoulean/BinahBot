use std::path::PathBuf;

use lobocorp_common::localizations::common::Locale;

pub const BASE_LIST_PATH_STR: &str = "./data/BaseList.xml";
pub const BASE_EQUIPMENT_PATH_STR: &str = "./data/BaseEquipment.xml";
pub const BASE_CREATURE_DIR: &str = "./data/BaseCreatures/";
pub const CHILD_CREATURE_DIR: &str = "./data/BaseCreatures/ChildCreatures/";

pub fn get_lcoalized_abno_file_path(locale: &Locale, abno_id: &str) -> PathBuf {
    format!("./data/Language/{0}/creatures/{1}_{0}.xml", locale.to_string(), abno_id).into()
}

pub fn get_localized_equipment_file_path(locale: &Locale) -> PathBuf {
    format!("./data/Language/Localize/{0}/Equipment_{0}.xml", locale.to_string()).into()
}
