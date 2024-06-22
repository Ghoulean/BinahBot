use crate::DiscordSecrets;

pub async fn get_discord_secrets(
    client: &aws_sdk_secretsmanager::Client,
    secret_id: &String,
) -> DiscordSecrets {
    serde_json::from_str(
        client
            .get_secret_value()
            .secret_id(secret_id)
            .send()
            .await
            .unwrap()
            .secret_string()
            .unwrap(),
    )
    .unwrap()
}
