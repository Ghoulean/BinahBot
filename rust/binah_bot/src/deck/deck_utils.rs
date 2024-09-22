use std::error::Error;

use crate::models::deck::DeckData;
use crate::models::discord::DiscordUser;

static BASE_DISCORD_URL: &str = "https://discord.com/api/v10";

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

// todo: enum errors
pub fn validate_deck(deck_data: &DeckData) -> Result<(), &str> {
    for x in deck_data.combat_page_ids.as_ref().into_iter() {
        if x.is_none() {
            return Err("invalid_deck_error_message");
        }
    }
    if deck_data.keypage_id.is_none() {
        return Err("invalid_deck_error_message");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_err_on_invalid_deck() {
        let deck_data = DeckData {
            keypage_id: Some("12".to_string()),
            passive_ids: vec![],
            combat_page_ids: [None, None, None, None, None, None, None, None, None],
        };

        assert!(validate_deck(&deck_data).is_err());
    }
}
