use std::collections::HashMap;
use std::str::FromStr;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use ruina::ruina_common::game_objects::common::PageType;
use ruina::ruina_common::localizations::common::Locale;
use unic_langid::LanguageIdentifier;

use crate::deck::deck_search::search_race;
use crate::macros::cast_enum_variant;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::deck::DeckMetadata;
use crate::models::deck::DeckSearchParams;
use crate::models::discord::AutocompleteResponse;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionData;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionOptions;
use crate::models::discord::DiscordInteractionResponseAutocomplete;
use crate::models::discord::DiscordInteractionResponseType;
use crate::utils::get_disambiguation_format;
use crate::utils::get_focused_option;
use crate::utils::get_option_value;

pub async fn list_deck(
    interaction: &DiscordInteraction,
    env: &BinahBotEnvironment,
) -> AutocompleteResponse {
    let command_args = interaction
        .data
        .as_ref()
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionData::ApplicationCommand))
        .and_then(|x| x.options.as_ref())
        .unwrap();

    let binah_locale: BinahBotLocale = interaction
        .locale
        .as_ref()
        .or(interaction.guild_locale.as_ref())
        .and_then(|x| BinahBotLocale::from_str(x).ok())
        .unwrap_or(BinahBotLocale::EnglishUS);
    let card_locale = Locale::from(&binah_locale);
    let lang_id = LanguageIdentifier::from(&binah_locale);

    let incomplete_name = get_option_value("name", command_args)
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::String));
    let keypage_option = get_option_value("keypage", command_args)
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::String));
    let author_id_option = get_option_value("author", command_args)
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::String));
    let focused = get_focused_option(command_args).map(|x| x.name.as_str());

    tracing::info!("Got the following: incomplete_name={:?}, keypage_option={:?}, author_id_option={:?}, focused={:?}",
        incomplete_name,
        keypage_option,
        author_id_option,
        focused
    );

    let choices = match focused {
        Some(x) => match x {
            "keypage" => get_choices_by_keypage_query(keypage_option, &card_locale, &lang_id, env),
            _ => {
                get_choices_by_deck_name(
                    incomplete_name,
                    author_id_option,
                    keypage_option,
                    &lang_id,
                    env,
                )
                .await
            }
        },
        None => unreachable!(),
    };

    tracing::info!("Returning choices={:?}", choices);

    AutocompleteResponse {
        r#type: DiscordInteractionResponseType::ApplicationCommandAutocompleteResult,
        data: Some(DiscordInteractionResponseAutocomplete {
            choices: Some(choices),
        }),
    }
}

pub async fn list_my_decks(
    interaction: &DiscordInteraction,
    env: &BinahBotEnvironment,
) -> AutocompleteResponse {
    let command_args = interaction
        .data
        .as_ref()
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionData::ApplicationCommand))
        .and_then(|x| x.options.as_ref())
        .unwrap();

    let binah_locale: BinahBotLocale = interaction
        .locale
        .as_ref()
        .or(interaction.guild_locale.as_ref())
        .and_then(|x| BinahBotLocale::from_str(x).ok())
        .unwrap_or(BinahBotLocale::EnglishUS);
    let lang_id = LanguageIdentifier::from(&binah_locale);

    let deck_name_query = get_option_value("name", command_args)
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::String));
    let author_id = &interaction
        .user
        .as_ref()
        .unwrap_or(interaction.member.as_ref().unwrap().user.as_ref().unwrap())
        .id;

    let focused = get_focused_option(command_args).map(|x| x.name.as_str());

    tracing::info!(
        "Got the following: incomplete_name={:?}, author_id={:?}, focused={:?}",
        deck_name_query,
        author_id,
        focused
    );
    let choices = match focused {
        Some(x) => match x {
            "name" => {
                get_choices_by_deck_name(deck_name_query, Some(author_id), None, &lang_id, env)
                    .await
            }
            _ => Vec::new(),
        },
        None => unreachable!(),
    };

    tracing::info!("Returning choices={:?}", choices);

    AutocompleteResponse {
        r#type: DiscordInteractionResponseType::ApplicationCommandAutocompleteResult,
        data: Some(DiscordInteractionResponseAutocomplete {
            choices: Some(choices),
        }),
    }
}

fn get_choices_by_keypage_query(
    query: Option<&String>,
    card_locale: &Locale,
    lang_id: &LanguageIdentifier,
    env: &BinahBotEnvironment,
) -> Vec<DiscordInteractionOptions> {
    let ids = ruina::ruina_index::query(query.unwrap_or(&"".to_string()));

    ids.iter()
        .filter(|x| x.0 == PageType::KeyPage)
        .take(10)
        .map(|parsed_id| {
            let display_name = get_disambiguation_format(parsed_id, card_locale, lang_id, env);

            DiscordInteractionOptions {
                name: display_name,
                name_localizations: None,
                value: DiscordInteractionOptionValue::String(parsed_id.1.clone()),
                focused: None,
            }
        })
        .collect::<Vec<_>>()
}

async fn get_choices_by_deck_name(
    query: Option<&String>,
    author_id_option: Option<&String>,
    keypage_option: Option<&String>,
    lang_id: &LanguageIdentifier,
    env: &BinahBotEnvironment,
) -> Vec<DiscordInteractionOptions> {
    let search_params = DeckSearchParams::new(
        query.map(|x| x.as_str()),
        author_id_option.map(|x| x.as_str()),
        keypage_option.map(|x| x.as_str()),
    );

    match search_race(&search_params, 0, env).await {
        Ok(entry) => entry
            .decks
            .iter()
            .take(10)
            .map(|x| {
                let display_name = env.locales.lookup_with_args(
                    lang_id,
                    "list_deck_name_author",
                    &HashMap::from([
                        ("deck_name", FluentValue::from(&x.name)),
                        ("author", FluentValue::from(&x.author_name)),
                    ]),
                );

                DiscordInteractionOptions {
                    name: display_name,
                    name_localizations: None,
                    value: DiscordInteractionOptionValue::String(get_discord_option_value(x)),
                    focused: None,
                }
            })
            .collect::<Vec<_>>(),
        Err(e) => {
            tracing::error!("deck search failed: {:?}", e);
            Vec::new()
        }
    }
}

fn get_discord_option_value(deck: &DeckMetadata) -> String {
    format!("{}#{}", deck.author_id, deck.name)
}
