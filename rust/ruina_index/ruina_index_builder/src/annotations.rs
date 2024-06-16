use std::collections::HashMap;
use std::str::FromStr;
use fluent_templates::Loader;
use fluent_templates::StaticLoader;
use ruina_common::game_objects::common::PageType;
use ruina_common::localizations::common::Locale;
use ruina_identifier::TypedId;
use serde::Deserialize;
use toml::from_str;

use crate::heuristics::create_heuristic;
use crate::heuristics::get_disambiguations_for_uniqueness_heuristic;

fluent_templates::static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

#[derive(Deserialize, Debug)]
pub struct TomlData {
    pagetype: String,
    id: String,
    localization_id: String
}

#[derive(strum_macros::EnumString)]
pub enum TomlPageType {
    AbnoPage,
    BattleSymbol,
    CombatPage,
    KeyPage,
    Passive,
}

impl From<TomlPageType> for PageType {
    fn from(value: TomlPageType) -> Self {
        match value {
            TomlPageType::AbnoPage => PageType::AbnoPage,
            TomlPageType::BattleSymbol => PageType::BattleSymbol,
            TomlPageType::CombatPage => PageType::CombatPage,
            TomlPageType::KeyPage => PageType::KeyPage,
            TomlPageType::Passive => PageType::Passive
        }
    }
}

pub type AnnotationMapping<'a> = HashMap<TypedId, HashMap<Locale, String>>;

pub fn precompute_annotations_map<'a>() -> AnnotationMapping<'a> {
    let manual_annotation_toml_map: HashMap<String, Vec<TomlData>> = from_str(
        include_str!("../data/manual/annotations.toml")
    ).unwrap();
    let manual_annotations_toml = manual_annotation_toml_map.get("entry").expect("couldn't find \"entry\" from toml");

    let manual_mappings = parse_manual_mappings(&LOCALES, &manual_annotations_toml);

    manual_mappings
}

pub fn precompute_disambiguations_map<'a>() -> AnnotationMapping<'a> {
    let mut manual_disambiguation_toml_map: HashMap<String, Vec<TomlData>> = from_str(
        include_str!("../data/manual/disambiguations.toml")
    ).unwrap();
    let manual_disambiguation_toml = manual_disambiguation_toml_map.remove("entry").take().expect("couldn't find \"entry\" from toml");

    let collectability_toml_str = include_str!("../data/collectability.toml");
    let collectable_heuristic = create_heuristic(&LOCALES, "availability_collectible", "collectable", collectability_toml_str);
    let collectable_map = get_disambiguations_for_uniqueness_heuristic(collectable_heuristic);
    let obtainable_heuristic = create_heuristic(&LOCALES, "availability_obtainable", "obtainable", collectability_toml_str);
    let obtainable_map = get_disambiguations_for_uniqueness_heuristic(obtainable_heuristic);
    let enemyonly_heuristic = create_heuristic(&LOCALES, "availability_enemy", "enemy_only", collectability_toml_str);
    let enemyonly_map = get_disambiguations_for_uniqueness_heuristic(enemyonly_heuristic);

    let manual_mappings = parse_manual_mappings(&LOCALES, &manual_disambiguation_toml);

    merge_all(&vec![
        enemyonly_map,
        obtainable_map,
        collectable_map,
        manual_mappings
    ])
}

// todo: impl FromStr
pub fn write_to_string(annotation_mapping: &AnnotationMapping) -> String {
    let mut builder = phf_codegen::Map::new();

    for (typed_id, locale) in annotation_mapping.iter() {
        let mut locale_builder = phf_codegen::Map::new();
        for (locale, annotation) in locale.iter() {
            locale_builder.entry(
                format!("{}", locale),
                &format!("\"{}\"", annotation)
            );
        }
        builder.entry(typed_id.to_string(), locale_builder.build().to_string().as_str());
    }
    format!(
        "phf::Map<&'static str, phf::Map<&'static str, &'static str>> = {};",
        builder.build()
    )
}

