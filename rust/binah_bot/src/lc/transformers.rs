use std::cmp;
use std::collections::HashMap;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use lobocorp::lobocorp_common::game_objects::abnormality::BreachingEntity;
use lobocorp::lobocorp_common::game_objects::abnormality::DontTouchMeInfo;
use lobocorp::lobocorp_common::game_objects::abnormality::NormalInfo;
use lobocorp::lobocorp_common::game_objects::abnormality::ToolInfo;
use lobocorp::lobocorp_common::game_objects::abnormality::WorkProbabilities;
use lobocorp::lobocorp_common::game_objects::common::DamageType;
use lobocorp::lobocorp_common::game_objects::common::Defenses;
use lobocorp::lobocorp_common::game_objects::common::Resistance;
use lobocorp::lobocorp_common::game_objects::common::ResistanceCategories;
use lobocorp::lobocorp_common::game_objects::common::RiskLevel;
use lobocorp::lobocorp_common::game_objects::common::Stat;
use lobocorp::lobocorp_common::game_objects::common::StatBonus;
use lobocorp::lobocorp_common::game_objects::equipment::EquipRequirement;
use lobocorp::lobocorp_common::game_objects::equipment::EquipRequirementKey;
use lobocorp::lobocorp_common::game_objects::equipment::Gift;
use lobocorp::lobocorp_common::game_objects::equipment::Slot;
use lobocorp::lobocorp_common::game_objects::equipment::Suit;
use lobocorp::lobocorp_common::game_objects::equipment::Weapon;
use lobocorp::lobocorp_common::game_objects::equipment::WeaponAttackSpeed;
use lobocorp::lobocorp_common::game_objects::equipment::WeaponAttackSpeedCategories;
use lobocorp::lobocorp_common::game_objects::equipment::WeaponDamageType;
use lobocorp::lobocorp_common::game_objects::equipment::WeaponRange;
use lobocorp::lobocorp_common::game_objects::equipment::WeaponRangeCategories;
use lobocorp::lobocorp_common::localizations::abnormality::BreachingEntityLocalization;
use lobocorp::lobocorp_common::localizations::common::Locale;
use lobocorp::lobocorp_common::localizations::equipment::LocalizationKey;
use lobocorp::lobocorp_reparser::get_abno_localization;
use lobocorp::lobocorp_reparser::get_localization;
use unic_langid::LanguageIdentifier;

use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordEmbedFields;
use crate::models::discord::DiscordEmbedFooter;
use crate::models::discord::DiscordEmbedImage;
use crate::models::discord::DiscordEmbedThumbnail;

pub fn transform_normal_info(
    entry: &NormalInfo,
    locale: &Locale,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    let locale_info = get_abno_localization(&entry.id, locale).expect("invalid id-locale pair");
    let lang_id = request_locale.into();

    let fields = vec![
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "risk_level_header"),
            value: format_risk_level(&entry.risk, env),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "qliphoth_counter_header"),
            value: entry
                .qliphoth_counter
                .map(|x| x.to_string())
                .unwrap_or("-".to_string()),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "work_damage_header"),
            value: format!(
                "{} {} - {}",
                get_damage_emoji(&entry.work_damage_type, env).unwrap_or(&"-".to_string()),
                &entry.work_damage_range.0,
                &entry.work_damage_range.1
            ),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "happiness_range_header"),
            value: format_happiness_ranges(&entry.work_happiness_ranges, env),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "work_probabilities_header"),
            value: format_work_probabilities(&entry.work_probabilities, env),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "defenses_header"),
            value: if entry.is_breachable {
                format_defenses(&entry.defenses, &lang_id, env)
            } else {
                env.locales.lookup(&lang_id, "non_breachable_entity_value")
            },
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "observation_level_header"),
            value: format_observation_levels(&entry.observation_level_bonuses, &lang_id, env),
            inline: Some(true),
        },
    ];

    let image_url = get_portrait_url(entry.id, env);

    DiscordEmbed {
        title: Some(env.locales.lookup_with_args(
            &lang_id,
            "encyclopedia_title_format",
            &HashMap::from([
                ("name", FluentValue::from(locale_info.name)),
                ("code", FluentValue::from(locale_info.code)),
            ]),
        )),
        description: locale_info.selection_text.map(|x| x.to_string()),
        color: Some(DiscordEmbedColors::from(&entry.risk) as i32),
        image: Some(DiscordEmbedImage { url: image_url }),
        thumbnail: None,
        footer: Some(DiscordEmbedFooter {
            text: format!("{} | {}", entry.id_str, entry.id),
            icon_url: None,
        }),
        author: None,
        url: None,
        fields: Some(fields),
    }
}

