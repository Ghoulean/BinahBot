use std::collections::HashMap;

use ruina_common::game_objects::common::PageType;
use ruina_common::localizations::common::Locale;
use ruina_identifier::TypedId;
use ruina_reparser::get_abno_page_locales_by_internal_name;
use ruina_reparser::get_battle_symbol_locales_by_internal_name;
use ruina_reparser::get_combat_page_locales_by_id;
use ruina_reparser::get_key_page_by_id;
use ruina_reparser::get_key_page_locales_by_text_id;
use ruina_reparser::get_passive_locales_by_id;

pub fn get_display_names(
    typed_id: &TypedId
) -> HashMap<Locale, String> {
    (match typed_id.0 {
        PageType::AbnoPage => {
            abno_lookup_fn
        }
        PageType::BattleSymbol => {
            battle_symbol_lookup_fn
        }
        PageType::CombatPage => {
            combat_page_lookup_fn
        }
        PageType::KeyPage => {
            key_page_lookup_fn
        }
        PageType::Passive => {
            passive_lookup_fn
        }
    })(&typed_id.1)
}

fn abno_lookup_fn(id: &str) -> HashMap<Locale, String> {
    get_abno_page_locales_by_internal_name(id).iter().map(|(x, y)| (x.clone(), y.card_name.to_owned())).collect()
}

fn battle_symbol_lookup_fn(id: &str) -> HashMap<Locale, String> {
    get_battle_symbol_locales_by_internal_name(id).iter().map(|(x, y)| (x.clone(), format!("{} {}", y.prefix, y.postfix))).collect()
}

fn combat_page_lookup_fn(id: &str) -> HashMap<Locale, String> {
    get_combat_page_locales_by_id(id).iter().map(|(x, y)| (x.clone(), y.name.to_owned())).collect()
}

fn key_page_lookup_fn(id: &str) -> HashMap<Locale, String> {
    get_key_page_by_id(id).map(|key_page| key_page.text_id.map(|text_id| {
        get_key_page_locales_by_text_id(text_id).iter().map(|(x, y)| (x.clone(), y.name.to_owned())).collect()
    })).flatten().unwrap_or(HashMap::new())
}

fn passive_lookup_fn(id: &str) -> HashMap<Locale, String> {
    get_passive_locales_by_id(id).iter().map(|(x, y)| (x.clone(), y.name.to_owned())).collect()
}
