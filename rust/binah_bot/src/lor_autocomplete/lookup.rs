use std::collections::HashMap;
use std::collections::HashSet;

use ruina_index::analyze;
use ruina_index::get_typed_ids;
use ruina_index::models::TypedId;
use ruina_index::Token;

pub fn get_typed_ids_from_query(query: &str) -> Vec<TypedId<'static>> {
    let tokens: Vec<Token> = analyze(query);
    let mut typed_ids = tokens
        .iter()
        .filter_map(|x| get_typed_ids(&x.0))
        .map(|x| set_to_hashmap_count(&x))
        .fold(HashMap::new(), |hm1, hm2| combine_hashmap_count(&hm1, &hm2))
        .into_iter()
        .collect::<Vec<_>>();
    typed_ids.sort_by(|a, b| b.1.cmp(&a.1));
    typed_ids
        .clone()
        .into_iter()
        .map(|(x, _)| x.clone())
        .collect()
}

fn set_to_hashmap_count<'a>(
    hs: &HashSet<&'a TypedId<'static>>,
) -> HashMap<&'a TypedId<'static>, i32> {
    hs.iter().map(|x| (*x, 1)).collect()
}

fn combine_hashmap_count<'a>(
    hm1: &HashMap<&'a TypedId<'static>, i32>,
    hm2: &HashMap<&'a TypedId<'static>, i32>,
) -> HashMap<&'a TypedId<'static>, i32> {
    let set1: HashSet<_> = hm1.keys().cloned().collect();
    let set2: HashSet<_> = hm2.keys().cloned().collect();

    let all_keys: HashSet<_> = set1.union(&set2).collect();
    let combined: HashMap<_, _> = all_keys
        .into_iter()
        .map(|x| (*x, hm1.get(x).unwrap_or(&0) + hm2.get(x).unwrap_or(&0)))
        .collect();
    combined
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruina_index::models::PageType;

    #[test]
    fn sanity_set_to_hashmap_count() {
        let weight_set = get_typed_ids("weight").unwrap();
        let weight_hashmap = set_to_hashmap_count(&weight_set);
        assert_eq!(weight_hashmap.len(), weight_set.len());
        weight_hashmap.values().for_each(|x| {
            assert_eq!(*x, 1);
        });
    }

    #[test]
    fn sanity_combine_hashmap_count() {
        let weight_set = get_typed_ids("weight").unwrap();
        let weight_hashmap = set_to_hashmap_count(&weight_set);
        let sin_set = get_typed_ids("sin").unwrap();
        let sin_hashmap = set_to_hashmap_count(&sin_set);

        let combined = combine_hashmap_count(&weight_hashmap, &sin_hashmap);
        assert_eq!(combined.len(), 2);
        combined.values().for_each(|x| {
            assert_eq!(*x, 2);
        });
    }

    #[test]
    fn sanity_degraded_pillar() {
        let typed_ids = get_typed_ids_from_query("degraded pillar");
        let degraded_pillar = TypedId(PageType::CombatPageId, "607204");
        // degraded pillar, pillar,
        // degraded fairy, degraded chain, degraded lock, degraded shockwave
        assert_eq!(typed_ids.len(), 6);
        assert_eq!(*typed_ids.first().unwrap(), degraded_pillar);
    }
}
