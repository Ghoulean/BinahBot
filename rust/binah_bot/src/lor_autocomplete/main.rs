use std::str::FromStr;

use lambda_http::tracing;
use ruina_common::localizations::common::Locale;
use ruina_index::get_autocomplete_entry;

use crate::lor_autocomplete::lookup::get_typed_ids_from_query;
use crate::models::binahbot::BinahBotLocale;
use crate::models::discord::AutocompleteResponse;
use crate::models::discord::DiscordInteractionOptions;
use crate::models::discord::DiscordInteractionResponseAutocomplete;
use crate::models::discord::DiscordInteractionResponseType;
use crate::DiscordInteraction;

pub fn lor_autocomplete(interaction: &DiscordInteraction) -> AutocompleteResponse {
    let command_args = &interaction.data.as_ref().unwrap().options;

    tracing::info!("Lor command: command args: {:#?}", command_args);

    let query = get_query_option(command_args);

    let binah_locale: BinahBotLocale = interaction
        .locale
        .as_ref()
        .or(interaction.guild_locale.as_ref())
        .map(|x| BinahBotLocale::from_str(x).ok())
        .flatten()
        .unwrap_or(BinahBotLocale::EnglishUS);

    // todo: override through args
    let locale: Locale = Locale::from(binah_locale.clone());

    let mut ids = get_typed_ids_from_query(&query);
    ids.truncate(5);
    ids.shrink_to_fit();

    let options: Vec<_> = ids
        .into_iter()
        .flat_map(|x| {
            get_autocomplete_entry(&x, &locale).map(|y| {
                let display_disambiguator = y
                    .disambiguator
                    .map(|x| format!(" ({})", x))
                    .unwrap_or("".to_string());
                DiscordInteractionOptions {
                    name: format!("{}{}", y.base, display_disambiguator),
                    name_localizations: None,
                    value: x.to_string(),
                }
            })
        })
        .collect();

    AutocompleteResponse {
        r#type: DiscordInteractionResponseType::ApplicationCommandAutocompleteResult,
        data: Some(DiscordInteractionResponseAutocomplete {
            choices: Some(options),
        }),
    }
}

fn get_query_option(vec: &Vec<DiscordInteractionOptions>) -> String {
    vec.into_iter()
        .find(|x| x.name == "query")
        .map(|x| x.value.clone())
        .unwrap_or(String::from(""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::discord::DiscordInteractionData;
    use crate::models::discord::DiscordInteractionType;

    // TODO: sanity test differentiated display
    #[test]
    fn sanity_weight_of_sin() {
        let weight_of_sin_query = "the weight of sin";
        let interaction = build_discord_interaction(weight_of_sin_query.to_string());

        let response = lor_autocomplete(&interaction);
        assert_eq!(
            response
                .data
                .as_ref()
                .expect("no data field found")
                .choices
                .as_ref()
                .expect("no embeds found")
                .len(),
            2
        );
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