pub fn transform_normal_info_managerial_guidance(
    entry: &NormalInfo,
    locale: &Locale,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    let locale_info = get_abno_localization(&entry.id, locale).expect("invalid id-locale pair");
    let lang_id = request_locale.into();

    let fields = locale_info
        .managerial_guidances
        .iter()
        .enumerate()
        .map(|(i, x)| DiscordEmbedFields {
            name: env.locales.lookup_with_args(
                &lang_id,
                "managerial_guidance_header",
                &HashMap::from([("index", FluentValue::from(i + 1))]),
            ),
            value: x.replace("$0", locale_info.name),
            inline: Some(false),
        })
        .collect::<Vec<_>>();

    let thumbnail_url = get_portrait_url(entry.id, env);

    DiscordEmbed {
        title: Some(env.locales.lookup_with_args(
            &lang_id,
            "encyclopedia_title_format",
            &HashMap::from([
                ("name", FluentValue::from(locale_info.name)),
                ("code", FluentValue::from(locale_info.code)),
            ]),
        )),
        description: None,
        color: Some(DiscordEmbedColors::from(&entry.risk) as i32),
        image: None,
        thumbnail: Some(DiscordEmbedThumbnail { url: thumbnail_url }),
        footer: Some(DiscordEmbedFooter {
            text: format!("{} | {}", entry.id_str, entry.id),
            icon_url: None,
        }),
        author: None,
        url: None,
        fields: Some(fields),
    }
}

pub fn transform_tool_info(
    entry: &ToolInfo,
    locale: &Locale,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    let locale_info = get_abno_localization(&entry.id, locale).expect("invalid id-locale pair");
    let lang_id = request_locale.into();

    let mut fields = vec![DiscordEmbedFields {
        name: env.locales.lookup(&lang_id, "risk_level_header"),
        value: format_risk_level(&entry.risk, env),
        inline: Some(true),
    }];

    let entries = 0..cmp::max(
        locale_info.story.len(),
        locale_info.managerial_guidances.len(),
    );

    entries.for_each(|i| {
        let text = format!(
            "{}\n{}",
            locale_info
                .story
                .get(i)
                .map(|x| x.replace("$0", locale_info.name))
                .unwrap_or("-".to_string()),
            locale_info
                .managerial_guidances
                .get(i)
                .map(|x| x.replace("$0", locale_info.name))
                .unwrap_or("-".to_string()),
        );

        fields.push(DiscordEmbedFields {
            name: env.locales.lookup_with_args(
                &lang_id,
                "managerial_guidance_seconds_header",
                &HashMap::from([
                    ("index", FluentValue::from(i + 1)),
                    ("seconds", FluentValue::from(entry.seconds_unlock[i])),
                ]),
            ),
            value: text,
            inline: Some(false),
        });
    });

    let image_url = get_portrait_url(entry.id, env);

    DiscordEmbed {
        title: Some(env.locales.lookup_with_args(
            &lang_id,
            "encyclopedia_title_format",
            &HashMap::from([
                ("name", FluentValue::from(locale_info.name)),
                ("code", FluentValue::from(locale_info.code)),
            ]),
        )),
        description: locale_info.selection_text.map(|x| x.to_string()),
        color: Some(DiscordEmbedColors::from(&entry.risk) as i32),
        image: Some(DiscordEmbedImage { url: image_url }),
        thumbnail: None,
        footer: Some(DiscordEmbedFooter {
            text: format!("{} | {}", entry.id_str, entry.id),
            icon_url: None,
        }),
        author: None,
        url: None,
        fields: Some(fields),
    }
}

