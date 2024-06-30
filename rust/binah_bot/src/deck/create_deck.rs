use std::collections::HashMap;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use unic_langid::LanguageIdentifier;

use crate::ddb::put_deck;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::deck::Deck;
use crate::models::deck::TiphDeck;
use crate::models::discord::AllowedMentions;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordMessageFlag;
use crate::models::discord::MessageResponse;
use crate::tiph::decode;
use crate::utils::get_binahbot_locale;
use crate::utils::get_option_value;

use super::deck_utils::build_generic_error_message_response;

static DEFAULT_TIPH_DECK_VERSION: i32 = 1;

pub async fn create_deck(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> MessageResponse {
    let command_args = &interaction.data.as_ref().unwrap().options;

    let tiph_deck_str = match get_option_value("deck", command_args).expect("no deck option") {
        DiscordInteractionOptionValue::String(x) => x,
        _ => unreachable!()
    };
    let tiph_deck = TiphDeck(tiph_deck_str.to_string(), DEFAULT_TIPH_DECK_VERSION);
    let deck_data_result = decode(
        &env.reqwest_client.as_ref().expect("no reqwest client"),
        &tiph_deck
    ).await;

    let deck_data = match deck_data_result {
        Ok(x) => x,
        // todo: early return MessageResponse
        Err(_) => panic!()
    };

    let deck_name = match get_option_value("name", command_args).expect("no deck option") {
        DiscordInteractionOptionValue::String(x) => x,
        _ => unreachable!()
    };
    let description = get_option_value("name", command_args).map(|x| {
        match x {
            DiscordInteractionOptionValue::String(y) => y,
            _ => unreachable!()
        }
    });
    let author_id = &interaction.user.as_ref().unwrap_or(interaction.member.as_ref().unwrap().user.as_ref().unwrap()).id;

    let deck = Deck {
        name: deck_name.to_string(),
        author: author_id.to_string(),
        description: description.cloned(),
        deck_data: deck_data,
        tiph_deck: Some(tiph_deck)
    };

    let put_deck_result = put_deck(
        &env.ddb_client.as_ref().expect("no reqwest client"),
        &env.ddb_table_name,
        &deck,
        false
    ).await;

    let lang_id = LanguageIdentifier::from(&get_binahbot_locale(interaction));
    
    match put_deck_result {
        Ok(_) => {
            MessageResponse {
                r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
                data: Some(DiscordInteractionResponseMessage {
                    allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
                    content: None,
                    embeds: Some(vec![
                        DiscordEmbed {
                            title: None,
                            description: Some(env.locales.lookup_with_args(
                                &lang_id,
                                "create_deck_success",
                                &HashMap::from([
                                    ("deck_name", FluentValue::from(deck_name)),
                                ])
                            )),
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
        },
        Err(_) => {
            // todo: check for error type
            build_generic_error_message_response(&lang_id, env)
        }
    }
}

