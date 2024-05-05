use ruina_common::game_objects::abno_page::AbnoPage;
use ruina_reparser::get_abno_page_locales_by_internal_name;

use super::tagger::PageType;
use super::tagger::Tag;
use super::tagger::Tagger;
use super::tagger::TypedId;

const DEFAULT_ABNO_TAGS: [&str; 1] = ["abno"];

impl Tagger for AbnoPage<'_> {
    fn get_typed_id(&self) -> TypedId {
        TypedId(PageType::AbnoPageId, String::from(self.internal_name))
    }

    fn generate_tags(&self) -> Vec<Tag> {
        let default_tags = DEFAULT_ABNO_TAGS.to_vec();
        get_abno_page_locales_by_internal_name(self.internal_name)
            .values()
            .flat_map(|abno_page_locale| {
                vec![abno_page_locale.card_name, abno_page_locale.abnormality]
            })
            .chain(default_tags)
            .map(String::from)
            .map(Tag)
            .collect()
    }
}
