use std::string::ToString;

use fluent_templates::Loader;
use ruina::ruina_common::game_objects::battle_symbol::BattleSymbolSlot;
use ruina::ruina_common::game_objects::combat_page::Die;
use ruina::ruina_common::game_objects::combat_page::DieType;
use ruina::ruina_common::game_objects::common::PageType;
use ruina::ruina_common::localizations::combat_page_locale::CombatPageLocale;
use ruina::ruina_common::localizations::common::Locale;
use ruina::ruina_index::models::ParsedTypedId;
use ruina::ruina_reparser::get_abno_page_by_internal_name;
use ruina::ruina_reparser::get_abno_page_locales_by_internal_name;
use ruina::ruina_reparser::get_battle_symbol_by_internal_name;
use ruina::ruina_reparser::get_battle_symbol_locales_by_internal_name;
use ruina::ruina_reparser::get_card_effect_locales_by_id;
use ruina::ruina_reparser::get_combat_page_by_id;
use ruina::ruina_reparser::get_combat_page_locales_by_id;
use ruina::ruina_reparser::get_key_page_by_id;
use ruina::ruina_reparser::get_passive_by_id;
use ruina::ruina_reparser::get_passive_locales_by_id;
use unic_langid::LanguageIdentifier;

use crate::models::binahbot::get_dietype_emoji;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::binahbot::Emojis;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordEmbedFields;
use crate::models::discord::DiscordEmbedFooter;
use crate::models::discord::DiscordEmbedImage;
use crate::utils::get_disambiguation_format;

static NOT_FOUND_IMAGE_NAME: &str = "404_Not_Found";
static TIPH_BASE_URL: &str = "https://tiphereth.zasz.su";

pub fn transform_abno_page(
    internal_name: &str,
    card_locale: &Locale,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    tracing::info!(
        "Transforming abno page with internal_name={}, card_locale={}, request_locale={}",
        internal_name,
        card_locale,
        request_locale
    );

    let page = get_abno_page_by_internal_name(internal_name).unwrap();
    let binding = get_abno_page_locales_by_internal_name(internal_name);
    let locale_page = binding.get(card_locale);
    let lang_id = LanguageIdentifier::from(request_locale);

    let abno_type_display = env.locales.lookup(
        &lang_id,
        if page.is_positive {
            "abno_type_display_awakening"
        } else {
            "abno_type_display_breakdown"
        },
    );
    let embed_color = if page.is_positive {
        DiscordEmbedColors::AwakeningAbnoPage
    } else {
        DiscordEmbedColors::BreakdownAbnoPage
    };

    let img_url = format!(
        "https://{0}.s3.amazonaws.com/{1}.png",
        env.s3_bucket_name, page.artwork
    );

    DiscordEmbed {
        title: Some(
            locale_page
                .map(|x| x.card_name)
                .unwrap_or(internal_name)
                .to_string(),
        ),
        description: None,
        color: Some(embed_color as i32),
        image: Some(DiscordEmbedImage { url: img_url }),
        thumbnail: None,
        footer: Some(DiscordEmbedFooter {
            text: internal_name.to_string(),
            icon_url: None,
        }),
        author: None,
        url: Some(format!(
            "{}/abno_pages/{}/{}",
            TIPH_BASE_URL, page.sephirah, page.id
        )),
        fields: Some(vec![
            DiscordEmbedFields {
                name: env.locales.lookup(&lang_id, "abno_flavor_text_header"),
                value: locale_page.map(|x| x.flavor_text).unwrap_or("").to_string(),
                inline: Some(true),
            },
            DiscordEmbedFields {
                name: env.locales.lookup(&lang_id, "abno_effect_header"),
                value: locale_page.map(|x| x.description).unwrap_or("").to_string(),
                inline: Some(true),
            },
            DiscordEmbedFields {
                name: env.locales.lookup(&lang_id, "abno_bias_header"),
                value: page.bias.unwrap_or(0).to_string(),
                inline: Some(true),
            },
            DiscordEmbedFields {
                name: env.locales.lookup(&lang_id, "abno_type_header"),
                value: abno_type_display.to_string(),
                inline: Some(true),
            },
            DiscordEmbedFields {
                name: env.locales.lookup(&lang_id, "abno_tier_header"),
                value: page.tier.unwrap_or(0).to_string(),
                inline: Some(true),
            },
            DiscordEmbedFields {
                name: env.locales.lookup(&lang_id, "abno_floor_header"),
                value: page.sephirah.to_string(),
                inline: Some(true),
            },
        ]),
    }
}

