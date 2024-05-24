use ruina_common::localizations::common::Locale;

use crate::autocomplete::autocomplete::generate_naive_autocomplete_map;
use crate::autocomplete::disambiguate::disambiguate;
use crate::autocomplete::models::DisambiguationDisplay;
use crate::autocomplete::models::IncompleteAutocompleteMap;

pub fn generate_serialized_autocomplete_objs(locale: &Locale) -> String {
    let naive_map = generate_naive_autocomplete_map(locale);
    let serialized_disambiguation_pages = serialize_disambiguation_page_array(&naive_map, locale);

    let disambiguated_map = disambiguate(&naive_map);

    let serialized_disambiguation_map = serialized_disambiguation_map(&disambiguated_map, locale);

    [
        serialized_disambiguation_pages,
        serialized_disambiguation_map,
    ]
    .join("\n")
}

fn serialize_disambiguation_page_array(
    naive_map: &IncompleteAutocompleteMap,
    locale: &Locale,
) -> String {
    let mut builder = phf_codegen::Map::new();
    for (ambiguous_autocomplete, ambiguous_autocomplete_map) in &(naive_map.0) {
        if ambiguous_autocomplete_map.0.len() <= 1 {
            dbg!(
                "autocomplete entry does not need disambiguation:",
                ambiguous_autocomplete
            );
            continue;
        }

        let base_autocomplete = ambiguous_autocomplete.0.clone();
        let associated_page_ids = ambiguous_autocomplete_map
            .0
            .keys()
            .map(|typed_id| format!("{:?}", typed_id))
            .collect::<Vec<String>>()
            .join(",");

        // todo: default pageId determination
        // serialized Option<&str>
        let default = "None";
        builder.entry(
            base_autocomplete.clone(),
            &format!(
                "DisambiguationPage {{
                id: r#\"{base_autocomplete}\"#,
                typed_ids: &[{associated_page_ids}],
                default: {default}
            }}"
            ),
        );
    }
    format!(
        "static DISAMBIGUATION_PAGES_{}: phf::Map<&'static str, DisambiguationPage> = {};",
        locale.to_string().to_ascii_uppercase(),
        builder.build()
    )
}

fn serialized_disambiguation_map(
    disambiguated_map: &IncompleteAutocompleteMap,
    locale: &Locale,
) -> String {
    let mut builder = phf_codegen::Map::new();

    disambiguated_map
        .0
        .values()
        .into_iter()
        .for_each(|inverse_map| {
            inverse_map
                .0
                .clone()
                .into_iter()
                .for_each(|(typed_id, disambiguated_autocomplete)| {
                    let base = disambiguated_autocomplete.0 .0;
                    let disambiguator = serialize_option(disambiguated_autocomplete.1);

                    builder.entry(
                        format!("{}", typed_id),
                        &format!(
                            "Autocomplete {{
                        base: r#\"{base}\"#,
                        disambiguator: {disambiguator},
                    }}"
                        ),
                    );
                });
        });

    format!(
        "static DISAMBIGUATION_MAP_{}: phf::Map<&'static str, Autocomplete> = {};",
        locale.to_string().to_ascii_uppercase(),
        builder.build()
    )
}

fn serialize_option(option: Option<DisambiguationDisplay>) -> String {
    if option.is_none() {
        return String::from("None");
    } else {
        return format!("Some(\"{:}\")", option.unwrap().0);
    }
}
