use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use lobocorp_common::game_objects::abnormality::ToolType;
use lobocorp_common::game_objects::abnormality::WorkProbabilities;
use lobocorp_common::game_objects::common::DamageRange;
use lobocorp_common::game_objects::common::DamageType;
use lobocorp_common::game_objects::common::Defenses;
use lobocorp_common::game_objects::common::RiskLevel;
use lobocorp_common::game_objects::common::Stat;
use lobocorp_common::game_objects::common::StatBonus;
use roxmltree::Document;
use roxmltree::Node;

use crate::equipment::get_defense_val;
use crate::list::ListEntry;
use crate::path::BASE_CREATURE_DIR;
use crate::path::CHILD_CREATURE_DIR;
use crate::xml::find_unique_node_with_name_and_attribute;
use crate::xml::get_first_node;
use crate::xml::get_nodes;
use crate::xml::get_unique_node;
use crate::xml::get_unique_node_text;

#[derive(Debug)]
pub enum PartialEncyclopediaInfo {
    Normal(PartialNormalInfo),
    Tool(PartialToolInfo),
    DontTouchMe(PartialDontTouchMeInfo),
}

#[derive(Debug)]
pub struct PartialNormalInfo {
    pub risk: RiskLevel,
    pub work_probabilities: WorkProbabilities,
    pub qliphoth_counter: Option<i32>,
    pub work_damage_type: DamageType,
    pub work_damage_range: DamageRange,
    pub work_happiness_ranges: [i32; 3], // <= for :(, <= for :|, <= for :)
    pub work_speed: f64,
    pub work_cooldown: i32,
    pub max_probability_reduction_count: i32,
    pub is_breachable: bool,
    pub defenses: Option<Defenses>, // can be None even if is_breachable == true
    pub observation_level_bonuses: [StatBonus; 4],
    pub related_equipment: Vec<PartialRelatedEquipmentInfo>,
    pub breaching_entities: Vec<PartialBreachingEntity>, // includes self if is_breachable == true
    pub image: Option<String>,
}

#[derive(Debug)]
pub struct PartialRelatedEquipmentInfo {
    pub id: u32,
    pub observation_level: i32,
    pub obtain_number: ObtainEquipmentNumber,
}

#[derive(Debug)]
pub struct PartialDontTouchMeInfo {
    pub risk: RiskLevel,
    pub image: Option<String>,
}

#[derive(Debug)]
pub enum ObtainEquipmentNumber {
    Cost(i32),
    Probability(f64)
}

#[derive(Debug)]
pub struct PartialBreachingEntity {
    pub id: String,
    pub hp: i32,
    pub speed: f64,
    pub defenses: Defenses,
    pub damage_type: DamageType,
    pub risk_level: RiskLevel
}

#[derive(Debug)]
pub struct PartialToolInfo {
    pub risk: RiskLevel,
    pub tool_type: ToolType,
    pub image: Option<String>,
}

pub fn load_encyclopedia(list: &[ListEntry]) -> HashMap<ListEntry, PartialEncyclopediaInfo> {
    list.iter().map(|x| {
        let stat_path = PathBuf::from(format!("{}{}.xml", BASE_CREATURE_DIR, x.stat));
        let stat_str = fs::read_to_string(stat_path.as_path()).expect(&format!("cannot read {:?}", stat_path));
        let doc: Document = Document::parse(&stat_str).expect(&format!("failed parsing {:?}", stat_path));

        (x.clone(), parse(x.id, &doc))
    }).collect()
}

fn parse(id: u32, doc: &Document) -> PartialEncyclopediaInfo {
    let creature_node = get_unique_node(&doc.root(), "creature").expect("couldn't find creature");
    let is_tool = get_unique_node(&creature_node, "stat")
        .and_then(|x| get_unique_node(&x, "workType"))
        .is_ok_and(|x| x.text() == Some("kit"));

    if id == 100024 {
        PartialEncyclopediaInfo::DontTouchMe(PartialDontTouchMeInfo {
            risk: RiskLevel::Zayin, image: Some(id.to_string()),
        })
    } else if is_tool {
        PartialEncyclopediaInfo::Tool(parse_tool_abno(id, doc))
    } else {
        PartialEncyclopediaInfo::Normal(parse_normal_abno(id, doc))
    }
}

