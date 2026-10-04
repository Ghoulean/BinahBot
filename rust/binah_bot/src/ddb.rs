use std::collections::HashMap;
use std::error::Error;
use std::string::String;

use crate::models::binahbot::InteractionTtl;
use crate::models::deck::Deck;
use crate::models::deck::DeckMetadata;
use crate::models::deck::DeckSearchCacheEntry;
use crate::models::deck::DeckSearchParams;
use crate::models::deck::ListDecksInteraction;
use crate::models::deck::TiphDeck;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::types::PutRequest;
use aws_sdk_dynamodb::types::WriteRequest;

// todo: https://crates.io/crates/snafu
pub async fn get_deck(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    name: &str,
    author: &str,
) -> Result<Deck, Box<dyn Error + Send + Sync>> {
    tracing::info!("Calling GetDeck with name={}; author={}", name, author);
    let binding = client
        .get_item()
        .table_name(table_name)
        .key("author", AttributeValue::S(author.to_string()))
        .key("deck_name", AttributeValue::S(name.to_string()))
        .send()
        .await?;

    let result = binding.item().ok_or("failed ddb call in get_deck")?;

    Deck::try_from(result)
}

pub async fn put_deck(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    deck: &Deck,
    overwrite: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!("Calling PutDeck with deck={:?}", deck);
    let condition_expression = if overwrite {
        None
    } else {
        Some("attribute_not_exists(author)".to_string())
    };
    Ok(client
        .put_item()
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
    keypage_id: Option<&str>,
) -> Result<Vec<DeckMetadata>, Box<dyn Error + Send + Sync>> {
    tracing::info!(
        "Calling ListDeck with author={:?}, keypage_id={:?}",
        author,
        keypage_id
    );

    let mut decks = Vec::new();
    let mut exclusive_start_key = None;

    loop {
        let (dynamodb_page, last_evaluated_key) =
            list_decks_dynamodb_page(client, table_name, author, keypage_id, exclusive_start_key)
                .await?;
        decks.extend(dynamodb_page);

        match last_evaluated_key {
            Some(x) => exclusive_start_key = Some(x),
            None => break,
        }
    }

    Ok(decks)
}

async fn list_decks_dynamodb_page(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    author: Option<&str>,
    keypage_id: Option<&str>,
    exclusive_start_key: Option<HashMap<String, AttributeValue>>,
) -> Result<
    (Vec<DeckMetadata>, Option<HashMap<String, AttributeValue>>),
    Box<dyn Error + Send + Sync>,
> {
    let (items, last_evaluated_key) = if keypage_id.is_none() && author.is_none() {
        let output = client
            .scan()
            .table_name(table_name)
            .projection_expression("author, author_name, deck_name")
            .set_exclusive_start_key(exclusive_start_key)
            .send()
            .await?;
        (output.items.unwrap_or_default(), output.last_evaluated_key)
    } else {
        let mut query_builder = client.query();
        let mut key_condition_expression = Vec::new();

        if let Some(keypage_str) = keypage_id {
            query_builder = query_builder
                .index_name("gsi1")
                .expression_attribute_values(
                    ":keypage",
                    AttributeValue::S(keypage_str.to_string()),
                );
            key_condition_expression.push("keypage = :keypage");
        }
        if let Some(author_str) = author {
            query_builder = query_builder
                .expression_attribute_values(":author", AttributeValue::S(author_str.to_string()));
            key_condition_expression.push("author = :author");
        }

        let output = query_builder
            .table_name(table_name)
            .key_condition_expression(key_condition_expression.join(" and "))
            .projection_expression("author, author_name, deck_name")
            .set_exclusive_start_key(exclusive_start_key)
            .send()
            .await?;
        (output.items.unwrap_or_default(), output.last_evaluated_key)
    };

    let decks = items
        .iter()
        .map(DeckMetadata::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((decks, last_evaluated_key))
}

pub async fn get_deck_search_cache(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    search_params: &DeckSearchParams,
    cache_page: usize,
) -> Result<DeckSearchCacheEntry, Box<dyn Error + Send + Sync>> {
    let key = deck_search_cache_key(search_params, cache_page);
    tracing::info!("Calling GetDeckSearchCache with key={:?}", key);

    let binding = client
        .get_item()
        .table_name(table_name)
        .key("interaction_id", AttributeValue::S(key))
        .send()
        .await?;
    let item = binding.item().ok_or("deck search cache miss")?;

    DeckSearchCacheEntry::try_from(item)
}

pub async fn put_deck_search_cache_pages(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    search_params: &DeckSearchParams,
    cache_pages: &[(usize, DeckSearchCacheEntry)],
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!(
        "Calling PutDeckSearchCachePages with search_params={:?}, cache_pages={:?}",
        search_params,
        cache_pages
            .iter()
            .map(|(cache_page, _)| cache_page)
            .collect::<Vec<_>>()
    );

    let write_requests = cache_pages
        .iter()
        .map(
            |(cache_page, entry)| -> Result<WriteRequest, Box<dyn Error + Send + Sync>> {
                let mut item = HashMap::<String, AttributeValue>::from(entry);
                item.insert(
                    "interaction_id".to_string(),
                    AttributeValue::S(deck_search_cache_key(search_params, *cache_page)),
                );
                Ok(WriteRequest::builder()
                    .put_request(PutRequest::builder().set_item(Some(item)).build()?)
                    .build())
            },
        )
        .collect::<Result<Vec<_>, _>>()?;

    let output = client
        .batch_write_item()
        .request_items(table_name, write_requests)
        .send()
        .await?;

    if output
        .unprocessed_items()
        .is_some_and(|x| x.values().any(|requests| !requests.is_empty()))
    {
        return Err("some deck search cache pages were not written".into());
    }

    Ok(())
}

pub async fn put_listdecks_interaction(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    listdecks_interaction: &ListDecksInteraction,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!(
        "Calling PutListDecksInteraction with listdecks_interaction={:?}",
        listdecks_interaction
    );
    Ok(client
        .put_item()
        .table_name(table_name)
        .set_item(Some(HashMap::<String, AttributeValue>::from(
            listdecks_interaction,
        )))
        .send()
        .await
        .map(|_| ())?)
}

pub async fn get_listdecks_interaction(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    interaction_id: &str,
) -> Result<ListDecksInteraction, Box<dyn Error + Send + Sync>> {
    tracing::info!(
        "Calling GetListDecksInteraction with interaction_id={:?}",
        interaction_id
    );

    let binding = client
        .get_item()
        .table_name(table_name)
        .key(
            "interaction_id",
            AttributeValue::S(listdecks_interaction_key(interaction_id)),
        )
        .send()
        .await?;
    let item = binding
        .item()
        .ok_or("could not get listdecks interaction")?;

    ListDecksInteraction::try_from(item)
}

pub async fn delete_deck(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    name: &str,
    author: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!("Calling DeleteDeck with name={}; author={}", name, author);
    Ok(client
        .delete_item()
        .table_name(table_name)
        .key("author", AttributeValue::S(author.to_string()))
        .key("deck_name", AttributeValue::S(name.to_string()))
        .send()
        .await
        .map(|_| ())?)
}

pub async fn put_interaction_token(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    interaction_ttl: &InteractionTtl,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!(
        "Calling PutInteractionToken with interaction_ttl={:?}",
        interaction_ttl
    );
    Ok(client
        .put_item()
        .table_name(table_name)
        .set_item(HashMap::<String, AttributeValue>::try_from(interaction_ttl).ok())
        .send()
        .await
        .map(|_| ())?)
}

pub async fn get_interaction_token(
    client: &aws_sdk_dynamodb::Client,
    table_name: &str,
    interaction_id: &str,
) -> Result<InteractionTtl, Box<dyn Error + Send + Sync>> {
    tracing::info!(
        "Calling GetInteractionToken with interaction_id={:?}",
        interaction_id
    );

    let binding = client
        .get_item()
        .table_name(table_name)
        .key(
            "interaction_id",
            AttributeValue::S(interaction_id.to_string()),
        )
        .send()
        .await?;
    let interaction_ttl = binding.item().ok_or("could not get token")?;

    InteractionTtl::try_from(interaction_ttl)
}

fn failed_attributevalue_cast(_: &AttributeValue) -> String {
    "".to_string()
}

const LISTDECKS_INTERACTION_PREFIX: &str = "listdeck#";
const DECK_SEARCH_CACHE_PREFIX: &str = "decksearch#";

fn listdecks_interaction_key(interaction_id: &str) -> String {
    format!("{}{}", LISTDECKS_INTERACTION_PREFIX, interaction_id)
}

// decksearch#<query>#<author>#<keypage>#<cache page>
// query may contain `#`
fn deck_search_cache_key(search_params: &DeckSearchParams, cache_page: usize) -> String {
    format!(
        "{}{}#{}#{}#{}",
        DECK_SEARCH_CACHE_PREFIX,
        search_params.query,
        search_params.author.as_deref().unwrap_or(""),
        search_params.keypage.as_deref().unwrap_or(""),
        cache_page
    )
}

fn optional_string_attribute(value: &Option<String>) -> AttributeValue {
    value
        .as_ref()
        .map(|x| AttributeValue::S(x.clone()))
        .unwrap_or(AttributeValue::Null(true))
}

impl From<&ListDecksInteraction> for HashMap<String, AttributeValue> {
    fn from(value: &ListDecksInteraction) -> Self {
        HashMap::from([
            (
                "interaction_id".to_string(),
                AttributeValue::S(listdecks_interaction_key(&value.interaction_id)),
            ),
            (
                "query".to_string(),
                AttributeValue::S(value.search_params.query.clone()),
            ),
            (
                "author".to_string(),
                optional_string_attribute(&value.search_params.author),
            ),
            (
                "keypage".to_string(),
                optional_string_attribute(&value.search_params.keypage),
            ),
            ("ttl".to_string(), AttributeValue::N(value.ttl.to_string())),
        ])
    }
}

impl TryFrom<&HashMap<String, AttributeValue>> for ListDecksInteraction {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let interaction_id = value
            .get("interaction_id")
            .ok_or("no interaction_id")?
            .as_s()
            .map_err(failed_attributevalue_cast)?;

        Ok(ListDecksInteraction {
            interaction_id: interaction_id
                .strip_prefix(LISTDECKS_INTERACTION_PREFIX)
                .unwrap_or(interaction_id)
                .to_string(),
            search_params: DeckSearchParams {
                query: value
                    .get("query")
                    .ok_or("no query")?
                    .as_s()
                    .map_err(failed_attributevalue_cast)?
                    .clone(),
                author: value.get("author").and_then(|x| x.as_s().ok()).cloned(),
                keypage: value.get("keypage").and_then(|x| x.as_s().ok()).cloned(),
            },
            ttl: value
                .get("ttl")
                .ok_or("no ttl")?
                .as_n()
                .map_err(failed_attributevalue_cast)?
                .parse()?,
        })
    }
}

impl From<&DeckSearchCacheEntry> for HashMap<String, AttributeValue> {
    fn from(value: &DeckSearchCacheEntry) -> Self {
        HashMap::from([
            (
                "decks".to_string(),
                AttributeValue::L(
                    value
                        .decks
                        .iter()
                        .map(|x| AttributeValue::M(HashMap::from(x)))
                        .collect(),
                ),
            ),
            (
                "result_count".to_string(),
                AttributeValue::N(value.result_count.to_string()),
            ),
            ("ttl".to_string(), AttributeValue::N(value.ttl.to_string())),
        ])
    }
}

impl TryFrom<&HashMap<String, AttributeValue>> for DeckSearchCacheEntry {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        Ok(DeckSearchCacheEntry {
            decks: value
                .get("decks")
                .ok_or("no decks")?
                .as_l()
                .map_err(failed_attributevalue_cast)?
                .iter()
                .map(|x| DeckMetadata::try_from(x.as_m().map_err(failed_attributevalue_cast)?))
                .collect::<Result<Vec<_>, _>>()?,
            result_count: value
                .get("result_count")
                .ok_or("no result_count")?
                .as_n()
                .map_err(failed_attributevalue_cast)?
                .parse()?,
            ttl: value
                .get("ttl")
                .ok_or("no ttl")?
                .as_n()
                .map_err(failed_attributevalue_cast)?
                .parse()?,
        })
    }
}

