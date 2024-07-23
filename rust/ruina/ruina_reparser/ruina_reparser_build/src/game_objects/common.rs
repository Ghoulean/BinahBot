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

#[derive(Debug)]
pub struct ParserProps<'a> {
    pub document_strings: Vec<String>,
    pub collectability_map: &'a CollectabilityMap
}