fn parse_normal_abno(id: u32, doc: &Document) -> PartialNormalInfo {
    let creature_node = get_unique_node(&doc.root(), "creature").expect("couldn't find creature");
    let stat_node = get_unique_node(&creature_node, "stat").expect("couldn't find stat node");

    let risk = get_unique_node_text(&stat_node, "riskLevel").ok()
        .and_then(|x| x.parse::<i32>().ok())
        .and_then(|x| RiskLevel::try_from(x).ok())
        .expect("couldn't get risk level");

    let work_probabilities = WorkProbabilities {
        instinct: find_unique_node_with_name_and_attribute(&stat_node, "workProb", "type", "R").ok()
            .map(|x| parse_work_probability(&x))
            .expect("couldn't parse instinct probabilities"),
        insight: find_unique_node_with_name_and_attribute(&stat_node, "workProb", "type", "W").ok()
            .map(|x| parse_work_probability(&x))
            .expect("couldn't parse insight probabilities"),
        attachment: find_unique_node_with_name_and_attribute(&stat_node, "workProb", "type", "B").ok()
            .map(|x| parse_work_probability(&x))
            .expect("couldn't parse attachment probabilities"),
        repression: find_unique_node_with_name_and_attribute(&stat_node, "workProb", "type", "P").ok()
            .map(|x| parse_work_probability(&x))
            .expect("couldn't parse repression probabilities"),
    };

    let qliphoth_counter = get_unique_node_text(&stat_node, "qliphoth").ok()
        .and_then(|x| x.parse::<i32>().ok());

    let work_damage_node = get_unique_node(&stat_node, "workDamage").expect("couldn't find work damage node");
    let work_damage_type = work_damage_node.attribute("type").and_then(|x| DamageType::try_from(x).ok()).expect("couldn't find work damage type");
    let work_damage_range = DamageRange(
        work_damage_node.attribute("min").and_then(|x| x.parse::<i32>().ok()).expect("couldn't find work damage min"),
        work_damage_node.attribute("max").and_then(|x| x.parse::<i32>().ok()).expect("couldn't find work damage max"),
    );

    let mut binding = get_unique_node(&stat_node, "feelingStateCubeBounds").ok()
        .map(|x| get_nodes(&x, "cube"))
        .expect("couldn't get happiness ranges")
        .iter()
        .map(|x| {
            x.text().and_then(|y| y.parse::<i32>().ok()).expect("failed parsing happiness range")
        })
        .collect::<Vec<_>>();
    while binding.len() < 3 {
        binding.push(binding[binding.len() - 1]);
    }

    let work_happiness_ranges: [i32; 3] = binding.try_into().expect("failed cast");

    let work_speed = get_unique_node_text(&stat_node, "workSpeed").ok()
        .and_then(|x| x.parse::<f64>().ok())
        .expect("couldn't get work speed");

    let work_cooldown = get_unique_node_text(&stat_node, "workCooltime").ok()
        .and_then(|x| x.parse::<i32>().ok())
        .expect("couldn't get work speed");

    let max_probability_reduction_count = get_unique_node_text(&stat_node, "maxProbReductionCounter").ok()
        .and_then(|x| x.parse::<i32>().ok())
        .filter(|x| *x >= 0)
        .unwrap_or(get_unique_node_text(&stat_node, "riskLevel").ok()
            .and_then(|x| x.parse::<i32>().ok())
            .expect("couldn't get max probability_work_count")
        );

    // missing escapable tag defaults to TRUE
    // missing OR equal to true => not (present AND not equal to true)
    let is_breachable = !get_unique_node_text(&stat_node, "escapeable").is_ok_and(|x| x.to_lowercase().trim() != "true");

    let defense_node = if is_breachable { get_unique_node(&stat_node, "defense") } else { Err("") };
    let defenses = defense_node.map(|x| {
        Defenses {
            red: get_defense_val(&x, "R"),
            white: get_defense_val(&x, "W"),
            black: get_defense_val(&x, "B"),
            pale: get_defense_val(&x, "P"),
        }
    }).ok();

    let observation_level_bonuses: [StatBonus; 4] = get_nodes(&stat_node, "observeBonus").iter().map(|x| {
        let stat = match x.attribute("type").expect("no type").to_lowercase().trim() {
            "speed" => Stat::WorkSpeed,
            "prob" => Stat::SuccessRate,
            _ => unreachable!()
        };
        let val = x.text().and_then(|y| y.parse::<i32>().ok()).expect("no observation level value");
        StatBonus(stat, val)
    }).collect::<Vec<_>>().try_into().expect("failed cast");

    let related_equipment = get_nodes(&stat_node, "equipment").iter().map(|x| {
        let id = x.attribute("equipId").and_then(|x| x.parse::<u32>().ok()).expect("couldn't get equipment id");
        let observation_level = x.attribute("level").and_then(|x| x.parse::<i32>().ok()).expect("couldn't get equipment level");
        let obtain_number = x.attribute("cost")
            .and_then(|x| x.parse::<i32>().ok())
            .map(|x| ObtainEquipmentNumber::Cost(x))
            .unwrap_or_else(|| {
                x.attribute("prob")
                    .and_then(|x| x.parse::<f64>().ok())
                    .map(|x| ObtainEquipmentNumber::Probability(x))
                    .expect("couldn't get equipment obtain number")
                }
            );

        PartialRelatedEquipmentInfo {
            id, observation_level, obtain_number
        }
    }).collect::<Vec<_>>();

    let hp = get_unique_node_text(&stat_node, "hp").ok().and_then(|x| x.parse::<i32>().ok());
    let speed = get_unique_node_text(&stat_node, "speed").ok().and_then(|x| x.parse::<f64>().ok());
    let breaching_damage_type = get_unique_node(&stat_node, "specialDamage").ok()
        .and_then(|x| get_first_node(&x, "damage"))
        .and_then(|x| x.attribute("type"))
        .and_then(|x| DamageType::try_from(x).ok())
        .unwrap_or(work_damage_type.clone());

    let mut breaching_entities = Vec::new();
    if let (Some(hp), Some(speed), Some(defenses)) = (hp, speed, defenses.clone()) {
        breaching_entities.push(PartialBreachingEntity {
            id: id.to_string(),
            hp, speed,
            defenses: defenses,
            damage_type: breaching_damage_type,
            risk_level: risk.clone(),
        })
    };

    let child_abno_stat_id = get_unique_node_text(&creature_node, "child").ok();
    if let Some(child_abno_stat) = child_abno_stat_id {
        // not apo bird -- apo bird is a top-level entry, not a child of small/long/big birds
        if child_abno_stat != "BossBird_stat" {
            let child_stat_path = PathBuf::from(format!("{}{}.xml", CHILD_CREATURE_DIR, child_abno_stat));
            let child_stat_str = fs::read_to_string(child_stat_path.as_path()).expect(&format!("cannot read {:?}", child_stat_path));
            let doc: Document = Document::parse(&child_stat_str).expect(&format!("failed parsing {:?}", child_stat_path));
            let children = parse_child_abno(child_abno_stat, &doc);
            breaching_entities.extend(children);
        }
    };
    if id == 100038 { // apo bird
        let children = vec![
            "BossGateWay_stat", "BigEgg_stat", "LongEgg_stat", "SmallEgg_stat"
        ].into_iter().map(|x| {
            let path_buf = PathBuf::from(format!("{}{}.xml", BASE_CREATURE_DIR, x));
            let file_str = fs::read_to_string(path_buf.as_path()).expect("failed file read");
            parse_egg(&x, &Document::parse(&file_str).expect("failed parse"))
        }).collect::<Vec<_>>();
        breaching_entities.extend(children);
    }

    let image = Some(id.to_string());

    PartialNormalInfo {
        risk, work_probabilities, qliphoth_counter, work_damage_type, work_damage_range, work_happiness_ranges,
        work_speed, work_cooldown, max_probability_reduction_count, is_breachable, defenses, observation_level_bonuses,
        related_equipment, breaching_entities, image,
    }
}

