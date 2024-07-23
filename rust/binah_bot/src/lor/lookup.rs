use ruina::ruina_common::game_objects::common::Collectability;
use ruina::ruina_common::localizations::common::Locale;
use ruina::ruina_index::models::ParsedTypedId;
use ruina::ruina_reparser::get_combat_page_by_id;
use ruina::ruina_reparser::get_key_page_by_id;
use ruina::ruina_reparser::get_passive_by_id;

use crate::utils::get_display_name_locale;

pub fn lookup<'a>(query: &'a str, locale: &'a Locale, all: bool) -> impl Iterator<Item = ParsedTypedId> + 'a {
    let ids = ruina::ruina_index::query(query);
    ids.into_iter()
        .filter(move |x| {
            all || is_collectable_or_obtainable(x)
        })
        .filter(move |x| {
            all || get_display_name_locale(x, &locale).is_some()
        })
}

pub fn is_collectable_or_obtainable(parsed_typed_id: &ParsedTypedId) -> bool {
    match parsed_typed_id.0 {
        ruina::ruina_common::game_objects::common::PageType::CombatPage => {
            get_combat_page_by_id(&parsed_typed_id.1).map(|x| {
                x.collectability == Collectability::Obtainable || x.collectability == Collectability::Collectable
            }).unwrap_or(false)
        },
        ruina::ruina_common::game_objects::common::PageType::KeyPage => {
            get_key_page_by_id(&parsed_typed_id.1).map(|x| {
                x.collectability == Collectability::Obtainable || x.collectability == Collectability::Collectable
            }).unwrap_or(false)   
        },
        ruina::ruina_common::game_objects::common::PageType::Passive => {
            get_passive_by_id(&parsed_typed_id.1).map(|x| {
                x.collectability == Collectability::Obtainable || x.collectability == Collectability::Collectable
            }).unwrap_or(false)
        },
        _ => true
    }
}