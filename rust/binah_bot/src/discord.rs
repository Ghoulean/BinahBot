use std::error::Error;

use crate::models::binahbot::DiscordSecrets;

pub async fn delete_interaction(_token: &str, _secrets: &DiscordSecrets) -> Result<(), Box<dyn Error + Send + Sync>> {
    todo!()
}