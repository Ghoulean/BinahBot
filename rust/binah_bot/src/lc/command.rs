use std::str::FromStr;

use lobocorp::lobocorp_common::game_objects::abnormality::EncyclopediaInfo;
use lobocorp::lobocorp_common::localizations::common::Locale;
use lobocorp::lobocorp_reparser::get_encyclopedia_info;

use crate::lc::button::build_buttons;
use crate::lc::button::Code;
use crate::lc::transformers::transform_donttouchme;
use crate::lc::transformers::transform_normal_info;
use crate::lc::transformers::transform_tool_info;
use crate::macros::cast_enum_variant;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::discord::AllowedMentions;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionData;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordMessageFlag;
use crate::models::discord::MessageResponse;
use crate::utils::get_binahbot_locale;
use crate::utils::get_option_value;

pub fn lc_command(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> MessageResponse {
    let command_args = interaction
        .data
        .as_ref()
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionData::ApplicationCommand))
        .and_then(|x| x.options.as_ref())
        .unwrap();

    tracing::info!("Lc command: command args: {:#?}", command_args);

    let binah_locale: BinahBotLocale = get_binahbot_locale(interaction);

    let locale: Locale = get_option_value("locale", command_args)
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::String))
        .and_then(|x| Locale::from_str(x.as_str()).ok())
        .unwrap_or(Locale::from(binah_locale.clone()));

    let query = get_option_value("query", command_args)
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::String))
        .and_then(|x| {
            x.parse::<u32>()
                .ok()
                .or_else(|| lobocorp::lobocorp_index::query(x).first().copied())
        });
    let query = match query {
        Some(x) => x,
        None => todo!(),
    };

    let entry = get_encyclopedia_info(&query).expect("couldn't find entry");

    let embed: DiscordEmbed = match &entry {
        EncyclopediaInfo::Normal(x) => transform_normal_info(&x, &locale, &binah_locale, env),
        EncyclopediaInfo::Tool(x) => transform_tool_info(&x, &locale, &binah_locale, env),
        EncyclopediaInfo::DontTouchMe(x) => transform_donttouchme(&x, &locale, &binah_locale, env),
    };

    let is_private = get_option_value("private", command_args)
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::Bool))
        .is_some_and(|x| *x);

    let flags = is_private.then_some(DiscordMessageFlag::EphemeralMessage as i32);

    let components = build_buttons(query, &locale, &binah_locale, &(Code::Encyclopedia, 0), env);

    MessageResponse {
        r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
        data: Some(DiscordInteractionResponseMessage {
            allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
            content: None,
            embeds: Some(vec![embed]),
            flags: flags,
            components: Some(components),
        }),
    }
}

#[cfg(test)]
mod tests {
    use lobocorp::lobocorp_reparser::get_abno_localization;
    use lobocorp::lobocorp_reparser::get_all_encyclopedia_ids;
    use strum::IntoEnumIterator;

    use crate::models::discord::DiscordApplicationCommandInteractionData;
    use crate::models::discord::DiscordInteractionOptions;
    use crate::models::discord::DiscordInteractionType;
    use crate::models::discord::DiscordUser;
    use crate::test_utils::build_mocked_binahbot_env;

    use super::*;

    #[test]
    fn check_no_crashing() {
        let env = &build_mocked_binahbot_env();
        Locale::iter().for_each(|locale| {
            get_all_encyclopedia_ids()
                .iter()
                .flat_map(|x| get_encyclopedia_info(x))
                .map(|x| {
                    let id = match x {
                        EncyclopediaInfo::Normal(x) => &x.id,
                        EncyclopediaInfo::Tool(x) => &x.id,
                        EncyclopediaInfo::DontTouchMe(x) => &x.id,
                    };
                    get_abno_localization(id, &locale)
                        .map(|x| x.name)
                        .expect("no name")
                })
                .map(|name| build_discord_interaction(name.to_string(), locale.clone()))
                .for_each(|interaction| {
                    let _does_not_crash = lc_command(&interaction, env);
                })
        });
    }

    fn build_discord_interaction(query_string: String, locale: Locale) -> DiscordInteraction {
        DiscordInteraction {
            id: "id".to_string(),
            application_id: "app_id".to_string(),
            r#type: DiscordInteractionType::ApplicationCommand,
            data: Some(DiscordInteractionData::ApplicationCommand(
                DiscordApplicationCommandInteractionData {
                    id: "id".to_string(),
                    name: "lc".to_string(),
                    options: Some(vec![
                        DiscordInteractionOptions {
                            name: "query".to_string(),
                            name_localizations: None,
                            value: DiscordInteractionOptionValue::String(query_string),
                            focused: None,
                        },
                        DiscordInteractionOptions {
                            name: "locale".to_string(),
                            name_localizations: None,
                            value: DiscordInteractionOptionValue::String(locale.to_string()),
                            focused: None,
                        },
                    ]),
                },
            )),
            channel_id: None,
            token: "token".to_string(),
            locale: None,
            guild_locale: None,
            user: Some(DiscordUser {
                id: "snowflake".to_string(),
                username: "username".to_string(),
                avatar: Some("hash".to_string()),
            }),
            member: None,
            message: None,
        }
    }
}
