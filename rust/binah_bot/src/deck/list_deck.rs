use std::collections::HashMap;
use std::str::FromStr;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use ruina_common::game_objects::common::PageType;
use ruina_common::localizations::common::Locale;
use ruina_index::get_disambiguation;
use ruina_reparser::get_key_page_by_id;
use ruina_reparser::get_key_page_locales_by_text_id;
use unic_langid::LanguageIdentifier;

use crate::ddb::list_decks;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::discord::AutocompleteResponse;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionOptions;
use crate::models::discord::DiscordInteractionResponseAutocomplete;
use crate::models::discord::DiscordInteractionResponseType;
use crate::utils::get_focused_option;
use crate::utils::get_option_value;

pub async fn list_deck(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> AutocompleteResponse {
    let command_args = &interaction.data.as_ref().unwrap().options;

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
            DiscordInteractionOptionValue::User(x) => x,
            _ => unreachable!()
    }).map(|x| &x.id);
    let focused = get_focused_option(&command_args).map(|x| x.name.as_str());

    tracing::info!("Got the following: incomplete_name={:?}, keypage_option={:?}, author_id_option={:?}, focused={:?}",
        incomplete_name,
        keypage_option,
        author_id_option,
        focused
    );

    let choices = match focused {
        Some(x) => match x {
            "keypage" => {
                let ids = ruina_index::query(&keypage_option.unwrap_or(&"".to_string()));

                ids.iter()
                    .filter(|x| x.0 == PageType::KeyPage)
                    .take(10)
                    .map(|parsed_id| {
                        // todo: please don't copy paste this. it pains me
                        let display_name_page = get_key_page_by_id(&parsed_id.1)
                            .and_then(|x| x.text_id)
                            .and_then(|x| get_key_page_locales_by_text_id(x).get(&card_locale).cloned())
                            .map(|x| x.name);
                        let display_name_option;
            
                        let disambiguation = get_disambiguation(&parsed_id, &card_locale);
                        if let Some(disambiguation_str) = disambiguation {
                            display_name_option = env.locales.lookup_with_args(
                                &lang_id,
                                "autocomplete_display_disambiguation",
                                &HashMap::from([
                                    ("display", FluentValue::from(display_name_page.unwrap_or(&parsed_id.to_string()))),
                                    ("disambiguation", FluentValue::from(*disambiguation_str)),
                                ])
                            )
                        } else {
                            display_name_option = env.locales.lookup_with_args(
                                &lang_id,
                                "autocomplete_display",
                                &HashMap::from([
                                    ("display", FluentValue::from(display_name_page.unwrap_or(&parsed_id.to_string()))),
                                ])
                            )
                        }

                        DiscordInteractionOptions {
                            name: display_name_option,
                            name_localizations: None,
                            value: DiscordInteractionOptionValue::String(parsed_id.1.clone()),
                            focused: None
                        }
                        // todo: end copy paste
                    })
                    .collect::<Vec<_>>()
            }
            _ => {
                let decks = list_decks(
                    env.ddb_client.as_ref().expect("no ddb client"),
                    &env.ddb_table_name,
                    author_id_option.map(|x| x.as_str()),
                    keypage_option.map(|x| x.as_str()),
                ).await;

                tracing::info!("got decks={:?}", decks);

                let binding = &"".to_string();
                let query = incomplete_name.unwrap_or(binding);

                match decks {
                    Ok(results) => {
                        let mut closeness = results.iter().map(|x| {
                            (x, longest_common_subsequence(&x.name, query))
                        }).collect::<Vec<_>>();
                        closeness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

                        closeness.into_iter().take(10)
                            .map(|(x, _)| {
                                let display_name = env.locales.lookup_with_args(
                                    &lang_id,
                                    "list_deck_name_author",
                                    &HashMap::from([
                                        ("deck_name", FluentValue::from(&x.name)),
                                        ("author", FluentValue::from(&x.author_name)),
                                    ])
                                );

                                DiscordInteractionOptions {
                                    name: display_name,
                                    name_localizations: None,
                                    value: DiscordInteractionOptionValue::String(format!("{}#{}", x.author_id, x.name)),
                                    focused: None
                                }
                            }).collect::<Vec<_>>()
                    },
                    Err(_) => Vec::new()
                }
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

// https://github.com/TheAlgorithms/Rust/blob/218c4a8758667fc6d3784bda563fbe1e98fc04b4/src/dynamic_programming/longest_common_subsequence.rs
pub fn longest_common_subsequence(a: &str, b: &str) -> String {
    let a: Vec<_> = a.chars().collect();
    let b: Vec<_> = b.chars().collect();
    let (na, nb) = (a.len(), b.len());

    let mut solutions = vec![vec![0; nb + 1]; na + 1];

    for (i, ci) in a.iter().enumerate() {
        for (j, cj) in b.iter().enumerate() {
            solutions[i + 1][j + 1] = if ci == cj {
                solutions[i][j] + 1
            } else {
                solutions[i][j + 1].max(solutions[i + 1][j])
            }
        }
    }

    let mut result: Vec<char> = Vec::new();
    let (mut i, mut j) = (na, nb);
    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            result.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if solutions[i - 1][j] > solutions[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    result.reverse();
    result.iter().collect()
}


#[cfg(test)]
mod tests {
    // use super::*;
}
