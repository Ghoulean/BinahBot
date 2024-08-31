use std::error::Error;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::about_command::about_command;
use crate::ddb::get_interaction_token;
use crate::ddb::put_interaction_token;
use crate::deck::create_deck::create_deck;
use crate::deck::delete_deck::delete_deck;
use crate::deck::list_deck::list_deck;
use crate::deck::list_deck::list_my_decks;
use crate::deck::read_deck::read_deck;
use crate::deck::update_deck::update_deck;
use crate::discord::delete_interaction;
use crate::lc::autocomplete::lc_autocomplete;
use crate::lc::button::lc_button;
use crate::lc::button::LC_BUTTON_PREFIX;
use crate::lc::command::lc_command;
use crate::lor::autocomplete::lor_autocomplete;
use crate::lor::command::lor_command;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::InteractionTtl;
use crate::models::discord::DeferredUpdateResponse;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionData;
use crate::models::discord::DiscordInteractionResponse;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordInteractionType;
use crate::models::discord::PingResponse;
use crate::rollcalc_command::rollcalc_command;
use crate::utils::DELETE_BUTTON_CUSTOM_ID;

const ABOUT_COMMAND_NAME: &str = "about";
const LC_COMMAND_NAME: &str = "lc";
const LOR_COMMAND_NAME: &str = "lor";
const CREATE_DECK_COMMAND_NAME: &str = "createdeck";
const READ_DECK_COMMAND_NAME: &str = "deck";
const UPDATE_DECK_COMMAND_NAME: &str = "updatedeck";
const DELETE_DECK_COMMAND_NAME: &str = "deletedeck";
const ROLLCALC_COMMAND_NAME: &str = "rollcalc";

pub async fn get_response(
    discord_interaction: &DiscordInteraction,
    binahbot_env: &BinahBotEnvironment,
) -> Result<DiscordInteractionResponse, Box<dyn Error + Send + Sync>> {
    tracing::info!("Calling router with interaction type={:?}", &discord_interaction.r#type);

    let (response, _) = tokio::join!(
        route(discord_interaction, binahbot_env),
        put_interaction_ttl(discord_interaction, binahbot_env)
    );

    response
}

async fn route(
    discord_interaction: &DiscordInteraction,
    binahbot_env: &BinahBotEnvironment
) -> Result<DiscordInteractionResponse, Box<dyn Error + Send + Sync>> {
    match &discord_interaction.r#type {
        DiscordInteractionType::Ping => Ok(DiscordInteractionResponse::Ping(PingResponse {
            r#type: DiscordInteractionResponseType::Pong,
        })),
        DiscordInteractionType::ApplicationCommand => {
            let data = match discord_interaction.data.as_ref().expect("no data") {
                DiscordInteractionData::ApplicationCommand(x) => x,
                _ => unreachable!()
            };
            Ok(DiscordInteractionResponse::Message(match data.name.as_str() {
                LC_COMMAND_NAME => lc_command(discord_interaction, binahbot_env),
                LOR_COMMAND_NAME => lor_command(discord_interaction, binahbot_env),
                CREATE_DECK_COMMAND_NAME => create_deck(discord_interaction, binahbot_env).await,
                READ_DECK_COMMAND_NAME => read_deck(discord_interaction, binahbot_env).await,
                UPDATE_DECK_COMMAND_NAME => update_deck(discord_interaction, binahbot_env).await,
                DELETE_DECK_COMMAND_NAME => delete_deck(discord_interaction, binahbot_env).await,
                ABOUT_COMMAND_NAME => about_command(discord_interaction, binahbot_env),
                ROLLCALC_COMMAND_NAME => rollcalc_command(discord_interaction, binahbot_env),
                _ => unreachable!()
            }))
        }
        DiscordInteractionType::ApplicationCommandAutocomplete => {
            let data = match discord_interaction.data.as_ref().expect("no data") {
                DiscordInteractionData::ApplicationCommand(x) => x,
                _ => unreachable!()
            };
            Ok(DiscordInteractionResponse::Autocomplete(match data.name.as_str() {
                LOR_COMMAND_NAME => lor_autocomplete(discord_interaction, binahbot_env),
                READ_DECK_COMMAND_NAME => list_deck(discord_interaction, binahbot_env).await,
                UPDATE_DECK_COMMAND_NAME | DELETE_DECK_COMMAND_NAME => list_my_decks(discord_interaction, binahbot_env).await,
                LC_COMMAND_NAME => lc_autocomplete(discord_interaction, binahbot_env), 
                _ => unreachable!()
            }))
        },
        DiscordInteractionType::MessageComponent => {
            let data = match discord_interaction.data.as_ref().expect("no data") {
                DiscordInteractionData::MessageComponent(x) => x,
                _ => unreachable!()
            };

            let custom_id = &data.custom_id;

            let result = if custom_id == DELETE_BUTTON_CUSTOM_ID {
                let _ = process_delete_button(discord_interaction, binahbot_env).await;

                return Ok(DiscordInteractionResponse::DeferredUpdateMessage(DeferredUpdateResponse {
                    r#type: DiscordInteractionResponseType::DeferredUpdateMessage
                }))
            } else if custom_id.starts_with(LC_BUTTON_PREFIX) {
                lc_button(&discord_interaction, binahbot_env)
            } else {
                unreachable!()
            };

            match result {
                Ok(x) => Ok(DiscordInteractionResponse::UpdateMessage(x)),
                Err(_) => Ok(DiscordInteractionResponse::DeferredUpdateMessage(DeferredUpdateResponse {
                    r#type: DiscordInteractionResponseType::DeferredUpdateMessage
                })),
            }
        }
        _ => unreachable!(),
    }
}