fn parse_tool_abno(id: u32, doc: &Document) -> PartialToolInfo {
    let creature_node = get_unique_node(&doc.root(), "creature").expect("couldn't find creature");
    let stat_node = get_unique_node(&creature_node, "stat").expect("couldn't find stat node");

    let risk = get_unique_node_text(&stat_node, "riskLevel").ok()
        .and_then(|x| x.parse::<i32>().ok())
        .and_then(|x| RiskLevel::try_from(x).ok())
        .expect("couldn't get risk level");

    let tool_type = get_unique_node_text(&stat_node, "kitType").ok()
        .and_then(|x| ToolType::try_from(x).ok())
        .expect("couldn't get tool type");

    let image = Some(id.to_string());

    PartialToolInfo {
        risk, tool_type, image,
    }
}

fn parse_child_abno(id: &str, doc: &Document) -> Vec<PartialBreachingEntity> {
    let creature_node = get_unique_node(&doc.root(), "creature").expect("couldn't find creature");
    let stat_node = get_unique_node(&creature_node, "stat").expect("couldn't find stat node");

    let hp = get_unique_node_text(&stat_node, "hp").ok()
        .and_then(|x| x.parse::<i32>().ok())
        .expect("couldn't get hp");

    let speed = get_unique_node_text(&stat_node, "speed").ok()
        .and_then(|x| x.parse::<f64>().ok())
        .expect("couldn't get hp");

    let damage_type = get_unique_node_text(&creature_node, "attackType").ok()
        .and_then(|x| parse_damage_type_for_children(x));

    let risk_level = get_unique_node_text(&creature_node, "riskLevel").ok()
        .and_then(|x| RiskLevel::try_from(x).ok())
        .expect("couldn't get risk_level");

    get_nodes(&stat_node, "defense").iter().map(|x| {
        let defenses = Defenses {
            red: get_defense_val(&x, "R"),
            white: get_defense_val(&x, "W"),
            black: get_defense_val(&x, "B"),
            pale: get_defense_val(&x, "P"),
        };
        // hardcode apostle damage type
        let damage_type = if id == "DeathAngelApostle_stat" {
            let defense_id = x.attribute("id").and_then(|x| x.parse::<usize>().ok()).expect("couldn't get apostle id");
            [DamageType::Red, DamageType::Pale, DamageType::White, DamageType::Black][defense_id - 1].clone()
        } else {
            damage_type.clone().unwrap()
        };
        let differentiated_id = if id == "DeathAngelApostle_stat" {
            let defense_id = x.attribute("id").and_then(|x| x.parse::<usize>().ok()).expect("couldn't get apostle id");
            format!("{}-{}", id, defense_id)
        } else {
            id.to_string()
        };
        PartialBreachingEntity {
            hp, speed, defenses,
            id: differentiated_id,
            damage_type: damage_type,
            risk_level: risk_level.clone()
        }
    }).collect()
}

