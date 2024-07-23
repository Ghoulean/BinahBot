#[derive(Debug, PartialEq, strum_macros::Display)]
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

#[derive(Debug)]
pub struct BattleSymbol<'a> {
    pub id: &'a str,
    pub internal_name: &'a str,
    pub resource: Option<&'a str>,
    pub slot: BattleSymbolSlot,
    pub hidden: bool,
    pub count: Option<u8>,
}
