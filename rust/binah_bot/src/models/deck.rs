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
    pub tiph_deck: Option<TiphDeck>
}

#[derive(Debug)]
pub struct DeckMetadata {
    pub name: String,
    pub author_id: String,
    pub author_name: String,
}