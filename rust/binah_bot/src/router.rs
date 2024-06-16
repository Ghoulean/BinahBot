use crate::lor_autocomplete::main::lor_autocomplete;
use crate::lor_command::main::lor_command;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionResponse;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordInteractionType;
use crate::models::discord::PingResponse;

#[derive(strum::Display)]
#[strum(serialize_all = "lowercase")]
enum CommandName {
    Lor,
}

pub fn get_response(
    discord_interaction: &DiscordInteraction,
    binahbot_env: &BinahBotEnvironment,
) -> DiscordInteractionResponse {
    // switch to static hashmap later, for now just use switch-case
    match (&discord_interaction.r#type, &discord_interaction.data) {
        (DiscordInteractionType::Ping, _) => {
            DiscordInteractionResponse::Ping(PingResponse {
                r#type: DiscordInteractionResponseType::Pong,
            })
        }
        (DiscordInteractionType::ApplicationCommand, Some(data))
            if data.name == CommandName::Lor.to_string() =>
        {
            DiscordInteractionResponse::Message(lor_command(
                discord_interaction,
                binahbot_env,
            ))
        }
        (DiscordInteractionType::ApplicationCommandAutocomplete, Some(data))
            if data.name == CommandName::Lor.to_string() =>
        {
            DiscordInteractionResponse::Autocomplete(lor_autocomplete(discord_interaction, binahbot_env))
        }
        _ => panic!(),
    }
}
