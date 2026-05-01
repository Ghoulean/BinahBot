use std::error::Error;

use crate::models::deck::DeckData;
use crate::models::discord::DiscordUser;

static BASE_DISCORD_URL: &str = "https://discord.com/api/v10";

pub struct DeckKey(pub String, pub String);

pub async fn get_user(
    client: &reqwest::Client,
    bot_auth_token: &str,
    user_id: &str,
) -> Result<DiscordUser, Box<dyn Error + Send + Sync>> {
    let response = client
        .get(&format!("{}{}{}", BASE_DISCORD_URL, "/users/", user_id))
        .header("Authorization", format!("Bot {}", bot_auth_token))
        .send()
        .await?
        .text()
        .await?;

    let user = serde_json::from_str(&response)?;

    Ok(user)
}

pub enum DeckValidationError {
    MissingCombatPages,
    MissingKeypage,
}

impl DeckValidationError {
    pub fn as_error_key(&self) -> &'static str {
        match self {
            DeckValidationError::MissingCombatPages => "missing_combat_pages_error_message",
            DeckValidationError::MissingKeypage => "missing_keypage_error_message",
        }
    }
}

pub fn validate_deck(deck_data: &DeckData) -> Result<(), DeckValidationError> {
    if !deck_data.combat_page_ids.iter().all(|x| x.is_some()) {
        return Err(DeckValidationError::MissingCombatPages);
    }
    if deck_data.keypage_id.is_none() {
        return Err(DeckValidationError::MissingKeypage);
    }
    Ok(())
}

pub fn parse_deck_name_option(name_option: &str) -> Result<DeckKey, ()> {
    let mut split: Vec<_> = name_option.split('#').collect();
    if split.len() >= 2 {
        let split2 = split.split_off(1);
        Ok(DeckKey(
            split.first().unwrap().to_string(),
            split2.join("#"),
        ))
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full_combat_pages() -> [Option<String>; 9] {
        std::array::from_fn(|i| Some(i.to_string()))
    }

    #[test]
    fn should_pass_on_valid_deck() {
        let deck_data = DeckData {
            keypage_id: Some("12".to_string()),
            passive_ids: vec![],
            combat_page_ids: full_combat_pages(),
        };
        assert!(validate_deck(&deck_data).is_ok());
    }

    #[test]
    fn should_err_on_missing_combat_pages() {
        let deck_data = DeckData {
            keypage_id: Some("12".to_string()),
            passive_ids: vec![],
            combat_page_ids: [None, None, None, None, None, None, None, None, None],
        };
        let err = validate_deck(&deck_data).unwrap_err();
        assert_eq!(err.as_error_key(), "missing_combat_pages_error_message");
    }

    #[test]
    fn should_err_on_partially_filled_combat_pages() {
        let mut pages = full_combat_pages();
        pages[4] = None;
        let deck_data = DeckData {
            keypage_id: Some("12".to_string()),
            passive_ids: vec![],
            combat_page_ids: pages,
        };
        let err = validate_deck(&deck_data).unwrap_err();
        assert_eq!(err.as_error_key(), "missing_combat_pages_error_message");
    }

    #[test]
    fn should_err_on_missing_keypage() {
        let deck_data = DeckData {
            keypage_id: None,
            passive_ids: vec![],
            combat_page_ids: full_combat_pages(),
        };
        let err = validate_deck(&deck_data).unwrap_err();
        assert_eq!(err.as_error_key(), "missing_keypage_error_message");
    }

    #[test]
    fn should_err_on_missing_combat_pages_when_both_missing() {
        let deck_data = DeckData {
            keypage_id: None,
            passive_ids: vec![],
            combat_page_ids: [None, None, None, None, None, None, None, None, None],
        };
        let err = validate_deck(&deck_data).unwrap_err();
        assert_eq!(err.as_error_key(), "missing_combat_pages_error_message");
    }
}
