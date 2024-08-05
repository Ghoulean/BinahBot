use std::collections::HashMap;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use lobocorp::lobocorp_common::game_objects::abnormality::EncyclopediaInfo;
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
use lobocorp::lobocorp_common::localizations::abnormality::EncyclopediaInfoLocalization;
use lobocorp::lobocorp_common::localizations::common::Locale;
use lobocorp::lobocorp_reparser::get_abno_localization;
use lobocorp::lobocorp_reparser::get_encyclopedia_info;
use unic_langid::LanguageIdentifier;

use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordEmbedFields;
use crate::models::discord::DiscordEmbedFooter;

static NOT_FOUND_IMAGE_NAME: &str = "404_Not_Found";

pub fn transform_encyclopedia_page(
    id: &u32,
    abno_locale: &Locale,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment
) -> DiscordEmbed {
    let encyclopedia = get_encyclopedia_info(id).expect("invalid id");
    let locale_info = get_abno_localization(id, abno_locale).expect("invalid id-locale pair");
    let lang_id = abno_locale.into();

    match encyclopedia {
        EncyclopediaInfo::Normal(x) => transform_normal_info(&x, &locale_info, &lang_id, env),
        EncyclopediaInfo::Tool(x) => transform_tool_info(&x, &locale_info, &lang_id, env),
        EncyclopediaInfo::DontTouchMe => todo!(),
    }
}

fn transform_normal_info(
    entry: &NormalInfo,
    locale_info: &EncyclopediaInfoLocalization,
    lang_id: &LanguageIdentifier,
    env: &BinahBotEnvironment
) -> DiscordEmbed {
    let mut fields = vec![
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "risk_level_header"),
            value: entry.risk.to_string(), // todo: localize
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "qliphoth_counter_header"),
            value: entry.qliphoth_counter.map(|x| x.to_string()).unwrap_or("-".to_string()),
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
                format_defenses(&entry.defenses, lang_id, env)
            } else {
                env.locales.lookup(&lang_id, "non_breachable_entity_value")
            },
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "observation_level_header"),
            value: format_observation_levels(
                &entry.observation_level_bonuses,
                lang_id,
                env
            ),
            inline: Some(true),
        },
    ];

    locale_info.managerial_guidances.iter().enumerate().for_each(|(i, x)| {
        fields.push(DiscordEmbedFields {
            name: env.locales.lookup_with_args(
                lang_id,
                "managerial_guidance_header",
                &HashMap::from([
                    ("index", FluentValue::from(i + 1)),
                ])
            ),
            value: x.replace("$0", locale_info.name),
            inline: Some(false),
        });
    });

    DiscordEmbed {
        title: Some(env.locales.lookup_with_args(
            lang_id,
            "encyclopedia_title_format",
            &HashMap::from([
                ("name", FluentValue::from(locale_info.name)),
                ("code", FluentValue::from(locale_info.code)),
            ])
        )),
        description: locale_info.selection_text.map(|x| x.to_string()),
        color: Some(DiscordEmbedColors::from(&entry.risk) as i32),
        image: None, // todo: image
        footer: Some(DiscordEmbedFooter {
            text: entry.id.to_string(),
            icon_url: None,
        }),
        author: None,
        url: None,
        fields: Some(fields),
    }
}