pub fn transform_donttouchme(
    entry: &DontTouchMeInfo,
    locale: &Locale,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    let locale_info = get_abno_localization(&entry.id, locale).expect("invalid id-locale pair");
    let lang_id = request_locale.into();

    let fields = vec![DiscordEmbedFields {
        name: env.locales.lookup(&lang_id, "risk_level_header"),
        value: format_risk_level(&entry.risk, env),
        inline: Some(true),
    }];

    let image_url = get_portrait_url(entry.id, env);

    DiscordEmbed {
        title: Some(env.locales.lookup_with_args(
            &lang_id,
            "encyclopedia_title_format",
            &HashMap::from([
                ("name", FluentValue::from(locale_info.name)),
                ("code", FluentValue::from(locale_info.code)),
            ]),
        )),
        description: locale_info.selection_text.map(|x| x.to_string()),
        color: Some(DiscordEmbedColors::from(&entry.risk) as i32),
        image: Some(DiscordEmbedImage { url: image_url }),
        thumbnail: None,
        footer: Some(DiscordEmbedFooter {
            text: format!("{} | {}", entry.id_str, entry.id),
            icon_url: None,
        }),
        author: None,
        url: None,
        fields: Some(fields),
    }
}

pub fn transform_weapon(
    weapon: &Weapon,
    locale: &Locale,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    let lang_id = LanguageIdentifier::from(request_locale);

    let mut fields = vec![
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "risk_level_header"),
            value: format_risk_level(&weapon.risk, env),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "equipment_cost_header"),
            value: weapon
                .cost
                .map(|x| x.to_string())
                .unwrap_or("-".to_string()), // todo: N/A string
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "equipment_max_amount_header"),
            value: weapon.max_collectable_amount.to_string(),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "weapon_damage_header"),
            value: format!(
                "{} {} - {}",
                get_weapon_damage_emoji(&weapon.damage_type, env).unwrap_or("-".to_string()),
                &weapon.damage_range.0,
                &weapon.damage_range.1
            ),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "weapon_attack_speed_header"),
            value: lookup_weapon_attack_speed_str(&lang_id, &weapon.attack_speed, env),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "weapon_attack_range_header"),
            value: lookup_weapon_range_str(&lang_id, &weapon.range, env),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env
                .locales
                .lookup(&lang_id, "equipment_observation_level_header"),
            value: weapon
                .observation_level
                .map(|x| x.to_string())
                .unwrap_or("-".to_string()), // todo: N/A string
            inline: Some(true),
        },
    ];

    if !weapon.equip_requirements.is_empty() {
        fields.push(DiscordEmbedFields {
            name: env
                .locales
                .lookup(&lang_id, "equipment_equip_requirement_header"),
            value: format_equip_requirements(weapon.equip_requirements, env),
            inline: Some(true),
        })
    }

    weapon
        .desc_id
        .and_then(|x| get_localization(&LocalizationKey(x, locale.clone())).map(|x| x.to_string()))
        .inspect(|x| {
            fields.push(DiscordEmbedFields {
                name: env.locales.lookup(&lang_id, "equipment_description_header"),
                value: x.to_string(),
                inline: None,
            })
        });
    weapon
        .special_desc_id
        .and_then(|x| get_localization(&LocalizationKey(x, locale.clone())).map(|x| x.to_string()))
        .inspect(|x| {
            fields.push(DiscordEmbedFields {
                name: env
                    .locales
                    .lookup(&lang_id, "equipment_ability_description_header"),
                value: x.to_string(),
                inline: None,
            })
        });

    let image_url = get_weapon_url(weapon.id, env);

    DiscordEmbed {
        title: get_localization(&LocalizationKey(weapon.name_id, locale.clone()))
            .map(|x| x.to_string()),
        description: None,
        color: Some(DiscordEmbedColors::from(&weapon.risk) as i32),
        image: Some(DiscordEmbedImage { url: image_url }),
        thumbnail: None,
        footer: Some(DiscordEmbedFooter {
            text: weapon.id.to_string(),
            icon_url: None,
        }),
        author: None,
        url: None,
        fields: Some(fields),
    }
}

