use ruina_common::localizations::common::Locale;
use ruina_index::models::ParsedTypedId;

use crate::utils::get_display_name_locale;
use crate::utils::is_collectable_or_obtainable;

pub fn lookup<'a>(query: &'a str, locale: &'a Locale, all: bool) -> impl Iterator<Item = ParsedTypedId> + 'a {
    let ids = ruina_index::query(query);
    ids.into_iter()
        .filter(move |x| {
            all || is_collectable_or_obtainable(x)
        })
        .filter(move |x| {
            all || get_display_name_locale(x, &locale).is_some()
        })
}