fn transform_tool_info(
    entry: &ToolInfo,
    locale_info: &EncyclopediaInfoLocalization,
    lang_id: &LanguageIdentifier,
    env: &BinahBotEnvironment
) -> DiscordEmbed {
    DiscordEmbed {
        title: Some(locale_info.name.to_string()),
        description: locale_info.selection_text.map(|x| x.to_string()),
        color: Some(DiscordEmbedColors::from(&entry.risk) as i32),
        image: todo!(),
        footer: todo!(),
        author: todo!(),
        url: todo!(),
        fields: todo!(),
    }
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

fn get_damage_emoji<'a>(damage_type: &'a DamageType, env: &'a BinahBotEnvironment) -> Option<&'a String> {
    match damage_type {
        DamageType::Red => env.emojis.red_damage.as_ref(),
        DamageType::White => env.emojis.white_damage.as_ref(),
        DamageType::Black => env.emojis.black_damage.as_ref(),
        DamageType::Pale => env.emojis.pale_damage.as_ref(),
    }
}

fn format_work_probabilities(work_probabilities: &WorkProbabilities, env: &BinahBotEnvironment) -> String {
    let missing_emoji_binding = "-".to_string();
    format!(
        "{}: {}\n{}: {}\n{}: {}\n{}: {}",
        env.emojis.instinct.as_ref().unwrap_or(&missing_emoji_binding),
        format_work_probability(&work_probabilities.instinct),
        env.emojis.insight.as_ref().unwrap_or(&missing_emoji_binding),
        format_work_probability(&work_probabilities.insight),
        env.emojis.attachment.as_ref().unwrap_or(&missing_emoji_binding),
        format_work_probability(&work_probabilities.attachment),
        env.emojis.repression.as_ref().unwrap_or(&missing_emoji_binding),
        format_work_probability(&work_probabilities.repression),
    )
}

fn format_work_probability(probabilities: &[f64; 5]) -> String {
    probabilities.map(|x| format!("{}", (x * 100.0).round())).join("/")
}

fn format_happiness_ranges(ranges: &[i32; 3], env: &BinahBotEnvironment) -> String {
    [
        (ranges[1], ranges[2], &env.emojis.good_mood),
        (ranges[0], ranges[1], &env.emojis.normal_mood),
        (-1, ranges[0], &env.emojis.bad_mood),
    ].iter().flat_map(|(low, high, emoji)| {
        if low == high {
            None
        } else {
            Some(format_happiness_range(&(low + 1), high, emoji))
        }
    }).collect::<Vec<_>>()
    .join("\n")

}

fn format_defenses(defenses: &Option<Defenses>, lang_id: &LanguageIdentifier, env: &BinahBotEnvironment) -> String {
    format!(
        "{}\n{}\n{}\n{}",
        format_defense(defenses.as_ref().map(|x| &x.red), lang_id, &env.emojis.red_damage, env),
        format_defense(defenses.as_ref().map(|x| &x.white), lang_id, &env.emojis.white_damage, env),
        format_defense(defenses.as_ref().map(|x| &x.black), lang_id, &env.emojis.black_damage, env),
        format_defense(defenses.as_ref().map(|x| &x.pale), lang_id, &env.emojis.pale_damage, env),
    )
}

fn format_defense(resistance: Option<&Resistance>, lang_id: &LanguageIdentifier, emoji: &Option<String>, env: &BinahBotEnvironment) -> String {
    let label = resistance.as_ref().map(|x| ResistanceCategories::from(x.0))
        .map(|x| lookup_defenses_str(lang_id, &x, env))
        .unwrap_or(env.locales.lookup(&lang_id, "unknown_defenses_value"));
    let val = resistance.as_ref().map(|x| x.0.to_string()).unwrap_or("-".to_string());

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

fn lookup_defenses_str(lang_id: &LanguageIdentifier, resistance_category: &ResistanceCategories, env: &BinahBotEnvironment) -> String {
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

fn format_observation_levels(observation_levels: &[StatBonus; 4], lang_id: &LanguageIdentifier, env: &BinahBotEnvironment) -> String {
    observation_levels.iter()
        .enumerate()
        .map(|(i, x)| format!("{}. {}", i + 1, format_observation_level(&x, lang_id, env)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_observation_level(observation_levels: &StatBonus, lang_id: &LanguageIdentifier, env: &BinahBotEnvironment) -> String {
    let key = match observation_levels.0 {
        Stat::SuccessRate => "observation_level_success_rate",
        Stat::WorkSpeed =>  "observation_level_speed_rate",
        _ => unreachable!()
    };
    env.locales.lookup_with_args(
        lang_id,
        key,
        &HashMap::from([
            ("percentage", FluentValue::from(observation_levels.1)),
        ])
    )
}

impl From<RiskLevel> for DiscordEmbedColors {
    fn from(value: RiskLevel) -> Self {
        DiscordEmbedColors::from(&value)
    }
}