pub fn transform_battle_symbol(
    internal_name: &str,
    card_locale: &Locale,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    tracing::info!(
        "Transforming battle symbol with internal_name={}, card_locale={}, request_locale={}",
        internal_name,
        card_locale,
        request_locale
    );

    let page = get_battle_symbol_by_internal_name(internal_name).unwrap();
    let binding = get_battle_symbol_locales_by_internal_name(internal_name);
    let locale_page = binding.get(card_locale).unwrap();
    let lang_id = LanguageIdentifier::from(request_locale);

    let slot_url = match page.slot {
        BattleSymbolSlot::Eye => "Eye",
        BattleSymbolSlot::Nose => "Nose",
        BattleSymbolSlot::Cheek => "Cheek",
        BattleSymbolSlot::Mouth => "Mouth",
        BattleSymbolSlot::Ear => "Ear",
        BattleSymbolSlot::Headwear1 => "HairAccessory",
        BattleSymbolSlot::Headwear2 => "Hood",
        BattleSymbolSlot::Headwear3 => "Mask",
        BattleSymbolSlot::Headwear4 => "Helmet",
        // todo: remove None enum (just use optional)
        _ => "",
    };

    let image_name = page
        .resource
        .map(|x| format!("{slot_url}_{x}_Front_Forward"))
        .unwrap_or(NOT_FOUND_IMAGE_NAME.to_string());
    let url = format!(
        "https://{0}.s3.amazonaws.com/battle_symbol/{image_name}.png",
        env.s3_bucket_name
    );

    let mut fields = vec![
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "battle_symbol_slot_header"),
            // TODO: slot -> localized name
            value: format!("{}", page.slot),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "battle_symbol_prefix_header"),
            value: locale_page.prefix.to_string(),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "battle_symbol_postfix_header"),
            value: locale_page.postfix.to_string(),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env
                .locales
                .lookup(&lang_id, "battle_symbol_description_header"),
            value: locale_page.description.unwrap_or("-").to_string(),
            inline: Some(false),
        },
        DiscordEmbedFields {
            name: env
                .locales
                .lookup(&lang_id, "battle_symbol_acquire_condition_header"),
            value: locale_page.acquire_condition.unwrap_or("-").to_string(),
            inline: Some(false),
        },
    ];

    if page.hidden {
        fields.push(DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "battle_symbol_hidden_header"),
            value: env
                .locales
                .lookup(&lang_id, "battle_symbol_is_hidden_display"),
            inline: Some(false),
        })
    }
    if page.count.is_some() {
        fields.push(DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "battle_symbol_count_header"),
            value: page.count.unwrap().to_string(),
            inline: Some(false),
        })
    }

    DiscordEmbed {
        title: Some(format!("{} {}", locale_page.prefix, locale_page.postfix)),
        description: None,
        color: Some(DiscordEmbedColors::Default as i32),
        image: Some(DiscordEmbedImage { url }),
        thumbnail: None,
        footer: Some(DiscordEmbedFooter {
            text: internal_name.to_string(),
            icon_url: None,
        }),
        author: None,
        url: Some(format!("{}/gifts/{}/", TIPH_BASE_URL, page.id)),
        fields: Some(fields),
    }
}

