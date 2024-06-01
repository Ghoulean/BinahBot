mod autocomplete;
mod index;
mod taggers;

use ruina_index_analyzer::analyze;

use index::index::Index;
use ruina_common::localizations::common::Locale;
use ruina_reparser::{
    get_all_abno_pages, get_all_battle_symbols, get_all_combat_pages, get_all_key_pages,
    get_all_passives,
};
use strum::IntoEnumIterator;

use taggers::tagger::Tag;

use crate::taggers::tagger::Tagger;
use crate::{
    autocomplete::main::generate_serialized_autocomplete_objs, index::inverse_index::InverseIndex,
};

pub fn build_index() -> String {
    let index = build_index_from(get_all_abno_pages())
        .merge(build_index_from(get_all_battle_symbols()))
        .merge(build_index_from(get_all_combat_pages()))
        .merge(build_index_from(get_all_key_pages()))
        .merge(build_index_from(get_all_passives()));

    let inverse_index = InverseIndex::from_index(index);

    let autocomplete_objs = Locale::iter()
        .map(|x| generate_serialized_autocomplete_objs(&x))
        .collect::<Vec<_>>()
        .join("\n");

    [
        inverse_index.to_serialized_phf_map("INVERSE_CARD_INDEX"),
        autocomplete_objs,
    ]
    .join("\n")
}

fn build_index_from(taggers: Vec<&impl Tagger>) -> Index {
    Index(
        taggers
            .iter()
            .map(|x| {
                let tags = x
                    .generate_tags()
                    .iter()
                    .map(|tag| tag.0.clone())
                    .flat_map(|txt| analyze(&txt))
                    .map(|token| token.0)
                    .map(Tag)
                    .collect();
                (x.get_typed_id(), tags)
            })
            .collect(),
    )
}