pub fn transform_suit(
    suit: &Suit,
    locale: &Locale,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    let lang_id = LanguageIdentifier::from(request_locale);

    let mut fields = vec![
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "risk_level_header"),
            value: format_risk_level(&suit.risk, env),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "equipment_cost_header"),
            value: suit.cost.to_string(),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "equipment_max_amount_header"),
            value: suit.max_collectable_amount.to_string(),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env
                .locales
                .lookup(&lang_id, "equipment_observation_level_header"),
            value: suit.observation_level.to_string(),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "defenses_header"),
            value: format_defenses(&Some(suit.defenses.clone()), &lang_id, env),
            inline: Some(true),
        },
    ];

    if !suit.equip_requirements.is_empty() {
        fields.push(DiscordEmbedFields {
            name: env
                .locales
                .lookup(&lang_id, "equipment_equip_requirement_header"),
            value: format_equip_requirements(suit.equip_requirements, env),
            inline: Some(true),
        })
    }

    suit.desc_id
        .and_then(|x| get_localization(&LocalizationKey(x, locale.clone())).map(|x| x.to_string()))
        .inspect(|x| {
            fields.push(DiscordEmbedFields {
                name: env.locales.lookup(&lang_id, "equipment_description_header"),
                value: x.to_string(),
                inline: None,
            })
        });
    suit.special_desc_id
        .and_then(|x| get_localization(&LocalizationKey(x, locale.clone())).map(|x| x.to_string()))
        .inspect(|x| {
            fields.push(DiscordEmbedFields {
                name: env
                    .locales
                    .lookup(&lang_id, "equipment_ability_description_header"),
                value: x.to_string(),
                inline: None,
            })
        });

    let image_url = get_suit_url(suit.id, env);

    DiscordEmbed {
        title: get_localization(&LocalizationKey(suit.name_id, locale.clone()))
            .map(|x| x.to_string()),
        description: None,
        color: Some(DiscordEmbedColors::from(&suit.risk) as i32),
        image: Some(DiscordEmbedImage { url: image_url }),
        thumbnail: None,
        footer: Some(DiscordEmbedFooter {
            text: suit.id.to_string(),
            icon_url: None,
        }),
        author: None,
        url: None,
        fields: Some(fields),
    }
}

pub fn transform_gift(
    gift: &Gift,
    locale: &Locale,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    let lang_id = LanguageIdentifier::from(request_locale);

    let fields = vec![
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "gift_probability_header"),
            value: gift
                .obtain_probability
                .map(|x| (x * 100.0).round())
                .map(|x| format!("{}%", x))
                .unwrap_or("-".to_string()), // todo: N/A string
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "gift_slot_header"),
            value: lookup_slot_str(&lang_id, &gift.slot, env),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env
                .locales
                .lookup(&lang_id, "equipment_observation_level_header"),
            value: gift
                .observation_level
                .map(|x| x.to_string())
                .unwrap_or("-".to_string()), // todo: N/A string
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "equipment_description_header"),
            value: get_localization(&LocalizationKey(&gift.desc_id, locale.clone()))
                .map(|x| x.to_string())
                .unwrap_or("-".to_string()),
            inline: None,
        },
    ];

    let image_url = get_gift_url(gift.id, env);

    DiscordEmbed {
        title: get_localization(&LocalizationKey(gift.name_id, locale.clone()))
            .map(|x| x.to_string()),
        description: None,
        color: Some(DiscordEmbedColors::Default as i32),
        image: Some(DiscordEmbedImage { url: image_url }),
        thumbnail: None,
        footer: Some(DiscordEmbedFooter {
            text: gift.id.to_string(),
            icon_url: None,
        }),
        author: None,
        url: None,
        fields: Some(fields),
    }
}

