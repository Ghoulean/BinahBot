use fluent_templates::Loader;
use fluent_templates::StaticLoader;
use ruina_common::game_objects::common::Chapter;
use ruina_common::game_objects::common::Collectability;
use ruina_common::game_objects::common::PageType;
use ruina_common::localizations::common::Locale;
use ruina_identifier::TypedId;
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;
use strum::IntoEnumIterator;
use toml::from_str;

use crate::heuristics::create_chapter_heuristic;
use crate::heuristics::create_obtainability_heuristic;
use crate::heuristics::create_pagetype_heuristic;
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
    localization_id: String,
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
            TomlPageType::Passive => PageType::Passive,
        }
    }
}

pub type AnnotationMapping<'a> = HashMap<TypedId, HashMap<Locale, String>>;

pub fn precompute_annotations_map<'a>() -> AnnotationMapping<'a> {
    let manual_annotation_toml_map: HashMap<String, Vec<TomlData>> =
        from_str(include_str!("../data/manual/annotations.toml")).unwrap();
    let manual_annotations_toml = manual_annotation_toml_map
        .get("entry")
        .expect("couldn't find \"entry\" from toml");

    parse_manual_mappings(&LOCALES, manual_annotations_toml)
}

pub fn precompute_disambiguations_map<'a>() -> AnnotationMapping<'a> {
    // parsed id -> (locale -> disambiguation string)
    let mut manual_disambiguation_toml_map: HashMap<String, Vec<TomlData>> =
        from_str(include_str!("../data/manual/disambiguations.toml")).unwrap();
    let manual_disambiguation_toml = manual_disambiguation_toml_map
        .remove("entry")
        .take()
        .expect("couldn't find \"entry\" from toml");

    let collectable_heuristic = create_obtainability_heuristic(
        &LOCALES,
        &Collectability::Collectable,
        "availability_collectible",
    );
    let obtainable_heuristic = create_obtainability_heuristic(
        &LOCALES,
        &Collectability::Obtainable,
        "availability_obtainable",
    );
    let enemyonly_heuristic =
        create_obtainability_heuristic(&LOCALES, &Collectability::EnemyOnly, "availability_enemy");

    let canard_heuristic = create_chapter_heuristic(&LOCALES, &Chapter::Canard, "chapter_canard");
    let um_heuristic =
        create_chapter_heuristic(&LOCALES, &Chapter::UrbanMyth, "chapter_urban_myth");
    let ul_heuristic =
        create_chapter_heuristic(&LOCALES, &Chapter::UrbanLegend, "chapter_urban_legend");
    let up_heuristic =
        create_chapter_heuristic(&LOCALES, &Chapter::UrbanPlague, "chapter_urban_plague");
    let un_heuristic = create_chapter_heuristic(
        &LOCALES,
        &Chapter::UrbanNightmare,
        "chapter_urban_nightmare",
    );
    let sotc_heuristic = create_chapter_heuristic(
        &LOCALES,
        &Chapter::StarOfTheCity,
        "chapter_star_of_the_city",
    );
    let ic_heuristic = create_chapter_heuristic(
        &LOCALES,
        &Chapter::ImpuritasCivitatis,
        "chapter_impuritas_civitatis",
    );

    let pagetype_heuristics = PageType::iter()
        .map(|x| create_pagetype_heuristic(&LOCALES, &x))
        .collect::<Vec<_>>();

    let manual_mappings = parse_manual_mappings(&LOCALES, &manual_disambiguation_toml);

    let mut heuristics = Vec::new();
    heuristics.extend(pagetype_heuristics);
    heuristics.extend(vec![
        collectable_heuristic,
        obtainable_heuristic,
        enemyonly_heuristic,
    ]);
    heuristics.extend(vec![
        canard_heuristic,
        um_heuristic,
        ul_heuristic,
        up_heuristic,
        un_heuristic,
        sotc_heuristic,
        ic_heuristic,
    ]);

    let mut vec = heuristics
        .into_iter()
        .map(|x| get_disambiguations_for_uniqueness_heuristic(x))
        .collect::<Vec<_>>();
    vec.insert(0, manual_mappings);

    merge_all(&vec)
}

