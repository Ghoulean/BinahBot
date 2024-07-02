use std::str::FromStr;

use crate::models::binahbot::BinahBotLocale;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionOptions;
use crate::models::discord::DiscordInteractionOptionValue;

pub fn get_option_value<'a>(option_name: &'a str, options: &'a [DiscordInteractionOptions]) -> Option<&'a DiscordInteractionOptionValue> {
    options.iter()
        .find(|x| x.name == option_name)
        .map(|x| &x.value )
}

pub fn get_focused_option(options: &[DiscordInteractionOptions]) -> Option<&DiscordInteractionOptions> {
    options.iter().find(|x| x.focused.is_some_and(|y| y))
}

pub fn get_binahbot_locale(interaction: &DiscordInteraction) -> BinahBotLocale {
    interaction
        .locale
        .as_ref()
        .or(interaction.guild_locale.as_ref())
        .and_then(|x| BinahBotLocale::from_str(x).ok())
        .unwrap_or(BinahBotLocale::EnglishUS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_get_option_value() {
        let options = vec![
            DiscordInteractionOptions {
                name: "query".to_string(),
                name_localizations: None,
                value: DiscordInteractionOptionValue::String("query_str".to_string()),
                focused: None
            },
            DiscordInteractionOptions {
                name: "private".to_string(),
                name_localizations: None,
                value: DiscordInteractionOptionValue::Bool(true),
                focused: None
            }
        ];

        assert_eq!(
            &DiscordInteractionOptionValue::String("query_str".to_string()),
            get_option_value("query", &options).unwrap()
        );
        assert_eq!(
            &DiscordInteractionOptionValue::Bool(true),
            get_option_value("private", &options).unwrap()
        );
    }

    #[test]
    fn sanity_get_focused_option() {
        let options = vec![
            DiscordInteractionOptions {
                name: "query".to_string(),
                name_localizations: None,
                value: DiscordInteractionOptionValue::String("query_str".to_string()),
                focused: None
            },
            DiscordInteractionOptions {
                name: "private".to_string(),
                name_localizations: None,
                value: DiscordInteractionOptionValue::Bool(true),
                focused: Some(true)
            }
        ];

        assert_eq!(
            options[1].name,
            get_focused_option(&options).unwrap().name
        );
    }
}