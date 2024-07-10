use std::error::Error;

use fluent_templates::Loader;
use unic_langid::LanguageIdentifier;

use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::discord::AllowedMentions;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordMessageFlag;
use crate::models::discord::DiscordUser;
use crate::models::discord::MessageResponse;

static BASE_DISCORD_URL: &str = "https://discord.com/api/v10";

pub fn build_generic_error_message_response(lang_id: &LanguageIdentifier, env: &BinahBotEnvironment) -> MessageResponse {
    MessageResponse {
        r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
        data: Some(DiscordInteractionResponseMessage {
            allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
            content: None,
            embeds: Some(vec![
                DiscordEmbed {
                    title: None,
                    description: Some(env.locales.lookup(&lang_id, "generic_error_message")),
                    color: Some(DiscordEmbedColors::Default as i32),
                    image: None,
                    footer: None,
                    author: None,
                    url: None,
                    fields: None
                }
            ]),
            flags: Some(DiscordMessageFlag::EphemeralMessage as i32)
        })
    }
}

pub async fn get_user(
    client: &reqwest::Client,
    bot_auth_token: &str,
    user_id: &str
) -> Result<DiscordUser, Box<dyn Error + Send + Sync>> {
    let response = client.get(&format!("{}{}{}", BASE_DISCORD_URL, "/users/", user_id))
        .header("Authorization", format!("Bot {}", bot_auth_token))
        .send()
        .await?
        .text()
        .await?;

    let user = serde_json::from_str(&response)?;

    Ok(user)
}