impl From<&DeckMetadata> for HashMap<String, AttributeValue> {
    fn from(value: &DeckMetadata) -> Self {
        HashMap::from([
            (
                "deck_name".to_string(),
                AttributeValue::S(value.name.clone()),
            ),
            (
                "author".to_string(),
                AttributeValue::S(value.author_id.clone()),
            ),
            (
                "author_name".to_string(),
                AttributeValue::S(value.author_name.clone()),
            ),
        ])
    }
}

impl TryFrom<&HashMap<String, AttributeValue>> for Deck {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let tiph_deck = value.get("tiph_deck").and_then(|x| x.as_s().ok());
        let tiph_version = value
            .get("tiph_version")
            .and_then(|x| x.as_n().ok().map(|x| x.parse::<i32>().ok()))
            .flatten();
        let description_av = value.get("description").ok_or("no description")?;
        let description = if description_av.is_s() {
            Some(description_av.as_s().unwrap())
        } else {
            None
        };

        Ok(Deck {
            name: value
                .get("deck_name")
                .ok_or("no deck name")?
                .as_s()
                .map_err(failed_attributevalue_cast)?
                .clone(),
            author_id: value
                .get("author")
                .ok_or("no author_id")?
                .as_s()
                .map_err(failed_attributevalue_cast)?
                .clone(),
            author_name: value
                .get("author_name")
                .ok_or("no author_name")?
                .as_s()
                .map_err(failed_attributevalue_cast)?
                .clone(),
            deck_data: serde_json::from_str(
                value
                    .get("deck_data")
                    .ok_or("no deck data")?
                    .as_s()
                    .map_err(failed_attributevalue_cast)?,
            )?,
            tiph_deck: match (tiph_deck, tiph_version) {
                (Some(a), Some(b)) => Some(TiphDeck(a.clone(), b)),
                _ => None,
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
            (
                "author".to_string(),
                AttributeValue::S(value.author_id.clone()),
            ),
            (
                "author_name".to_string(),
                AttributeValue::S(value.author_name.clone()),
            ),
            (
                "deck_name".to_string(),
                AttributeValue::S(value.name.clone()),
            ),
            (
                "deck_data".to_string(),
                AttributeValue::S(serde_json::to_string(&value.deck_data)?),
            ),
            (
                "description".to_string(),
                value
                    .description
                    .as_ref()
                    .map(|x| AttributeValue::S(x.clone()))
                    .unwrap_or(AttributeValue::Null(true)),
            ),
            (
                "keypage".to_string(),
                value
                    .deck_data
                    .keypage_id
                    .as_ref()
                    .map(|x| AttributeValue::S(x.clone()))
                    .unwrap_or(AttributeValue::Null(true)),
            ),
        ]);

        if let Some(tiph) = value.tiph_deck.as_ref() {
            hm.insert("tiph_deck".to_string(), AttributeValue::S(tiph.0.clone()));
            hm.insert(
                "tiph_version".to_string(),
                AttributeValue::N(tiph.1.to_string()),
            );
        }

        Ok(hm)
    }
}

