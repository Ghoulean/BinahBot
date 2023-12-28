mod reserializer;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::reserializer::game_objects::abno_page::reserialize_abno_pages;
use crate::reserializer::game_objects::battle_symbol::reserialize_battle_symbols;
use crate::reserializer::game_objects::combat_page::reserialize_combat_pages;
use crate::reserializer::game_objects::key_page::reserialize_key_pages;
use crate::reserializer::game_objects::passive::reserialize_passives;
use crate::reserializer::localization::abno_page_localization::reserialize_abno_locales;
use crate::reserializer::localization::battle_symbol_localization::reserialize_battle_symbol_locales;
use crate::reserializer::localization::card_effect_localization::reserialize_card_effect_locales;
use crate::reserializer::localization::combat_page_localization::reserialize_combat_page_locales;
use crate::reserializer::localization::key_page_localization::reserialize_key_page_locales;
use crate::reserializer::localization::passive_localization::reserialize_passive_locales;

fn main() {
    let out_file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(PathBuf::from("out.rs"));
    if out_file_path.exists() {
        dbg!(
            "artifacts already exist at {}; not rebuilding",
            out_file_path.to_str().unwrap()
        );
        return;
    }

    let abno_pages = reserialize_abno_pages();
    let battle_symbols = reserialize_battle_symbols();
    let combat_pages = reserialize_combat_pages();
    let key_pages = reserialize_key_pages();
    let passives = reserialize_passives();

    let abno_page_locales = reserialize_abno_locales();
    let battle_symbol_locales = reserialize_battle_symbol_locales();
    let card_effect_locales = reserialize_card_effect_locales();
    let combat_page_locales = reserialize_combat_page_locales();
    let key_page_locales = reserialize_key_page_locales();
    let passive_locales = reserialize_passive_locales();

    let output = [
        abno_pages,
        battle_symbols,
        combat_pages,
        key_pages,
        passives,
        abno_page_locales,
        battle_symbol_locales,
        card_effect_locales,
        combat_page_locales,
        key_page_locales,
        passive_locales,
    ]
    .join("\n");

    let mut out_file = File::create(out_file_path).unwrap();
    out_file.write_all(output.as_bytes()).unwrap();
    dbg!("wrote artifacts");
}