async fn put_interaction_ttl(
    discord_interaction: &DiscordInteraction,
    binahbot_env: &BinahBotEnvironment
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if discord_interaction.r#type != DiscordInteractionType::ApplicationCommand && discord_interaction.r#type != DiscordInteractionType::MessageComponent {
        return Ok(());
    }

    let epoch_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("couldn't calculate epoch time")
        .as_secs();

    let interaction_ttl: InteractionTtl = InteractionTtl {
        interaction_id: discord_interaction.id.clone(),
        token: discord_interaction.token.clone(),
        ttl: epoch_time,
        original_user_id: discord_interaction.user.as_ref().unwrap_or(discord_interaction.member.as_ref().unwrap().user.as_ref().unwrap()).id.clone(),
    };

    put_interaction_token(
        binahbot_env.ddb_client.as_ref().expect("no ddb client"),
        &binahbot_env.ddb_interaction_ttl_table_name,
        &interaction_ttl
    ).await
}

async fn process_delete_button(
    discord_interaction: &DiscordInteraction,
    binahbot_env: &BinahBotEnvironment
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let previous_interaction_id = &discord_interaction
        .message
        .as_ref()
        .expect("no message on this delete button interaction")
        .interaction_metadata
        .as_ref()
        .expect("no interaction_metadata field")
        .id;
    let user_id = discord_interaction.user.as_ref().unwrap_or(discord_interaction.member.as_ref().unwrap().user.as_ref().unwrap()).id.clone();

    let interaction_ttl = get_interaction_token(
        binahbot_env.ddb_client.as_ref().expect("no ddb client provided"),
        &binahbot_env.ddb_interaction_ttl_table_name,
        &previous_interaction_id
    ).await;

    if interaction_ttl.as_ref().is_ok_and(|x| user_id == x.original_user_id) {
        tracing::info!("Deleting interaction with id={}", previous_interaction_id);
        let _ = delete_interaction(
            binahbot_env.reqwest_client.as_ref().expect("no http client provided"),
            &binahbot_env.discord_secrets,
            &interaction_ttl.as_ref().unwrap().token
        ).await;
    };

    Ok(())
}