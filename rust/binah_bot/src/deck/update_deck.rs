use std::collections::HashMap;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use unic_langid::LanguageIdentifier;

use crate::ddb::get_deck;
use crate::ddb::put_deck;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::deck::TiphDeck;
use crate::models::discord::AllowedMentions;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordMessageFlag;
use crate::models::discord::MessageResponse;
use crate::thumbnail::generate_thumbnail;
use crate::tiph::decode;
use crate::utils::get_binahbot_locale;
use crate::utils::get_option_value;
use crate::utils::parse_tiph_deck_id;

use super::deck_utils::build_generic_error_message_response;

static DEFAULT_TIPH_DECK_VERSION: i32 = 1;
struct DeckKey((), String);

pub async fn update_deck(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> MessageResponse {
    let command_args = &interaction.data.as_ref().unwrap().options;

    let deck_name = match get_option_value("name", command_args).expect("no name option") {
        DiscordInteractionOptionValue::String(x) => x,
        _ => unreachable!()
    };
    let deck_key = match parse_deck_name_option(deck_name) {
        Ok(x) => x,
        Err(_) => panic!()
    };

    let tiph_deck_option = get_option_value("deck", command_args).map(|x| match x {
        DiscordInteractionOptionValue::String(y) => y,
        _ => unreachable!()
    }).map(|x| {
        TiphDeck(parse_tiph_deck_id(x), DEFAULT_TIPH_DECK_VERSION)
    });

    let description_option = get_option_value("description", command_args).map(|x| {
        match x {
            DiscordInteractionOptionValue::String(y) => y,
            _ => unreachable!()
        }
    });
    let author_id = &interaction.user.as_ref().unwrap_or(interaction.member.as_ref().unwrap().user.as_ref().unwrap()).id;

    let get_deck_result = get_deck(
        env.ddb_client.as_ref().unwrap(),
        &env.ddb_table_name,
        &deck_key.1,
        author_id
    ).await;

    let mut deck = match get_deck_result {
        Ok(x) => x,
        // todo: early return MessageResponse
        Err(_) => panic!()
    };

    if description_option.is_some() {
        deck.description = description_option.cloned()
    }

    if let Some(tiph_deck) = tiph_deck_option {
        let deck_data_result = decode(
            env.reqwest_client.as_ref().expect("no reqwest client"),
            &tiph_deck
        ).await;

        let deck_data = match deck_data_result {
            Ok(x) => x,
            // todo: early return MessageResponse
            Err(_) => panic!()
        };

        let _ = generate_thumbnail(
            env.lambda_client.as_ref().expect("no aws lambda client"),
            &env.thumbnail_lambda_name,
            &deck_data.combat_page_ids
        ).await;
        deck.deck_data = deck_data;
    }

    let put_deck_result = put_deck(
        env.ddb_client.as_ref().expect("no ddb client"),
        &env.ddb_table_name,
        &deck,
        true
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
                                "update_deck_success",
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

// todo: move to utils
fn parse_deck_name_option(name_option: &str) -> Result<DeckKey, ()> {
    let mut split: Vec<_> = name_option.split('#').collect();
    if split.len() >= 2 {
        let split2 = split.split_off(1);
        Ok(DeckKey((), split2.join("#")))
    } else {
        Err(())
    }
}