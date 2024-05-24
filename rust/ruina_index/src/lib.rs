pub mod models;

use std::collections::HashSet;

use crate::models::Autocomplete;
use crate::models::DisambiguationPage;
use crate::models::PageType::AbnoPageId;
use crate::models::PageType::BattleSymbolId;
use crate::models::PageType::CombatPageId;
use crate::models::PageType::KeyPageId;
use crate::models::PageType::PassiveId;
use crate::models::TypedId;
use ruina_common::localizations::common::Locale;
pub use ruina_index_analyzer::analyze;
pub use ruina_index_analyzer::Token;

include!(concat!(env!("OUT_DIR"), "/out.rs"));

pub fn get_typed_ids(index: &str) -> Option<HashSet<&TypedId<'static>>> {
    INVERSE_CARD_INDEX
        .get(index)
        .and_then(|x| Some(HashSet::from_iter(x.into_iter())))
}

pub fn get_disambiguation_page<'a>(
    display_name: &'a str,
    locale: &'a Locale,
) -> Option<&'a DisambiguationPage<'static>> {
    match locale {
        Locale::Korean => DISAMBIGUATION_PAGES_KOREAN.get(display_name),
        Locale::English => DISAMBIGUATION_PAGES_ENGLISH.get(display_name),
        Locale::Japanese => DISAMBIGUATION_PAGES_JAPANESE.get(display_name),
        Locale::Chinese => DISAMBIGUATION_PAGES_CHINESE.get(display_name),
        Locale::TraditionalChinese => DISAMBIGUATION_PAGES_TRADITIONALCHINESE.get(display_name),
    }
}

pub fn get_autocomplete_entry<'a>(
    typed_id: &'a TypedId,
    locale: &'a Locale,
) -> Option<&'a Autocomplete<'static>> {
    let serialized_typed_id = format!("{}", typed_id);
    match locale {
        Locale::Korean => DISAMBIGUATION_MAP_KOREAN.get(&serialized_typed_id),
        Locale::English => DISAMBIGUATION_MAP_ENGLISH.get(&serialized_typed_id),
        Locale::Japanese => DISAMBIGUATION_MAP_JAPANESE.get(&serialized_typed_id),
        Locale::Chinese => DISAMBIGUATION_MAP_CHINESE.get(&serialized_typed_id),
        Locale::TraditionalChinese => {
            DISAMBIGUATION_MAP_TRADITIONALCHINESE.get(&serialized_typed_id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_get_typed_ids() {
        let degraded_pillar = TypedId(CombatPageId, "607204");

        let pillar_hashset = get_typed_ids("pillar").expect("pillar hashset not found");
        assert!(pillar_hashset.get(&degraded_pillar).is_some());
        // pillar, degraded pillar
        assert_eq!(pillar_hashset.len(), 2);

        let degraded_hashset = get_typed_ids("degrad").expect("degrad hashset not found");
        // fairy, chain, lock, shockwave, pillar
        assert_eq!(degraded_hashset.len(), 5);
        assert!(degraded_hashset.get(&degraded_pillar).is_some());
    }

    #[test]
    fn sanity_get_disambiguation_page() {
        assert!(get_disambiguation_page("Prepared Mind", &Locale::English).is_some());
    }

    #[test]
    fn sanity_get_autocomplete_entry() {
        let degraded_pillar = TypedId(CombatPageId, "607204");

        assert!(get_autocomplete_entry(&degraded_pillar, &Locale::English).is_some());
    }
}