pub fn write_to_string(annotation_mapping: &AnnotationMapping) -> String {
    let mut builder = phf_codegen::Map::new();

    for (typed_id, locale) in annotation_mapping.iter() {
        let mut locale_builder = phf_codegen::Map::new();
        for (locale, annotation) in locale.iter() {
            locale_builder.entry(format!("{}", locale), &format!("\"{}\"", annotation));
        }
        builder.entry(
            typed_id.to_string(),
            locale_builder.build().to_string().as_str(),
        );
    }
    format!(
        "phf::Map<&'static str, phf::Map<&'static str, &'static str>> = {};",
        builder.build()
    )
}

fn parse_manual_mappings<'a>(
    locales: &'static StaticLoader,
    toml_data: &'a [TomlData],
) -> AnnotationMapping<'a> {
    let mut map = HashMap::new();

    toml_data.iter().for_each(|x| {
        let mut typed_id_map = HashMap::new();

        locales.locales().for_each(|y| {
            typed_id_map.insert(Locale::from(y), locales.lookup(y, &x.localization_id));
        });
    
        map.insert(
            TypedId(
                PageType::from(
                    TomlPageType::from_str(&x.pagetype).expect("bad pagetype found in toml"),
                ),
                x.id.clone(),
            ),
            typed_id_map,
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
fn merge<'a>(
    m1: &'a AnnotationMapping<'a>,
    m2: &'a AnnotationMapping<'a>,
) -> AnnotationMapping<'a> {
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

        assert!(disambiguation_map
            .get(&xiao)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "collectable"));
    }

    #[test]
    fn obtainable_disambiguation() {
        let disambiguation_map = precompute_disambiguations_map();

        let tao_tie = TypedId(PageType::CombatPage, "610022".to_string());

        assert_eq!(
            "obtainable",
            disambiguation_map
                .get(&tao_tie)
                .unwrap()
                .get(&Locale::English)
                .unwrap()
        );
    }

    #[test]
    fn ego_page_disambiguation() {
        let disambiguation_map = precompute_disambiguations_map();

        let fourth_match_flame = TypedId(PageType::CombatPage, "910001".to_string());
        let sound_of_a_star = TypedId(PageType::CombatPage, "910048".to_string());

        assert_eq!(
            "EGO page",
            disambiguation_map
                .get(&fourth_match_flame)
                .unwrap()
                .get(&Locale::English)
                .unwrap()
        );
        assert_eq!(
            "EGO page",
            disambiguation_map
                .get(&sound_of_a_star)
                .unwrap()
                .get(&Locale::English)
                .unwrap()
        );
    }

    #[test]
    fn pagetype_disambiguation() {
        let disambiguation_map = precompute_disambiguations_map();

        let coffin = TypedId(PageType::AbnoPage, "Butterfly_Casket".to_string());
        let coffin_angela = TypedId(PageType::CombatPage, "9910005".to_string());

        let your_shield_battle_symbol = TypedId(PageType::BattleSymbol, "YourShield".to_string());

        assert!(disambiguation_map
            .get(&coffin)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "abno page"));
        assert!(disambiguation_map
            .get(&coffin_angela)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "combat page"));

        assert!(disambiguation_map
            .get(&your_shield_battle_symbol)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "battle symbol"));
    }

    #[test]
    fn gather_intel() {
        let disambiguation_map = precompute_disambiguations_map();

        dbg!(&disambiguation_map);

        let enemy_only = TypedId(PageType::CombatPage, "202005".to_string());
        let collectable = TypedId(PageType::CombatPage, "202009".to_string());
        let handling_work = TypedId(PageType::CombatPage, "301001".to_string());

        assert!(disambiguation_map
            .get(&collectable)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "collectable"));
        assert!(disambiguation_map
            .get(&enemy_only)
            .is_some_and(|x| x.get(&Locale::English).unwrap() == "enemy"));

        assert!(disambiguation_map
            .get(&handling_work)
            .is_some_and(|x| x.get(&Locale::Korean).unwrap() == "collectable"));
        assert!(disambiguation_map
            .get(&enemy_only)
            .is_some_and(|x| x.get(&Locale::Korean).unwrap() == "enemy"));
    }
}
