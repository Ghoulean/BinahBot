use std::fmt;

use ruina_common::game_objects::common::{Rarity, Chapter};

pub fn get_rarity_from_str(str: &str) -> Rarity {
    match str {
        "Common" => Rarity::Paperback,
        "Uncommon" => Rarity::Hardcover,
        "Rare" => Rarity::Limited,
        "Unique" => Rarity::ObjetDArt,
        _ => panic!("unexpected missing/incorrect rarity entry"),
    }
}

pub fn get_chapter_from_str(str: &str) -> Chapter {
    match str {
        "1" => Chapter::Canard,
        "2" => Chapter::UrbanMyth,
        "3" => Chapter::UrbanLegend,
        "4" => Chapter::UrbanPlague,
        "5" => Chapter::UrbanNightmare,
        "6" => Chapter::StarOfTheCity,
        "7" => Chapter::ImpuritasCivitatis,
        "0" => Chapter::None,
        _ => panic!("unexpected missing/incorrect chapter entry: {}", str),
    }
}

// TODO: doesn't work with string, only numbers and bools. so reconsider interface
pub fn serialize_option<T>(option: Option<T>) -> String
where
    T: fmt::Display
{
    if option.is_some() {
        let unwrapped = option.unwrap();
        format!("Some({unwrapped})")
    } else {
        String::from("None")
    }
}

pub fn serialize_option_debug<T>(type_display_name: &str, option: Option<T>) -> String 
where
    T: fmt::Debug
{ 
    if option.is_none() {
        String::from("None")
    } else {
        let unwrapped = option.unwrap();
        format!("Some({type_display_name}::{unwrapped:?})")
    }
}


// TODO: dynamically determine whether or not to use a raw string literal, and how many # to minimally use
// For Ruina, exactly 1 consecutive # are found in TRCN localization in a few combat pages (one such example)
// and exactly 2 consecutive # are found in TRCN combat dialogues (all such examples).
// There are no instances where three consecutive # are found
pub fn serialize_option_str(option: Option<&str>) -> String {
    if option.is_some() {
        let unwrapped = option.unwrap();
        format!("Some(r#\"{unwrapped}\"#)")
    } else {
        String::from("None")
    }
}

// TODO: rename. what this function actually does is it serializes vector into ref of array (intentional)
// TODO: Raw string literals (see above)
pub fn serialize_str_vector(vec: Vec<&str>) -> String {
    let collected: Vec<_> = vec.into_iter().map(|x| format!("r#\"{}\"#", x)).collect();
    let joined = collected.join(",");
    format!("&[{}]", joined)
}
