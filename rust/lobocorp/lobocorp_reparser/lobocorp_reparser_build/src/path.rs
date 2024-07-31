use std::path::PathBuf;

use lobocorp_common::localizations::common::Locale;

pub const BASE_LIST_PATH_STR: &str = "./data/BaseList.xml";
pub const BASE_EQUIPMENT_PATH_STR: &str = "./data/BaseEquipment.xml";

pub fn get_localized_equipment_file_path(locale: &Locale) -> PathBuf {
    let str = locale.to_string();

    format!("./data/Language/Localize/{0}/Equipment_{0}.xml", str).into()
}
