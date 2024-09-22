use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use unic_langid::LanguageIdentifier;

use crate::macros::cast_enum_variant;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::discord::ActionRowComponent;
use crate::models::discord::AllowedMentions;
use crate::models::discord::DiscordComponent;
use crate::models::discord::DiscordComponentType;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordEmbedFields;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionData;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordMessageFlag;
use crate::models::discord::MessageResponse;
use crate::utils::build_delete_button_component;
use crate::utils::get_option_value;

#[derive(Debug, PartialEq)]
struct DiceDistribution(u128, u128, u128, u128); // Total, d1 win, d2 win, draw

pub fn rollcalc_command(
    interaction: &DiscordInteraction,
    env: &BinahBotEnvironment,
) -> MessageResponse {
    let command_args = interaction
        .data
        .as_ref()
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionData::ApplicationCommand))
        .and_then(|x| x.options.as_ref())
        .unwrap();

    tracing::info!("Rollcalc command: command args: {:#?}", command_args);

    let binah_locale: BinahBotLocale = interaction
        .locale
        .as_ref()
        .or(interaction.guild_locale.as_ref())
        .and_then(|x| BinahBotLocale::from_str(x).ok())
        .unwrap_or(BinahBotLocale::EnglishUS);
    let lang_id = LanguageIdentifier::from(&binah_locale);

    let is_private = get_option_value("private", command_args)
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::Bool))
        .is_some_and(|x| *x);

    let flags = is_private.then_some(DiscordMessageFlag::EphemeralMessage as i32);

    tracing::info!(
        "binah_locale={:?}, lang_id={:?}, flags={:?}",
        binah_locale,
        lang_id,
        flags
    );

    let min1 = get_option_value("min1", command_args)
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::Integer))
        .expect("no min1");
    let min2 = get_option_value("min2", command_args)
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::Integer))
        .expect("no min2");
    let max1 = get_option_value("max1", command_args)
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::Integer))
        .expect("no max1");
    let max2 = get_option_value("max2", command_args)
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::Integer))
        .expect("no max2");

    let title = env.locales.lookup_with_args(
        &lang_id,
        "title_format",
        &HashMap::from([
            ("min1", FluentValue::from(min1)),
            ("max1", FluentValue::from(max1)),
            ("min2", FluentValue::from(min2)),
            ("max2", FluentValue::from(max2)),
        ]),
    );

    let embed = if min1 > max1 || min2 > max2 {
        DiscordEmbed {
            title: Some(title),
            description: Some(env.locales.lookup(&lang_id, "dice_range_message")),
            color: Some(DiscordEmbedColors::Default as i32),
            image: None,
            thumbnail: None,
            footer: None,
            author: None,
            url: None,
            fields: None,
        }
    } else {
        let calc = calculate(*min1, *max1, *min2, *max2);
        DiscordEmbed {
            title: Some(title),
            description: None,
            color: Some(DiscordEmbedColors::Default as i32),
            image: None,
            thumbnail: None,
            footer: None,
            author: None,
            url: None,
            fields: Some(vec![
                DiscordEmbedFields {
                    name: env.locales.lookup(&lang_id, "dice_1_win_header"),
                    value: env.locales.lookup_with_args(
                        &lang_id,
                        "win_format",
                        &HashMap::from([
                            (
                                "percentage",
                                format_percent((calc.1 as f64) / (calc.0 as f64)),
                            ),
                            ("numerator", FluentValue::from(calc.1)),
                            ("denominator", FluentValue::from(calc.0)),
                        ]),
                    ),
                    inline: Some(true),
                },
                DiscordEmbedFields {
                    name: env.locales.lookup(&lang_id, "dice_2_win_header"),
                    value: env.locales.lookup_with_args(
                        &lang_id,
                        "win_format",
                        &HashMap::from([
                            (
                                "percentage",
                                format_percent((calc.2 as f64) / (calc.0 as f64)),
                            ),
                            ("numerator", FluentValue::from(calc.2)),
                            ("denominator", FluentValue::from(calc.0)),
                        ]),
                    ),
                    inline: Some(true),
                },
                DiscordEmbedFields {
                    name: env.locales.lookup(&lang_id, "draw_header"),
                    value: env.locales.lookup_with_args(
                        &lang_id,
                        "win_format",
                        &HashMap::from([
                            (
                                "percentage",
                                format_percent((calc.3 as f64) / (calc.0 as f64)),
                            ),
                            ("numerator", FluentValue::from(calc.3)),
                            ("denominator", FluentValue::from(calc.0)),
                        ]),
                    ),
                    inline: Some(true),
                },
            ]),
        }
    };

    let components =
        (!is_private).then_some(vec![DiscordComponent::ActionRow(ActionRowComponent {
            r#type: DiscordComponentType::ActionRow,
            components: vec![DiscordComponent::Button(build_delete_button_component(
                &lang_id, env,
            ))],
        })]);

    MessageResponse {
        r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
        data: Some(DiscordInteractionResponseMessage {
            allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
            content: None,
            embeds: Some(vec![embed]),
            flags: flags,
            components: components,
        }),
    }
}

