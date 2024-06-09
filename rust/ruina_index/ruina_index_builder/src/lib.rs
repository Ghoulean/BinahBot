use std::collections::HashMap;

use ruina_common::localizations::common::Locale;
use ruina_identifier::Identifier;
use ruina_identifier::TypedId;
use ruina_index_analyzer::analyze;
use ruina_index_analyzer::Ngram;
use ruina_index_annotations::precompute_annotations_map;
use ruina_index_annotations::precompute_disambiguations_map;
use ruina_index_annotations::AnnotationMapping;
use ruina_reparser::get_abno_page_locales_by_internal_name;
use ruina_reparser::get_all_abno_pages;
use ruina_reparser::get_all_battle_symbols;
use ruina_reparser::get_all_combat_pages;
use ruina_reparser::get_all_key_pages;
use ruina_reparser::get_all_passives;
use ruina_reparser::get_battle_symbol_locales_by_internal_name;
use ruina_reparser::get_combat_page_locales_by_id;
use ruina_reparser::get_key_page_locales_by_text_id;
use ruina_reparser::get_passive_locales_by_id;
use ruina_reparser::get_key_page_by_id;

type Frequency = i32;

pub fn precompute_index() -> String {

    let annotations = precompute_annotations_map();
    let disambiguations = precompute_disambiguations_map();

    let abno_map: HashMap<_, _> = generate_index(
        &get_all_abno_pages(),
        abno_lookup_fn,
        &annotations,
        &disambiguations
    );
    let battle_symbol_map: HashMap<_, _> = generate_index(
        &get_all_battle_symbols(),
        battle_symbol_lookup_fn,
        &annotations,
        &disambiguations
    );
    let combat_page_map: HashMap<_, _> = generate_index(
        &get_all_combat_pages(),
        combat_page_lookup_fn,
        &annotations,
        &disambiguations
    );
    let key_page_map: HashMap<_, _> = generate_index(
        &get_all_key_pages(),
        key_page_lookup_fn,
        &annotations,
        &disambiguations
    );
    let passive_map: HashMap<_, _> = generate_index(
        &get_all_passives(),
        passive_lookup_fn,
        &annotations,
        &disambiguations
    );

    let combined_map: HashMap<_, _> = abno_map.into_iter()
        .chain(battle_symbol_map)
        .chain(combat_page_map)
        .chain(key_page_map)
        .chain(passive_map)
        .collect();
    
    let inverse_index = invert_index(&combined_map);

    let mut builder = phf_codegen::Map::new();

    for (ngram, typed_id_map) in inverse_index.iter() {
        let mut typed_id_builder = phf_codegen::Map::new();
        for (typed_id, frequency) in typed_id_map.iter() {
            typed_id_builder.entry(typed_id.to_string(), frequency.to_string().as_str());
        }
        builder.entry(ngram.0.to_string(), typed_id_builder.build().to_string().as_str());
    }
    format!(
        // ngram -> (typed id, frequency)
        "static INVERSE_INDEX: phf::Map<&'static str, phf::Map<&'static str, i32>> = {};",
        builder.build()
    )
}

fn generate_index<T: Identifier>(
    identifier: &[T],
    locale_lookup_fn: fn(id: &str) -> HashMap<Locale, String>,
    annotations: &AnnotationMapping,
    disambiguations: &AnnotationMapping
) -> HashMap<TypedId, HashMap<Ngram, Frequency>> {
    identifier.iter()
        .map(|x| {
            (
                x.get_typed_id(),
                analyze(
                    &assemble_name(
                        &x.get_typed_id(),
                        locale_lookup_fn,
                        &annotations,
                        &disambiguations
                    )
                )
            )
        }).collect()
}

fn assemble_name(
    typed_id: &TypedId,
    locale_lookup_fn: fn(id: &str) -> HashMap<Locale, String>,
    annotations: &AnnotationMapping,
    disambiguations: &AnnotationMapping
) -> String {
    let annotations_vec = annotations.get(typed_id).map(|x| x.values().collect::<Vec<_>>()).unwrap_or(vec![]);
    let disambiguations_vec = disambiguations.get(typed_id).map(|x| x.values().collect::<Vec<_>>()).unwrap_or(vec![]);
    locale_lookup_fn(&typed_id.1).values()
        .chain(annotations_vec).chain(disambiguations_vec).map(|x| x.clone()).collect::<Vec<_>>().join(" ")
}

fn invert_index(
    map: &HashMap<TypedId, HashMap<Ngram, Frequency>>
) -> HashMap<Ngram, HashMap<TypedId, Frequency>> {
    let mut ret_val = HashMap::new();

    for (typed_id, ngram_map) in map.iter() {
        for (ngram, frequency) in ngram_map.iter() {
            let entry = ret_val.entry(ngram.clone()).or_insert(HashMap::new());
            entry.insert(typed_id.clone(), frequency.to_owned());
        }
    }

    ret_val
}

fn abno_lookup_fn(id: &str) -> HashMap<Locale, String> {
    get_abno_page_locales_by_internal_name(id).iter().map(|(x, y)| (x.clone(), y.card_name.to_owned())).collect()
}

fn battle_symbol_lookup_fn(id: &str) -> HashMap<Locale, String> {
    get_battle_symbol_locales_by_internal_name(id).iter().map(|(x, y)| (x.clone(), y.internal_name.to_owned())).collect()
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
