use std::string::ToString;

use ruina_common::game_objects::abno_page::AbnoPage;
use ruina_common::game_objects::battle_symbol::BattleSymbol;
use ruina_common::game_objects::combat_page::CombatPage;
use ruina_common::game_objects::key_page::KeyPage;
use ruina_common::game_objects::passive::Passive;
use ruina_common::game_objects::combat_page::Die;
use ruina_common::game_objects::combat_page::DieType;
use ruina_common::localizations::abno_page_locale::AbnoPageLocale;
use ruina_common::localizations::battle_symbol_locale::BattleSymbolLocale;
use ruina_common::localizations::combat_page_locale::CombatPageLocale;
use ruina_common::localizations::key_page_locale::KeyPageLocale;
use ruina_common::localizations::passive_locale::PassiveLocale;
use ruina_common::localizations::common::Locale;
use ruina_reparser::get_card_effect_locales_by_id;
use ruina_reparser::get_passive_locales_by_id;

use crate::models::binahbot::BinahBotLocale;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordEmbedFields;
use crate::models::discord::DiscordEmbedImage;

static NOT_FOUND_IMAGE_NAME: &str = "404_Not_Found.png";

pub fn transform_abno_page(
    page: &AbnoPage,
    locale_page: &AbnoPageLocale,
    s3_bucket_name: &String,
    _request_locale: &BinahBotLocale
) -> DiscordEmbed {
    let abno_type_display = if page.is_positive { "Awakening" } else { "Breakdown" };
    let embed_color = if page.is_positive {
        DiscordEmbedColors::AwakeningAbnoPage
    } else {
        DiscordEmbedColors::BreakdownAbnoPage
    };
    let url = format!("https://{s3_bucket_name}.s3.amazonaws.com/{0}.png", page.artwork);

    DiscordEmbed {
        title: Some(locale_page.card_name.to_string()),
        description: None,
        color: Some(embed_color as i32),
        image: Some(
            DiscordEmbedImage {
                url: url,
            }
        ),
        footer: None,
        author: None,
        fields: Some(
            vec!(
                DiscordEmbedFields { 
                    name: "Flavor text".to_string(),
                    value: locale_page.flavor_text.to_string(),
                    inline: Some(true)
                },
                DiscordEmbedFields { 
                    name: "Effect".to_string(),
                    value: locale_page.description.to_string(),
                    inline: Some(true)
                },
                DiscordEmbedFields { 
                    name: "Bias".to_string(),
                    value: page.bias.unwrap().to_string(),
                    inline: Some(true)
                },
                DiscordEmbedFields { 
                    name: "Type".to_string(),
                    value: abno_type_display.to_string(),
                    inline: Some(true)
                },
                DiscordEmbedFields { 
                    name: "Tier".to_string(),
                    value: page.tier.unwrap().to_string(),
                    inline: Some(true)
                },
                DiscordEmbedFields { 
                    name: "Floor".to_string(),
                    value: page.sephirah.to_string(),
                    inline: Some(true)
                }
            )
        ),
    }
}


