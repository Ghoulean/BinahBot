use std::error::Error;

use crate::deck::create_deck::create_deck;
use crate::deck::read_deck::read_deck;
use crate::lor_autocomplete::main::lor_autocomplete;
use crate::lor_command::main::lor_command;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionResponse;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordInteractionType;
use crate::models::discord::PingResponse;

static LOR_COMMAND_NAME: &str = "lor";
static CREATE_DECK_COMMAND_NAME: &str = "createdeck";
static READ_DECK_COMMAND_NAME: &str = "deck";
static UPDATE_DECK_COMMAND_NAME: &str = "updatedeck";
static DELETE_DECK_COMMAND_NAME: &str = "deletedeck";

pub async fn get_response(
    discord_interaction: &DiscordInteraction,
    binahbot_env: &BinahBotEnvironment,
) -> Result<DiscordInteractionResponse, Box<dyn Error + Send + Sync>> {
    // switch to static hashmap later, for now just use switch-case
    tracing::info!("Calling router with interaction type={:?}", &discord_interaction.r#type);
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
            todo!()
            // convert tiph deck -> actual deck (if provided)
            // convert rest of input to Deck obj (if provided)
            // make update item ddb call
            // check ddb result
            // dne exception -> err, use create (ephemeral)
            // other exception -> just crash ig
            // success -> return success message (ephemeral)
        }
        (DiscordInteractionType::ApplicationCommand, Some(data))
            if data.name == DELETE_DECK_COMMAND_NAME =>
        {
            todo!()
            // make delete item ddb call
            // check ddb result
            // dne exception -> err doesn't exist lol (ephemeral)
            // other exception -> just crash
            // success -> return success message (ephemeral)
        }
        (DiscordInteractionType::ApplicationCommandAutocomplete, Some(data))
            if data.name == READ_DECK_COMMAND_NAME =>
        {
            todo!()
            // listitem ddb based on keypage and author (if provided)
            // check ddb result
            // post-parse ddb results by name (very simple text match; nothing sophisticated like prebuilt index)
            // other exception -> crash or something
            // success -> return
        }
        (DiscordInteractionType::ApplicationCommandAutocomplete, Some(data))
            if data.name == UPDATE_DECK_COMMAND_NAME || data.name == DELETE_DECK_COMMAND_NAME =>
        {
            todo!()
            // get user id from input
            // listitem ddb on author
            // check ddb result
            // post-parse ddb results by name (very simple text match; nothing sophisticated like prebuilt index)
            // other exception -> crash or something
            // success -> return
        }
        _ => panic!(),
    }
}
