use ruina_common::game_objects::common::Chapter;
use ruina_common::game_objects::common::PageType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CollectabilityMap {
    pub collectable: CollectableMap,
    pub obtainable: ObtainableMap,
    pub enemy_only: EnemyOnlyMap,
}

#[derive(Debug, Deserialize)]
pub struct CollectableMap {
    pub combat_pages: Vec<String>,
    pub key_pages: Vec<String>,
    pub passives: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ObtainableMap {
    pub combat_pages: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct EnemyOnlyMap {
    pub combat_pages: Vec<String>,
    pub key_pages: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterMap {
    pub unranked: SingleChapterMap,
    pub canard: SingleChapterMap,
    pub urban_myth: SingleChapterMap,
    pub urban_legend: SingleChapterMap,
    pub urban_plague: SingleChapterMap,
    pub urban_nightmare: SingleChapterMap,
    pub star_of_the_city: SingleChapterMap,
    pub impuritas_civitatis: SingleChapterMap,
}

#[derive(Debug, Deserialize)]
pub struct SingleChapterMap {
    pub combat_pages: Vec<String>,
    pub key_pages: Vec<String>,
    pub passives: Vec<String>,
}

#[derive(Debug)]
pub struct ParserProps<'a> {
    pub document_strings: Vec<String>,
    pub collectability_map: &'a CollectabilityMap,
    pub chapter_map: &'a ChapterMap,
}

pub fn from_chapter_map<'a>(id: &str, pagetype: &PageType, map: &'a ChapterMap) -> Option<Chapter> {
    let f = |x: &'a SingleChapterMap| -> &'a Vec<String> {
        match pagetype {
            PageType::CombatPage => &x.combat_pages,
            PageType::KeyPage => &x.key_pages,
            PageType::Passive => &x.passives,
            _ => panic!("not part of the chapter map"),
        }
    };
    let vec = vec![
        (&map.unranked, None),
        (&map.canard, Some(Chapter::Canard)),
        (&map.urban_myth, Some(Chapter::UrbanMyth)),
        (&map.urban_legend, Some(Chapter::UrbanLegend)),
        (&map.urban_plague, Some(Chapter::UrbanPlague)),
        (&map.urban_nightmare, Some(Chapter::UrbanNightmare)),
        (&map.star_of_the_city, Some(Chapter::StarOfTheCity)),
        (&map.impuritas_civitatis, Some(Chapter::ImpuritasCivitatis)),
    ];
    for (scm, ch) in vec.into_iter() {
        if f(scm).contains(&id.to_string()) {
            return ch;
        }
    }
    None
}
