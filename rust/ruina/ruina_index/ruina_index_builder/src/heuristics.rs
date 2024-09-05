use std::collections::HashMap;

use fluent_templates::Loader;
use fluent_templates::StaticLoader;
use ruina_common::game_objects::common::Chapter;
use ruina_common::game_objects::common::Collectability;
use ruina_common::game_objects::common::PageType;
use ruina_common::localizations::common::Locale;
use ruina_identifier::Identifier;
use ruina_identifier::TypedId;
use ruina_reparser::get_all_abno_pages;
use ruina_reparser::get_all_battle_symbols;
use ruina_reparser::get_all_combat_pages;
use ruina_reparser::get_all_key_pages;
use ruina_reparser::get_all_passives;
use ruina_reparser::get_combat_page_by_id;
use ruina_reparser::get_key_page_by_id;
use ruina_reparser::get_passive_by_id;
use strum::IntoEnumIterator;
use unic_langid::LanguageIdentifier;
use crate::annotations::AnnotationMapping;
use crate::name::get_display_names;

type HeuristicFunction = Box<dyn Fn(&TypedId, &Locale) -> Option<String>>;

pub fn get_disambiguations_for_uniqueness_heuristic<'a>(
    f: HeuristicFunction
) -> AnnotationMapping<'a> {
    // locale -> (display -> vec<ids>)
    // apply heuristic
    // locale -> (filtered (display -> id (singleton)))
    // locale -> vec<ids>
    let disambiguations = Locale::iter().map(|x| {
        (
            x.clone(),
            group_ambiguities(&get_display_names_for_locale(&x))
                .into_iter()
                .filter_map(|(_, ids)| {
                    let vec = ids.into_iter().filter(|id| {
                        f(id, &x).is_some()
                    }).collect::<Vec<_>>();

                    if vec.len() == 1 {
                        Some(vec.first().unwrap().clone())
                    } else {
                        None
                    }
                }).collect::<Vec<_>>()
        )
    }).collect::<HashMap<_, _>>();

    let mut ret_val = HashMap::new();

    for (locale, vec_ids) in disambiguations {
        for id in vec_ids {
            ret_val.entry(id.clone())
                .and_modify(|x: &mut HashMap<Locale, String>| {
                    x.insert(locale.clone(), f(&id, &locale).unwrap());
                })
                .or_insert(HashMap::from(
                    [
                        (locale.clone(), f(&id, &locale).unwrap())
                    ]
                ));
        }
    }

    ret_val
}

pub fn create_pagetype_heuristic(
    locales: &'static StaticLoader,
    page_type: &PageType
) -> HeuristicFunction {
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

pub fn create_obtainability_heuristic(
    locales: &'static StaticLoader,
    collectability: &Collectability,
    disambiguation_key: &str
) -> HeuristicFunction {
    let binding = collectability.clone();
    let binding2 = disambiguation_key.to_owned();

    Box::new(move |typed_id: &TypedId, locale: &Locale| {
        if get_collectability(typed_id) == binding {
            let lang_id = LanguageIdentifier::from(locale);
            let str = locales.lookup(&lang_id, &binding2);
            Some(str.clone())
        } else {
            None
        }
    })
}

pub fn create_chapter_heuristic(
    locales: &'static StaticLoader,
    chapter: &Chapter,
    disambiguation_key: &str
) -> HeuristicFunction {
    let binding = chapter.clone();
    let binding2 = disambiguation_key.to_owned();

    Box::new(move |typed_id: &TypedId, locale: &Locale| {
        if get_collectability(typed_id) != Collectability::Collectable && get_chapter(typed_id).is_some_and(|x| x == binding) {
            let lang_id = LanguageIdentifier::from(locale);
            let str = locales.lookup(&lang_id, &binding2);
            Some(str.clone())
        } else {
            None
        }
    })
}

fn get_collectability(typed_id: &TypedId) -> Collectability {
    match typed_id.0 {
        PageType::AbnoPage => Some(Collectability::Collectable),
        PageType::BattleSymbol => Some(Collectability::Collectable),
        PageType::CombatPage => {
            get_combat_page_by_id(&typed_id.1).map(|x| x.collectability.clone())
        },
        PageType::KeyPage => {
            get_key_page_by_id(&typed_id.1).map(|x| x.collectability.clone())   
        },
        PageType::Passive => {
            get_passive_by_id(&typed_id.1).map(|x| x.collectability.clone())
        },
    }.unwrap()
}

fn get_chapter(typed_id: &TypedId) -> Option<Chapter> {
    match typed_id.0 {
        PageType::AbnoPage => None, // not useful
        PageType::BattleSymbol => None, // not useful
        PageType::CombatPage => {
            get_combat_page_by_id(&typed_id.1).and_then(|x| x.chapter.clone())
        },
        PageType::KeyPage => {
            get_key_page_by_id(&typed_id.1).and_then(|x| x.chapter.clone())   
        },
        PageType::Passive => None, // passive mappings not entirely correct; maybe enable after fixing
    }
}

fn get_display_names_for_locale(locale: &Locale) -> HashMap<TypedId, String> {
    get_all_abno_pages().into_iter().map(|x| x.get_typed_id()).chain(
        get_all_battle_symbols().into_iter().map(|x| x.get_typed_id())
    ).chain(
        get_all_combat_pages().into_iter().map(|x| x.get_typed_id())
    ).chain(
        get_all_key_pages().into_iter().map(|x| x.get_typed_id())
    ).chain(
        get_all_passives().into_iter().map(|x| x.get_typed_id())
    ).map(|x| (
        x.clone(),
        get_display_equivalence(get_display_names(&x).get(locale).unwrap_or(&x.to_string()))
    )).collect()
}

fn get_display_equivalence(s: &str) -> String {
    let suffixes = ["’s Page",
        "’ Page",
        "のページ",
        "사서 책장",
        "의 책장",
        "책장",
        "之页",
        "之頁"];
    let mut new_str = s;
    suffixes.iter().for_each(|x| {
        if new_str.ends_with(x) {
            new_str = &new_str[0..new_str.len() - x.len()];
        }
    });
    new_str.trim().to_string()
}

fn group_ambiguities(map: &HashMap<TypedId, String>) -> HashMap<String, Vec<TypedId>> {
    let mut inverse_map = HashMap::new();

    for (typed_id, display_name) in map.iter() {
        inverse_map.entry(display_name)
            .and_modify(|v: &mut Vec<TypedId>| v.push(typed_id.clone()))
            .or_insert(Vec::from([typed_id.clone()]));
    }
    
    inverse_map.iter().filter_map(|(k, v)| {
        if v.len() == 1 {
            None
        } else {
            Some(((*k).clone(), v.clone()))
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_display_equivalence() {
        assert_eq!(get_display_equivalence("Xiao’s Page"), get_display_equivalence("Xiao"));
        assert_eq!(get_display_equivalence(",D@;Q7Yのページ"), get_display_equivalence(",D@;Q7Y"));
        assert_eq!(get_display_equivalence("enlxmfflsdis"), get_display_equivalence("enlxmfflsdis"));
    }

    #[test]
    fn sanity_group_english_ambiguities() {
        let english = group_ambiguities(&get_display_names_for_locale(&Locale::English));

        assert!(english.contains_key("Xiao"));
        assert!(english.contains_key("얀샋ㄷ요무"));
        assert!(!english.contains_key("Concussion"));
    }
}