pub fn transform_breaching_entity(
    breaching_entity: &BreachingEntity,
    breaching_entity_localization: &BreachingEntityLocalization,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    let lang_id = LanguageIdentifier::from(request_locale);

    let fields = vec![
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "risk_level_header"),
            value: format_risk_level(&breaching_entity.risk_level, env),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env
                .locales
                .lookup(&lang_id, "breaching_entity_health_header"),
            value: breaching_entity.hp.to_string(),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env
                .locales
                .lookup(&lang_id, "breaching_entity_movespeed_header"),
            value: breaching_entity.speed.to_string(),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "defenses_header"),
            value: format_defenses(&Some(breaching_entity.defenses.clone()), &lang_id, env),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env
                .locales
                .lookup(&lang_id, "breaching_entity_damage_type_header"),
            value: get_damage_emoji(&breaching_entity.damage_type, env)
                .unwrap_or(&breaching_entity.damage_type.to_string())
                .clone(),
            inline: Some(true),
        },
    ];

    let title = breaching_entity_localization
        .code
        .map(|x| {
            env.locales.lookup_with_args(
                &lang_id,
                "encyclopedia_title_format",
                &HashMap::from([
                    (
                        "name",
                        FluentValue::from(breaching_entity_localization.name),
                    ),
                    ("code", FluentValue::from(x)),
                ]),
            )
        })
        .unwrap_or(breaching_entity_localization.name.to_string());

    DiscordEmbed {
        title: Some(title),
        description: None,
        color: Some(DiscordEmbedColors::from(&breaching_entity.risk_level) as i32),
        image: None, // todo: upload image
        thumbnail: None,
        footer: Some(DiscordEmbedFooter {
            text: breaching_entity.id.to_string(),
            icon_url: None,
        }),
        author: None,
        url: None,
        fields: Some(fields),
    }
}

fn get_portrait_url(id: u32, env: &BinahBotEnvironment) -> String {
    format!(
        "https://{0}.s3.amazonaws.com/lc/portrait/{id}.png",
        env.s3_bucket_name
    )
}

fn get_weapon_url(id: u32, env: &BinahBotEnvironment) -> String {
    format!(
        "https://{0}.s3.amazonaws.com/lc/weapon/{id}.png?1",
        env.s3_bucket_name
    )
}

fn get_suit_url(id: u32, env: &BinahBotEnvironment) -> String {
    format!(
        "https://{0}.s3.amazonaws.com/lc/suit/{id}.png",
        env.s3_bucket_name
    )
}

fn get_gift_url(id: u32, env: &BinahBotEnvironment) -> String {
    format!(
        "https://{0}.s3.amazonaws.com/lc/gift/{id}.png",
        env.s3_bucket_name
    )
}

impl From<&RiskLevel> for DiscordEmbedColors {
    fn from(value: &RiskLevel) -> Self {
        match value {
            RiskLevel::Zayin => DiscordEmbedColors::Zayin,
            RiskLevel::Teth => DiscordEmbedColors::Teth,
            RiskLevel::He => DiscordEmbedColors::He,
            RiskLevel::Waw => DiscordEmbedColors::Waw,
            RiskLevel::Aleph => DiscordEmbedColors::Aleph,
        }
    }
}

fn get_damage_emoji<'a>(
    damage_type: &'a DamageType,
    env: &'a BinahBotEnvironment,
) -> Option<&'a String> {
    match damage_type {
        DamageType::Red => env.emojis.red_damage.as_ref(),
        DamageType::White => env.emojis.white_damage.as_ref(),
        DamageType::Black => env.emojis.black_damage.as_ref(),
        DamageType::Pale => env.emojis.pale_damage.as_ref(),
    }
}

fn get_weapon_damage_emoji<'a>(
    damage_type: &'a WeaponDamageType,
    env: &'a BinahBotEnvironment,
) -> Option<String> {
    match damage_type {
        WeaponDamageType::Of(x) => get_damage_emoji(&x, env).cloned(),
        WeaponDamageType::All => {
            if let (Some(r), Some(w), Some(b), Some(p)) = (
                &env.emojis.red_damage,
                &env.emojis.white_damage,
                &env.emojis.black_damage,
                &env.emojis.pale_damage,
            ) {
                Some(format!("{}{}{}{}", r, w, b, p))
            } else {
                None
            }
        }
    }
}