fn parse_egg(id: &str, doc: &Document) -> PartialBreachingEntity {
    let creature_node = get_unique_node(&doc.root(), "creature").expect("couldn't find creature");
    let stat_node = get_unique_node(&creature_node, "stat").expect("couldn't find stat node");

    let hp = get_unique_node_text(&stat_node, "hp").ok()
        .and_then(|x| x.parse::<i32>().ok())
        .expect("couldn't get hp");

    let speed = get_unique_node_text(&stat_node, "speed").ok()
        .and_then(|x| x.parse::<f64>().ok())
        .expect("couldn't get hp");

    let defense_node = get_unique_node(&stat_node, "defense").expect("couldn't find stat node"); 
    let defenses = Defenses {
        red: get_defense_val(&defense_node, "R"),
        white: get_defense_val(&defense_node, "W"),
        black: get_defense_val(&defense_node, "B"),
        pale: get_defense_val(&defense_node, "P"),
    };

    let damage_type = get_unique_node(&creature_node, "workDamage").ok()
        .and_then(|x| x.attribute("type"))
        .and_then(|x| DamageType::try_from(x).ok())
        .expect("couldn't get damage_type");

    let risk_level = get_unique_node_text(&creature_node, "riskLevel").ok()
        .and_then(|x| x.parse::<i32>().ok())
        .and_then(|x| RiskLevel::try_from(x).ok())
        .expect("couldn't get risk_level");

    PartialBreachingEntity {
        id: id.to_string(), hp, speed, defenses, damage_type, risk_level
    }
}

fn parse_damage_type_for_children(str: &str) -> Option<DamageType> {
    Some(match str.to_lowercase().trim() {
        "black" => DamageType::Black,
        "red" => DamageType::Red,
        "white" => DamageType::White,
        "unknown" => return None,
        _ => unreachable!()
    })
}

fn parse_work_probability(node: &Node) -> [f64; 5] {
    let mut vec = vec![0.0; 5];
    get_nodes(node, "prob").iter().for_each(|x| {
        let index = x.attribute("level").and_then(|x| x.parse::<usize>().ok())
            .map(|x| x - 1)
            .expect("couldn't get probability index");
        let val = x.text().and_then(|x| x.parse::<f64>().ok())
            .expect("couldn't parse probability");

        vec[index] = val;
    });
    vec.try_into().expect("failed cast")
}