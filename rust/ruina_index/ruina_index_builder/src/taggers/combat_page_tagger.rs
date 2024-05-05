use ruina_common::game_objects::combat_page::CombatPage;
use ruina_reparser::get_combat_page_locales_by_id;

use super::tagger::PageType;
use super::tagger::Tag;
use super::tagger::Tagger;
use super::tagger::TypedId;

const DEFAULT_COMBAT_PAGE_TAGS: [&str; 2] = ["combat", "page"];

impl Tagger for CombatPage<'_> {
    fn get_typed_id(&self) -> TypedId {
        TypedId(PageType::CombatPageId, String::from(self.id))
    }

    fn generate_tags(&self) -> Vec<Tag> {
        let default_tags = DEFAULT_COMBAT_PAGE_TAGS.to_vec();
        get_combat_page_locales_by_id(self.id)
            .values()
            .flat_map(|combat_page_locale| vec![combat_page_locale.name])
            .chain(default_tags)
            .map(String::from)
            .map(Tag)
            .collect()
    }
}
