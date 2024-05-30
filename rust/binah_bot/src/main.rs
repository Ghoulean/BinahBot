mod lor_autocomplete;
mod lor_command;
mod models;
mod router;
mod secrets_accessor;

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use hex::FromHex;
use http::HeaderMap;
use lambda_http::{run, service_fn, tracing, Body, Request, Response};
use router::get_response;
use secrets_accessor::get_discord_secrets;
use std::env;
use std::ops::Deref;

use ruina_common::game_objects::common::Chapter;

use crate::models::binahbot::DiscordSecrets;
use crate::models::binahbot::Emojis;
use crate::models::discord::DiscordInteractionMetadata;
use models::{binahbot::BinahBotEnvironment, discord::DiscordInteraction};

include!(concat!(env!("OUT_DIR"), "/out.rs"));

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
) -> Result<Response<Body>, lambda_http::Error> {
    tracing::debug!("Rust function invoked");

    let request_body: String = String::from_utf8(event.body().deref().to_vec()).unwrap();
    let request_headers: &HeaderMap = event.headers();

    let event_metadata = DiscordInteractionMetadata {
        timestamp: get_header(request_headers, TIMESTAMP_HEADER),
        signature: get_header(request_headers, SIGNATURE_HEADER),
        json_body: request_body.clone(),
    };

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
    let response = get_response(&discord_interaction, binahbot_env);

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
    let discord_secrets = get_discord_secrets(&asm, &env::var("SECRETS_ID").unwrap()).await;

    let binahbot_env = BinahBotEnvironment {
        discord_secrets,
        discord_client_id: env::var("CLIENT_ID").unwrap(),
        s3_bucket_name: env::var("S3_BUCKET_NAME").unwrap(),
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
        spoiler_config: &SPOILER_CONFIG
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
    metadata: &DiscordInteractionMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
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
    use crate::SPOILER_CONFIG;

    pub fn build_mocked_binahbot_env() -> BinahBotEnvironment {
        BinahBotEnvironment {
            discord_secrets: DiscordSecrets {
                application_id: "app_id".to_string(),
                auth_token: "auth_token".to_string(),
                public_key: "pub_key".to_string(),
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
            spoiler_config: &SPOILER_CONFIG
        }
    }
}