pub fn transform_combat_page(
    id: &str,
    card_locale: &Locale,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    tracing::info!(
        "Transforming combat page with id={}, card_locale={}, request_locale={}",
        id,
        card_locale,
        request_locale
    );

    let page = get_combat_page_by_id(id).unwrap();
    let binding = get_combat_page_locales_by_id(id);
    let page_locale = binding.get(card_locale);
    let lang_id = LanguageIdentifier::from(request_locale);

    let display_name = get_disambiguation_format(
        &ParsedTypedId(PageType::CombatPage, id.to_string()),
        card_locale,
        &lang_id,
        env,
    );

    let embed_color = DiscordEmbedColors::from(&page.rarity);
    let url = format!(
        "https://{0}.s3.amazonaws.com/{1}.png",
        env.s3_bucket_name,
        page.artwork.unwrap_or(NOT_FOUND_IMAGE_NAME)
    );

    let mut fields = vec![
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "combat_page_cost_header"),
            value: page.cost.to_string(),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "combat_page_range_header"),
            value: page.range.to_string(),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "combat_page_rarity_header"),
            value: page.rarity.to_string(),
            inline: Some(true),
        },
    ];

    let single_use_prefix = if page.options.contains(&"ExhaustOnUse") {
        format!("{}\n", env.locales.lookup(&lang_id, "exhaust_on_use"))
    } else {
        "".to_string()
    };

    let page_desc = page
        .script_id
        .and_then(|x| {
            get_card_effect_locales_by_id(x)
                .get(card_locale)
                .map(|y| y.desc.join("\n").to_string())
        })
        .or(page_locale
            .as_ref()
            .and_then(|x| x.card_effect)
            .map(|x| x.to_string()))
        .unwrap_or("".to_string());
    let page_desc = format!("{}{}", single_use_prefix, page_desc);
    let page_desc = page_desc.trim().to_string();

    if !page_desc.is_empty() {
        fields.push(DiscordEmbedFields {
            name: env
                .locales
                .lookup(&lang_id, "combat_page_description_header"),
            value: page_desc,
            inline: Some(true),
        })
    }

    let dice_vec = page.dice.to_vec();
    fields.push(DiscordEmbedFields {
        name: env.locales.lookup(&lang_id, "combat_page_dice_header"),
        value: format_dice(&dice_vec, card_locale, &page_locale, &env.emojis),
        inline: Some(false),
    });

    DiscordEmbed {
        title: Some(display_name),
        description: None,
        color: Some(embed_color as i32),
        image: Some(DiscordEmbedImage { url }),
        thumbnail: None,
        footer: Some(DiscordEmbedFooter {
            text: id.to_string(),
            icon_url: None,
        }),
        author: None,
        url: Some(format!("{}/cards/{}/", TIPH_BASE_URL, page.id)),
        fields: Some(fields),
    }
}

