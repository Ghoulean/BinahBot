use std::collections::HashMap;
use std::str::FromStr;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use index_analyzer::generate_inverse_index;
use ruina::ruina_common::game_objects::common::PageType;
use ruina::ruina_common::localizations::common::Locale;
use unic_langid::LanguageIdentifier;

use crate::ddb::list_decks;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::deck::DeckMetadata;
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

pub async fn list_deck(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> AutocompleteResponse {
    let binding = match interaction.data.as_ref().expect("no data") {
        DiscordInteractionData::ApplicationCommand(x) => x,
        _ => unreachable!()
    };
    let command_args = binding.options.as_ref().unwrap();

    let binah_locale: BinahBotLocale = interaction
        .locale
        .as_ref()
        .or(interaction.guild_locale.as_ref())
        .and_then(|x| BinahBotLocale::from_str(x).ok())
        .unwrap_or(BinahBotLocale::EnglishUS);
    let card_locale = Locale::from(&binah_locale);
    let lang_id = LanguageIdentifier::from(&binah_locale);

    let incomplete_name = get_option_value("name", command_args).map(|x| match x {
            DiscordInteractionOptionValue::String(x) => x,
            _ => unreachable!()
    });
    let keypage_option = get_option_value("keypage", command_args).map(|x| match x {
            DiscordInteractionOptionValue::String(x) => x,
            _ => unreachable!()
    });

    let author_id_option = get_option_value("author", command_args).map(|x| match x {
            DiscordInteractionOptionValue::String(x) => x,
            _ => unreachable!()
    });
    let focused = get_focused_option(command_args).map(|x| x.name.as_str());

    tracing::info!("Got the following: incomplete_name={:?}, keypage_option={:?}, author_id_option={:?}, focused={:?}",
        incomplete_name,
        keypage_option,
        author_id_option,
        focused
    );

    let choices = match focused {
        Some(x) => match x {
            "keypage" => {
                get_choices_by_keypage_query(
                    keypage_option,
                    &card_locale,
                    &lang_id,
                    env
                )
            }
            _ => {
                get_choices_by_deck_name(
                    incomplete_name,
                    author_id_option,
                    keypage_option,
                    &lang_id,
                    env
                ).await
            }
        },
        None => unreachable!()
    };

    tracing::info!("Returning choices={:?}", choices);

    AutocompleteResponse {
        r#type: DiscordInteractionResponseType::ApplicationCommandAutocompleteResult,
        data: Some(DiscordInteractionResponseAutocomplete {
            choices: Some(choices),
        }),
    }
}

pub async fn list_my_decks(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> AutocompleteResponse {
    let binding = match interaction.data.as_ref().expect("no data") {
        DiscordInteractionData::ApplicationCommand(x) => x,
        _ => unreachable!()
    };
    let command_args = binding.options.as_ref().unwrap();

    let binah_locale: BinahBotLocale = interaction
        .locale
        .as_ref()
        .or(interaction.guild_locale.as_ref())
        .and_then(|x| BinahBotLocale::from_str(x).ok())
        .unwrap_or(BinahBotLocale::EnglishUS);
    let lang_id = LanguageIdentifier::from(&binah_locale);

    let deck_name_query = get_option_value("name", command_args).map(|x| match x {
            DiscordInteractionOptionValue::String(x) => x,
            _ => unreachable!()
    });
    let author_id = &interaction.user.as_ref().unwrap_or(interaction.member.as_ref().unwrap().user.as_ref().unwrap()).id;

    let focused = get_focused_option(command_args).map(|x| x.name.as_str());

    tracing::info!("Got the following: incomplete_name={:?}, author_id={:?}, focused={:?}",
        deck_name_query,
        author_id,
        focused
    );
    let choices = match focused {
        Some(x) => match x {
            "name" => {
                get_choices_by_deck_name(
                    deck_name_query,
                    Some(author_id),
                    None,
                    &lang_id,
                    env
                ).await
            },
            _ => Vec::new()
        },
        None => unreachable!()
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
    env: &BinahBotEnvironment
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
                focused: None
            }
        })
        .collect::<Vec<_>>()
}

async fn get_choices_by_deck_name(
    query: Option<&String>,
    author_id_option: Option<&String>,
    keypage_option: Option<&String>,
    lang_id: &LanguageIdentifier,
    env: &BinahBotEnvironment
) -> Vec<DiscordInteractionOptions> {
    let decks = list_decks(
        env.ddb_client.as_ref().expect("no ddb client"),
        &env.ddb_table_name,
        author_id_option.map(|x| x.as_str()),
        keypage_option.map(|x| x.as_str()),
    ).await;

    tracing::info!("got decks={:?}", decks);

    let binding = &"".to_string();
    let query = query.unwrap_or(binding).to_lowercase();

    match decks {
        Ok(results) => {
            get_top_matches(&query, &results, 10).into_iter()
                .map(|x| {
                    let display_name = env.locales.lookup_with_args(
                        lang_id,
                        "list_deck_name_author",
                        &HashMap::from([
                            ("deck_name", FluentValue::from(&x.name)),
                            ("author", FluentValue::from(&x.author_name)),
                        ])
                    );

                    DiscordInteractionOptions {
                        name: display_name,
                        name_localizations: None,
                        value: DiscordInteractionOptionValue::String(get_discord_option_value(x)),
                        focused: None
                    }
                }).collect::<Vec<_>>()
        },
        Err(_) => Vec::new()
    }
}

fn get_top_matches<'a>(query: &'a str, decks: &'a Vec<DeckMetadata>, top: usize) -> Vec<&'a DeckMetadata> {
    let map: HashMap<usize, &String> = decks.iter().enumerate().map(|(i, x)| (i, &x.name)).collect();
    let inverse_index = generate_inverse_index(&map);
    let query_results = index_analyzer::query(query, &inverse_index);
    query_results.into_iter().take(top).map(move |i| {
        &decks[*i]
    }).collect()
}

fn get_discord_option_value(deck: &DeckMetadata) -> String {
    format!("{}#{}", deck.author_id, deck.name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_get_top_matches() {
        let decks = vec![
            DeckMetadata { name: "Turbo Nikolai".to_string(), author_id: "1".to_string(), author_name: "gh".to_string() },
            DeckMetadata { name: "Bore Kali to death with this one neat trick!".to_string(), author_id: "2".to_string(), author_name: "al".to_string() },
            DeckMetadata { name: "Op".to_string(), author_id: "3".to_string(), author_name: "ai".to_string() },
            DeckMetadata { name: "Crimson Claws".to_string(), author_id: "4".to_string(), author_name: "eb".to_string() },
            DeckMetadata { name: "TurboNikolai".to_string(), author_id: "1".to_string(), author_name: "gh".to_string() },
        ];
        let top = get_top_matches("turbo nikolai", &decks, 2).iter().map(|x| x.name.as_str()).collect::<Vec<_>>();
        dbg!(&top);
        assert_eq!(2, top.len());
        assert_eq!(Some(&"Turbo Nikolai"), top.get(0));
        assert_eq!(Some(&"TurboNikolai"), top.get(1));

        let top = get_top_matches("one neat trick", &decks, 1).iter().map(|x| x.name.as_str()).collect::<Vec<_>>();
        assert_eq!(1, top.len());
        assert_eq!(Some(&"Bore Kali to death with this one neat trick!"), top.get(0));

        let top = get_top_matches("CCCCCCCC", &decks, 10).iter().map(|x| x.name.as_str()).collect::<Vec<_>>();
        assert_eq!(1, top.len());
        assert_eq!(Some(&"Crimson Claws"), top.get(0));

    }
}
