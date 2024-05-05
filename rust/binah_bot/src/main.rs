mod lor_command;
mod lor_autocomplete;
mod models;
mod router;
mod secrets_accessor;

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use models::{binahbot::BinahBotEnvironment, discord::DiscordInteraction};
use router::get_response;
use secrets_accessor::{get_discord_secrets};
use std::ops::Deref;
use std::env;
use http::HeaderMap;
use lambda_http::{run, service_fn, tracing, Body, Request, Response};
use tracing_subscriber;
use hex::FromHex;

use crate::models::binahbot::DiscordSecrets;
use crate::models::discord::DiscordInteractionMetadata;

static TIMESTAMP_HEADER: &str = "x-signature-timestamp";
static SIGNATURE_HEADER: &str = "x-signature-ed25519";

async fn function_handler(
    event: Request, 
    binahbot_env: &BinahBotEnvironment
) -> Result<Response<Body>, lambda_http::Error> {
    tracing::debug!("Rust function invoked");

    let request_body: String = String::from_utf8(event.body().deref().to_vec()).unwrap();
    let request_headers: &HeaderMap = event.headers();

    let event_metadata = DiscordInteractionMetadata {
        timestamp: get_header(request_headers, TIMESTAMP_HEADER),
        signature: get_header(request_headers, SIGNATURE_HEADER),
        json_body: request_body.clone()
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
    tracing_subscriber::fmt().json()
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
        discord_secrets: discord_secrets,
        discord_client_id: env::var("CLIENT_ID").unwrap(),
        s3_bucket_name: env::var("S3_BUCKET_NAME").unwrap()
    };
    let binahbot_env_ref = &binahbot_env;

    tracing::debug!("Rust function setup complete");

    run(service_fn(move |event: Request| {
        function_handler(event, binahbot_env_ref)
    })).await
}

fn get_header(header_map: &HeaderMap, header_key: &str) -> String {
    header_map.get(header_key).and_then(|x| x.to_str().ok()).unwrap_or("").to_owned()
}

fn validate_headers(
    discord_secrets: &DiscordSecrets,
    metadata: &DiscordInteractionMetadata
) -> Result<(), Box<dyn std::error::Error>> {
    let public_key_bytes = <[u8; 32]>::from_hex(&discord_secrets.public_key)?;
    let public_key = VerifyingKey::from_bytes(&public_key_bytes)?;

    let signature_bytes = <[u8; 64]>::from_hex(&metadata.signature)?;
    let concatenated = format!("{}{}", metadata.timestamp, metadata.json_body);

    Ok(public_key.verify(
        concatenated.as_bytes(),
        &Signature::from_slice(&signature_bytes)?
    )?)
}
