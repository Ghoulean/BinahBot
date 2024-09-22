use std::collections::HashMap;
use std::path::PathBuf;

use game_objects::common::ChapterMap;
use paths::BATTLE_SYMBOL_LOCALIZE_DIR;
use paths::CARD_EFFECT_LOCALIZE_DIR;
use paths::COMBAT_PAGE_LOCALIZE_DIR;
use paths::KEY_PAGE_LOCALIZE_DIR;
use paths::PASSIVE_LOCALIZE_DIR;
use ruina_common::localizations::common::Locale;
use strum::IntoEnumIterator;
use toml::from_str;

use game_objects::abno_page::reserialize_abno_pages;
use game_objects::battle_symbol::reserialize_battle_symbols;
use game_objects::combat_page::reserialize_combat_pages;
use game_objects::common::CollectabilityMap;
use game_objects::common::ParserProps;
use game_objects::key_page::reserialize_key_pages;
use game_objects::passive::reserialize_passives;
use localization::abno_page_localization::reserialize_abno_locales;
use localization::battle_symbol_localization::reserialize_battle_symbol_locales;
use localization::card_effect_localization::reserialize_card_effect_locales;
use localization::combat_page_localization::reserialize_combat_page_locales;
use localization::key_page_localization::reserialize_key_page_locales;
use localization::passive_localization::reserialize_passive_locales;
use paths::get_locale_path;
use paths::read_xml_files_in_dir;
use paths::ABNO_LOCALIZE_DIR;
use paths::ABNO_PAGE_PATH_STR;
use paths::BATTLE_SYMBOL_PATH_STR;
use paths::COMBAT_PAGE_PATH_STR;
use paths::KEY_PAGE_PATH_STR;
use paths::LOCALE_PAGE_PATHS;
use paths::MOST_PATHS;
use paths::PASSIVE_PATH_STR;

mod game_objects;
mod localization;
mod paths;
mod serde;
mod xml;

pub fn build_reparser() -> String {
    MOST_PATHS.iter().for_each(|x| {
        let path = PathBuf::from(x);
        if !path.exists() {
            panic!(
                "[reparser] cannot find BaseMod dir for {}; see README for more details",
                x
            )
        }
    });

    for locale in Locale::iter() {
        for path_str in LOCALE_PAGE_PATHS {
            let full_path = get_locale_path(&locale).join(path_str);
            if !full_path.exists() {
                panic!(
                    "[reparser] cannot find BaseMod dir for {}; see README for more details",
                    full_path.to_str().unwrap()
                )
            }
        }
    }

    let collectability_toml_str = include_str!("../data/collectability.toml");
    let collectability_toml_map: CollectabilityMap = from_str(collectability_toml_str).unwrap();

    let chapter_toml_str = include_str!("../data/chapter.toml");
    let chapter_toml_map: ChapterMap = from_str(chapter_toml_str).unwrap();

    let abno_pages = reparse(
        ABNO_PAGE_PATH_STR,
        &collectability_toml_map,
        &chapter_toml_map,
        reserialize_abno_pages,
    );
    let battle_symbols = reparse(
        BATTLE_SYMBOL_PATH_STR,
        &collectability_toml_map,
        &chapter_toml_map,
        reserialize_battle_symbols,
    );
    let combat_pages = reparse(
        COMBAT_PAGE_PATH_STR,
        &collectability_toml_map,
        &chapter_toml_map,
        reserialize_combat_pages,
    );
    let key_pages = reparse(
        KEY_PAGE_PATH_STR,
        &collectability_toml_map,
        &chapter_toml_map,
        reserialize_key_pages,
    );
    let passives = reparse(
        PASSIVE_PATH_STR,
        &collectability_toml_map,
        &chapter_toml_map,
        reserialize_passives,
    );

    let abno_page_locales = reparse_locale(ABNO_LOCALIZE_DIR, reserialize_abno_locales);
    let battle_symbol_locales = reparse_locale(
        BATTLE_SYMBOL_LOCALIZE_DIR,
        reserialize_battle_symbol_locales,
    );
    let card_effect_locales =
        reparse_locale(CARD_EFFECT_LOCALIZE_DIR, reserialize_card_effect_locales);
    let combat_page_locales =
        reparse_locale(COMBAT_PAGE_LOCALIZE_DIR, reserialize_combat_page_locales);
    let key_page_locales = reparse_locale(KEY_PAGE_LOCALIZE_DIR, reserialize_key_page_locales);
    let passive_locales = reparse_locale(PASSIVE_LOCALIZE_DIR, reserialize_passive_locales);

    [
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
    .join("\n")
}

fn reparse(
    path_str: &str,
    collectability_map: &CollectabilityMap,
    chapter_map: &ChapterMap,
    reserializer: fn(&ParserProps) -> String,
) -> String {
    let parser_props = ParserProps {
        document_strings: read_xml_files_in_dir(&PathBuf::from(path_str))
            .into_iter()
            .map(|x| x.1)
            .collect::<Vec<_>>(),
        collectability_map,
        chapter_map,
    };

    reserializer(&parser_props)
}

fn reparse_locale(
    dir_str: &str,
    reserializer: fn(&HashMap<Locale, Vec<String>>) -> String,
) -> String {
    reserializer(
        &Locale::iter()
            .map(|x| {
                let full_path = get_locale_path(&x).join(dir_str);
                let vec = read_xml_files_in_dir(&full_path)
                    .into_iter()
                    .map(|x| x.1.clone())
                    .collect::<Vec<_>>();
                (x.clone(), vec)
            })
            .collect::<HashMap<_, _>>(),
    )
}
