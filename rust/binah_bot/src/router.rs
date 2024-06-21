use crate::lor_autocomplete::main::lor_autocomplete;
use crate::lor_command::main::lor_command;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionResponse;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordInteractionType;
use crate::models::discord::PingResponse;

static LOR_COMMAND_NAME: &str = "lor";
static DECK_COMMAND_NAME: &str = "deck";

pub fn get_response(
    discord_interaction: &DiscordInteraction,
    binahbot_env: &BinahBotEnvironment,
) -> DiscordInteractionResponse {
    // switch to static hashmap later, for now just use switch-case
    match (&discord_interaction.r#type, &discord_interaction.data) {
        (DiscordInteractionType::Ping, _) => DiscordInteractionResponse::Ping(PingResponse {
            r#type: DiscordInteractionResponseType::Pong,
        }),
        (DiscordInteractionType::ApplicationCommand, Some(data))
            if data.name == LOR_COMMAND_NAME =>
        {
            DiscordInteractionResponse::Message(lor_command(discord_interaction, binahbot_env))
        }
        (DiscordInteractionType::ApplicationCommandAutocomplete, Some(data))
            if data.name == LOR_COMMAND_NAME =>
        {
            DiscordInteractionResponse::Autocomplete(lor_autocomplete(
                discord_interaction,
                binahbot_env,
            ))
        }
        (DiscordInteractionType::ApplicationCommand, Some(data))
            if data.name == DECK_COMMAND_NAME =>
        {
            todo!()
        }
        (DiscordInteractionType::ApplicationCommandAutocomplete, Some(data))
            if data.name == DECK_COMMAND_NAME =>
        {
            todo!()
        }
        _ => panic!(),
    }
}
