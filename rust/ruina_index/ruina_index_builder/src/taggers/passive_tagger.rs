use ruina_common::game_objects::passive::Passive;
use ruina_reparser::get_passive_locales_by_id;

use super::tagger::PageType;
use super::tagger::Tag;
use super::tagger::Tagger;
use super::tagger::TypedId;

const DEFAULT_PASSIVE_TAGS: [&str; 1] = ["passive"];

impl Tagger for Passive<'_> {
    fn get_typed_id(&self) -> TypedId {
        TypedId(PageType::PassiveId, String::from(self.id))
    }

    fn generate_tags(&self) -> Vec<Tag> {
        let default_tags = DEFAULT_PASSIVE_TAGS.to_vec();
        get_passive_locales_by_id(self.id)
            .values()
            .flat_map(|passive_locale| vec![passive_locale.name])
            .chain(default_tags)
            .map(String::from)
            .map(Tag)
            .collect()
    }
}
