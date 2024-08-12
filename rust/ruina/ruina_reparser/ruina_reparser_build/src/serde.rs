use std::fmt;

use ruina_common::game_objects::common::{Chapter, Rarity};

pub fn get_rarity_from_str(str: &str) -> Rarity {
    match str {
        "Common" => Rarity::Paperback,
        "Uncommon" => Rarity::Hardcover,
        "Rare" => Rarity::Limited,
        "Unique" => Rarity::ObjetDArt,
        _ => panic!("unexpected incorrect rarity entry"),
    }
}

pub fn _get_chapter_from_str(str: &str) -> Option<Chapter> {
    Some(match str {
        "1" => Chapter::Canard,
        "2" => Chapter::UrbanMyth,
        "3" => Chapter::UrbanLegend,
        "4" => Chapter::UrbanPlague,
        "5" => Chapter::UrbanNightmare,
        "6" => Chapter::StarOfTheCity,
        "7" => Chapter::ImpuritasCivitatis,
        "0" => return None,
        _ => panic!("unexpected incorrect chapter entry: {}", str),
    })
}

pub fn serialize_option_2<T>(
    option: Option<T>,
    serializer: fn(&T) -> String
) -> String {
    if option.is_some() {
        let serialized = serializer(option.as_ref().unwrap());
        format!("Some({serialized})")
    } else {
        "None".to_string()
    }
}

// Numbers and boolean
pub fn display_serializer<T>(x: &T) -> String 
where T: fmt::Display,
{
    x.to_string()
}

// Determines whether or not to use a raw string literal, and if so then how many # to minimally use
// Note that in Ruina's case, exactly 1 consecutive # are found in TRCN localization in a few combat pages (one such example)
// and exactly 2 consecutive # are found in TRCN combat dialogues (all such examples).
// There are no instances where three consecutive # are found
// I could have written this in a way that doesn't care about the Ruina-specific usecase, but I decided that I didn't care.
// TODO: make this more performant
pub fn string_literal_serializer(x: &&str) -> String {
    if !x.contains('\"') {
        format!("r\"{x}\"")
    } else if x.contains("##") {
        format!("r###\"{x}\"###")
    } else if x.contains('#') {
        format!("r##\"{x}\"##")
    } else {
        format!("r#\"{x}\"#")
    }
}

pub fn str_array_serializer(vec: &[&str]) -> String {
    let collected: Vec<_> = vec.iter().map(string_literal_serializer).collect();
    let joined = collected.join(",");
    format!("&[{}]", joined)
}

pub fn chapter_enum_serializer(x: &Chapter) -> String {
    format!("Chapter::{}", x)
}

pub fn rarity_enum_serializer(x: &Rarity) -> String {
    format!("Rarity::{}", x)
}
