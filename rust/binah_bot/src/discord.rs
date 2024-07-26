use std::error::Error;

use crate::models::binahbot::DiscordSecrets;

pub async fn delete_interaction(
    client: &reqwest::Client,
    secrets: &DiscordSecrets,
    token: &str
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let url = format!("https://discord.com/api/v10/webhooks/{0}/{1}/messages/@original", secrets.application_id, token);
    let params = [
        ("Authorization", format!("Bot {}", secrets.bot_token))
    ];

    let url = reqwest::Url::parse_with_params(&url, &params)?;
    client.delete(url).send().await?;

    Ok(())
}