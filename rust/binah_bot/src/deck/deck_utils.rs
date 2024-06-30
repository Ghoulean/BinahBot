use fluent_templates::Loader;
use unic_langid::LanguageIdentifier;

use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::discord::AllowedMentions;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordMessageFlag;
use crate::models::discord::MessageResponse;

pub fn build_generic_error_message_response(lang_id: &LanguageIdentifier, env: &BinahBotEnvironment) -> MessageResponse {
    MessageResponse {
        r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
        data: Some(DiscordInteractionResponseMessage {
            allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
            content: None,
            embeds: Some(vec![
                DiscordEmbed {
                    title: None,
                    description: Some(env.locales.lookup(&lang_id, "generic_error_message")),
                    color: Some(DiscordEmbedColors::Default as i32),
                    image: None,
                    footer: None,
                    author: None,
                    url: None,
                    fields: None
                }
            ]),
            flags: Some(DiscordMessageFlag::EphemeralMessage as i32)
        })
    }
}