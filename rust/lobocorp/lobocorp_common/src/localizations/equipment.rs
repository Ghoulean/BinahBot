use crate::localizations::common::Locale;

// key, locale
// e.g. ("SnowWhite_weapon_name", Locale::English) -> "Green Stem"
pub struct LocalizationKey<'a>(&'a str, Locale);