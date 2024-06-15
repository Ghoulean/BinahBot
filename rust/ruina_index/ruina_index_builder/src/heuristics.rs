use std::collections::HashMap;

use fluent_templates::Loader;
use fluent_templates::StaticLoader;
use ruina_common::localizations::common::Locale;
use ruina_identifier::Identifier;
use ruina_identifier::TypedId;
use ruina_reparser::get_all_abno_pages;
use ruina_reparser::get_all_battle_symbols;
use ruina_reparser::get_all_combat_pages;
use ruina_reparser::get_all_key_pages;
use ruina_reparser::get_all_passives;
use strum::IntoEnumIterator;
use toml::from_str;
use unic_langid::LanguageIdentifier;
use crate::annotations::AnnotationMapping;
use crate::name::get_display_names;

pub fn create_heuristic(
    locales: &'static StaticLoader,
    locale: &Locale,
    disambiguation_key: &str,
    toml_key: &str,
    toml_str: &str
) -> Box<dyn Fn(&TypedId) -> Option<String>> {
    let toml_map: HashMap<String, Vec<String>> = from_str(
        toml_str
    ).unwrap();
    let toml_vec: Vec<String> = toml_map.get(toml_key).unwrap().clone();
    let lang_id = LanguageIdentifier::from(locale);
    let str = locales.lookup(&lang_id, disambiguation_key);

    Box::new(move |typed_id: &TypedId| {
        if toml_vec.contains(&typed_id.to_string()) {
            Some(str.clone())
        } else {
            None    
        }
    })
}

pub fn get_disambiguations_for_uniqueness_heuristic<'a>(
    f: Box<dyn Fn(&TypedId) -> Option<String>>
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
                        f(&id).is_some()
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
                    x.insert(locale.clone(), f(&id).unwrap());
                })
                .or_insert(HashMap::new());
        }
    }

    ret_val
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
    ).into_iter().map(|x| (
        x.clone(),
        get_display_names(&x).get(locale).unwrap_or(&x.to_string()).clone()
    )).collect()
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
    fn sanity_group_english_ambiguities() {
        let english = group_ambiguities(&get_display_names_for_locale(&Locale::English));

        assert!(english.contains_key("Xiao’s Page"));
        assert!(!english.contains_key("Concussion"));
    }
}