fn parse_manual_mappings<'a>(locales: &'static StaticLoader, toml_data: &'a [TomlData]) -> AnnotationMapping<'a> {
    let mut map = HashMap::new();
    
    toml_data.iter().for_each(|x| {
        let mut typed_id_map = HashMap::new();

        locales.locales().for_each(|y| {
            typed_id_map.insert(Locale::from(y), locales.lookup(&y, &x.localization_id));
        });

        map.insert(
            TypedId(
                PageType::from(TomlPageType::from_str(&x.pagetype).expect("bad pagetype found in toml")),
                x.id.clone()
            ), 
            typed_id_map
        );
    });

    map
}

// in case of collision, the later annotation mappings get priority over the earlier.
// that is, mappings[x] has lesser priority than mappings[x+1] w.r.t. collisions
fn merge_all<'a>(mappings: &'a [AnnotationMapping<'a>]) -> AnnotationMapping<'a> {
    if mappings.len() == 0 {
        return HashMap::new();
    }
    if mappings.len() == 1 {
        return mappings.first().unwrap().clone();
    }
    let merge = merge(&mappings[mappings.len() - 2], &mappings[mappings.len() - 1]);
    let mut vec = mappings[0..mappings.len() - 2].iter().cloned().collect::<Vec<_>>();
    vec.push(merge);

    merge_all(&vec)
}

// do not use this function except in merge_all due to flipped priority (until refactor ig)
// in case of collision, m1 > m2 priority
fn merge<'a>(m1: &'a AnnotationMapping<'a>, m2: &'a AnnotationMapping<'a>) -> AnnotationMapping<'a> {
    let mut ret_val = m1.clone();

    for (typed_id, map) in m2 {
        if ret_val.contains_key(typed_id) {
            ret_val.entry(typed_id.clone()).and_modify(|x| {
                for (locale, str) in map {
                    x.entry(locale.clone()).or_insert(str.to_string());
                }
            });
        } else {
            ret_val.insert(typed_id.clone(), map.clone());
        }
    }

    ret_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let _does_not_crash = precompute_annotations_map();
        let _does_not_crash_2 = precompute_disambiguations_map();
    }

    #[test]
    fn yujin_kizuna_manual_disambiguation() {
        let disambiguation_map = precompute_disambiguations_map();
        let yujin_kizuna = TypedId(PageType::Passive, "241001".to_string());

        assert!(disambiguation_map.contains_key(&yujin_kizuna));
    }

    #[test]
    fn collectable_disambiguation() {
        let disambiguation_map = precompute_disambiguations_map();

        let xiao = TypedId(PageType::KeyPage, "250036".to_string());

        assert_eq!(disambiguation_map.get(&xiao)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "collectable"), true);
    }

    #[test]
    fn obtainable_disambiguation() {
        let disambiguation_map = precompute_disambiguations_map();

        let fourth_match_flame = TypedId(PageType::CombatPage, "910001".to_string());
        let sound_of_a_star = TypedId(PageType::CombatPage, "910048".to_string());

        assert_eq!(disambiguation_map.get(&fourth_match_flame)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "obtainable"), true);
        assert_eq!(disambiguation_map.get(&sound_of_a_star)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "obtainable"), true);
    }

    #[test]
    fn gather_intel() {
        let disambiguation_map = precompute_disambiguations_map();

        dbg!(&disambiguation_map);

        let enemy_only = TypedId(PageType::CombatPage, "202005".to_string());
        let collectable = TypedId(PageType::CombatPage, "202009".to_string());
        let handling_work = TypedId(PageType::CombatPage, "301001".to_string());

        assert_eq!(disambiguation_map.get(&collectable)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "collectable"), true);
        assert_eq!(disambiguation_map.get(&enemy_only)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "enemy"), true);

        assert_eq!(disambiguation_map.get(&handling_work)
            .is_some_and(|x| x.get(&Locale::Korean).unwrap() == "collectable"), true);
        assert_eq!(disambiguation_map.get(&enemy_only)
            .is_some_and(|x| x.get(&Locale::Korean).unwrap() == "enemy"), true);


    }
}
