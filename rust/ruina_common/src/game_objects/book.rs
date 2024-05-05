use crate::game_objects::common::Chapter;

#[derive(Debug)]
pub struct Book<'a> {
    pub id: &'a str,
    pub text_id: &'a str,
    pub icon: &'a str,
    pub chapter: Chapter,
    pub key_page_drops: &'a [&'a str],
    pub combat_page_drops: &'a [&'a str],
}
