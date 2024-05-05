use crate::game_objects::common::Floor;

#[derive(Debug, PartialEq)]
pub enum AbnoTargetting {
    SelectOne,
    All,
    AllIncludingEnemy // Obsession only afaik
}

#[derive(Debug, PartialEq)]
pub enum Abno {
    // Malkuth
    ScorchedGirl,
    HappyTeddyBear,
    FairyFestival,
    QueenBee,
    SnowWhitesApple,

    // Yesod
    ForsakenMurderer,
    AllAroundHelper,
    SingingMachine,
    FuneralOfTheDeadButterflies,
    DerFreischutz,

    // Hod
    TodaysShyLook,
    RedShoes,
    SpiderBud,
    Laetitia,
    DreamOfABlackSwan,

    // Netzach
    FragmentOfTheUniverse,
    ChildOfTheGalaxy,
    Porccubus,
    Alriune,
    TheSilentOrchestra,

    // Tiphereth
    QueenOfHatred,
    KnightOfDespair,
    KingOfGreed,
    ServantOfWrath,
    JesterOfNihil,

    // Gebura
    LittleRedRidingHoodedMercenary,
    BigAndWillBeBadWolf,
    MountainOfSmilingBodies,
    Nosferatu,
    NothingThere,

    // Chesed
    ScarecrowSearchingForWisdom,
    WarmHeartedWoodsman,
    RoadHome,
    Ozma,
    AdultWhoTellsLies,

    // Binah
    BigBird,
    PunishingBird,
    JudgementBird,
    ApocalypseBird,

    // Hokma
    BurrowingHeaven,
    PriceOfSilence,
    BlueStar,
    WhiteNight,

    // Keter
    Bloodbath,
    HeartOfAspiration,
    Pinocchio,
    SnowQueen,
    SilentGirl,

    // Other
    None,
}

#[derive(Debug)]
pub struct AbnoPage<'a> {
    pub id: &'a str,
    pub internal_name: &'a str,
    pub script_id: &'a str,
    pub artwork: &'a str,
    pub tier: Option<u8>,
    pub bias: Option<i8>,
    pub sephirah: Floor,
    pub abno: Abno,
    pub is_positive: bool,
    pub targetting: AbnoTargetting,
}
