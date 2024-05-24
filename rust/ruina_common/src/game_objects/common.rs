#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum Rarity {
    Paperback,
    Hardcover,
    Limited,
    ObjetDArt,
}

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum Floor {
    Malkuth,
    Yesod,
    Netzach,
    Hod,
    Tiphereth,
    Gebura,
    Chesed,
    Binah,
    Hokma,
    Keter,
    None,
}

#[derive(Debug, PartialEq, strum_macros::Display)]
pub enum Chapter {
    Canard,
    UrbanMyth,
    UrbanLegend,
    UrbanPlague,
    UrbanNightmare,
    StarOfTheCity,
    ImpuritasCivitatis,
    None,
}
