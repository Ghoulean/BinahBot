use std::collections::HashMap;
use std::str::FromStr;
use fluent_templates::Loader;
use fluent_templates::StaticLoader;
use ruina_common::game_objects::common::PageType;
use ruina_common::localizations::common::Locale;
use ruina_identifier::TypedId;
use serde::Deserialize;
use toml::from_str;
use unic_langid::LanguageIdentifier;
use strum::IntoEnumIterator;

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

    

    parse_manual_mappings(&LOCALES, manual_annotations_toml)
}

pub fn precompute_disambiguations_map<'a>() -> AnnotationMapping<'a> {
    let mut manual_disambiguation_toml_map: HashMap<String, Vec<TomlData>> = from_str(
        include_str!("../data/manual/disambiguations.toml")
    ).unwrap();
    let manual_disambiguation_toml = manual_disambiguation_toml_map.remove("entry").take().expect("couldn't find \"entry\" from toml");

    let collectability_toml_str = include_str!("../data/collectability.toml");
    let collectable_heuristic = create_obtainability_heuristic(&LOCALES, "availability_collectible", "collectable", collectability_toml_str);
    let obtainable_heuristic = create_obtainability_heuristic(&LOCALES, "availability_obtainable", "obtainable", collectability_toml_str);
    let enemyonly_heuristic = create_obtainability_heuristic(&LOCALES, "availability_enemy", "enemy_only", collectability_toml_str);

    let pagetype_heuristics = PageType::iter().map(|x| create_pagetype_heuristic(&LOCALES, &x)).collect::<Vec<_>>();

    let manual_mappings = parse_manual_mappings(&LOCALES, &manual_disambiguation_toml);

    let mut heuristics = Vec::new();
    heuristics.extend(pagetype_heuristics);
    heuristics.extend(vec![
        collectable_heuristic,
        obtainable_heuristic,
        enemyonly_heuristic
    ]);

    let mut vec = heuristics.into_iter().map(|x| get_disambiguations_for_uniqueness_heuristic(x)).collect::<Vec<_>>();
    vec.insert(0, manual_mappings);

    merge_all(&vec)
}

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
            typed_id_map.insert(Locale::from(y), locales.lookup(y, &x.localization_id));
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

// in case of collision, earlier > later
fn merge_all<'a>(mappings: &'a [AnnotationMapping<'a>]) -> AnnotationMapping<'a> {
    if mappings.is_empty() {
        return HashMap::new();
    }
    if mappings.len() == 1 {
        return mappings.first().unwrap().clone();
    }
    let merge = merge(&mappings[mappings.len() - 2], &mappings[mappings.len() - 1]);
    let mut vec = mappings[0..mappings.len() - 2].to_vec();
    vec.push(merge);

    merge_all(&vec)
}

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

fn create_obtainability_heuristic(
    locales: &'static StaticLoader,
    disambiguation_key: &str,
    toml_key: &str,
    toml_str: &str
) -> Box<dyn Fn(&TypedId, &Locale) -> Option<String>> {
    let toml_map: HashMap<String, Vec<String>> = from_str(
        toml_str
    ).unwrap();
    let toml_vec: Vec<String> = toml_map.get(toml_key).unwrap().clone();
    let binding = disambiguation_key.to_owned().clone();

    Box::new(move |typed_id: &TypedId, locale: &Locale| {
        if toml_vec.contains(&typed_id.to_string()) {
            let lang_id = LanguageIdentifier::from(locale);
            let str = locales.lookup(&lang_id, &binding);
            Some(str.clone())
        } else {
            None
        }
    })
}

fn create_pagetype_heuristic(
    locales: &'static StaticLoader,
    page_type: &PageType
) -> Box<dyn Fn(&TypedId, &Locale) -> Option<String>> {
    let pagetype_key = match page_type {
        PageType::AbnoPage => "page_type_abno_page",
        PageType::BattleSymbol => "page_type_battle_symbol",
        PageType::CombatPage => "page_type_combat_page",
        PageType::KeyPage => "page_type_key_page",
        PageType::Passive => "page_type_passive",
    };
    let binding = page_type.clone();

    Box::new(move |typed_id: &TypedId, locale: &Locale| {
        if typed_id.0 == binding {
            let lang_id = LanguageIdentifier::from(locale);
            let str = locales.lookup(&lang_id, pagetype_key);
            Some(str.clone())
        } else {
            None
        }
    })
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

        assert!(disambiguation_map.get(&xiao)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "collectable"));
    }

    #[test]
    fn obtainable_disambiguation() {
        let disambiguation_map = precompute_disambiguations_map();

        let fourth_match_flame = TypedId(PageType::CombatPage, "910001".to_string());
        let sound_of_a_star = TypedId(PageType::CombatPage, "910048".to_string());

        assert!(disambiguation_map.get(&fourth_match_flame)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "obtainable"));
        assert!(disambiguation_map.get(&sound_of_a_star)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "obtainable"));
    }

    #[test]
    fn pagetype_disambiguation() {
        let disambiguation_map = precompute_disambiguations_map();

        let coffin = TypedId(PageType::AbnoPage, "Butterfly_Casket".to_string());
        let coffin_angela = TypedId(PageType::CombatPage, "9910005".to_string());

        assert!(disambiguation_map.get(&coffin)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "abno page"));
        assert!(disambiguation_map.get(&coffin_angela)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "combat page"));
    }

    #[test]
    fn gather_intel() {
        let disambiguation_map = precompute_disambiguations_map();

        dbg!(&disambiguation_map);

        let enemy_only = TypedId(PageType::CombatPage, "202005".to_string());
        let collectable = TypedId(PageType::CombatPage, "202009".to_string());
        let handling_work = TypedId(PageType::CombatPage, "301001".to_string());

        assert!(disambiguation_map.get(&collectable)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "collectable"));
        assert!(disambiguation_map.get(&enemy_only)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "enemy"));

        assert!(disambiguation_map.get(&handling_work)
            .is_some_and(|x| x.get(&Locale::Korean).unwrap() == "collectable"));
        assert!(disambiguation_map.get(&enemy_only)
            .is_some_and(|x| x.get(&Locale::Korean).unwrap() == "enemy"));
    }
}
