use std::str::FromStr;

use lambda_http::tracing;
use ruina_common::localizations::common::Locale;
use unic_langid::LanguageIdentifier;

use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::discord::AutocompleteResponse;
use crate::models::discord::DiscordInteractionOptions;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionResponseAutocomplete;
use crate::models::discord::DiscordInteractionResponseType;
use crate::utils::get_disambiguation_format;
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

    let ids = ruina_index::query(&query);

    let options: Vec<_> = ids
        .into_iter()
        .take(10)
        .map(|x| {
            let display_name = get_disambiguation_format(&x, &locale, &lang_id, env);

            DiscordInteractionOptions {
                name: display_name,
                name_localizations: None,
                value: DiscordInteractionOptionValue::String(x.to_string()),
                focused: None
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
        .map(|x| { 
            match &x.value {
                DiscordInteractionOptionValue::String(s) => s,
                _ => unreachable!()
            }
        })
        .unwrap_or(&"".to_string())
        .to_string()
}

fn get_locale_option(vec: &[DiscordInteractionOptions]) -> Option<String> {
    vec.iter()
        .find(|x| x.name == "locale")
        .map(|x| { 
            match &x.value {
                DiscordInteractionOptionValue::String(s) => s,
                _ => unreachable!()
            }
        })
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::discord::DiscordInteractionData;
    use crate::models::discord::DiscordInteractionType;
    use crate::models::discord::DiscordUser;
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
                    value: DiscordInteractionOptionValue::String(query_string),
                    focused: None
                }],
            }),
            channel_id: None,
            token: "token".to_string(),
            locale: None,
            guild_locale: None,
            user: Some(DiscordUser {
                id: "snowflake".to_string(),
                username: "username".to_string(),
                avatar: "hash".to_string(),
            }),
            member: None
        }
    }
}
