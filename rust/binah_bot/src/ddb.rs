use std::error::Error;
use std::collections::HashMap;
use std::string::String;

use aws_sdk_dynamodb::types::AttributeValue;
use crate::models::deck::Deck;
use crate::models::deck::DeckMetadata;
use crate::models::deck::TiphDeck;

// todo: https://crates.io/crates/snafu

pub async fn get_deck(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    name: &str,
    author: &str
) -> Result<Deck, Box<dyn Error + Send + Sync>> {
    tracing::info!("Calling GetDeck with name={}; author={}", name, author);
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
    deck: &Deck,
    overwrite: bool
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!("Calling PutDeck with deck={:?}", deck);
    let condition_expression = if overwrite { None } else { Some("attribute_not_exists(author)".to_string()) };
    Ok(client.put_item()
        .table_name(table_name)
        .set_item(HashMap::<String, AttributeValue>::try_from(deck).ok())
        .set_condition_expression(condition_expression)
        .send()
        .await
        .map(|_| ())?)
}

pub async fn list_decks(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    author: Option<&str>,
    keypage_id: Option<&str>
) -> Result<Vec<DeckMetadata>, Box<dyn Error + Send + Sync>> {
    tracing::info!("Calling ListDeck with author={:?}, keypage_id={:?}", author, keypage_id);

    let mut query_builder = client.query();
    let mut key_condition_expression = Vec::new();

    if let Some(keypage_str) = keypage_id {
        query_builder = query_builder
            .index_name("gsi1")
            .expression_attribute_values(
                ":keypage",
                AttributeValue::S(keypage_str.to_string())
            );
        key_condition_expression.push("keypage = :keypage");
    }
    if let Some(author_str) = author {
        query_builder = query_builder.expression_attribute_values(
            ":author",
            AttributeValue::S(author_str.to_string())
        );
        key_condition_expression.push("author = :author");
    }

    let items: Vec<HashMap<String, AttributeValue>> = if keypage_id.is_none() && author.is_none() {
        client.scan()
            .table_name(table_name)
            .projection_expression("author, author_name, deck_name")
            .into_paginator()
            .items()
            .send()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
    } else {
        query_builder
            .table_name(table_name)
            .key_condition_expression(key_condition_expression.join(" and "))
            .projection_expression("author, author_name, deck_name")
            .into_paginator()
            .items()
            .send()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
    };

    items.into_iter().map(|x| -> Result<DeckMetadata, Box<dyn Error + Send + Sync>> {
        DeckMetadata::try_from(&x)
    }).collect()
}

pub async fn delete_deck(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    name: &str,
    author: &str
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!("Calling DeleteDeck with name={}; author={}", name, author);
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
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let tiph_deck = value.get("tiph_deck").and_then(|x| x.as_s().ok());
        let tiph_version = value.get("tiph_version").and_then(|x| x.as_n().ok().map(|x| x.parse::<i32>().ok())).flatten();
        let description_av = value.get("description").ok_or("no description")?;
        let description = if description_av.is_s() {
            Some(description_av.as_s().unwrap())
        } else {
            None
        };

        Ok(Deck {
            name: value.get("deck_name").ok_or("no deck name")?.as_s().map_err(failed_attributevalue_cast)?.clone(),
            author_id: value.get("author").ok_or("no author_id")?.as_s().map_err(failed_attributevalue_cast)?.clone(),
            author_name: value.get("author_name").ok_or("no author_name")?.as_s().map_err(failed_attributevalue_cast)?.clone(),
            deck_data: serde_json::from_str(value.get("deck_data").ok_or("no deck data")?.as_s().map_err(failed_attributevalue_cast)?)?,
            tiph_deck: match (tiph_deck, tiph_version) {
                (Some(a), Some(b)) => Some(TiphDeck(a.clone(), b)),
                _ => None
            },
            description: description.cloned(),
        })
    }
}

impl TryFrom<Deck> for HashMap<String, AttributeValue> {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: Deck) -> Result<Self, Self::Error> {
        HashMap::<String, AttributeValue>::try_from(&value)
    }
}

impl TryFrom<&Deck> for HashMap<String, AttributeValue> {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: &Deck) -> Result<Self, Self::Error> {
        let mut hm = HashMap::from([
            ("author".to_string(), AttributeValue::S(value.author_id.clone())),
            ("author_name".to_string(), AttributeValue::S(value.author_name.clone())),
            ("deck_name".to_string(), AttributeValue::S(value.name.clone())),
            ("deck_data".to_string(), AttributeValue::S(serde_json::to_string(&value.deck_data)?)),
            ("description".to_string(), value.description.as_ref().map(|x| AttributeValue::S(x.clone())).unwrap_or(AttributeValue::Null(true))),
            ("keypage".to_string(), value.deck_data.keypage_id.as_ref().map(|x| AttributeValue::S(x.clone())).unwrap_or(AttributeValue::Null(true)) )
        ]);

        if let Some(tiph) = value.tiph_deck.as_ref() {
            hm.insert("tiph_deck".to_string(), AttributeValue::S(tiph.0.clone()));
            hm.insert("tiph_version".to_string(), AttributeValue::N(tiph.1.to_string()));
        }

        Ok(hm)
    }
}

impl TryFrom<&HashMap<String, AttributeValue>> for DeckMetadata {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        Ok(DeckMetadata {
            name: value.get("deck_name").ok_or("no deck name")?.as_s().map_err(failed_attributevalue_cast)?.clone(),
            author_id: value.get("author").ok_or("no author id")?.as_s().map_err(failed_attributevalue_cast)?.clone(),
            author_name: value.get("author_name").ok_or("no author name")?.as_s().map_err(failed_attributevalue_cast)?.clone(),
        })
    }
}