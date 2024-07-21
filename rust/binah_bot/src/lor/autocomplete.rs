use std::str::FromStr;

use lambda_http::tracing;
use ruina_common::localizations::common::Locale;
use unic_langid::LanguageIdentifier;

use crate::lor::lookup::lookup;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::discord::AutocompleteResponse;
use crate::models::discord::DiscordInteractionOptions;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionResponseAutocomplete;
use crate::models::discord::DiscordInteractionResponseType;
use crate::utils::get_disambiguation_format;
use crate::utils::get_option_value;
use crate::DiscordInteraction;

static MAX_AUTOCOMPLETE_OPTIONS: usize = 10;

pub fn lor_autocomplete(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> AutocompleteResponse {
    let command_args = interaction.data.as_ref().unwrap().options.as_ref().unwrap();

    tracing::info!("Lor autocomplete: command args: {:#?}", command_args);

    let binding = "".to_string();
    let query = get_option_value("query", command_args).map(|x| match x {
        DiscordInteractionOptionValue::String(y) => y,
        _ => unreachable!()
    }).unwrap_or(&binding);

    let binah_locale: BinahBotLocale = interaction
        .locale
        .as_ref()
        .or(interaction.guild_locale.as_ref())
        .and_then(|x| BinahBotLocale::from_str(x).ok())
        .unwrap_or(BinahBotLocale::EnglishUS);

    let locale: Locale = get_option_value("locale", command_args).map(|x| match x {
        DiscordInteractionOptionValue::String(y) => y,
        _ => unreachable!()
    }).and_then(|x| Locale::from_str(x.as_str()).ok()).unwrap_or(Locale::from(binah_locale.clone()));

    let all: bool = get_option_value("all", command_args).map(|x| match x {
        DiscordInteractionOptionValue::Bool(y) => y.to_owned(),
        _ => unreachable!()
    }).unwrap_or(false);

    let lang_id = LanguageIdentifier::from(&binah_locale);

    let options: Vec<_> = lookup(query, &locale, all)
        .take(MAX_AUTOCOMPLETE_OPTIONS)
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
        let interaction = build_discord_interaction(weight_of_sin_query.to_string(), true);

        let response = lor_autocomplete(&interaction, &build_mocked_binahbot_env());
        let choices = response.data.as_ref().expect("no data field found")
            .choices.as_ref().expect("no embeds found")
            .iter().map(|x| x.name.clone()).collect::<Vec<_>>();

        assert!(choices.len() <= MAX_AUTOCOMPLETE_OPTIONS);
        assert!(choices.contains(&"\u{2068}The Weight of Sin\u{2069} (\u{2068}abno page\u{2069})".to_string()));
        assert!(choices.contains(&"\u{2068}The Weight of Sin\u{2069} (\u{2068}passive\u{2069})".to_string()));
    }

    #[test]
    fn sanity_xiao() {
        let xiao_query = "Xiao";
        let interaction = build_discord_interaction(xiao_query.to_string(), true);

        let response = lor_autocomplete(&interaction, &build_mocked_binahbot_env());
        let choices = response.data.as_ref().expect("no data field found")
            .choices.as_ref().expect("no embeds found")
            .iter().map(|x| x.name.clone()).collect::<Vec<_>>();

        assert!(choices.len() <= MAX_AUTOCOMPLETE_OPTIONS);
        assert!(choices.contains(&"\u{2068}Xiao\u{2069} (\u{2068}passive\u{2069})".to_string()));
        assert!(choices.contains(&"\u{2068}Xiao’s Page\u{2069} (\u{2068}collectable\u{2069})".to_string()));
        assert!(choices.contains(&"Xiao’s Page".to_string()));
    }

    #[test]
    fn sanity_not_all() {
        let xiao_query = "Xiao";
        let interaction = build_discord_interaction(xiao_query.to_string(), false);

        let response = lor_autocomplete(&interaction, &build_mocked_binahbot_env());
        let choices = response.data.as_ref().expect("no data field found")
            .choices.as_ref().expect("no embeds found")
            .iter().map(|x| x.name.clone()).collect::<Vec<_>>();

        assert!(choices.len() <= MAX_AUTOCOMPLETE_OPTIONS);
        assert!(!choices.contains(&"\u{2068}Xiao\u{2069} (\u{2068}passive\u{2069})".to_string()));
        assert!(choices.contains(&"\u{2068}Xiao’s Page\u{2069} (\u{2068}collectable\u{2069})".to_string()));
        assert!(!choices.contains(&"Xiao’s Page".to_string()));

    }

    fn build_discord_interaction(query_string: String, is_all: bool) -> DiscordInteraction {
        DiscordInteraction {
            id: "id".to_string(),
            application_id: "app_id".to_string(),
            r#type: DiscordInteractionType::ApplicationCommandAutocomplete,
            data: Some(DiscordInteractionData {
                id: "id".to_string(),
                name: "lor".to_string(),
                options: Some(vec![DiscordInteractionOptions {
                    name: "query".to_string(),
                    name_localizations: None,
                    value: DiscordInteractionOptionValue::String(query_string),
                    focused: None
                }, DiscordInteractionOptions {
                    name: "all".to_string(),
                    name_localizations: None,
                    value: DiscordInteractionOptionValue::Bool(is_all),
                    focused: None
                }]),
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
