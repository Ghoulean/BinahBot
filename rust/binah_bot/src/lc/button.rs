use std::collections::HashMap;
use std::iter;
use std::str::FromStr;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use lobocorp::lobocorp_common::game_objects::abnormality::EncyclopediaInfo;
use lobocorp::lobocorp_common::localizations::common::Locale;
use lobocorp::lobocorp_common::localizations::equipment::LocalizationKey;
use lobocorp::lobocorp_reparser::get_abno_localization;
use lobocorp::lobocorp_reparser::get_encyclopedia_info;
use lobocorp::lobocorp_reparser::get_localization;
use unic_langid::LanguageIdentifier;

use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::discord::ActionRowComponent;
use crate::models::discord::AllowedMentions;
use crate::models::discord::ButtonComponent;
use crate::models::discord::ButtonStyle;
use crate::models::discord::DiscordComponent;
use crate::models::discord::DiscordComponentType;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionData;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::MessageResponse;
use crate::utils::build_delete_button_component;
use crate::utils::get_binahbot_locale;

use super::transformers::transform_breaching_entity;
use super::transformers::transform_donttouchme;
use super::transformers::transform_gift;
use super::transformers::transform_normal_info;
use super::transformers::transform_normal_info_managerial_guidance;
use super::transformers::transform_suit;
use super::transformers::transform_tool_info;
use super::transformers::transform_weapon;

// format: lc#<code>#<id>#<locale>#<index>
pub const LC_BUTTON_PREFIX: &str = "lc#";

pub const ENCYCLOPEDIA_MAIN_PAGE_INDEX: usize = 0;
pub const ENCYCLOPEDIA_MANAGERIAL_GUIDANCE_INDEX: usize = 1;

#[derive(Debug, PartialEq, strum_macros::Display, strum::EnumString)]
pub enum Code {
    #[strum(serialize = "e")]
    Encyclopedia,
    #[strum(serialize = "w")]
    Weapon,
    #[strum(serialize = "s")]
    Suit,
    #[strum(serialize = "g")]
    Gift,
    #[strum(serialize = "b")]
    BreachingEntity,
}

const SEPERATOR: &str = "#";