impl TryFrom<&HashMap<String, AttributeValue>> for DeckMetadata {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        Ok(DeckMetadata {
            name: value
                .get("deck_name")
                .ok_or("no deck name")?
                .as_s()
                .map_err(failed_attributevalue_cast)?
                .clone(),
            author_id: value
                .get("author")
                .ok_or("no author id")?
                .as_s()
                .map_err(failed_attributevalue_cast)?
                .clone(),
            author_name: value
                .get("author_name")
                .ok_or("no author name")?
                .as_s()
                .map_err(failed_attributevalue_cast)?
                .clone(),
        })
    }
}

impl TryFrom<&InteractionTtl> for HashMap<String, AttributeValue> {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: &InteractionTtl) -> Result<Self, Self::Error> {
        Ok(HashMap::from([
            (
                "interaction_id".to_string(),
                AttributeValue::S(value.interaction_id.clone()),
            ),
            ("token".to_string(), AttributeValue::S(value.token.clone())),
            ("ttl".to_string(), AttributeValue::N(value.ttl.to_string())),
            (
                "original_user_id".to_string(),
                AttributeValue::S(value.original_user_id.clone()),
            ),
        ]))
    }
}

impl TryFrom<&HashMap<String, AttributeValue>> for InteractionTtl {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        Ok(InteractionTtl {
            interaction_id: value
                .get("interaction_id")
                .ok_or("no interaction_id")?
                .as_s()
                .map_err(failed_attributevalue_cast)?
                .clone(),
            token: value
                .get("token")
                .ok_or("no token")?
                .as_s()
                .map_err(failed_attributevalue_cast)?
                .clone(),
            ttl: value
                .get("ttl")
                .ok_or("no ttl")?
                .as_n()
                .map_err(failed_attributevalue_cast)?
                .parse()?,
            original_user_id: value
                .get("original_user_id")
                .ok_or("no original_user_id")?
                .as_s()
                .map_err(failed_attributevalue_cast)?
                .clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deck_metadata(name: &str) -> DeckMetadata {
        DeckMetadata {
            name: name.to_string(),
            author_id: "456".to_string(),
            author_name: "gh".to_string(),
        }
    }

    #[test]
    fn listdecks_interaction_round_trips() {
        let listdecks_interaction = ListDecksInteraction {
            interaction_id: "123".to_string(),
            search_params: DeckSearchParams {
                query: "turbo".to_string(),
                author: Some("456".to_string()),
                keypage: None,
            },
            ttl: 1000,
        };

        let item = HashMap::<String, AttributeValue>::from(&listdecks_interaction);
        assert_eq!(
            Some(&AttributeValue::S("listdeck#123".to_string())),
            item.get("interaction_id")
        );
        assert_eq!(
            listdecks_interaction,
            ListDecksInteraction::try_from(&item).unwrap()
        );
    }

    #[test]
    fn deck_search_cache_entry_round_trips() {
        let entry = DeckSearchCacheEntry {
            decks: vec![
                deck_metadata("Turbo Nikolai"),
                deck_metadata("Crimson Claws"),
            ],
            result_count: 12,
            ttl: 1000,
        };

        let item = HashMap::<String, AttributeValue>::from(&entry);
        assert_eq!(entry, DeckSearchCacheEntry::try_from(&item).unwrap());
    }

    #[test]
    fn deck_search_cache_key_format() {
        let search_params = DeckSearchParams {
            query: "a#b".to_string(),
            author: None,
            keypage: Some("250022".to_string()),
        };
        assert_eq!(
            "decksearch#a#b##250022#3",
            deck_search_cache_key(&search_params, 3)
        );
    }
}