pub fn transform_key_page(
    id: &str,
    card_locale: &Locale,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    tracing::info!(
        "Transforming key page with id={}, card_locale={}, request_locale={}",
        id,
        card_locale,
        request_locale
    );

    let page = get_key_page_by_id(id).unwrap();
    let lang_id = LanguageIdentifier::from(request_locale);

    let display_name = get_disambiguation_format(
        &ParsedTypedId(PageType::KeyPage, id.to_string()),
        card_locale,
        &lang_id,
        env,
    );

    let embed_color = DiscordEmbedColors::from(&page.rarity);
    let hp_resists = format_to_indented_list(&[
        format!(
            "{}: {}",
            get_dietype_emoji(&env.emojis, &DieType::Slash),
            page.resists.hp_slash
        ),
        format!(
            "{}: {}",
            get_dietype_emoji(&env.emojis, &DieType::Pierce),
            page.resists.hp_pierce
        ),
        format!(
            "{}: {}",
            get_dietype_emoji(&env.emojis, &DieType::Blunt),
            page.resists.hp_blunt
        ),
    ]);
    let stagger_resists = format_to_indented_list(&[
        format!(
            "{}: {}",
            get_dietype_emoji(&env.emojis, &DieType::CSlash),
            page.resists.stagger_slash
        ),
        format!(
            "{}: {}",
            get_dietype_emoji(&env.emojis, &DieType::CPierce),
            page.resists.stagger_pierce
        ),
        format!(
            "{}: {}",
            get_dietype_emoji(&env.emojis, &DieType::CBlunt),
            page.resists.stagger_blunt
        ),
    ]);
    let url = format!(
        "https://{0}.s3.amazonaws.com/Sprite/{1}.png",
        env.s3_bucket_name, page.id
    );

    let mut fields = vec![
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "key_page_hp_header"),
            value: page.hp.to_string(),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "key_page_stagger_header"),
            value: page.stagger.to_string(),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "key_page_speed_header"),
            value: format!("{}-{}", page.min_speed, page.max_speed),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "key_page_hp_resist_header"),
            value: hp_resists,
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env
                .locales
                .lookup(&lang_id, "key_page_stagger_resist_header"),
            value: stagger_resists,
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "key_page_rarity_header"),
            value: page.rarity.to_string(),
            inline: Some(true),
        },
    ];

    if page.base_light != 3 {
        fields.push(DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "key_page_base_light_header"),
            value: page.base_light.to_string(),
            inline: Some(false),
        })
    }
    if !page.passive_ids.is_empty() {
        let passive_names: Vec<_> = page
            .passive_ids
            .iter()
            .map(|x| {
                get_passive_locales_by_id(x)
                    .get(card_locale)
                    .map(|y| y.name.to_string())
                    .unwrap_or(x.to_string())
            })
            .collect();
        fields.push(DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "key_page_passives_header"),
            value: format_to_indented_list(&passive_names),
            inline: Some(true),
        })
    }

    DiscordEmbed {
        title: Some(display_name),
        description: None,
        color: Some(embed_color as i32),
        image: Some(DiscordEmbedImage { url }),
        thumbnail: None,
        footer: Some(DiscordEmbedFooter {
            text: id.to_string(),
            icon_url: None,
        }),
        author: None,
        url: Some(format!("{}/keypages/{}/", TIPH_BASE_URL, page.id)),
        fields: Some(fields),
    }
}

pub fn transform_passive(
    id: &str,
    card_locale: &Locale,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    tracing::info!(
        "Transforming passive with id={}, card_locale={}, request_locale={}",
        id,
        card_locale,
        request_locale
    );

    let page = get_passive_by_id(id).unwrap();
    let binding = get_passive_locales_by_id(id);
    let locale_page = binding.get(card_locale).unwrap();
    let lang_id = LanguageIdentifier::from(request_locale);

    let display_name = get_disambiguation_format(
        &ParsedTypedId(PageType::Passive, id.to_string()),
        card_locale,
        &lang_id,
        env,
    );

    let embed_color = page
        .rarity
        .as_ref()
        .map_or(DiscordEmbedColors::Default, DiscordEmbedColors::from);
    let mut fields = vec![
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "passive_cost_header"),
            value: page.cost.map_or("-".to_string(), |x| x.to_string()),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "passive_rarity_header"),
            value: page
                .rarity
                .as_ref()
                .map_or("-".to_string(), |x| x.to_string()),
            inline: Some(true),
        },
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "passive_description_header"),
            value: locale_page.description.to_string(),
            inline: Some(false),
        },
    ];
    if page.transferable.is_some_and(|x| !x) {
        fields.push(DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "passive_transferable_header"),
            value: env
                .locales
                .lookup(&lang_id, "passive_not_transferable_display"),
            inline: Some(false),
        })
    }

    DiscordEmbed {
        title: Some(display_name),
        description: None,
        color: Some(embed_color as i32),
        image: None,
        thumbnail: None,
        footer: Some(DiscordEmbedFooter {
            text: id.to_string(),
            icon_url: None,
        }),
        author: None,
        url: Some(format!("{}/passives/{}/", TIPH_BASE_URL, page.id)),
        fields: Some(fields),
    }
}

fn format_dice(
    dice: &[Die],
    locale: &Locale,
    combat_page_locale: &Option<&&CombatPageLocale>,
    emojis: &Emojis,
) -> String {
    let formatted_die = dice
        .iter()
        .enumerate()
        .map(|(i, die)| {
            let binding = die
                .script
                .map(get_card_effect_locales_by_id)
                .and_then(|x| x.get(locale).map(|y| y.desc))
                .map(|x| x.join("\n"));

            let desc = combat_page_locale
                .and_then(|x| {
                    if x.dice_description_override.len() > i {
                        x.dice_description_override[i]
                    } else {
                        None
                    }
                })
                .or(binding.as_deref())
                .unwrap_or("");

            format!(
                "{} {}-{} {}",
                get_dietype_emoji(emojis, &die.die_type),
                die.min,
                die.max,
                desc
            )
        })
        .collect::<Vec<_>>();
    format_to_indented_list(&formatted_die)
}

