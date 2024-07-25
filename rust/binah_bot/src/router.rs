use std::error::Error;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::about_command::about_command;
use crate::ddb::put_interaction_token;
use crate::deck::create_deck::create_deck;
use crate::deck::delete_deck::delete_deck;
use crate::deck::list_deck::list_deck;
use crate::deck::list_deck::list_my_decks;
use crate::deck::read_deck::read_deck;
use crate::deck::update_deck::update_deck;
use crate::lor::autocomplete::lor_autocomplete;
use crate::lor::command::lor_command;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::InteractionTtl;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionResponse;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordInteractionType;
use crate::models::discord::PingResponse;

static ABOUT_COMMAND_NAME: &str = "about";
static LOR_COMMAND_NAME: &str = "lor";
static CREATE_DECK_COMMAND_NAME: &str = "createdeck";
static READ_DECK_COMMAND_NAME: &str = "deck";
static UPDATE_DECK_COMMAND_NAME: &str = "updatedeck";
static DELETE_DECK_COMMAND_NAME: &str = "deletedeck";

pub async fn get_response(
    discord_interaction: &DiscordInteraction,
    binahbot_env: &BinahBotEnvironment,
) -> Result<DiscordInteractionResponse, Box<dyn Error + Send + Sync>> {
    // todo: switch to static hashmap later, for now just use switch-case
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
    match (&discord_interaction.r#type, &discord_interaction.data) {
        (DiscordInteractionType::Ping, _) => Ok(DiscordInteractionResponse::Ping(PingResponse {
            r#type: DiscordInteractionResponseType::Pong,
        })),
        (DiscordInteractionType::ApplicationCommand, Some(data))
            if data.name == LOR_COMMAND_NAME =>
        {
            Ok(DiscordInteractionResponse::Message(lor_command(discord_interaction, binahbot_env)))
        }
        (DiscordInteractionType::ApplicationCommandAutocomplete, Some(data))
            if data.name == LOR_COMMAND_NAME =>
        {
            Ok(DiscordInteractionResponse::Autocomplete(lor_autocomplete(
                discord_interaction,
                binahbot_env,
            )))
        }
        (DiscordInteractionType::ApplicationCommand, Some(data))
            if data.name == CREATE_DECK_COMMAND_NAME =>
        {
            Ok(DiscordInteractionResponse::Message(create_deck(discord_interaction, binahbot_env).await))
        }
        (DiscordInteractionType::ApplicationCommand, Some(data))
            if data.name == READ_DECK_COMMAND_NAME =>
        {
            Ok(DiscordInteractionResponse::Message(read_deck(discord_interaction, binahbot_env).await))
        }
        (DiscordInteractionType::ApplicationCommand, Some(data))
            if data.name == UPDATE_DECK_COMMAND_NAME =>
        {
            Ok(DiscordInteractionResponse::Message(update_deck(discord_interaction, binahbot_env).await))
        }
        (DiscordInteractionType::ApplicationCommand, Some(data))
            if data.name == DELETE_DECK_COMMAND_NAME =>
        {
            Ok(DiscordInteractionResponse::Message(delete_deck(discord_interaction, binahbot_env).await))
        }
        (DiscordInteractionType::ApplicationCommandAutocomplete, Some(data))
            if data.name == READ_DECK_COMMAND_NAME =>
        {
            Ok(DiscordInteractionResponse::Autocomplete(list_deck(
                discord_interaction,
                binahbot_env,
            ).await))
        }
        (DiscordInteractionType::ApplicationCommandAutocomplete, Some(data))
            if data.name == UPDATE_DECK_COMMAND_NAME || data.name == DELETE_DECK_COMMAND_NAME =>
        {
            Ok(DiscordInteractionResponse::Autocomplete(list_my_decks(
                discord_interaction,
                binahbot_env,
            ).await))
        }
        (DiscordInteractionType::ApplicationCommand, Some(data))
            if data.name == ABOUT_COMMAND_NAME =>
        {
            Ok(DiscordInteractionResponse::Message(about_command(discord_interaction, binahbot_env)))
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
    };

    put_interaction_token(
        binahbot_env.ddb_client.as_ref().expect("no ddb client"),
        &binahbot_env.ddb_interaction_ttl_table_name,
        &interaction_ttl
    ).await
}