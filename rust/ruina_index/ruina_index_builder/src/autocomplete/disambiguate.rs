use std::collections::HashMap;
use std::collections::HashSet;

use crate::autocomplete::differentiators::typed_id_disambiguator;
use crate::autocomplete::models::AmbiguousAutocompleteMap;
use crate::autocomplete::models::DisambiguatedAutocomplete;
use crate::autocomplete::models::DisambiguationDisplay;
use crate::autocomplete::models::IncompleteAutocompleteMap;
use crate::taggers::tagger::TypedId;

pub fn disambiguate(naive_map: &IncompleteAutocompleteMap) -> IncompleteAutocompleteMap {
    let mut map = naive_map.clone();

    map = apply_differentiator_to_autocomplete_map(&map, typed_id_disambiguator);

    map
}

fn apply_differentiator_to_autocomplete_map(
    map: &IncompleteAutocompleteMap,
    predicate: fn(&TypedId) -> Option<DisambiguationDisplay>,
) -> IncompleteAutocompleteMap {
    let mut new_map = HashMap::new();
    map.0.iter().for_each(|(x, y)| {
        new_map.insert(
            x.clone(),
            apply_differentiator_to_ambiguous_map(&y, predicate),
        );
    });
    IncompleteAutocompleteMap(new_map)
}

fn apply_differentiator_to_ambiguous_map(
    ambiguous_autocomplete_map: &AmbiguousAutocompleteMap,
    predicate: fn(&TypedId) -> Option<DisambiguationDisplay>,
) -> AmbiguousAutocompleteMap {
    let matches = ambiguous_autocomplete_map
        .0
        .iter()
        .filter(|(_, y)| y.1.is_none())
        .filter(|(x, _)| predicate(x).is_some())
        .map(|(x, y)| (x, (y, predicate(x))))
        .collect::<HashMap<_, _>>();
    let mut disambiguated_map = HashMap::new();
    let mut unique_predicates_set = HashSet::new();

    matches.into_iter().for_each(|(x, (y, z))| {
        if unique_predicates_set.contains(&z) {
            disambiguated_map.remove(x);
        } else {
            disambiguated_map.insert(x.clone(), DisambiguatedAutocomplete(y.0.clone(), z.clone()));
            unique_predicates_set.insert(z);
        }
    });
    ambiguous_autocomplete_map.0.iter().for_each(|(x, y)| {
        if !disambiguated_map.contains_key(&x) {
            disambiguated_map.insert(x.clone(), y.clone());
        }
    });

    AmbiguousAutocompleteMap(disambiguated_map)
}
