use std::collections::HashMap;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use unic_langid::LanguageIdentifier;

use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::discord::AllowedMentions;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordMessageFlag;
use crate::models::discord::MessageResponse;
use crate::utils::build_error_message_response;
use crate::utils::get_binahbot_locale;
use crate::utils::get_option_value;

struct DeckKey((), String);

pub async fn delete_deck(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> MessageResponse {
    let command_args = interaction.data.as_ref().unwrap().options.as_ref().unwrap();

    let deck_name = match get_option_value("name", command_args).expect("no name option") {
        DiscordInteractionOptionValue::String(x) => x,
        _ => unreachable!()
    };
    let deck_key = match parse_deck_name_option(deck_name) {
        Ok(x) => x,
        Err(_) => panic!()
    };
    let author_id = &interaction.user.as_ref().unwrap_or(interaction.member.as_ref().unwrap().user.as_ref().unwrap()).id;

    let delete_deck_result = crate::ddb::delete_deck(
        env.ddb_client.as_ref().expect("no ddb client"),
        &env.ddb_table_name,
        &deck_key.1,
        author_id
    ).await;

    let lang_id = LanguageIdentifier::from(&get_binahbot_locale(interaction));
    
    match delete_deck_result {
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
                                "delete_deck_success",
                                &HashMap::from([
                                    ("deck_name", FluentValue::from(deck_key.1)),
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
            build_error_message_response(&lang_id, "generic_error_message", env)
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