fn format_percent<'a>(n: f64) -> FluentValue<'a> {
    FluentValue::from(format!("{}", (n * 1000000.0).round() / 10000.0))
}

// prereq: all >= 1, min1 <= max1, and min2 <= max2
fn calculate(min1: i32, max1: i32, min2: i32, max2: i32) -> DiceDistribution {
    let r1: u128 = (max1 - min1 + 1).try_into().unwrap();
    let r2: u128 = (max2 - min2 + 1).try_into().unwrap();

    let overlap: u128 = (cmp::max(0, cmp::min(max1, max2) - cmp::max(min1, min2) + 1))
        .try_into()
        .unwrap();

    let d1_autolose: u128 = (cmp::max(0, min2 - min1)).try_into().unwrap();
    let d1_autowin: u128 = (cmp::max(0, max1 - max2)).try_into().unwrap();
    let d2_autolose: u128 = (cmp::max(0, min1 - min2)).try_into().unwrap();
    let d2_autowin: u128 = (cmp::max(0, max2 - max1)).try_into().unwrap();

    let half: u128 = (overlap * overlap - overlap) / 2;

    DiceDistribution(
        (r1 * r2).try_into().unwrap(),
        d1_autowin * overlap + d2_autolose * overlap + d1_autowin * d2_autolose + half,
        d2_autowin * overlap + d1_autolose * overlap + d2_autowin * d1_autolose + half,
        overlap,
    )
}

#[cfg(test)]
mod tests {
    use crate::models::discord::DiscordApplicationCommandInteractionData;
    use crate::models::discord::DiscordInteractionOptions;
    use crate::models::discord::DiscordInteractionType;
    use crate::models::discord::DiscordUser;
    use crate::test_utils::build_mocked_binahbot_env;

    use super::*;

    #[test]
    fn sanity_calculate() {
        let result = calculate(7, 10, 2, 6);
        assert_eq!(DiceDistribution(20, 20, 0, 0), result);

        let result = calculate(5, 10, 5, 10);
        assert_eq!(DiceDistribution(36, 15, 15, 6), result);

        let result = calculate(5, 7, 5, 10);
        assert_eq!(DiceDistribution(18, 3, 12, 3), result);

        let result = calculate(2, 2, 2, 2);
        assert_eq!(DiceDistribution(1, 0, 0, 1), result);
    }

    #[test]
    fn performance_calculate() {
        let result = calculate(1, 65536, 1, 65536);
        assert_eq!(
            DiceDistribution(4294967296, 2147450880, 2147450880, 65536),
            result
        );
    }

    #[test]
    fn sanity_does_not_crash() {
        let interaction = DiscordInteraction {
            id: "id".to_string(),
            application_id: "app_id".to_string(),
            r#type: DiscordInteractionType::ApplicationCommand,
            data: Some(DiscordInteractionData::ApplicationCommand(
                DiscordApplicationCommandInteractionData {
                    id: "id".to_string(),
                    name: "lor".to_string(),
                    options: Some(vec![
                        DiscordInteractionOptions {
                            name: "min1".to_string(),
                            name_localizations: None,
                            value: DiscordInteractionOptionValue::Integer(3),
                            focused: None,
                        },
                        DiscordInteractionOptions {
                            name: "min2".to_string(),
                            name_localizations: None,
                            value: DiscordInteractionOptionValue::Integer(6),
                            focused: None,
                        },
                        DiscordInteractionOptions {
                            name: "max1".to_string(),
                            name_localizations: None,
                            value: DiscordInteractionOptionValue::Integer(7),
                            focused: None,
                        },
                        DiscordInteractionOptions {
                            name: "max2".to_string(),
                            name_localizations: None,
                            value: DiscordInteractionOptionValue::Integer(19),
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
                avatar: "hash".to_string(),
            }),
            member: None,
            message: None,
        };
        let _does_not_crash = rollcalc_command(&interaction, &build_mocked_binahbot_env());
    }
}
