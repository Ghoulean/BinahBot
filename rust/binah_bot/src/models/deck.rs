use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeckData {
    pub keypage_id: Option<String>,
    pub passive_ids: Vec<String>,
    pub combat_page_ids: [Option<String>; 9],
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TiphDeck(pub String, pub i32);

#[derive(Serialize, Deserialize, Debug)]
pub struct Deck {
    pub name: String,
    pub author_id: String,
    pub author_name: String,
    pub description: Option<String>,
    pub deck_data: DeckData,
    pub tiph_deck: Option<TiphDeck>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeckMetadata {
    pub name: String,
    pub author_id: String,
    pub author_name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeckSearchParams {
    pub query: String,
    pub author: Option<String>,
    pub keypage: Option<String>,
}

impl DeckSearchParams {
    pub fn new(query: Option<&str>, author: Option<&str>, keypage: Option<&str>) -> Self {
        DeckSearchParams {
            query: query.unwrap_or("").trim().to_lowercase(),
            author: author.map(|x| x.to_string()),
            keypage: keypage.map(|x| x.to_string()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DeckSearchCacheEntry {
    pub decks: Vec<DeckMetadata>,
    pub result_count: usize,
    pub ttl: u64,
}

#[derive(Debug, PartialEq)]
pub struct ListDecksInteraction {
    pub interaction_id: String,
    pub search_params: DeckSearchParams,
    pub ttl: u64,
}
