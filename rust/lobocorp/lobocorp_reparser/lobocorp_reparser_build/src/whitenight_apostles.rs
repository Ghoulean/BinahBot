use lobocorp_common::localizations::common::Locale;

// todo: move into data file
pub fn get_apostle_names(locale: &Locale) -> [String; 4] {
    match locale {
        // todo: locales other than English
        _ => ["Scythe Apostle", "Guardian Apostle", "Staff Apostle", "Spear Apostle"],
    }.iter().map(|x| x.to_string()).collect::<Vec<_>>().try_into().unwrap()
}