pub fn lc_button(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> MessageResponse {
    let data = match interaction.data.as_ref().expect("no data") {
        DiscordInteractionData::MessageComponent(x) => x,
        _ => unreachable!()
    };
    let custom_id = &data.custom_id;
    if !custom_id.starts_with(LC_BUTTON_PREFIX) {
        panic!("custom id did not start with {}", LC_BUTTON_PREFIX);
    };

    let binahbot_locale = get_binahbot_locale(&interaction);
    let (code, id, locale, index) = parse_custom_id(&custom_id);
    let entry = get_encyclopedia_info(&id).expect("couldn't find entry");

    let embed = match (&code, &entry, &index) {
        (Code::Encyclopedia, EncyclopediaInfo::Normal(x), &ENCYCLOPEDIA_MAIN_PAGE_INDEX) => {
            transform_normal_info(&x, &locale, &binahbot_locale, env)
        },
        (Code::Encyclopedia, EncyclopediaInfo::Normal(x), &ENCYCLOPEDIA_MANAGERIAL_GUIDANCE_INDEX) => {
            transform_normal_info_managerial_guidance(&x, &locale, &binahbot_locale, env)
        },
        (Code::Encyclopedia, EncyclopediaInfo::Tool(x), _) => {
            transform_tool_info(&x, &locale, &binahbot_locale, env)
        },
        (Code::Encyclopedia, EncyclopediaInfo::DontTouchMe(x), _) => {
            transform_donttouchme(&x, &locale, &binahbot_locale, env)
        },
        (Code::Weapon, EncyclopediaInfo::Normal(x), _) => {
            transform_weapon(
                    &x.weapon.as_ref().expect("no weapon"),
                    &locale,
                    &binahbot_locale,
                    env
            )
        },
        (Code::Suit, EncyclopediaInfo::Normal(x), _) => {
            transform_suit(
                &x.suit.as_ref().expect("no suit"),
                &locale,
                &binahbot_locale,
                env
            )
        },
        (Code::Gift, EncyclopediaInfo::Normal(x), _) => {
            transform_gift(
                &x.gifts.get(index).expect("bad gift index"),
                &locale,
                &binahbot_locale,
                env
            )
        },
        (Code::BreachingEntity, EncyclopediaInfo::Normal(x), _) => {
            let localization = get_abno_localization(&id, &locale).expect("no localization found");
            transform_breaching_entity(
                &x.breaching_entities.get(index).expect("bad breaching entity index"),
                &localization.breaching_entity_localizations.get(index).expect("bad breaching entity index"),
                &binahbot_locale,
                env
            )
        },
        (Code::BreachingEntity, EncyclopediaInfo::Tool(_), _) => todo!(), // yang
        _ => panic!("encountered unexpected custom_id format"),
    };

    let components = build_buttons(
        id, &locale, &binahbot_locale, &(code, index), env
    );

    MessageResponse {
        r#type: DiscordInteractionResponseType::UpdateMessage,
        data: Some(DiscordInteractionResponseMessage {
            allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
            content: None,
            embeds: Some(vec![embed]),
            flags: None, // todo
            components: Some(components),
        }),
    }
}

pub fn build_buttons(
    id: u32,
    abno_locale: &Locale,
    request_locale: &BinahBotLocale,
    current: &(Code, usize),
    env: &BinahBotEnvironment
) -> Vec<DiscordComponent> {
    let entry = get_encyclopedia_info(&id).expect("couldn't find entry");
    let locale_info = get_abno_localization(&id, abno_locale).expect("invalid id-locale pair");
    let lang_id: LanguageIdentifier = request_locale.into();

    let encyclopedia_button = ButtonComponent {
        r#type: DiscordComponentType::Button,
        style: ButtonStyle::Primary,
        label: Some(locale_info.name.to_string()),
        custom_id: Some(build_custom_id(&Code::Encyclopedia, &id, abno_locale, &ENCYCLOPEDIA_MAIN_PAGE_INDEX)),
        disabled: Some(current.0 == Code::Encyclopedia && current.1 == ENCYCLOPEDIA_MAIN_PAGE_INDEX),
    };

    let additional_buttons = match entry {
        EncyclopediaInfo::Normal(x) => {
            let mut vec = Vec::new();

            vec.push(ButtonComponent {
                r#type: DiscordComponentType::Button,
                style: ButtonStyle::Primary,
                label: Some(env.locales.lookup(&lang_id, "managerial_guidance_button_label")),
                custom_id: Some(build_custom_id(&Code::Encyclopedia, &id, abno_locale, &ENCYCLOPEDIA_MANAGERIAL_GUIDANCE_INDEX)),
                disabled: Some(current.0 == Code::Encyclopedia && current.1 == ENCYCLOPEDIA_MANAGERIAL_GUIDANCE_INDEX),
            });

            x.breaching_entities.iter().enumerate().for_each(|(i, _)| {
                let breaching_locale = locale_info.breaching_entity_localizations.get(i).expect("couldn't get breaching entity locale");
                let label = if i == 0 && x.is_breachable {
                    env.locales.lookup(&lang_id, "breaching_button_label")
                } else {
                    env.locales.lookup_with_args(
                        &lang_id,
                        "breaching_child_button_label",
                        &HashMap::from([
                            ("name", FluentValue::from(breaching_locale.name)),
                        ])
                    )
                };

                vec.push(ButtonComponent {
                    r#type: DiscordComponentType::Button,
                    style: ButtonStyle::Primary,
                    label: Some(label),
                    custom_id: Some(build_custom_id(&Code::BreachingEntity, &id, abno_locale, &i)),
                    disabled: Some(current.0 == Code::BreachingEntity && current.1 == i),
                })
            });
            x.weapon.as_ref().inspect(|x| {
                let label = get_localization(&LocalizationKey(x.name_id, abno_locale.clone()))
                    .map(|x| {
                        env.locales.lookup_with_args(
                            &lang_id,
                            "weapon_button_label",
                            &HashMap::from([
                                ("name", FluentValue::from(*x)),
                            ])
                        )
                    });
                
                vec.push(ButtonComponent {
                    r#type: DiscordComponentType::Button,
                    style: ButtonStyle::Primary,
                    label: label,
                    custom_id: Some(build_custom_id(&Code::Weapon, &id, abno_locale, &0)),
                    disabled: Some(current.0 == Code::Weapon),
                });
            });
            x.suit.as_ref().inspect(|x| {
                let label = get_localization(&LocalizationKey(x.name_id, abno_locale.clone()))
                    .map(|x| {
                        env.locales.lookup_with_args(
                            &lang_id,
                            "suit_button_label",
                            &HashMap::from([
                                ("name", FluentValue::from(*x)),
                            ])
                        )
                    });

                vec.push(ButtonComponent {
                    r#type: DiscordComponentType::Button,
                    style: ButtonStyle::Primary,
                    label: label,
                    custom_id: Some(build_custom_id(&Code::Suit, &id, abno_locale, &0)),
                    disabled: Some(current.0 == Code::Suit),
                });
            });
            x.gifts.iter().enumerate().for_each(|(i, x)| {
                let label = get_localization(&LocalizationKey(x.name_id, abno_locale.clone()))
                    .map(|x| {
                        env.locales.lookup_with_args(
                            &lang_id,
                            "gift_button_label",
                            &HashMap::from([
                                ("name", FluentValue::from(*x)),
                            ])
                        )
                    });

                vec.push(ButtonComponent {
                    r#type: DiscordComponentType::Button,
                    style: ButtonStyle::Primary,
                    label: label,
                    custom_id: Some(build_custom_id(&Code::Gift, &id, abno_locale, &i)),
                    disabled: Some(current.0 == Code::Gift && current.1 == i),
                })
            });
            vec
        },
        _ => vec![]
    };

    let delete_button = build_delete_button_component(&lang_id, env);

    let all_buttons = iter::once(encyclopedia_button)
        .chain(additional_buttons)
        .chain(iter::once(delete_button))
        .map(|x| DiscordComponent::Button(x))
        .collect::<Vec<_>>();

    all_buttons.chunks(5).map(|x| {
        DiscordComponent::ActionRow(ActionRowComponent {
            r#type: DiscordComponentType::ActionRow,
            components: x.to_vec()
        })
    }).collect::<Vec<_>>()
}

fn parse_custom_id(custom_id: &str) -> (Code, u32, Locale, usize) {
    let vec = custom_id.split(SEPERATOR)
        .collect::<Vec<&str>>();
    // discard lc# prefix
    (
        vec.get(1).and_then(|x| Code::from_str(x).ok()).expect("couldn't get code"),
        vec.get(2).and_then(|x| x.parse::<u32>().ok()).expect("couldn't get id"),
        vec.get(3).and_then(|x| Locale::from_str(x).ok()).expect("couldn't get locale"),
        vec.get(4).and_then(|x| x.parse::<usize>().ok()).expect("couldn't get index"),
    )
}

fn build_custom_id(code: &Code, id: &u32, locale: &Locale, index: &usize) -> String {
    format!("{}{}#{}#{}#{}", LC_BUTTON_PREFIX, code.to_string(), id, locale.to_string(), index)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_parse_custom_id() {
        let str = "lc#e#100038#en#0";
        assert_eq!(
            (Code::Encyclopedia, 100038, Locale::English, 0),
            parse_custom_id(str)
        );
    }
}