use std::str::FromStr;

use lambda_http::tracing;
use ruina_common::localizations::common::Locale;
use ruina_index::models::PageType;
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
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionOptions;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
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
        .map(|x| BinahBotLocale::from_str(x).ok())
        .flatten()
        .unwrap_or(BinahBotLocale::EnglishUS);

    // todo: override through args
    let locale: Locale = Locale::from(binah_locale.clone());

    let embed: DiscordEmbed = match typed_id.0 {
        PageType::AbnoPageId => {
            let abno_page = get_abno_page_by_internal_name(&typed_id.1).unwrap();
            let abno_page_locales = get_abno_page_locales_by_internal_name(&typed_id.1);
            let abno_page_locale = abno_page_locales.get(&locale).unwrap();
            transform_abno_page(
                abno_page,
                abno_page_locale,
                &env.s3_bucket_name,
                &binah_locale,
            )
        }
        PageType::BattleSymbolId => {
            let battle_symbol = get_battle_symbol_by_internal_name(&typed_id.1).unwrap();
            let battle_symbol_locales = get_battle_symbol_locales_by_internal_name(&typed_id.1);
            let battle_symbol_locale = battle_symbol_locales.get(&locale).unwrap();
            transform_battle_symbol(
                battle_symbol,
                battle_symbol_locale,
                &env.s3_bucket_name,
                &binah_locale,
            )
        }
        PageType::CombatPageId => {
            let combat_page = get_combat_page_by_id(&typed_id.1).unwrap();
            let combat_page_locales = get_combat_page_locales_by_id(&typed_id.1);
            let combat_page_locale = combat_page_locales.get(&locale).unwrap();
            transform_combat_page(
                combat_page,
                combat_page_locale,
                &env.s3_bucket_name,
                &env.emojis,
                &locale,
                &binah_locale,
            )
        }
        PageType::KeyPageId => {
            let key_page = get_key_page_by_id(&typed_id.1).unwrap();
            let key_page_locale = key_page
                .text_id
                .map(|x| *get_key_page_locales_by_text_id(x).get(&locale).unwrap());
            transform_key_page(
                key_page,
                key_page_locale,
                &env.s3_bucket_name,
                &env.emojis,
                &locale,
                &binah_locale,
            )
        }
        PageType::PassiveId => {
            let passive = get_passive_by_id(&typed_id.1).unwrap();
            let passive_locales = get_passive_locales_by_id(&typed_id.1);
            let passive_locale = passive_locales.get(&locale).unwrap();
            transform_passive(passive, passive_locale, &binah_locale)
        }
    };

    MessageResponse {
        r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
        data: Some(DiscordInteractionResponseMessage {
            embeds: Some(vec![embed]),
        }),
    }
}

fn get_query_option(vec: &Vec<DiscordInteractionOptions>) -> String {
    vec.into_iter()
        .find(|x| x.name == "query")
        .map(|x| x.value.clone())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::binahbot::DiscordSecrets;
    use crate::models::binahbot::Emojis;
    use crate::models::discord::DiscordInteractionData;
    use crate::models::discord::DiscordInteractionType;

    #[test]
    fn get_query_option_sanity() {
        let query_value = "value";
        let interaction = build_discord_interaction(query_value.to_string());
        let options = interaction.data.unwrap().options;
        assert_eq!(get_query_option(&options), query_value)
    }

    #[test]
    fn sanity_weight_of_sin() {
        let weight_of_sin_id = "a#LongBird_Sin";
        let interaction = build_discord_interaction(weight_of_sin_id.to_string());
        let env = build_binahbot_env();

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
    fn sanity_degraded_pillar() {
        let degraded_pillar_id = "c#607204";
        let interaction = build_discord_interaction(degraded_pillar_id.to_string());
        let env = build_binahbot_env();

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

    fn build_discord_interaction(query_string: String) -> DiscordInteraction {
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
                    value: query_string,
                }],
            }),
            channel_id: None,
            token: "token".to_string(),
            locale: None,
            guild_locale: None,
        }
    }

    fn build_binahbot_env() -> BinahBotEnvironment {
        BinahBotEnvironment {
            discord_secrets: DiscordSecrets {
                application_id: "app_id".to_string(),
                auth_token: "auth_token".to_string(),
                public_key: "pub_key".to_string(),
            },
            discord_client_id: "id".to_string(),
            s3_bucket_name: "bucket_name".to_string(),
            emojis: Emojis {
                slash_emoji_id: None,
                pierce_emoji_id: None,
                blunt_emoji_id: None,
                block_emoji_id: None,
                evade_emoji_id: None,
                c_slash_emoji_id: None,
                c_pierce_emoji_id: None,
                c_blunt_emoji_id: None,
                c_block_emoji_id: None,
                c_evade_emoji_id: None,
            },
        }
    }
}