fn format_to_indented_list(v: &[String]) -> String {
    v.iter()
        .map(|x| format!("- {}", x))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use unic_langid::langid;

    use crate::test_utils::build_mocked_binahbot_env;

    use super::*;

    #[test]
    fn sanity_format_to_indented_list() {
        let input = vec!["item 1".to_string(), "item 2".to_string()];
        let expected_output = "- item 1\n- item 2";
        assert_eq!(expected_output, format_to_indented_list(&input));
    }

    #[test]
    fn sanity_abno_page_url() {
        let internal_name = "LongBird_Sin";
        let card_locale = Locale::English;
        let request_locale = BinahBotLocale::EnglishUS;
        let env = build_mocked_binahbot_env();

        let embed = transform_abno_page(internal_name, &card_locale, &request_locale, &env);

        assert!(embed
            .url
            .expect("no url")
            .contains("https://tiphereth.zasz.su/abno_pages/Binah/7"));
    }

    #[test]
    fn sanity_format_dice() {
        let env = build_mocked_binahbot_env();

        let fmf_card = get_combat_page_by_id("9901101").unwrap();
        let binding = get_combat_page_locales_by_id("9901101");
        let fmf_locale = binding.get(&Locale::English);

        let fmf_format_dice =
            format_dice(fmf_card.dice, &Locale::English, &fmf_locale, &env.emojis);

        assert!(fmf_format_dice.contains("[On Hit] Inflict 10 Burn"));

        let uncontrollable_instinct_card = get_combat_page_by_id("9906215").unwrap();
        let binding = get_combat_page_locales_by_id("9906215");
        let uncontrollable_instinct_locale = binding.get(&Locale::English);

        let uncontrollable_instinct_format_dice = format_dice(
            uncontrollable_instinct_card.dice,
            &Locale::English,
            &uncontrollable_instinct_locale,
            &env.emojis,
        );

        assert!(uncontrollable_instinct_format_dice.contains(
            "Roll this die 5 times without processing damage, then deal damage equal to the sum of rolls that would have won clashes or hit the target"
        ));
    }

    #[test]
    fn combat_page_desc_override() {
        let env = build_mocked_binahbot_env();

        let binding = get_combat_page_locales_by_id("9910108");
        let self_loathing_locale = binding.get(&Locale::English).unwrap();

        let page_desc_header = env
            .locales
            .lookup(&langid!("en-US"), "combat_page_description_header");
        let embed = transform_combat_page(
            "9910108",
            &Locale::English,
            &BinahBotLocale::EnglishUS,
            &env,
        );
        let binding = embed.fields.unwrap();
        let page_desc_field = binding
            .iter()
            .find(|x| x.name == page_desc_header)
            .expect("couldn't find page description field");

        assert_eq!(
            self_loathing_locale.card_effect.unwrap(),
            page_desc_field.value
        )
    }

    #[test]
    fn exhaust_on_use() {
        let env = build_mocked_binahbot_env();

        let embed =
            transform_combat_page("504001", &Locale::English, &BinahBotLocale::EnglishUS, &env);

        let binding = embed.fields.unwrap();
        let page_description = binding.iter().find(|y| y.name == "Page description");
        assert!(page_description.is_some_and(|x| x.value.contains("Single-use")));

        let embed =
            transform_combat_page("504001", &Locale::Japanese, &BinahBotLocale::Japanese, &env);

        let binding = embed.fields.unwrap();
        // todo: update Japanese localization; it is currently falling back to the English localization
        let page_description = binding.iter().find(|y| y.name == "Page description");
        assert!(page_description.is_some_and(|x| x.value.contains("使い捨て")));
    }
}
