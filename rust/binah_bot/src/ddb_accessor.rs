use std::error::Error;
use std::collections::HashMap;
use std::string::String;

use aws_sdk_dynamodb::types::AttributeValue;
use crate::models::deck::Deck;
use crate::models::deck::DeckMetadata;
use crate::models::deck::TiphDeck;

pub async fn get_deck(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    name: &str,
    author: &str
) -> Result<Deck, Box<dyn Error>> {
    let binding = client.get_item()
        .table_name(table_name)
        .key("author", AttributeValue::S(author.to_string()))
        .key("deck_name", AttributeValue::S(name.to_string()))
        .send()
        .await?;
    
    let result = binding.item()
        .ok_or("failed ddb call in get_deck")?;

    Deck::try_from(result)
}

pub async fn put_deck(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    deck: &Deck
) -> Result<(), Box<dyn Error>> {
    Ok(client.put_item()
        .table_name(table_name)
        .set_item(HashMap::<String, AttributeValue>::try_from(deck).ok())
        .send()
        .await
        .map(|_| ())?)
}

// TODO: combine all list_decks with `author: Option<&str>` and `keypage: Option<&str>` interface?
pub async fn list_decks_by_author(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    author: &str,
) -> Result<Vec<DeckMetadata>, Box<dyn Error>> {
    let items: Vec<HashMap<String, AttributeValue>> = client.query()
        .table_name(table_name)
        .key_condition_expression("author = :author")
        .expression_attribute_names(":author", author)
        .projection_expression("author, deck_name")
        .into_paginator()
        .items()
        .send()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    
    items.into_iter().map(|x| -> Result<DeckMetadata, Box<dyn Error>> {
        DeckMetadata::try_from(&x)
    }).collect()
}

pub async fn list_decks_by_keypage(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    keypage_id: &str
) -> Result<Vec<DeckMetadata>, Box<dyn Error>> {
    let items: Vec<HashMap<String, AttributeValue>> = client.query()
        .table_name(table_name)
        .index_name("gsi1")
        .key_condition_expression("keypage = :keypage")
        .expression_attribute_names(":keypage", keypage_id)
        .projection_expression("author, deck_name")
        .into_paginator()
        .items()
        .send()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    
    items.into_iter().map(|x| -> Result<DeckMetadata, Box<dyn Error>> {
        DeckMetadata::try_from(&x)
    }).collect()
}


pub async fn list_decks_by_author_and_keypage(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    author: &str,
    keypage_id: &str
) -> Result<Vec<DeckMetadata>, Box<dyn Error>> {
    let items: Vec<HashMap<String, AttributeValue>> = client.query()
        .table_name(table_name)
        .index_name("gsi1")
        .key_condition_expression("keypage = :keypage and author = :author")
        .expression_attribute_names(":keypage", keypage_id)
        .expression_attribute_names(":author", author)
        .projection_expression("author, deck_name")
        .into_paginator()
        .items()
        .send()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    
    items.into_iter().map(|x| -> Result<DeckMetadata, Box<dyn Error>> {
        DeckMetadata::try_from(&x)
    }).collect()
}

pub async fn delete_deck(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    name: &str,
    author: &str
) -> Result<(), Box<dyn Error>> {
    Ok(client.delete_item()
        .table_name(table_name)
        .key("author", AttributeValue::S(author.to_string()))
        .key("deck_name", AttributeValue::S(name.to_string()))
        .send()
        .await
        .map(|_| ())?)
}

fn failed_attributevalue_cast(
    _: &AttributeValue
) -> String {
    "".to_string()
}

impl TryFrom<&HashMap<String, AttributeValue>> for Deck {
    type Error = Box<dyn Error>;

    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        Ok(Deck {
            name: value.get("deck_name").ok_or("no deck name")?.as_s().map_err(failed_attributevalue_cast)?.clone(),
            author: value.get("author").ok_or("no author")?.as_s().map_err(failed_attributevalue_cast)?.clone(),
            deck_data: serde_json::from_str(value.get("deck_data").ok_or("no deck data")?.as_s().map_err(failed_attributevalue_cast)?)?,
            tiph_deck: TiphDeck(
                value.get("tiph_deck").ok_or("no tiph deck")?.as_s().map_err(failed_attributevalue_cast)?.clone(),
                value.get("tiph_version").ok_or("no tiph version")?.as_n().map_err(failed_attributevalue_cast)?.parse()?,
            ),
            description: value.get("description").ok_or("no description")?.as_s().map_err(failed_attributevalue_cast)?.clone(),
        })
    }
}

impl TryFrom<Deck> for HashMap<String, AttributeValue> {
    type Error = Box<dyn Error>;

    fn try_from(value: Deck) -> Result<Self, Self::Error> {
        HashMap::<String, AttributeValue>::try_from(&value)
    }
}

impl TryFrom<&Deck> for HashMap<String, AttributeValue> {
    type Error = Box<dyn Error>;

    fn try_from(value: &Deck) -> Result<Self, Self::Error> {
        Ok(HashMap::from([
            ("author".to_string(), AttributeValue::S(value.author.clone())),
            ("deck_name".to_string(), AttributeValue::S(value.name.clone())),
            ("deck_data".to_string(), AttributeValue::S(serde_json::to_string(&value.deck_data)?)),
            ("tiph_deck".to_string(), AttributeValue::S(value.tiph_deck.0.clone())),
            ("description".to_string(), AttributeValue::S(value.description.clone())),
            ("keypage".to_string(), AttributeValue::S(value.deck_data.keypage_id.clone())),
            ("tiph_version".to_string(), AttributeValue::N(value.tiph_deck.1.to_string()))
        ]))
    }
}

impl TryFrom<&HashMap<String, AttributeValue>> for DeckMetadata {
    type Error = Box<dyn Error>;

    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        Ok(DeckMetadata {
            name: value.get("deck_name").ok_or("no deck name")?.as_s().map_err(failed_attributevalue_cast)?.clone(),
            author: value.get("author").ok_or("no author")?.as_s().map_err(failed_attributevalue_cast)?.clone(),
        })
    }
}