pub fn transform_battle_symbol(
    page: &BattleSymbol,
    locale_page: &BattleSymbolLocale,
    s3_bucket_name: &String,
    _request_locale: &BinahBotLocale
) -> DiscordEmbed {
    // TODO: upload battle symbol images + model them as optional in commons + reparser
    let url = format!("https://{s3_bucket_name}.s3.amazonaws.com/{NOT_FOUND_IMAGE_NAME}.png");
    let mut fields = vec!(DiscordEmbedFields {
        name: "Slot".to_string(),
        value: format!("{}", page.slot),
        inline: Some(true)
    }, DiscordEmbedFields {
        name: "Prefix".to_string(),
        value: locale_page.prefix.to_string(),
        inline: Some(true)
    }, DiscordEmbedFields {
        name: "Postfix".to_string(),
        value: locale_page.postfix.to_string(),
        inline: Some(true)
    }, DiscordEmbedFields {
        name: "Description".to_string(),
        value: locale_page.description.unwrap_or("-").to_string(),
        inline: Some(false)
    }, DiscordEmbedFields {
        name: "Acquire condition".to_string(),
        value: locale_page.acquire_condition.unwrap_or("-").to_string(),
        inline: Some(false)
    });
    if page.hidden {
        fields.push(DiscordEmbedFields {
            name: "Hidden".to_string(),
            value: "true".to_string(),
            inline: Some(false)
        })
    }
    if page.count.is_some() {
        fields.push(DiscordEmbedFields {
            name: "Count".to_string(),
            value: page.count.unwrap().to_string(),
            inline: Some(false)
        })
    }

    DiscordEmbed {
        title: Some(format!("{} {}", locale_page.prefix, locale_page.postfix)),
        description: None,
        color: Some(DiscordEmbedColors::Default as i32),
        image: Some(
            DiscordEmbedImage {
                url: url,
            }
        ),
        footer: None,
        author: None,
        fields: Some(fields),
    }
}

pub fn transform_combat_page(
    page: &CombatPage,
    locale_page: &CombatPageLocale,
    s3_bucket_name: &String,
    card_locale: &Locale,
    _request_locale: &BinahBotLocale
) -> DiscordEmbed {
    let embed_color = DiscordEmbedColors::from(&page.rarity);
    let url = format!("https://{s3_bucket_name}.s3.amazonaws.com/{0}.png", page.artwork.unwrap_or(NOT_FOUND_IMAGE_NAME));

    let mut fields = vec!(DiscordEmbedFields {
        name: "Cost".to_string(),
        value: page.cost.to_string(),
        inline: Some(true)
    }, DiscordEmbedFields {
        name: "Range".to_string(),
        value: page.range.to_string(),
        inline: Some(true)
    }, DiscordEmbedFields {
        name: "Rarity".to_string(),
        value: page.rarity.to_string(),
        inline: Some(true)
    });

    if page.script_id.is_some() {
        let page_desc = get_card_effect_locales_by_id(&page.script_id.unwrap())
            .get(card_locale)
            .unwrap()
            .desc
            .join("\n");
        fields.push(DiscordEmbedFields {
            name: "Page Description".to_string(),
            value: page_desc.to_string(),
            inline: Some(true)
        })
    }

    let dice_vec = page.dice.to_vec();
    fields.push(DiscordEmbedFields {
        name: "Dice".to_string(),
        value: format_dice(&dice_vec, &card_locale),
        inline: Some(false)
    });

    DiscordEmbed {
        title: Some(locale_page.name.to_string()),
        description: None,
        color: Some(embed_color as i32),
        image: Some(
            DiscordEmbedImage {
                url: url,
            }
        ),
        footer: None,
        author: None,
        fields: Some(fields),
    }
}

