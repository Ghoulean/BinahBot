use std::collections::HashMap;
use std::str::FromStr;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use lobocorp::lobocorp_common::localizations::common::Locale;
use lobocorp::lobocorp_reparser::get_abno_localization;
use unic_langid::LanguageIdentifier;

use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::discord::AutocompleteResponse;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionData;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionOptions;
use crate::models::discord::DiscordInteractionResponseAutocomplete;
use crate::models::discord::DiscordInteractionResponseType;
use crate::utils::get_binahbot_locale;
use crate::utils::get_option_value;


static MAX_AUTOCOMPLETE_OPTIONS: usize = 10;

pub fn lc_autocomplete(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> AutocompleteResponse {
    let binding = match interaction.data.as_ref().expect("no data") {
        DiscordInteractionData::ApplicationCommand(x) => x,
        _ => unreachable!()
    };
    let command_args = binding.options.as_ref().unwrap();

    tracing::info!("Lc autocomplete: command args: {:#?}", command_args);

    let binding = "".to_string();
    let query = get_option_value("query", command_args).map(|x| match x {
        DiscordInteractionOptionValue::String(y) => y,
        _ => unreachable!()
    }).unwrap_or(&binding);

    let binah_locale: BinahBotLocale = get_binahbot_locale(interaction);

    let locale: Locale = get_option_value("locale", command_args).map(|x| match x {
        DiscordInteractionOptionValue::String(y) => y,
        _ => unreachable!()
    }).and_then(|x| Locale::from_str(x.as_str()).ok()).unwrap_or(Locale::from(binah_locale.clone()));

    let lang_id = LanguageIdentifier::from(&binah_locale);

    let options: Vec<_> = lobocorp::lobocorp_index::query(query)
        .into_iter()
        .take(MAX_AUTOCOMPLETE_OPTIONS)
        .map(|x| {
            let locale_info = get_abno_localization(&x, &locale).expect("can't get localization data");
            let display_name = env.locales.lookup_with_args(
                &lang_id,
                "encyclopedia_title_format",
                &HashMap::from([
                    ("name", FluentValue::from(locale_info.name)),
                    ("code", FluentValue::from(locale_info.code)),
                ])
            );

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