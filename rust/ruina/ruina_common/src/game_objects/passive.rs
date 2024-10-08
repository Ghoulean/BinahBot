use crate::game_objects::common::Chapter;
use crate::game_objects::common::Collectability;
use crate::game_objects::common::Rarity;

#[derive(Debug)]
pub struct Passive<'a> {
    pub id: &'a str,
    pub cost: Option<u8>,
    pub rarity: Option<Rarity>,
    pub hidden: Option<bool>,
    pub transferable: Option<bool>,
    pub inner_type: Option<u32>,
    pub collectability: Collectability,
    pub chapter: Option<Chapter>,
}
