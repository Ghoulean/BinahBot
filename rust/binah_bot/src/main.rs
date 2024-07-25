mod about_command;
mod ddb;
mod deck;
mod discord;
mod lor;
mod models;
mod router;
mod secrets;
mod thumbnail;
mod tiph;
mod utils;

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use hex::FromHex;
use http::HeaderMap;
use lambda_http::{run, service_fn, tracing, Body, Request, Response};
use models::binahbot::BinahBotEnvironment;
use models::binahbot::DiscordSecrets;
use models::binahbot::Emojis;
use models::discord::DiscordInteraction;
use models::discord::DiscordInteractionValidationData;
use router::get_response;
use secrets::get_discord_secrets;
use std::env;
use std::error::Error;
use std::ops::Deref;

fluent_templates::static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

static TIMESTAMP_HEADER: &str = "x-signature-timestamp";
static SIGNATURE_HEADER: &str = "x-signature-ed25519";

async fn function_handler(
    event: Request,
    binahbot_env: &BinahBotEnvironment,
) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    tracing::info!("Rust function invoked with event={:?}", event);

    let request_body: String = String::from_utf8(event.body().deref().to_vec()).unwrap();
    let request_headers: &HeaderMap = event.headers();

    let event_metadata = DiscordInteractionValidationData {
        timestamp: get_header(request_headers, TIMESTAMP_HEADER),
        signature: get_header(request_headers, SIGNATURE_HEADER),
        json_body: request_body.clone(),
    };

    tracing::info!("request_body={:?}", request_body);

    // header validation must complete first due to `get_response` causing potential side effects
    let validate_headers_result = validate_headers(&binahbot_env.discord_secrets, &event_metadata);
    if validate_headers_result.is_err() {
        tracing::info!("Failed header validation");

        let resp = Response::builder()
            .status(401)
            .header("content-type", "application/json")
            .body(Body::Empty)?;
        return Ok(resp);
    }

    let discord_interaction: DiscordInteraction = serde_json::from_str(request_body.as_str())?;
    tracing::info!("discord_interaction={:?}", discord_interaction);

    let response = get_response(&discord_interaction, binahbot_env).await?;
    tracing::info!("Returning response={:?}", response);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::Text(serde_json::to_string(&response).unwrap()))?;

    tracing::debug!("Rust function finished invocation");
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_current_span(false)
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .init();

    let config = aws_config::load_from_env().await;
    let asm = aws_sdk_secretsmanager::Client::new(&config);
    let ddb = aws_sdk_dynamodb::Client::new(&config);
    let lambda = aws_sdk_lambda::Client::new(&config);
    let http = reqwest::Client::new();
    let discord_secrets = get_discord_secrets(&asm, &env::var("SECRETS_ID").expect("no SECRETS_ID")).await;

    let binahbot_env = BinahBotEnvironment {
        discord_secrets,
        discord_client_id: env::var("CLIENT_ID").expect("no CLIENT_ID"),
        s3_bucket_name: env::var("S3_BUCKET_NAME").expect("no S3_BUCKET_NAME"),
        emojis: Emojis {
            slash_emoji_id: env::var("SLASH_EMOJI_ID").ok(),
            pierce_emoji_id: env::var("PIERCE_EMOJI_ID").ok(),
            blunt_emoji_id: env::var("BLUNT_EMOJI_ID").ok(),
            block_emoji_id: env::var("BLOCK_EMOJI_ID").ok(),
            evade_emoji_id: env::var("EVADE_EMOJI_ID").ok(),
            c_slash_emoji_id: env::var("C_SLASH_EMOJI_ID").ok(),
            c_pierce_emoji_id: env::var("C_PIERCE_EMOJI_ID").ok(),
            c_blunt_emoji_id: env::var("C_BLUNT_EMOJI_ID").ok(),
            c_block_emoji_id: env::var("C_BLOCK_EMOJI_ID").ok(),
            c_evade_emoji_id: env::var("C_EVADE_EMOJI_ID").ok(),
        },
        locales: &LOCALES,
        ddb_table_name: env::var("DECK_REPOSITORY_NAME").expect("no DECK_REPOSITORY_NAME"),
        ddb_interaction_ttl_table_name: env::var("INTERACTION_TTL_NAME").expect("no INTERACTION_TTL_NAME"),
        thumbnail_lambda_name: env::var("THUMBNAIL_LAMBDA_ARN").expect("no THUMBNAIL_LAMBDA_ARN"),
        ddb_client: Some(ddb),
        lambda_client: Some(lambda),
        reqwest_client: Some(http)
    };
    let binahbot_env_ref = &binahbot_env;

    tracing::debug!("Rust function setup complete");

    run(service_fn(move |event: Request| {
        function_handler(event, binahbot_env_ref)
    }))
    .await
}

fn get_header(header_map: &HeaderMap, header_key: &str) -> String {
    header_map
        .get(header_key)
        .and_then(|x| x.to_str().ok())
        .unwrap_or("")
        .to_owned()
}

fn validate_headers(
    discord_secrets: &DiscordSecrets,
    metadata: &DiscordInteractionValidationData,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let public_key_bytes = <[u8; 32]>::from_hex(&discord_secrets.public_key)?;
    let public_key = VerifyingKey::from_bytes(&public_key_bytes)?;

    let signature_bytes = <[u8; 64]>::from_hex(&metadata.signature)?;
    let concatenated = format!("{}{}", metadata.timestamp, metadata.json_body);

    Ok(public_key.verify(
        concatenated.as_bytes(),
        &Signature::from_slice(&signature_bytes)?,
    )?)
}

#[cfg(test)]
pub mod test_utils {
    use crate::models::binahbot::BinahBotEnvironment;
    use crate::models::binahbot::DiscordSecrets;
    use crate::models::binahbot::Emojis;
    use crate::LOCALES;

    pub fn build_mocked_binahbot_env() -> BinahBotEnvironment {
        BinahBotEnvironment {
            discord_secrets: DiscordSecrets {
                application_id: "app_id".to_string(),
                auth_token: "auth_token".to_string(),
                public_key: "pub_key".to_string(),
                bot_token: "bot_token".to_string()
            },
            discord_client_id: "id".to_string(),
            s3_bucket_name: "bucket_name".to_string(),
            emojis: Emojis {
                slash_emoji_id: None,
                pierce_emoji_id: None,
                blunt_emoji_id: None,
                block_emoji_id: None,
                evade_emoji_id: None,
                c_slash_emoji_id: None,
                c_pierce_emoji_id: None,
                c_blunt_emoji_id: None,
                c_block_emoji_id: None,
                c_evade_emoji_id: None,
            },
            locales: &LOCALES,
            ddb_table_name: "table_name".to_string(),
            ddb_interaction_ttl_table_name: "interaction_ttl_table_name".to_string(),
            thumbnail_lambda_name: "thumb_lambda_name".to_string(),
            ddb_client: None,
            lambda_client: None,
            reqwest_client: None,
        }
    }
}
