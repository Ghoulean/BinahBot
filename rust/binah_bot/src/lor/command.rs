use std::str::FromStr;

use fluent_templates::Loader;
use lambda_http::tracing;
use ruina::ruina_common::game_objects::common::PageType;
use ruina::ruina_common::localizations::common::Locale;
use ruina::ruina_index::models::ParsedTypedId;
use unic_langid::LanguageIdentifier;

use crate::lor::lookup::lookup;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::discord::AllowedMentions;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordMessageFlag;
use crate::models::discord::MessageResponse;

use crate::lor::transformers::transform_abno_page;
use crate::lor::transformers::transform_battle_symbol;
use crate::lor::transformers::transform_combat_page;
use crate::lor::transformers::transform_key_page;
use crate::lor::transformers::transform_passive;
use crate::utils::get_option_value;

pub fn lor_command(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> MessageResponse {
    let command_args = interaction.data.as_ref().unwrap().options.as_ref().unwrap();

    tracing::info!("Lor command: command args: {:#?}", command_args);

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

    let lang_id = LanguageIdentifier::from(&binah_locale);

    let all: bool = get_option_value("all", command_args).map(|x| match x {
        DiscordInteractionOptionValue::Bool(y) => y.to_owned(),
        _ => unreachable!()
    }).unwrap_or(false);

    let query = get_option_value("query", command_args).map(|x| match x {
        DiscordInteractionOptionValue::String(y) => y,
        _ => unreachable!()
    }).and_then(|x| {
        match ParsedTypedId::from_str(x) {
            Ok(y) => Some(y),
            Err(_) => lookup(&x, &locale, all).next()
        }
    });

    let typed_id = match query {
        Some(x) => x,
        None => return no_match_found(&lang_id, env)
    };

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

    let flags = if get_option_value("private", command_args).map(|x| match x {
        DiscordInteractionOptionValue::Bool(y) => y,
        _ => unreachable!()
    }).is_some_and(|x| *x) {
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
            flags
        }),
    }
}

fn no_match_found(lang_id: &LanguageIdentifier, env: &BinahBotEnvironment) -> MessageResponse {
    MessageResponse {
        r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
        data: Some(DiscordInteractionResponseMessage {
            allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
            content: None,
            embeds: Some(vec![DiscordEmbed {
                title: None,
                description: Some(env.locales.lookup(&lang_id, "no_page_error_message")),
                color: Some(DiscordEmbedColors::Default as i32),
                image: None,
                footer: None,
                author: None,
                url: None,
                fields: None
            }]),
            flags: Some(DiscordMessageFlag::EphemeralMessage as i32)
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;
    use ruina::ruina_reparser::get_all_abno_pages;
    use ruina::ruina_reparser::get_all_battle_symbols;
    use ruina::ruina_reparser::get_all_combat_pages;
    use ruina::ruina_reparser::get_all_key_pages;
    use ruina::ruina_reparser::get_all_passives;
    use crate::lor::lookup::is_collectable_or_obtainable;
    use crate::models::discord::DiscordInteractionOptions;
    use crate::models::discord::DiscordUser;
    use crate::models::discord::DiscordInteractionData;
    use crate::models::discord::DiscordInteractionType;
    use crate::test_utils::build_mocked_binahbot_env;
    use crate::utils::get_display_name_locale;

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
    #[ignore]
    fn regenerative_mimicry_passive_realization() {
        let regenerative = "p#605532";
        let interaction = build_discord_interaction(regenerative.to_string(), Locale::English);
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
    fn best_match() {
        let liu_section_1_collectable = "250019";
        let liu_section_1_enemy_query = "Liu Section 1 enemy";
        let env = build_mocked_binahbot_env();

        // "all" option is false
        let interaction = build_discord_interaction(liu_section_1_enemy_query.to_string(), Locale::English);

        let response = lor_command(&interaction, &env);

        assert_eq!(response
            .data
            .expect("no data field found")
            .embeds
            .expect("no embeds found")
            .first()
            .expect("no embeds")
            .footer
            .as_ref()
            .expect("no footer")
            .text,
            liu_section_1_collectable
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

    #[test]
    fn check_no_crashing() {
        Locale::iter().for_each(|locale: Locale| {
            let abno_page_ids = get_all_abno_pages().iter().map(|x| ParsedTypedId(PageType::AbnoPage, x.internal_name.to_string())).collect::<Vec<_>>();
            let battle_symbol_ids = get_all_battle_symbols().iter().map(|x| ParsedTypedId(PageType::BattleSymbol, x.internal_name.to_string())).collect::<Vec<_>>();
            let combat_page_ids = get_all_combat_pages().iter().map(|x| ParsedTypedId(PageType::CombatPage, x.id.to_string())).collect::<Vec<_>>();
            let keypage_ids = get_all_key_pages().iter().map(|x| ParsedTypedId(PageType::KeyPage, x.id.to_string())).collect::<Vec<_>>();
            let passive_ids = get_all_passives().iter().map(|x| ParsedTypedId(PageType::Passive, x.id.to_string())).collect::<Vec<_>>();

            abno_page_ids.into_iter()
                .chain(battle_symbol_ids)
                .chain(combat_page_ids)
                .chain(keypage_ids)
                .chain(passive_ids)
                .filter(is_collectable_or_obtainable)
                .filter(|x| get_display_name_locale(x, &locale).is_some())
                .for_each(|x| {
                    let interaction = build_discord_interaction(x.to_string(), locale.clone());
                    let env = build_mocked_binahbot_env();

                    let _does_not_crash = lor_command(&interaction, &env);
                });
        });
    }

    fn build_discord_interaction(query_string: String, locale: Locale) -> DiscordInteraction {
        DiscordInteraction {
            id: "id".to_string(),
            application_id: "app_id".to_string(),
            r#type: DiscordInteractionType::ApplicationCommand,
            data: Some(DiscordInteractionData {
                id: "id".to_string(),
                name: "lor".to_string(),
                options: Some(vec![DiscordInteractionOptions {
                    name: "query".to_string(),
                    name_localizations: None,
                    value: DiscordInteractionOptionValue::String(query_string),
                    focused: None,
                }, DiscordInteractionOptions {
                    name: "locale".to_string(),
                    name_localizations: None,
                    value: DiscordInteractionOptionValue::String(locale.to_string()),
                    focused: None,
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
