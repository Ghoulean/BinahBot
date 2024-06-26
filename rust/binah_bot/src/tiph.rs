use std::error::Error;

use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;

use crate::models::deck::DeckData;
use crate::models::deck::TiphDeck;

static BASE_TIPH_URL: &str = "https://tiphereth.zasz.su";

#[derive(Serialize, Deserialize, Debug)]
struct TiphDeckDecodeData {
    cards: Vec<i32>,
    keypage: Option<i32>,
    passives: Vec<i32>
}

#[derive(Serialize, Deserialize, Debug)]
struct TiphDeckDecode {
    data: TiphDeckDecodeData,
    status: String,
    val: String
}

#[derive(Serialize, Deserialize, Debug)]
struct TiphDeckEncode {
    status: String,
    url: String,
    val: String
}

pub async fn decode(
    client: &reqwest::Client,
    tiph: &TiphDeck
) -> Result<DeckData, Box<dyn Error + Send + Sync>> {
    let txt = client.post(format!("{}{}{}", BASE_TIPH_URL, "/internal/dvi_decode/?d=", tiph.0))
        .send()
        .await?
        .text()
        .await?;

    DeckData::try_from(&serde_json::from_str(&txt)?)
}

pub async fn encode(
    client: &reqwest::Client,
    deck_data: &DeckData
) -> Result<TiphDeck, Box<dyn Error + Send + Sync>> {
    let mut query_params = Vec::new();
    query_params.push(deck_data.keypage_id.as_ref().map(|x| ("k", x)));
    deck_data.passive_ids.iter().for_each(|x| {
        query_params.push(Some(("p", x)));        
    });
    let no_combat_page_binding = "-1".to_string();
    deck_data.combat_page_ids.iter().for_each(|x| {
        query_params.push(Some(("c", x.as_ref().unwrap_or(&no_combat_page_binding))));
    });

    let txt = client.post(
        Url::parse_with_params(
            &format!("{}{}", BASE_TIPH_URL, "/internal/dvi_encode/"),
            query_params.iter().flatten().collect::<Vec<_>>()
        )?
    ).send()
        .await?
        .text()
        .await?;

    Ok(TiphDeck(serde_json::from_str::<TiphDeckEncode>(&txt)?.val, 1))
}

impl TryFrom<&TiphDeckDecode> for DeckData {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(value: &TiphDeckDecode) -> Result<Self, Self::Error> {
        let mut resized_combat_page_ids: Vec<_> = value.data.cards.iter().map(|x| Some(x.to_string())).collect();
        resized_combat_page_ids.resize(9, None);
        let combat_page_array: [Option<String>; 9] = match resized_combat_page_ids.try_into() {
            Ok(x) => x,
            Err(_) => return Err("failed cast from TiphDeckDecode into DeckData".into())
        };

        Ok(DeckData {
            keypage_id: value.data.keypage.map(|x| x.to_string()),
            passive_ids: value.data.passives.iter().map(|x| x.to_string()).collect(),
            combat_page_ids: combat_page_array
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sanity_decode() {
        let client = reqwest::Client::new();
        let turbo_nikolai = TiphDeck("CS-iRmsieV9ddwW4-BA1C~n".to_string(), 1);
        let decode = decode(&client, &turbo_nikolai).await.expect("error with decode");
        assert_eq!(
            decode.combat_page_ids.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>(),
            vec![
                "608014", "608014", "608014",
                "608015", "608015", "608015",
                "608009", "608009", "608004"
            ]
        );
        assert_eq!(decode.keypage_id, Some("250023".to_string()));
        assert_eq!(
            decode.passive_ids,
            vec![
                "230018", "240118", "250025", "250151"
            ]
        );
    }

    #[tokio::test]
    async fn sanity_encode() {
        let client = reqwest::Client::new();
        let turbo_nikolai = DeckData {
            keypage_id: Some("250023".to_string()),
            passive_ids: vec!["230018", "240118", "250025", "250151"].iter().map(|x| x.to_string()).collect(),
            combat_page_ids: [
                "608014", "608014", "608014",
                "608015", "608015", "608015",
                "608009", "608009", "608004"
            ].iter().map(|x| Some(x.to_string())).collect::<Vec<_>>().try_into().unwrap()
        };
        let encode = encode(&client, &turbo_nikolai).await.expect("error with encode");

        assert_eq!(encode, TiphDeck("CS-iRmsieV9ddwW4-BA1C~n".to_string(), 1));
    }
}
