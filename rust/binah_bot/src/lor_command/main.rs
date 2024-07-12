use std::str::FromStr;

use lambda_http::tracing;
use ruina_common::game_objects::common::PageType;
use ruina_common::localizations::common::Locale;
use ruina_index::models::ParsedTypedId;

use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::discord::AllowedMentions;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionOptions;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordMessageFlag;
use crate::models::discord::MessageResponse;

use crate::lor_command::transformers::transform_abno_page;
use crate::lor_command::transformers::transform_battle_symbol;
use crate::lor_command::transformers::transform_combat_page;
use crate::lor_command::transformers::transform_key_page;
use crate::lor_command::transformers::transform_passive;

pub fn lor_command(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> MessageResponse {
    let command_args = &interaction.data.as_ref().unwrap().options;

    tracing::info!("Lor command: command args: {:#?}", command_args);

    let typed_id = ParsedTypedId::from_str(get_query_option(command_args).as_str()).unwrap();

    let binah_locale: BinahBotLocale = interaction
        .locale
        .as_ref()
        .or(interaction.guild_locale.as_ref())
        .and_then(|x| BinahBotLocale::from_str(x).ok())
        .unwrap_or(BinahBotLocale::EnglishUS);

    let locale: Locale = get_locale_option(command_args).and_then(|x| Locale::from_str(x.as_str()).ok()).unwrap_or(Locale::from(binah_locale.clone()));

    let embed: DiscordEmbed = match typed_id.0 {
        PageType::AbnoPage => transform_abno_page,
        PageType::BattleSymbol => transform_battle_symbol,
        PageType::CombatPage => transform_combat_page,
        PageType::KeyPage => transform_key_page,
        PageType::Passive => transform_passive
    }(
        &typed_id.1,
        &locale,
        &binah_locale,
        env
    );

    let flags = if get_private_option(command_args).is_some_and(|x| x == true) {
        Some(DiscordMessageFlag::EphemeralMessage as i32)
    } else {
        None
    };

    MessageResponse {
        r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
        data: Some(DiscordInteractionResponseMessage {
            allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
            content: None,
            embeds: Some(vec![embed]),
            flags: flags
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
        .expect("no query option found")
        .to_string()
}

fn get_locale_option(vec: &[DiscordInteractionOptions]) -> Option<String> {
    vec.iter()
        .find(|x| &x.name == "locale")
        .map(|x| { 
            match &x.value {
                DiscordInteractionOptionValue::String(s) => s,
                _ => unreachable!()
            }
        })
        .cloned()
}

fn get_private_option(vec: &[DiscordInteractionOptions]) -> Option<bool> {
    vec.iter()
        .find(|x| x.name == "private")
        .map(|x| { 
            match &x.value {
                DiscordInteractionOptionValue::Bool(b) => b,
                _ => unreachable!()
            }
        })
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::discord::DiscordUser;
    use crate::test_utils::build_mocked_binahbot_env;
    use crate::models::discord::DiscordInteractionData;
    use crate::models::discord::DiscordInteractionType;

    #[test]
    fn sanity_get_query_option() {
        let query_value = "value";
        let interaction = build_discord_interaction(query_value.to_string(), Locale::English);
        let options = interaction.data.unwrap().options;
        assert_eq!(get_query_option(&options), query_value)
    }

    #[test]
    fn sanity_weight_of_sin() {
        let weight_of_sin_id = "a#LongBird_Sin";
        let interaction = build_discord_interaction(weight_of_sin_id.to_string(), Locale::English);
        let interaction_kr = build_discord_interaction(weight_of_sin_id.to_string(), Locale::Korean);
        let env = build_mocked_binahbot_env();

        let response = lor_command(&interaction, &env);
        let response_kr = lor_command(&interaction_kr, &env);
        assert_eq!(
            response
                .data.as_ref()
                .expect("no data field found")
                .embeds.as_ref()
                .expect("no embeds found")
                .len(),
            1
        );
        assert_eq!(
            response_kr
                .data.as_ref()
                .expect("no data field found")
                .embeds.as_ref()
                .expect("no embeds found")
                .len(),
            1
        );
        assert_ne!(
            response.data.as_ref().unwrap().embeds.as_ref().unwrap().first().unwrap().title.as_ref().unwrap().clone(),
            response_kr.data.as_ref().unwrap().embeds.as_ref().unwrap().first().unwrap().title.as_ref().unwrap().clone()
        );
    }

    #[test]
    fn sanity_degraded_pillar() {
        let degraded_pillar_id = "c#607204";
        let interaction = build_discord_interaction(degraded_pillar_id.to_string(), Locale::English);
        let env = build_mocked_binahbot_env();

        let response = lor_command(&interaction, &env);
        assert_eq!(
            response
                .data
                .expect("no data field found")
                .embeds
                .expect("no embeds found")
                .len(),
            1
        );
    }

    #[test]
    fn card_script_without_locale() {
        // Enemy-only FMF contains a card script and a die script that doesn't have
        // an associated locale with it.
        let enemy_fourth_match_flame = "c#9901101";
        let interaction = build_discord_interaction(enemy_fourth_match_flame.to_string(), Locale::English);
        let env = build_mocked_binahbot_env();

        let response = lor_command(&interaction, &env);
        assert_eq!(
            response
                .data
                .expect("no data field found")
                .embeds
                .expect("no embeds found")
                .len(),
            1
        );
    }

    fn build_discord_interaction(query_string: String, locale: Locale) -> DiscordInteraction {
        DiscordInteraction {
            id: "id".to_string(),
            application_id: "app_id".to_string(),
            r#type: DiscordInteractionType::ApplicationCommand,
            data: Some(DiscordInteractionData {
                id: "id".to_string(),
                name: "lor".to_string(),
                options: vec![DiscordInteractionOptions {
                    name: "query".to_string(),
                    name_localizations: None,
                    value: DiscordInteractionOptionValue::String(query_string),
                    focused: None,
                }, DiscordInteractionOptions {
                    name: "locale".to_string(),
                    name_localizations: None,
                    value: DiscordInteractionOptionValue::String(locale.to_string()),
                    focused: None,
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
