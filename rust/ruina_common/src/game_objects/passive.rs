use crate::game_objects::common::Rarity;

pub struct Passive<'a> {
    pub id: &'a str,
    pub cost: Option<u8>,
    pub rarity: Option<Rarity>,
    pub hidden: Option<bool>,
    pub transferable: Option<bool>,
    pub inner_type: Option<u32>,
}
