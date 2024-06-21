
use std::collections::HashMap;
use std::str::FromStr;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use lambda_http::tracing;
use ruina_common::game_objects::common::PageType;
use ruina_common::localizations::common::Locale;
use ruina_index::get_disambiguation;
use ruina_index::models::ParsedTypedId;
use ruina_reparser::get_abno_page_locales_by_internal_name;
use ruina_reparser::get_battle_symbol_locales_by_internal_name;
use ruina_reparser::get_combat_page_locales_by_id;
use ruina_reparser::get_key_page_by_id;
use ruina_reparser::get_key_page_locales_by_text_id;
use ruina_reparser::get_passive_locales_by_id;
use unic_langid::LanguageIdentifier;

use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::discord::AutocompleteResponse;
use crate::models::discord::DiscordInteractionOptions;
use crate::models::discord::DiscordInteractionResponseAutocomplete;
use crate::models::discord::DiscordInteractionResponseType;
use crate::DiscordInteraction;

pub fn lor_autocomplete(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> AutocompleteResponse {
    let command_args = &interaction.data.as_ref().unwrap().options;

    tracing::info!("Lor command: command args: {:#?}", command_args);

    let query = get_query_option(command_args);

    let binah_locale: BinahBotLocale = interaction
        .locale
        .as_ref()
        .or(interaction.guild_locale.as_ref())
        .and_then(|x| BinahBotLocale::from_str(x).ok())
        .unwrap_or(BinahBotLocale::EnglishUS);

    let locale: Locale = get_locale_option(command_args).and_then(|x| Locale::from_str(x.as_str()).ok()).unwrap_or(Locale::from(binah_locale.clone()));

    let lang_id = LanguageIdentifier::from(&binah_locale);

    let mut ids = ruina_index::query(&query);
    ids.truncate(10);
    ids.shrink_to_fit();

    let options: Vec<_> = ids
        .into_iter()
        .map(|x| {
            let display_name;

            let disambiguation = get_disambiguation(&x, &locale);
            if let Some(disambiguation_str) = disambiguation {
                display_name = env.locales.lookup_with_args(
                    &lang_id,
                    "autocomplete_display_disambiguation",
                    &HashMap::from([
                        ("display", FluentValue::from(get_display_name_locale(&x, &locale).unwrap_or(x.to_string()))),
                        ("disambiguation", FluentValue::from(*disambiguation_str)),
                    ])
                )
            } else {
                display_name = env.locales.lookup_with_args(
                    &lang_id,
                    "autocomplete_display",
                    &HashMap::from([
                        ("display", FluentValue::from(get_display_name_locale(&x, &locale).unwrap_or(x.to_string()))),
                    ])
                )
            }

            DiscordInteractionOptions {
                name: display_name,
                name_localizations: None,
                value: x.to_string(),
            }
        })
        .collect();

    AutocompleteResponse {
        r#type: DiscordInteractionResponseType::ApplicationCommandAutocompleteResult,
        data: Some(DiscordInteractionResponseAutocomplete {
            choices: Some(options),
        }),
    }
}

fn get_query_option(vec: &[DiscordInteractionOptions]) -> String {
    vec.iter()
        .find(|x| x.name == "query")
        .map(|x| x.value.clone())
        .unwrap_or(String::from(""))
}

fn get_locale_option(vec: &[DiscordInteractionOptions]) -> Option<String> {
    vec.iter()
        .find(|x| x.name == "locale")
        .map(|x| x.value.clone())
}

fn get_display_name_locale(
    typed_id: &ParsedTypedId,
    locale: &Locale
) -> Option<String> {
    match typed_id.0 {
        PageType::AbnoPage => {
            get_abno_page_locales_by_internal_name(&typed_id.1).get(locale).map(|x| x.card_name.to_string())
        }
        PageType::BattleSymbol => {
            get_battle_symbol_locales_by_internal_name(&typed_id.1).get(locale).map(|x| format!("{} {}", x.prefix, x.postfix))
        }
        PageType::CombatPage => {
            get_combat_page_locales_by_id(&typed_id.1).get(locale).map(|x| x.name.to_string())
        }
        PageType::KeyPage => {
            get_key_page_by_id(&typed_id.1).and_then(|key_page| {
                key_page.text_id.map(|text_id| {
                    get_key_page_locales_by_text_id(text_id)
                        .get(locale)
                        .map(|key_page_locale| {
                            key_page_locale.name.to_string()
                        })
                }).or(key_page.skin.map(|skin| Some(skin.to_string()))).flatten()
            })
        }
        PageType::Passive => {
            get_passive_locales_by_id(&typed_id.1).get(locale).map(|x| x.name.to_string())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::discord::DiscordInteractionData;
    use crate::models::discord::DiscordInteractionType;
    use crate::test_utils::build_mocked_binahbot_env;

    // TODO: \u{2068} and \u{2069} are known as left-to-right marks. Their inclusion
    // is intentional w.r.t. localization. Implement helper test function to
    // remove these in order to increase test readability
    #[test]
    fn sanity_weight_of_sin() {
        let weight_of_sin_query = "the weight of sin";
        let interaction = build_discord_interaction(weight_of_sin_query.to_string());

        let response = lor_autocomplete(&interaction, &build_mocked_binahbot_env());
        let choices = response.data.as_ref().expect("no data field found")
            .choices.as_ref().expect("no embeds found")
            .iter().map(|x| x.name.clone()).collect::<Vec<_>>();
        assert_eq!(choices.len(), 10);
        assert!(choices.contains(&"\u{2068}The Weight of Sin\u{2069} (\u{2068}abno page\u{2069})".to_string()));
        assert!(choices.contains(&"\u{2068}The Weight of Sin\u{2069} (\u{2068}passive\u{2069})".to_string()));
    }

    #[test]
    fn sanity_xiao() {
        let xiao_query = "Xiao";
        let interaction = build_discord_interaction(xiao_query.to_string());

        let response = lor_autocomplete(&interaction, &build_mocked_binahbot_env());
        let choices = response.data.as_ref().expect("no data field found")
            .choices.as_ref().expect("no embeds found")
            .iter().map(|x| x.name.clone()).collect::<Vec<_>>();
        assert_eq!(choices.len(), 10);
        assert!(choices.contains(&"\u{2068}Xiao\u{2069} (\u{2068}passive\u{2069})".to_string()));
        assert!(choices.contains(&"\u{2068}Xiao’s Page\u{2069} (\u{2068}collectable\u{2069})".to_string()));
    }

    fn build_discord_interaction(query_string: String) -> DiscordInteraction {
        DiscordInteraction {
            id: "id".to_string(),
            application_id: "app_id".to_string(),
            r#type: DiscordInteractionType::ApplicationCommandAutocomplete,
            data: Some(DiscordInteractionData {
                id: "id".to_string(),
                name: "lor".to_string(),
                options: vec![DiscordInteractionOptions {
                    name: "query".to_string(),
                    name_localizations: None,
                    value: query_string,
                }],
            }),
            channel_id: None,
            token: "token".to_string(),
            locale: None,
            guild_locale: None,
        }
    }
}
