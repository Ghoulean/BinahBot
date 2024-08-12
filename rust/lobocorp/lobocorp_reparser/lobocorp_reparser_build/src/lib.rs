use abno_localizations::write_abno_localizations;
use abnormalities::load_encyclopedia;
use equipment::load_equipment;
use list::load_encyclopedia_list;
use localization_index::write_localization_index;
use rewrite::write_encyclopedia_info;

mod abno_localizations;
mod abnormalities;
mod equipment;
mod list;
mod localization_index;
mod path;
mod rewrite;
mod serde;
mod xml;

pub fn build_reparser() -> String {
    let abno_list = load_encyclopedia_list();
    let equipment_list = load_equipment();
    let encyclopedia = load_encyclopedia(&abno_list);

    vec![
        write_localization_index(),
        write_encyclopedia_info(&encyclopedia, &equipment_list),
        write_abno_localizations(&abno_list, &encyclopedia),
    ].join("\n")
}
