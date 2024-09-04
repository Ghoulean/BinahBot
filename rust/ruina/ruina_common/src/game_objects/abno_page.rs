use crate::game_objects::common::Floor;

use super::common::Chapter;

#[derive(Debug, PartialEq)]
pub enum AbnoTargetting {
    SelectOne,
    All,
    AllIncludingEnemy, // Obsession only afaik
}

#[derive(Debug, Clone, PartialEq)]
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
    EnemyOnly,
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

impl From<Abno> for Chapter {
    fn from(value: Abno) -> Self {
        match value {
            Abno::ScorchedGirl => Chapter::Canard,
            Abno::HappyTeddyBear => Chapter::UrbanLegend,
            Abno::FairyFestival => Chapter::UrbanPlague,
            Abno::QueenBee => Chapter::UrbanNightmare,
            Abno::SnowWhitesApple => Chapter::UrbanNightmare,
            Abno::ForsakenMurderer => Chapter::UrbanMyth,
            Abno::AllAroundHelper => Chapter::UrbanLegend,
            Abno::SingingMachine => Chapter::UrbanPlague,
            Abno::FuneralOfTheDeadButterflies => Chapter::UrbanNightmare,
            Abno::DerFreischutz => Chapter::UrbanNightmare,
            Abno::TodaysShyLook => Chapter::UrbanMyth,
            Abno::RedShoes => Chapter::UrbanLegend,
            Abno::SpiderBud => Chapter::UrbanPlague,
            Abno::Laetitia => Chapter::UrbanNightmare,
            Abno::DreamOfABlackSwan => Chapter::UrbanNightmare,
            Abno::FragmentOfTheUniverse => Chapter::UrbanLegend,
            Abno::ChildOfTheGalaxy => Chapter::UrbanLegend,
            Abno::Porccubus => Chapter::UrbanPlague,
            Abno::Alriune => Chapter::UrbanNightmare,
            Abno::TheSilentOrchestra => Chapter::UrbanNightmare,
            Abno::QueenOfHatred => Chapter::UrbanPlague,
            Abno::KnightOfDespair => Chapter::UrbanPlague,
            Abno::KingOfGreed => Chapter::UrbanNightmare,
            Abno::ServantOfWrath => Chapter::StarOfTheCity,
            Abno::JesterOfNihil => Chapter::StarOfTheCity,
            Abno::LittleRedRidingHoodedMercenary => Chapter::UrbanNightmare,
            Abno::BigAndWillBeBadWolf => Chapter::UrbanNightmare,
            Abno::MountainOfSmilingBodies => Chapter::StarOfTheCity,
            Abno::Nosferatu => Chapter::StarOfTheCity,
            Abno::NothingThere => Chapter::StarOfTheCity,
            Abno::ScarecrowSearchingForWisdom => Chapter::UrbanNightmare,
            Abno::WarmHeartedWoodsman => Chapter::UrbanNightmare,
            Abno::RoadHome => Chapter::StarOfTheCity,
            Abno::Ozma => Chapter::StarOfTheCity,
            Abno::AdultWhoTellsLies => Chapter::StarOfTheCity,
            Abno::BigBird => Chapter::StarOfTheCity,
            Abno::PunishingBird => Chapter::StarOfTheCity,
            Abno::JudgementBird => Chapter::StarOfTheCity,
            Abno::ApocalypseBird => Chapter::ImpuritasCivitatis,
            Abno::BurrowingHeaven => Chapter::StarOfTheCity,
            Abno::PriceOfSilence => Chapter::StarOfTheCity,
            Abno::BlueStar => Chapter::StarOfTheCity,
            Abno::WhiteNight => Chapter::ImpuritasCivitatis,
            Abno::Bloodbath => Chapter::Canard,
            Abno::HeartOfAspiration => Chapter::UrbanLegend,
            Abno::Pinocchio => Chapter::UrbanNightmare,
            Abno::SnowQueen => Chapter::StarOfTheCity,
            Abno::SilentGirl => Chapter::ImpuritasCivitatis,
            Abno::EnemyOnly => Chapter::Canard,
        }
    }
}
