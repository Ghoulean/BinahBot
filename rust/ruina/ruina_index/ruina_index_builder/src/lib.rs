mod annotations;
mod heuristics;
mod name;

use std::collections::HashMap;

use name::get_display_names;
use index_analyzer::analyze;
use index_analyzer::Ngram;
use ruina_identifier::Identifier;
use ruina_identifier::TypedId;
use ruina_reparser::get_all_abno_pages;
use ruina_reparser::get_all_battle_symbols;
use ruina_reparser::get_all_combat_pages;
use ruina_reparser::get_all_key_pages;
use ruina_reparser::get_all_passives;
use crate::annotations::AnnotationMapping;

pub use crate::annotations::precompute_annotations_map;
pub use crate::annotations::precompute_disambiguations_map;
pub use crate::annotations::write_to_string;

type Frequency = i32;

pub fn precompute_index() -> String {

    let annotations = precompute_annotations_map();
    let disambiguations = precompute_disambiguations_map();

    let abno_map: HashMap<_, _> = generate_index(
        &get_all_abno_pages(),
        &annotations,
        &disambiguations
    );
    let battle_symbol_map: HashMap<_, _> = generate_index(
        &get_all_battle_symbols(),
        &annotations,
        &disambiguations
    );
    let combat_page_map: HashMap<_, _> = generate_index(
        &get_all_combat_pages(),
        &annotations,
        &disambiguations
    );
    let key_page_map: HashMap<_, _> = generate_index(
        &get_all_key_pages(),
        &annotations,
        &disambiguations
    );
    let passive_map: HashMap<_, _> = generate_index(
        &get_all_passives(),
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
                        annotations,
                        disambiguations
                    )
                )
            )
        }).collect()
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

fn assemble_name(
    typed_id: &TypedId,
    annotations: &AnnotationMapping,
    disambiguations: &AnnotationMapping
) -> String {
    let annotations_vec = annotations.get(typed_id).map(|x| x.values().collect::<Vec<_>>()).unwrap_or_default();
    let disambiguations_vec = disambiguations.get(typed_id).map(|x| x.values().collect::<Vec<_>>()).unwrap_or_default();
    get_display_names(typed_id).values()
        .chain(annotations_vec).chain(disambiguations_vec).cloned().collect::<Vec<_>>().join(" ")
}
