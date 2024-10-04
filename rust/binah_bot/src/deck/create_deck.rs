use std::collections::HashMap;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use unic_langid::LanguageIdentifier;

use crate::ddb::put_deck;
use crate::macros::cast_enum_variant;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::deck::Deck;
use crate::models::deck::TiphDeck;
use crate::models::discord::AllowedMentions;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionData;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordMessageFlag;
use crate::models::discord::MessageResponse;
use crate::thumbnail::generate_thumbnail;
use crate::tiph::decode;
use crate::utils::build_error_message_response;
use crate::utils::get_binahbot_locale;
use crate::utils::get_option_value;
use crate::utils::parse_tiph_deck_id;

use super::deck_utils::validate_deck;

static DEFAULT_TIPH_DECK_VERSION: i32 = 1;

pub async fn create_deck(
    interaction: &DiscordInteraction,
    env: &BinahBotEnvironment,
) -> MessageResponse {
    let command_args = interaction
        .data
        .as_ref()
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionData::ApplicationCommand))
        .and_then(|x| x.options.as_ref())
        .unwrap();

    let tiph_deck_str = get_option_value("deck", command_args)
        .as_ref()
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::String))
        .unwrap();
    let tiph_deck = TiphDeck(parse_tiph_deck_id(tiph_deck_str), DEFAULT_TIPH_DECK_VERSION);

    let deck_name = get_option_value("name", command_args)
        .as_ref()
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::String))
        .unwrap();
    let description = get_option_value("description", command_args)
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::String));

    let author = interaction
        .user
        .as_ref()
        .unwrap_or(interaction.member.as_ref().unwrap().user.as_ref().unwrap());

    let author_id = &author.id;
    let author_name = &author.username;

    let lang_id = LanguageIdentifier::from(&get_binahbot_locale(interaction));

    let deck_data_result = decode(
        env.reqwest_client.as_ref().expect("no reqwest client"),
        &tiph_deck,
    )
    .await;

    let deck_data = match deck_data_result {
        Ok(x) => x,
        Err(_) => {
            return build_error_message_response(&lang_id, "cant_parse_deck_error_message", env)
        }
    };

    if let Err(e) = validate_deck(&deck_data) {
        return build_error_message_response(&lang_id, e, env);
    }

    let _ = generate_thumbnail(
        env.lambda_client.as_ref().expect("no aws lambda client"),
        &env.thumbnail_lambda_name,
        &deck_data.combat_page_ids,
    )
    .await;

    let deck = Deck {
        name: deck_name.to_string(),
        author_id: author_id.to_string(),
        author_name: author_name.to_string(),
        description: description.cloned(),
        deck_data,
        tiph_deck: Some(tiph_deck),
    };

    let put_deck_result = put_deck(
        env.ddb_client.as_ref().expect("no ddb client"),
        &env.ddb_table_name,
        &deck,
        false,
    )
    .await;

    match put_deck_result {
        Ok(_) => MessageResponse {
            r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
            data: Some(DiscordInteractionResponseMessage {
                allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
                content: None,
                embeds: Some(vec![DiscordEmbed {
                    title: None,
                    description: Some(env.locales.lookup_with_args(
                        &lang_id,
                        "create_deck_success",
                        &HashMap::from([("deck_name", FluentValue::from(deck_name))]),
                    )),
                    color: Some(DiscordEmbedColors::Default as i32),
                    image: None,
                    thumbnail: None,
                    footer: None,
                    author: None,
                    url: None,
                    fields: None,
                }]),
                flags: Some(DiscordMessageFlag::EphemeralMessage as i32),
                components: None,
            }),
        },
        Err(_) => {
            // todo: check for error type
            build_error_message_response(&lang_id, "generic_error_message", env)
        }
    }
}
