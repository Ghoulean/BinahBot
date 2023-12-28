#[derive(Debug, PartialEq)]
pub enum BattleSymbolSlot {
    Eye,
    Nose,
    Cheek,
    Mouth,
    Ear,
    Headwear1,
    Headwear2,
    Headwear3,
    Headwear4,
    None,
}

pub struct BattleSymbol<'a> {
    pub id: &'a str,
    pub internal_name: &'a str,
    pub resource: &'a str,
    pub slot: BattleSymbolSlot,
    pub hidden: bool,
    pub count: Option<u8>,
}
