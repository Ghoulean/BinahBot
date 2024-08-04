use std::fmt;

use lobocorp_common::game_objects::common::Defenses;

pub fn serialize_option<T>(
    option: &Option<T>,
    serializer: fn(&T) -> String
) -> String {
    option.as_ref().map(|x| format!("Some({})", serializer(&x))).unwrap_or("None".to_string())
}

// Numbers, boolean, raw strings
pub fn display_serializer<T>(x: &T) -> String 
where T: fmt::Display,
{
    x.to_string()
}

pub fn str_serializer(x: &String) -> String {
    format!("\"{}\"", x)
}

pub fn defenses_serializer(x: &Defenses) -> String {
    let red = x.red.0;
    let white = x.white.0;
    let black = x.black.0;
    let pale = x.pale.0;
    format!("Defenses {{
        red: Resistance({red:?}),
        white: Resistance({white:?}),
        black: Resistance({black:?}),
        pale: Resistance({pale:?})
    }}")
}

pub fn write_vec(v: &[String]) -> String {
    format!("[{}]", v.join(","))
}