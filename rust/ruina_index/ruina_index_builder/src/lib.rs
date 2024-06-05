use std::collections::HashMap;

use ruina_common::localizations::common::Locale;
use ruina_identifier::Identifier;
use ruina_identifier::TypedId;
use ruina_index_annotations::precompute_annotations_map;
use ruina_index_annotations::precompute_disambiguations_map;
use ruina_index_annotations::AnnotationMapping;
use ruina_reparser::get_abno_page_locales_by_internal_name;
use ruina_reparser::get_all_abno_pages;

pub fn precompute_index() -> String {

    let annotations = precompute_annotations_map();
    let disambiguations = precompute_disambiguations_map();

    // let mut _index = HashMap::new();

    let _ = get_all_abno_pages().iter().map(|x| assemble_name(&x.get_typed_id(), abno_lookup_fn, &annotations, &disambiguations));


    todo!()
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

fn abno_lookup_fn(id: &str) -> HashMap<Locale, String> {
    get_abno_page_locales_by_internal_name(id).iter().map(|(x, y)| (x.clone(), y.card_name.to_owned())).collect()
}