fn get_equip_requirement_emoji<'a>(
    key: &'a EquipRequirementKey,
    env: &'a BinahBotEnvironment,
) -> Option<&'a String> {
    match key {
        EquipRequirementKey::AgentLevel => env.emojis.agent.as_ref(),
        EquipRequirementKey::Fortitude => env.emojis.instinct.as_ref(),
        EquipRequirementKey::Prudence => env.emojis.insight.as_ref(),
        EquipRequirementKey::Temperance => env.emojis.attachment.as_ref(),
        EquipRequirementKey::Justice => env.emojis.repression.as_ref(),
    }
}

fn format_work_probabilities(
    work_probabilities: &WorkProbabilities,
    env: &BinahBotEnvironment,
) -> String {
    let missing_emoji_binding = "-".to_string();
    format!(
        "{}: {}\n{}: {}\n{}: {}\n{}: {}",
        env.emojis
            .instinct
            .as_ref()
            .unwrap_or(&missing_emoji_binding),
        format_work_probability(&work_probabilities.instinct),
        env.emojis
            .insight
            .as_ref()
            .unwrap_or(&missing_emoji_binding),
        format_work_probability(&work_probabilities.insight),
        env.emojis
            .attachment
            .as_ref()
            .unwrap_or(&missing_emoji_binding),
        format_work_probability(&work_probabilities.attachment),
        env.emojis
            .repression
            .as_ref()
            .unwrap_or(&missing_emoji_binding),
        format_work_probability(&work_probabilities.repression),
    )
}

fn format_work_probability(probabilities: &[f64; 5]) -> String {
    probabilities
        .map(|x| format!("{}", (x * 100.0).round()))
        .join("/")
}

fn format_happiness_ranges(ranges: &[i32; 3], env: &BinahBotEnvironment) -> String {
    [
        (ranges[1], ranges[2], &env.emojis.good_mood),
        (ranges[0], ranges[1], &env.emojis.normal_mood),
        (-1, ranges[0], &env.emojis.bad_mood),
    ]
    .iter()
    .flat_map(|(low, high, emoji)| {
        if low == high {
            None
        } else {
            Some(format_happiness_range(&(low + 1), high, emoji))
        }
    })
    .collect::<Vec<_>>()
    .join("\n")
}

fn format_defenses(
    defenses: &Option<Defenses>,
    lang_id: &LanguageIdentifier,
    env: &BinahBotEnvironment,
) -> String {
    format!(
        "{}\n{}\n{}\n{}",
        format_defense(
            defenses.as_ref().map(|x| &x.red),
            lang_id,
            &env.emojis.red_damage,
            env
        ),
        format_defense(
            defenses.as_ref().map(|x| &x.white),
            lang_id,
            &env.emojis.white_damage,
            env
        ),
        format_defense(
            defenses.as_ref().map(|x| &x.black),
            lang_id,
            &env.emojis.black_damage,
            env
        ),
        format_defense(
            defenses.as_ref().map(|x| &x.pale),
            lang_id,
            &env.emojis.pale_damage,
            env
        ),
    )
}

fn format_defense(
    resistance: Option<&Resistance>,
    lang_id: &LanguageIdentifier,
    emoji: &Option<String>,
    env: &BinahBotEnvironment,
) -> String {
    let label = resistance
        .as_ref()
        .map(|x| ResistanceCategories::from(x.0))
        .map(|x| lookup_defenses_str(lang_id, &x, env))
        .unwrap_or(env.locales.lookup(&lang_id, "unknown_defenses_value"));
    let val = resistance
        .as_ref()
        .map(|x| format!("{:.1}", x.0))
        .unwrap_or("-".to_string());

    format!(
        "{} {}",
        emoji.as_ref().unwrap_or(&"-".to_string()),
        env.locales.lookup_with_args(
            lang_id,
            "resistances_format",
            &HashMap::from([
                ("label", FluentValue::from(label)),
                ("val", FluentValue::from(val)),
            ])
        )
    )
}

