use std::collections::HashMap;

use ruina_common::localizations::common::Locale;
use ruina_reparser::{
    get_abno_page_locales_by_internal_name, get_battle_symbol_locales_by_internal_name,
    get_combat_page_locales_by_id, get_key_page_locales_by_text_id, get_passive_locales_by_id,
};
use ruina_reparser::{
    get_all_abno_pages, get_all_battle_symbols, get_all_combat_pages, get_all_key_pages,
    get_all_passives,
};

use crate::autocomplete::models::DisambiguatedAutocomplete;
use crate::taggers::tagger::Tagger;
use crate::taggers::tagger::TypedId;

use super::models::{AmbiguousAutocomplete, AmbiguousAutocompleteMap, IncompleteAutocompleteMap};

// todo: please generalize this. it hurts to look at.
pub fn generate_naive_autocomplete_map(locale: &Locale) -> IncompleteAutocompleteMap {
    let mut ret_val = IncompleteAutocompleteMap(HashMap::new());

    get_all_abno_pages()
        .iter()
        .filter_map(|page| {
            get_abno_page_locales_by_internal_name(page.internal_name)
                .get(locale)
                .map(|x| (page.get_typed_id(), x.card_name))
        })
        .for_each(|(id, autocomplete)| {
            try_insert_incomplete_map(
                &mut ret_val,
                id,
                AmbiguousAutocomplete(String::from(autocomplete)),
            );
        });

    get_all_battle_symbols()
        .iter()
        .filter_map(|page| {
            get_battle_symbol_locales_by_internal_name(page.internal_name)
                .get(locale)
                .map(|x| (page.get_typed_id(), format!("{} {}", x.prefix, x.postfix)))
        })
        .for_each(|(id, autocomplete)| {
            try_insert_incomplete_map(
                &mut ret_val,
                id,
                AmbiguousAutocomplete(String::from(autocomplete)),
            );
        });

    get_all_combat_pages()
        .iter()
        .filter_map(|page| {
            get_combat_page_locales_by_id(page.id)
                .get(locale)
                .map(|x| (page.get_typed_id(), x.name))
        })
        .for_each(|(id, autocomplete)| {
            try_insert_incomplete_map(
                &mut ret_val,
                id,
                AmbiguousAutocomplete(String::from(autocomplete)),
            );
        });

    get_all_key_pages()
        .iter()
        .filter_map(|page| {
            page.text_id
                .map(|x| {
                    get_key_page_locales_by_text_id(x)
                        .get(locale)
                        .map(|x| (page.get_typed_id(), x.name))
                })
                .flatten()
        })
        .for_each(|(id, autocomplete)| {
            try_insert_incomplete_map(
                &mut ret_val,
                id,
                AmbiguousAutocomplete(String::from(autocomplete)),
            );
        });

    get_all_passives()
        .iter()
        .filter_map(|page| {
            get_passive_locales_by_id(page.id)
                .get(locale)
                .map(|x| (page.get_typed_id(), x.name))
        })
        .for_each(|(id, autocomplete)| {
            try_insert_incomplete_map(
                &mut ret_val,
                id,
                AmbiguousAutocomplete(String::from(autocomplete)),
            );
        });

    ret_val
}

fn try_insert_incomplete_map(
    incomplete_map: &mut IncompleteAutocompleteMap,
    id: TypedId,
    autocomplete: AmbiguousAutocomplete,
) {
    let ambiguous_autocomplete_map = incomplete_map
        .0
        .entry(autocomplete.clone())
        .or_insert(AmbiguousAutocompleteMap(HashMap::new()));
    assert!(
        ambiguous_autocomplete_map
            .0
            .insert(
                id.clone(),
                DisambiguatedAutocomplete(autocomplete.clone(), None)
            )
            .is_none(),
        "duplicate entry detected! trying to insert: {id:?}->{autocomplete:?}"
    );
}
