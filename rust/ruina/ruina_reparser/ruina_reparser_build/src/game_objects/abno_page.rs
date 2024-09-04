use std::collections::HashMap;

use roxmltree::{Document, Node};
use ruina_common::game_objects::abno_page::{Abno, AbnoTargetting};
use ruina_common::game_objects::common::Floor;

use crate::game_objects::common::ParserProps;
use crate::serde::{display_serializer, serialize_option_2};
use crate::xml::{get_nodes, get_unique_node, get_unique_node_text};

type AbnoKey = String;
type AbnoValue = String;

pub fn reserialize_abno_pages(parser_props: &ParserProps) -> String {
    let abnos: HashMap<_, _> = parser_props.document_strings
        .iter()
        .flat_map(|x| process_abno_page_file(x))
        .collect();

    let mut builder = phf_codegen::Map::new();
    for abno_entry in abnos {
        builder.entry(abno_entry.0.clone(), &abno_entry.1);
    }
    format!(
        "static ABNO_PAGES: phf::Map<&'static str, AbnoPage> = {};",
        builder.build()
    )
}

fn process_abno_page_file(document_string: &str) -> HashMap<AbnoKey, AbnoValue> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "EmotionCardXmlRoot").unwrap();
    let abno_node_list = get_nodes(xml_root_node, "EmotionCard");

    abno_node_list
        .into_iter()
        .map(|x| parse_abno_page(x))
        .collect()
}

fn parse_abno_page(abno_node: Node) -> (AbnoKey, AbnoValue) {
    let id = abno_node.attribute("ID").unwrap();
    let internal_name = get_unique_node_text(abno_node, "Name").unwrap();
    let script_id = get_unique_node_text(abno_node, "Script").unwrap();
    let artwork = internal_name;
    let tier = serialize_option_2(
        get_unique_node_text(abno_node, "EmotionLevel").map(|x| x.parse::<u8>().unwrap()),
        display_serializer
    );
    let bias = serialize_option_2(
        get_unique_node_text(abno_node, "EmotionRate").map(|x| x.parse::<i8>().unwrap()),
        display_serializer
    );
    let sephirah = get_floor_from_str(get_unique_node_text(abno_node, "Sephirah").unwrap());
    let abno = get_abno_from_str(internal_name);
    let is_positive = get_unique_node_text(abno_node, "State").unwrap() == "Positive";
    let targetting =
        get_abno_targetting_from_str(get_unique_node_text(abno_node, "TargetType").unwrap());

    (
        internal_name.to_string(),
        format!(
    "AbnoPage {{
        id: \"{id}\",
        internal_name: \"{internal_name}\",
        script_id: \"{script_id }\",
        artwork: \"{artwork}\",
        tier: {tier},
        bias: {bias},
        sephirah: Floor::{sephirah:?},
        abno: Abno::{abno:?},
        is_positive: {is_positive},
        targetting: AbnoTargetting::{targetting:?}
    }}"
        ),
    )
}

fn get_floor_from_str(str: &str) -> Floor {
    match str {
        "Keter" => Floor::Keter,
        "Malkuth" => Floor::Malkuth,
        "Yesod" => Floor::Yesod,
        "Hod" => Floor::Hod,
        "Netzach" => Floor::Netzach,
        "Tiphereth" => Floor::Tiphereth,
        "Gebura" => Floor::Gebura,
        "Chesed" => Floor::Chesed,
        "Binah" => Floor::Binah,
        "Hokma" => Floor::Hokma,
        "None" => Floor::None,
        _ => panic!("unexpected missing/incorrect sefirot entry"),
    }
}

fn get_abno_targetting_from_str(str: &str) -> AbnoTargetting {
    match str {
        "SelectOne" => AbnoTargetting::SelectOne,
        "All" => AbnoTargetting::All,
        "AllIncludingEnemy" => AbnoTargetting::AllIncludingEnemy,
        _ => panic!("unexpected missing/incorrect abno targetting entry"),
    }
}

fn get_abno_from_str(str: &str) -> Abno {
    let abno_prefix = str
        .split('_')
        .next()
        .expect("couldn't parse prefix from internal name");
    match abno_prefix {
        "Enemy" => Abno::EnemyOnly,
        "Bloodbath" => Abno::Bloodbath,
        "HeartofAspiration" => Abno::HeartOfAspiration,
        "Pinocchio" => Abno::Pinocchio,
        "TheSnowQueen" => Abno::SnowQueen,
        "QuietKid" => Abno::SilentGirl,
        "ScorchedGirl" => Abno::ScorchedGirl,
        "HappyTeddyBear" => Abno::HappyTeddyBear,
        "FairyCarnival" => Abno::FairyFestival,
        "QueenBee" => Abno::QueenBee,
        "SnowWhite" => Abno::SnowWhitesApple,
        "ForsakenMurderer" => Abno::ForsakenMurderer,
        "LittleHelper" => Abno::AllAroundHelper,
        "SingingMachine" => Abno::SingingMachine,
        "Butterfly" => Abno::FuneralOfTheDeadButterflies,
        "Matan" => Abno::DerFreischutz,
        "UniverseZogak" => Abno::FragmentOfTheUniverse,
        "ChildofGalaxy" => Abno::ChildOfTheGalaxy,
        "Porccubus" => Abno::Porccubus,
        "Alriune" => Abno::Alriune,
        "SilentOrchestra" => Abno::TheSilentOrchestra,
        "ShyLookToday" => Abno::TodaysShyLook,
        "RedShoes" => Abno::RedShoes,
        "SpiderBud" => Abno::SpiderBud,
        "Laetitia" => Abno::Laetitia,
        "BlackSwan" => Abno::DreamOfABlackSwan,
        "QueenOfHatred" => Abno::QueenOfHatred,
        "KnightOfDespair" => Abno::KnightOfDespair,
        "Greed" => Abno::KingOfGreed,
        "Angry" => Abno::ServantOfWrath,
        "NihilClown" => Abno::JesterOfNihil,
        "Redhood" => Abno::LittleRedRidingHoodedMercenary,
        "BigBadWolf" => Abno::BigAndWillBeBadWolf,
        "Mountain" => Abno::MountainOfSmilingBodies,
        "Nosferatu" => Abno::Nosferatu,
        "NothingThere" => Abno::NothingThere,
        "ScareCrow" | "Scarecrow" => Abno::ScarecrowSearchingForWisdom,
        "LumberJack" => Abno::WarmHeartedWoodsman,
        "House" => Abno::RoadHome,
        "Ozma" => Abno::Ozma,
        "Oz" => Abno::AdultWhoTellsLies,
        "Bigbird" => Abno::BigBird,
        "SmallBird" | "Smallbird" => Abno::PunishingBird,
        "LongBird" => Abno::JudgementBird,
        "ApocalypseBird" => Abno::ApocalypseBird,
        "Bloodytree" => Abno::BurrowingHeaven,
        "Clock" => Abno::PriceOfSilence,
        "BlueStar" => Abno::BlueStar,
        "WhiteNight" => Abno::WhiteNight,

        _ => panic!("unexpected missing/incorrect abno"),
    }
}
