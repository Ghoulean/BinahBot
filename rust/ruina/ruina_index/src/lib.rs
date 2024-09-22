pub mod models;

use crate::models::ParsedTypedId;

use index_analyzer::analyze;
use ruina_common::game_objects::common::Page;
use ruina_common::game_objects::common::PageType;
use ruina_common::localizations::common::Locale;
use ruina_common::localizations::common::PageLocale;
use ruina_reparser::get_abno_page_by_internal_name;
use ruina_reparser::get_abno_page_locales_by_internal_name;
use ruina_reparser::get_battle_symbol_by_internal_name;
use ruina_reparser::get_battle_symbol_locales_by_internal_name;
use ruina_reparser::get_combat_page_by_id;
use ruina_reparser::get_combat_page_locales_by_id;
use ruina_reparser::get_key_page_by_id;
use ruina_reparser::get_key_page_locales_by_text_id;
use ruina_reparser::get_passive_by_id;
use ruina_reparser::get_passive_locales_by_id;
use std::cmp::min;
use std::collections::HashMap;
use std::str::FromStr;

include!(concat!(env!("OUT_DIR"), "/out.rs"));

pub fn get_disambiguation(
    parsed_typed_id: &ParsedTypedId,
    locale: &Locale,
) -> Option<&'static &'static str> {
    DISAMBIGUATIONS_MAP
        .get(&parsed_typed_id.to_string())
        .and_then(|x| x.get(&locale.to_string()))
}

pub fn query(query: &str) -> Vec<ParsedTypedId> {
    let ngrams = analyze(query);

    let mut scorekeeper = HashMap::new();

    ngrams.iter().for_each(|(ngram, freq1)| {
        if let Some(map) = INVERSE_INDEX.get(&ngram.0) {
            map.into_iter().for_each(|(typed_id_str, freq2)| {
                scorekeeper
                    .entry(typed_id_str)
                    .and_modify(|x: &mut i32| *x += min(*freq1, *freq2))
                    .or_insert(min(*freq1, *freq2));
            });
        }
    });

    let mut vec: Vec<_> = scorekeeper.iter().collect();
    vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    vec.iter()
        .map(|(typed_id_str, _)| ParsedTypedId::from_str(typed_id_str).unwrap())
        .collect()
}

// todo: where to put this function?
pub fn get_page(typed_id: &ParsedTypedId) -> Option<Page> {
    match typed_id.0 {
        PageType::AbnoPage => get_abno_page_by_internal_name(&typed_id.1).map(Page::Abno),
        PageType::BattleSymbol => {
            get_battle_symbol_by_internal_name(&typed_id.1).map(Page::BattleSymbol)
        }
        PageType::CombatPage => get_combat_page_by_id(&typed_id.1).map(Page::CombatPage),
        PageType::KeyPage => get_key_page_by_id(&typed_id.1).map(Page::KeyPage),
        PageType::Passive => get_passive_by_id(&typed_id.1).map(Page::Passive),
    }
}

// todo: where to put this function?
pub fn get_page_locale<'a>(
    page_type: &'a PageType,
    id: &'a str,
    locale: &'a Locale,
) -> Option<PageLocale<'a>> {
    match page_type {
        PageType::AbnoPage => get_abno_page_locales_by_internal_name(id)
            .get(locale)
            .map(|x| PageLocale::Abno(x)),
        PageType::BattleSymbol => get_battle_symbol_locales_by_internal_name(id)
            .get(locale)
            .map(|x| PageLocale::BattleSymbol(x)),
        PageType::CombatPage => get_combat_page_locales_by_id(id)
            .get(locale)
            .map(|x| PageLocale::CombatPage(x)),
        PageType::KeyPage => get_key_page_by_id(id)
            .and_then(|key_page| {
                key_page.text_id.map(|text_id| {
                    get_key_page_locales_by_text_id(text_id)
                        .get(locale)
                        .map(|x| PageLocale::KeyPage(x))
                })
            })
            .flatten(),
        PageType::Passive => get_passive_locales_by_id(id)
            .get(locale)
            .map(|x| PageLocale::Passive(x)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruina_common::game_objects::common::PageType;

    #[test]
    fn sanity_query() {
        let return_padding = 2;

        let degraded_pillar = ParsedTypedId(PageType::CombatPage, "607204".to_string());

        let pillar_vec = query("pillar");
        let pillar_position = pillar_vec
            .iter()
            .position(|x| *x == degraded_pillar)
            .expect("couldn't find degraded pillar");

        // pillar, degraded pillar
        assert!(pillar_position <= 1 + return_padding);

        let degraded_vec = query("degraded");
        let degraded_position = degraded_vec
            .iter()
            .position(|x| *x == degraded_pillar)
            .expect("couldn't find degraded pillar");

        // degraded shockwave, pillar, chain, lock, fairy
        assert!(degraded_position <= 4 + return_padding);
    }

    #[test]
    fn xiao() {
        let return_padding = 2;

        let binding = ["250036", "150020", "150036", "150038"];
        let xiaos = binding
            .iter()
            .map(|x| ParsedTypedId(PageType::KeyPage, x.to_string()));

        let query_return = query("Xiao");

        xiaos.for_each(|x| {
            let position = query_return
                .iter()
                .position(|y| *y == x)
                .unwrap_or_else(|| panic!("couldn't find xiao {}", x));

            assert!(position <= 4 + return_padding);
        });
    }

    #[test]
    fn sanity_get_disambiguation() {
        assert!(get_disambiguation(
            &ParsedTypedId(PageType::CombatPage, "202002".to_string()),
            &Locale::English
        )
        .is_some());
    }
}