fn lookup_defenses_str(
    lang_id: &LanguageIdentifier,
    resistance_category: &ResistanceCategories,
    env: &BinahBotEnvironment,
) -> String {
    let lookup_key = match resistance_category {
        ResistanceCategories::Absorb => "absorb_defenses_value",
        ResistanceCategories::Immune => "immune_defenses_value",
        ResistanceCategories::Resistant => "resistant_defenses_value",
        ResistanceCategories::Endured => "endured_defenses_value",
        ResistanceCategories::Normal => "normal_defenses_value",
        ResistanceCategories::Weak => "weak_defenses_value",
        ResistanceCategories::Vulnerable => "vulnerable_defenses_value",
    };
    env.locales.lookup(&lang_id, lookup_key)
}

fn format_happiness_range(low: &i32, high: &i32, emoji: &Option<String>) -> String {
    let binding = "".to_string();
    format!("{} {} - {}", emoji.as_ref().unwrap_or(&binding), low, high)
}

fn format_observation_levels(
    observation_levels: &[StatBonus; 4],
    lang_id: &LanguageIdentifier,
    env: &BinahBotEnvironment,
) -> String {
    observation_levels
        .iter()
        .enumerate()
        .map(|(i, x)| format!("{}. {}", i + 1, format_observation_level(&x, lang_id, env)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_observation_level(
    observation_levels: &StatBonus,
    lang_id: &LanguageIdentifier,
    env: &BinahBotEnvironment,
) -> String {
    let key = match observation_levels.0 {
        Stat::SuccessRate => "observation_level_success_rate",
        Stat::WorkSpeed => "observation_level_speed_rate",
        _ => unreachable!(),
    };
    env.locales.lookup_with_args(
        lang_id,
        key,
        &HashMap::from([("percentage", FluentValue::from(observation_levels.1))]),
    )
}

fn format_equip_requirements(
    equip_requirements: &[EquipRequirement],
    env: &BinahBotEnvironment,
) -> String {
    equip_requirements
        .iter()
        .map(|x| {
            format!(
                "{} {}",
                get_equip_requirement_emoji(&x.0, env).unwrap_or(&"?".to_string()),
                x.1
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn format_risk_level(risk_level: &RiskLevel, env: &BinahBotEnvironment) -> String {
    let binding = match risk_level {
        RiskLevel::Zayin => &env.emojis.risk_zayin,
        RiskLevel::Teth => &env.emojis.risk_teth,
        RiskLevel::He => &env.emojis.risk_he,
        RiskLevel::Waw => &env.emojis.risk_waw,
        RiskLevel::Aleph => &env.emojis.risk_aleph,
    };
    let binding2 = &"".to_string();
    let emoji = binding.as_ref().unwrap_or(binding2);
    format!("{}{}", emoji, risk_level.to_string())
}

fn lookup_weapon_range_str(
    lang_id: &LanguageIdentifier,
    weapon_range: &WeaponRange,
    env: &BinahBotEnvironment,
) -> String {
    let category = WeaponRangeCategories::from(weapon_range);
    let key = match category {
        WeaponRangeCategories::VeryLong => "weapon_range_verylong",
        WeaponRangeCategories::Long => "weapon_range_long",
        WeaponRangeCategories::Medium => "weapon_range_medium",
        WeaponRangeCategories::Short => "weapon_range_short",
        WeaponRangeCategories::VeryShort => "weapon_range_veryshort",
    };
    env.locales.lookup_with_args(
        lang_id,
        key,
        &HashMap::from([("range", FluentValue::from(weapon_range.0))]),
    )
}

fn lookup_weapon_attack_speed_str(
    lang_id: &LanguageIdentifier,
    weapon_attack_speed: &WeaponAttackSpeed,
    env: &BinahBotEnvironment,
) -> String {
    let category = WeaponAttackSpeedCategories::from(weapon_attack_speed);
    let key = match category {
        WeaponAttackSpeedCategories::VeryFast => "weapon_attack_speed_veryfast",
        WeaponAttackSpeedCategories::Fast => "weapon_attack_speed_fast",
        WeaponAttackSpeedCategories::Normal => "weapon_attack_speed_normal",
        WeaponAttackSpeedCategories::Slow => "weapon_attack_speed_slow",
        WeaponAttackSpeedCategories::VerySlow => "weapon_attack_speed_veryslow",
    };
    env.locales.lookup_with_args(
        lang_id,
        key,
        &HashMap::from([("speed", FluentValue::from(weapon_attack_speed.0))]),
    )
}

fn lookup_slot_str(lang_id: &LanguageIdentifier, slot: &Slot, env: &BinahBotEnvironment) -> String {
    let key = match slot {
        Slot::Brooch => "slot_brooch",
        Slot::Cheek => "slot_cheek",
        Slot::Eye => "slot_eye",
        Slot::Face => "slot_face",
        Slot::Hand1 => "slot_hand1",
        Slot::Hand2 => "slot_hand2",
        Slot::Hat => "slot_hat",
        Slot::Helmet => "slot_helmet",
        Slot::LeftBack => "slot_leftback",
        Slot::Mouth1 => "slot_mouth1",
        Slot::Mouth2 => "slot_mouth2",
        Slot::Neckwear => "slot_neckwear",
        Slot::RightBack => "slot_rightback",
        Slot::Special => "slot_special",
    };
    env.locales.lookup(lang_id, key)
}

impl From<RiskLevel> for DiscordEmbedColors {
    fn from(value: RiskLevel) -> Self {
        DiscordEmbedColors::from(&value)
    }
}

#[cfg(test)]
mod tests {
    use lobocorp::lobocorp_common::game_objects::abnormality::EncyclopediaInfo;
    use lobocorp::lobocorp_reparser::get_all_encyclopedia_ids;
    use lobocorp::lobocorp_reparser::get_encyclopedia_info;

    use crate::test_utils::build_mocked_binahbot_env;

    use super::*;

    #[test]
    fn correct_breachability_display() {
        // normal abnos only
        let env = &build_mocked_binahbot_env();
        let non_breachable = [
            100053, 100009, 100028, 100014, 100007, 100013, 100027, 100037, 100059, 100103, 100002,
            100041, 100017, 100045,
        ];
        let breaching_no_defenses = [100005, 100019];
        let breaching_with_defenses = [
            100015, 100018, 100036, 100054, 100043, 100011, 100057, 100029, 100033, 100008, 100035,
            100055, 100061, 100038, 100058, 100064, 100056, 100063, 100042,
        ];

        get_all_encyclopedia_ids()
            .iter()
            .flat_map(|x| get_encyclopedia_info(&x))
            .flat_map(|x| match x {
                EncyclopediaInfo::Normal(x) => Some(x),
                _ => None,
            })
            .map(|x| {
                (
                    x,
                    transform_normal_info(&x, &Locale::English, &BinahBotLocale::EnglishUS, env),
                )
            })
            .for_each(|(entry, embed)| {
                let lang_id: LanguageIdentifier = BinahBotLocale::EnglishUS.into();
                let id = entry.id;
                let name = env.locales.lookup(&lang_id, "defenses_header");
                let binding = embed.fields.expect("no fields");
                let defense_field = binding
                    .iter()
                    .find(|x| x.name == name)
                    .expect("no defense field");
                dbg!(&id);
                if non_breachable.contains(&id) {
                    assert_eq!(
                        env.locales.lookup(&lang_id, "non_breachable_entity_value"),
                        defense_field.value
                    );
                } else if breaching_no_defenses.contains(&id) {
                    assert_eq!(4, defense_field.value.split("\n").collect::<Vec<_>>().len());
                    assert!(defense_field
                        .value
                        .contains(&env.locales.lookup(&lang_id, "unknown_defenses_value")));
                } else if breaching_with_defenses.contains(&id) {
                    assert_eq!(4, defense_field.value.split("\n").collect::<Vec<_>>().len());
                }
            })
    }
}
