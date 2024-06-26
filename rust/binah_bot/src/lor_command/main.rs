use std::str::FromStr;

use lambda_http::tracing;
use ruina_common::game_objects::common::PageType;
use ruina_common::localizations::common::Locale;
use ruina_index::models::ParsedTypedId;
use ruina_reparser::get_abno_page_by_internal_name;
use ruina_reparser::get_abno_page_locales_by_internal_name;
use ruina_reparser::get_battle_symbol_by_internal_name;
use ruina_reparser::get_battle_symbol_locales_by_internal_name;
use ruina_reparser::get_combat_page_by_id;
use ruina_reparser::get_combat_page_locales_by_id;
use ruina_reparser::get_key_page_by_id;
use ruina_reparser::get_key_page_locales_by_text_id;
use ruina_reparser::get_passive_by_id;
use ruina_reparser::get_passive_locales_by_id;

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
        PageType::AbnoPage => {
            let abno_page = get_abno_page_by_internal_name(&typed_id.1).unwrap();
            let abno_page_locales = get_abno_page_locales_by_internal_name(&typed_id.1);
            let abno_page_locale = abno_page_locales.get(&locale).unwrap();
            transform_abno_page(
                abno_page,
                abno_page_locale,
                &binah_locale,
                env
            )
        }
        PageType::BattleSymbol => {
            let battle_symbol = get_battle_symbol_by_internal_name(&typed_id.1).unwrap();
            let battle_symbol_locales = get_battle_symbol_locales_by_internal_name(&typed_id.1);
            let battle_symbol_locale = battle_symbol_locales.get(&locale).unwrap();
            transform_battle_symbol(
                battle_symbol,
                battle_symbol_locale,
                &binah_locale,
                env
            )
        }
        PageType::CombatPage => {
            let combat_page = get_combat_page_by_id(&typed_id.1).unwrap();
            let combat_page_locales = get_combat_page_locales_by_id(&typed_id.1);
            let combat_page_locale = combat_page_locales.get(&locale).unwrap();
            transform_combat_page(
                combat_page,
                combat_page_locale,
                &locale,
                &binah_locale,
                env
            )
        }
        PageType::KeyPage => {
            let key_page = get_key_page_by_id(&typed_id.1).unwrap();
            let key_page_locale = key_page
                .text_id
                .map(|x| *get_key_page_locales_by_text_id(x).get(&locale).unwrap());
            transform_key_page(
                key_page,
                key_page_locale,
                &locale,
                &binah_locale,
                env
            )
        }
        PageType::Passive => {
            let passive = get_passive_by_id(&typed_id.1).unwrap();
            let passive_locales = get_passive_locales_by_id(&typed_id.1);
            let passive_locale = passive_locales.get(&locale).unwrap();
            transform_passive(passive, passive_locale, &binah_locale, env)
        }
    };

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
    fn get_query_option_sanity() {
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
                }, DiscordInteractionOptions {
                    name: "locale".to_string(),
                    name_localizations: None,
                    value: DiscordInteractionOptionValue::String(locale.to_string()),
                }],
            }),
            channel_id: None,
            token: "token".to_string(),
            locale: None,
            guild_locale: None,
            user: Some(DiscordUser {
                id: "snowflake".to_string(),
            }),
            member: None
        }
    }
}
