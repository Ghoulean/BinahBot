use std::collections::HashMap;

use index_analyzer::analyze;
use index_analyzer::Ngram;
use lobocorp_common::localizations::common::Locale;
use lobocorp_reparser::get_abno_localization;
use lobocorp_reparser::get_all_encyclopedia_ids;
use strum::IntoEnumIterator;

type Frequency = i32;

// todo: generalize this with ruina index builder
// todo: annotation on lookup type (abno/weapon/etc)
pub fn write_index() -> String {
    let index = generate_index();
    let inverse_index = invert_index(&index);
    let mut builder = phf_codegen::Map::new();

    for (ngram, id_map) in inverse_index.iter() {
        let mut id_builder = phf_codegen::Map::new();
        for (id, frequency) in id_map.iter() {
            id_builder.entry(id, frequency.to_string().as_str());
        }
        builder.entry(ngram.0.to_string(), id_builder.build().to_string().as_str());
    }

    format!(
        // ngram -> (id, frequency)
        "static INVERSE_INDEX: phf::Map<&'static str, phf::Map<u32, i32>> = {};",
        builder.build()
    )
}

fn generate_index() -> HashMap<u32, HashMap<Ngram, Frequency>> {
    get_all_encyclopedia_ids()
        .iter()
        .map(|x| (**x, analyze(&assemble_name(x))))
        .collect()
}

fn assemble_name(id: &u32) -> String {
    Locale::iter()
        .map(|locale| {
            let localization =
                get_abno_localization(id, &locale).expect(&format!("id={} not found", id));
            format!(
                "{} {} {}",
                localization.name,
                localization.other_names.join(" "),
                localization.code
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// todo: pull this out to index_analyzer?
fn invert_index(
    map: &HashMap<u32, HashMap<Ngram, Frequency>>,
) -> HashMap<Ngram, HashMap<u32, Frequency>> {
    let mut ret_val = HashMap::new();

    for (typed_id, ngram_map) in map.iter() {
        for (ngram, frequency) in ngram_map.iter() {
            let entry = ret_val.entry(ngram.clone()).or_insert(HashMap::new());
            entry.insert(typed_id.clone(), frequency.to_owned());
        }
    }

    ret_val
}
