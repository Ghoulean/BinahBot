use std::collections::HashMap;
use std::str::FromStr;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use ruina::ruina_common::game_objects::common::PageType;
use ruina::ruina_common::localizations::common::Locale;
use ruina::ruina_index::get_disambiguation;
use ruina::ruina_index::models::ParsedTypedId;
use ruina::ruina_reparser::get_abno_page_locales_by_internal_name;
use ruina::ruina_reparser::get_battle_symbol_locales_by_internal_name;
use ruina::ruina_reparser::get_combat_page_locales_by_id;
use ruina::ruina_reparser::get_key_page_by_id;
use ruina::ruina_reparser::get_key_page_locales_by_text_id;
use ruina::ruina_reparser::get_passive_locales_by_id;
use unic_langid::LanguageIdentifier;

use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::discord::AllowedMentions;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionOptions;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordMessageFlag;
use crate::models::discord::MessageResponse;

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

pub fn get_disambiguation_format(
    parsed_typed_id: &ParsedTypedId,
    locale: &Locale,
    lang_id: &LanguageIdentifier,
    env: &BinahBotEnvironment
) -> String {
    let disambiguation = get_disambiguation(parsed_typed_id, locale);
    if let Some(disambiguation_str) = disambiguation {
        env.locales.lookup_with_args(
            lang_id,
            "autocomplete_display_disambiguation",
            &HashMap::from([
                ("display", FluentValue::from(get_display_name_locale(parsed_typed_id, locale).unwrap_or(parsed_typed_id.to_string()))),
                ("disambiguation", FluentValue::from(*disambiguation_str)),
            ])
        )
    } else {
        env.locales.lookup_with_args(
            lang_id,
            "autocomplete_display",
            &HashMap::from([
                ("display", FluentValue::from(get_display_name_locale(parsed_typed_id, locale).unwrap_or(parsed_typed_id.to_string()))),
            ])
        )
    }
}

pub fn get_display_name_locale(
    typed_id: &ParsedTypedId,
    locale: &Locale
) -> Option<String> {
    match typed_id.0 {
        PageType::AbnoPage => {
            get_abno_page_locales_by_internal_name(&typed_id.1).get(locale).map(|x| x.card_name.to_string())
        }
        PageType::BattleSymbol => {
            get_battle_symbol_locales_by_internal_name(&typed_id.1).get(locale).map(|x| format!("{} {}", x.prefix, x.postfix))
        }
        PageType::CombatPage => {
            get_combat_page_locales_by_id(&typed_id.1).get(locale).map(|x| x.name.to_string())
        }
        PageType::KeyPage => {
            get_key_page_by_id(&typed_id.1).and_then(|key_page| {
                key_page.text_id.map(|text_id| {
                    get_key_page_locales_by_text_id(text_id)
                        .get(locale)
                        .map(|key_page_locale| {
                            key_page_locale.name.to_string()
                        })
                }).or(key_page.skin.map(|skin| Some(skin.to_string()))).flatten()
            })
        }
        PageType::Passive => {
            get_passive_locales_by_id(&typed_id.1).get(locale).map(|x| x.name.to_string())
        }
    }
}

pub fn parse_tiph_deck_id(raw_input: &str) -> String {
    let url = "https://tiphereth.zasz.su/u/decks";
    let mut ret_val: String = raw_input.to_string();
    if ret_val.starts_with(url) {
        ret_val = ret_val[url.len()..ret_val.len()].to_string();
    }
    if ret_val.starts_with('/') {
        ret_val = ret_val[1..ret_val.len()].to_string();
    }
    if ret_val.ends_with('/') {
        ret_val = ret_val[0..ret_val.len() - 1].to_string();
    }
    ret_val
}

pub fn build_error_message_response(lang_id: &LanguageIdentifier, err_code: &str, env: &BinahBotEnvironment) -> MessageResponse {
    MessageResponse {
        r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
        data: Some(DiscordInteractionResponseMessage {
            allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
            content: None,
            embeds: Some(vec![
                DiscordEmbed {
                    title: None,
                    description: Some(env.locales.lookup(lang_id, err_code)),
                    color: Some(DiscordEmbedColors::Default as i32),
                    image: None,
                    footer: None,
                    author: None,
                    url: None,
                    fields: None
                }
            ]),
            flags: Some(DiscordMessageFlag::EphemeralMessage as i32)
        })
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

    #[test]
    fn sanity_parse_tiph_deck_id() {
        let inputs = vec![
            "CS-iRmsieV9ddwW4-BA1C~n",
            "https://tiphereth.zasz.su/u/decks/CS-iRmsieV9ddwW4-BA1C~n/",
            "https://tiphereth.zasz.su/u/decks/CS-iRmsieV9ddwW4-BA1C~n",
            "/CS-iRmsieV9ddwW4-BA1C~n/",
            "CS-iRmsieV9ddwW4-BA1C~n/",
            "/CS-iRmsieV9ddwW4-BA1C~n",
        ];
        let expected_output = "CS-iRmsieV9ddwW4-BA1C~n";
        
        for str in inputs {
            assert_eq!(expected_output, parse_tiph_deck_id(str));
        }
    }

    #[test]
    #[ignore]
    fn check_no_lookup_display_collisions() {
        Locale::iter().for_each(|locale: Locale| {
            let mut lookups: HashMap<String, Vec<ParsedTypedId>> = HashMap::new();            

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
                // todo: move this file somewhere else
                // .filter(is_collectable_or_obtainable)
                .filter(|x| get_display_name_locale(x, &locale).is_some())
                .for_each(|x| {
                    let display = get_display_name_locale(&x, &locale);
                    let disambig_entry = get_disambiguation(&x, &locale);
                    let fmt = format!("{}#{}", display.unwrap_or("".to_string()), disambig_entry.cloned().unwrap_or(""));
                    lookups.entry(fmt).and_modify(|v| { v.push(x.clone()) }).or_insert(vec![x.clone()]);
                });

            let mut failures = HashMap::new();

            lookups.iter().for_each(|(display, vec)| {
                if vec.len() != 1 {
                    failures.insert(display, vec);
                }
            });

            assert!(failures.is_empty());
        });
    }
}