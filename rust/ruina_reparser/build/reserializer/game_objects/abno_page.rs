use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use roxmltree::{Document, Node};
use ruina_common::game_objects::abno_page::{Abno, AbnoTargetting};
use ruina_common::game_objects::common::Floor;

use crate::reserializer::commons::paths::{game_obj_path, read_xml_files_in_dir};
use crate::reserializer::commons::serde::serialize_option;
use crate::reserializer::commons::xml::{get_nodes, get_unique_node, get_unique_node_text};

fn abno_path() -> &'static PathBuf {
    static ABNO_PATH: OnceLock<PathBuf> = OnceLock::new();
    ABNO_PATH.get_or_init(|| {
        fs::canonicalize(game_obj_path().as_path().join(PathBuf::from("EmotionCard"))).unwrap()
    })
}

pub fn reserialize_abno_pages() -> String {
    let abnos: Vec<String> = read_xml_files_in_dir(abno_path())
        .into_iter()
        .map(|path_and_document_string| path_and_document_string.1)
        .flat_map(|document_string| process_abno_page_file(document_string.as_str()))
        .collect();
    format!(
        "pub const ABNO_PAGES: [AbnoPage; {}] = [{}];",
        abnos.len(),
        abnos.join(",")
    )
}

fn process_abno_page_file(document_string: &str) -> Vec<String> {
    let doc: Box<Document> = Box::new(Document::parse(document_string).unwrap());
    let xml_root_node = get_unique_node(doc.root(), "EmotionCardXmlRoot").unwrap();
    let abno_node_list = get_nodes(xml_root_node, "EmotionCard");

    abno_node_list
        .into_iter()
        .map(|x| parse_abno_page(x))
        .collect()
}

fn parse_abno_page(abno_node: Node) -> String {
    let id = abno_node.attribute("ID").unwrap();
    let internal_name = get_unique_node_text(abno_node, "Name").unwrap();
    let script_id = get_unique_node_text(abno_node, "Script").unwrap();
    let artwork = internal_name;
    let tier = serialize_option(
        get_unique_node_text(abno_node, "EmotionLevel").map(|x| x.parse::<u8>().unwrap()),
    );
    let bias = serialize_option(
        get_unique_node_text(abno_node, "EmotionRate").map(|x| x.parse::<i8>().unwrap()),
    );
    let sephirah = get_floor_from_str(get_unique_node_text(abno_node, "Sephirah").unwrap());
    let abno = get_abno_from_str(internal_name);
    let is_positive = get_unique_node_text(abno_node, "State").unwrap() == "Positive";
    let targetting =
        get_abno_targetting_from_str(get_unique_node_text(abno_node, "TargetType").unwrap());

    format!(
        "AbnoPage {{
        id: \"{id}\",
        internal_name: \"{internal_name}\",
        script_id: \"{script_id}\",
        artwork: \"{artwork}\",
        tier: {tier},
        bias: {bias},
        sephirah: Floor::{sephirah:?},
        abno: Abno::{abno:?},
        is_positive: {is_positive},
        targetting: AbnoTargetting::{targetting:?}
    }}"
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
        "Enemy" => Abno::None,
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
