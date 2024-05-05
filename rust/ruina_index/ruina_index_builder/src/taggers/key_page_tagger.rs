use ruina_common::game_objects::key_page::KeyPage;
use ruina_reparser::get_key_page_locales_by_text_id;

use super::tagger::PageType;
use super::tagger::Tag;
use super::tagger::Tagger;
use super::tagger::TypedId;

const DEFAULT_KEY_PAGE_TAGS: [&str; 2] = ["key", "page"];

impl Tagger for KeyPage<'_> {
    fn get_typed_id(&self) -> TypedId {
        TypedId(PageType::KeyPageId, String::from(self.id))
    }

    fn generate_tags(&self) -> Vec<Tag> {
        let default_tags = DEFAULT_KEY_PAGE_TAGS.to_vec();
        self.text_id.map_or(vec!(), |x| {
            get_key_page_locales_by_text_id(x)
                .values()
                .map(|key_page_locale| key_page_locale.name)
                .chain(default_tags)
                .map(String::from)
                .map(Tag)
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruina_reparser::get_key_page_by_id;

    #[test]
    fn sanity() {
        let xiao_key_page = get_key_page_by_id("250036").expect("couldn't find xiao key page (player)");
        let actual_tags = xiao_key_page.generate_tags();
        let expected_tag = Tag("Xiao’s Page".to_string());
        assert!(actual_tags.iter().find(|x| **x == expected_tag).is_some());
    }
}
