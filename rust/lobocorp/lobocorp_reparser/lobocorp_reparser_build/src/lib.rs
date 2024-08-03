use abnormalities::load_encyclopedia;
use equipment::load_equipment;
use list::load_encyclopedia_list;
use localization_index::write_localization_index;
use rewrite::write_encyclopedia_info;

mod abnormalities;
mod equipment;
mod list;
mod localization_index;
mod path;
mod rewrite;
mod xml;

pub fn build_reparser() -> String {
    let abno_list = load_encyclopedia_list();
    let equipment_list = load_equipment();
    let encyclopedia = load_encyclopedia(&abno_list);

    vec![
        write_localization_index(),
        write_encyclopedia_info(&encyclopedia, &equipment_list)
    ].join("\n")
}