pub fn transform_key_page(
    page: &KeyPage,
    locale_page: Option<&KeyPageLocale>,
    card_locale: &Locale,
    _request_locale: &BinahBotLocale
) -> DiscordEmbed {
    let embed_color = DiscordEmbedColors::from(&page.rarity);
    let hp_resists = format_to_indented_list(&vec!(
        format!("{}: {}", DieType::Slash, page.resists.hp_slash),
        format!("{}: {}", DieType::Pierce, page.resists.hp_pierce),
        format!("{}: {}", DieType::Blunt, page.resists.hp_blunt),
    ));
    let stagger_resists = format_to_indented_list(&vec!(
        format!("{}: {}", DieType::Slash, page.resists.stagger_slash),
        format!("{}: {}", DieType::Pierce, page.resists.stagger_pierce),
        format!("{}: {}", DieType::Blunt, page.resists.stagger_blunt),
    ));

    let mut fields = vec!(DiscordEmbedFields {
        name: "HP".to_string(),
        value: page.hp.to_string(),
        inline: Some(true)
    }, DiscordEmbedFields {
        name: "Stagger".to_string(),
        value: page.stagger.to_string(),
        inline: Some(true)
    }, DiscordEmbedFields {
        name: "Speed".to_string(),
        value: format!("{}-{}", page.min_speed, page.max_speed),
        inline: Some(true)
    }, DiscordEmbedFields {
        name: "HP Resist".to_string(),
        value: hp_resists,
        inline: Some(true)
    }, DiscordEmbedFields {
        name: "Stagger Resist".to_string(),
        value: stagger_resists,
        inline: Some(true)
    }, DiscordEmbedFields {
        name: "Rarity".to_string(),
        value: page.rarity.to_string(),
        inline: Some(true)
    });

    if page.base_light != 3 {
        fields.push(DiscordEmbedFields {
            name: "Base Light".to_string(),
            value: page.base_light.to_string(),
            inline: Some(false)
        })
    }
    if page.passive_ids.len() > 0 {
        let passive_names: Vec<_> = page.passive_ids
            .to_vec()
            .into_iter()
            .map(|x| {
                get_passive_locales_by_id(x)
                    .get(card_locale)
                    .expect("could not get PassiveLocale")
                    .name
                    .to_string()
            })
            .collect();
        fields.push(DiscordEmbedFields {
            name: "Passives".to_string(),
            value: format_to_indented_list(&passive_names),
            inline: Some(true)
        })
    }

    DiscordEmbed {
        title: locale_page.map(|x| x.name.to_string()),
        description: None,
        color: Some(embed_color as i32),
        image: None,
        footer: None,
        author: None,
        fields: Some(fields),
    }
}

pub fn transform_passive(
    page: &Passive,
    locale_page: &PassiveLocale,
    _request_locale: &BinahBotLocale
) -> DiscordEmbed {
    let embed_color = page.rarity.as_ref().map_or(
        DiscordEmbedColors::Default,
        |x| DiscordEmbedColors::from(x)
    );
    let mut fields = vec!(DiscordEmbedFields {
        name: "Cost".to_string(),
        value: page.cost.map_or("-".to_string(), |x| x.to_string()),
        inline: Some(true)
    }, DiscordEmbedFields {
        name: "Rarity".to_string(),
        value: page.rarity.as_ref().map_or("-".to_string(), |x| x.to_string()),
        inline: Some(true)
    }, DiscordEmbedFields {
        name: "Description".to_string(),
        value: locale_page.description.to_string(),
        inline: Some(false)
    });
    if page.transferable.is_some_and(|x| x == false) {
        fields.push(DiscordEmbedFields {
            name: "Transferable".to_string(),
            value: "false".to_string(),
            inline: Some(false)
        })
    }

    DiscordEmbed {
        title: Some(locale_page.name.to_string()),
        description: None,
        color: Some(embed_color as i32),
        image: None,
        footer: None,
        author: None,
        fields: Some(fields),
    }
}

fn format_dice(
    dice: &Vec<Die>,
    locale: &Locale
) -> String {
    let formatted_die = dice.into_iter().map(|die| {
        let desc = die.script
            .map(get_card_effect_locales_by_id)
            .map(|x| x.get(locale).unwrap().desc)
            .unwrap_or(&[])
            .join("\n");
        format!("{} {}-{} {}", die.die_type, die.min, die.max, desc)
    }).collect::<Vec<_>>();
    format_to_indented_list(&formatted_die)
}

fn format_to_indented_list(v: &Vec<String>) -> String {
    v.into_iter().map(|x| {
        format!(" > - {}", x)
    }).collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_format_to_indented_list() {
        let input = vec!(
            "item 1".to_string(),
            "item 2".to_string()
        );
        let expected_output = " > - item 1\n > - item 2";
        assert_eq!(expected_output, format_to_indented_list(&input));
    }
}