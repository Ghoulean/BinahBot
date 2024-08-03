use crate::localizations::common::Locale;

// key, locale
// e.g. ("SnowWhite_weapon_name", Locale::English) -> "Green Stem"
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocalizationKey<'a>(pub &'a str, pub Locale);
