use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

use fluent_templates::Loader;
use fluent_templates::fluent_bundle::FluentValue;
use ruina::ruina_common::localizations::common::Locale;
use ruina::ruina_reparser::get_combat_page_locales_by_id;
use ruina::ruina_reparser::get_key_page_by_id;
use ruina::ruina_reparser::get_key_page_locales_by_text_id;
use ruina::ruina_reparser::get_passive_locales_by_id;
use unic_langid::LanguageIdentifier;

use crate::ddb::get_deck;
use crate::deck::deck_utils::get_user;
use crate::models::discord::DiscordEmbedAuthor;
use crate::models::discord::DiscordEmbedFields;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::deck::Deck;
use crate::models::discord::AllowedMentions;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordEmbedImage;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::MessageResponse;
use crate::utils::build_error_message_response;
use crate::utils::get_binahbot_locale;
use crate::utils::get_option_value;
use crate::thumbnail::generate_thumb_name;

struct DeckKey(String, String);

pub async fn read_deck(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> MessageResponse {
    let command_args = interaction.data.as_ref().unwrap().options.as_ref().unwrap();

    let name_option = match get_option_value("name", command_args).expect("couldn't find required arg") {
        DiscordInteractionOptionValue::String(x) => x,
        _ => unreachable!()
    };
    let deck_key = match parse_deck_name_option(name_option) {
        Ok(x) => x,
        Err(_) => panic!()
    };

    tracing::info!(name_option);
    tracing::info!("{}, {}", deck_key.0, deck_key.1);

    let deck_result = get_deck(
        env.ddb_client.as_ref().unwrap(),
        &env.ddb_table_name,
        &deck_key.1,
        &deck_key.0
    ).await;

    let request_locale = get_binahbot_locale(interaction);
    let lang_id = LanguageIdentifier::from(&request_locale);

    match deck_result {
        Ok(x) => {
            let embed = transform_deck(&x, &request_locale, env).await.expect("failed deck processing");
            MessageResponse {
                r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
                data: Some(DiscordInteractionResponseMessage {
                    allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
                    content: None,
                    embeds: Some(vec![
                        embed
                    ]),
                    flags: None
                })
            }
        },
        Err(_) => {
            // todo: check for error type
            build_error_message_response(&lang_id, "generic_error_message", env)
        }
    }
}

// todo: move to utils
fn parse_deck_name_option(name_option: &str) -> Result<DeckKey, ()> {
    let mut split: Vec<_> = name_option.split('#').collect();
    if split.len() >= 2 {
        let split2 = split.split_off(1);
        Ok(DeckKey(split.first().unwrap().to_string(), split2.join("#")))
    } else {
        Err(())
    }
}

async fn transform_deck(
    deck: &Deck,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> Result<DiscordEmbed, Box<dyn Error + Send + Sync>> {
    let lang_id = LanguageIdentifier::from(request_locale);
    let card_locale = Locale::from(request_locale);

    let tiph_deck_url = deck.tiph_deck.as_ref().map(|x| {
        format!("https://tiphereth.zasz.su/u/decks/{}/", x.0)
    });

    let _combat_page_names = deck.deck_data.combat_page_ids.iter().map(|x| {
        x.as_ref().and_then(|y| {
            get_combat_page_locales_by_id(y).get(&Locale::from(request_locale)).map(|z| {
                z.name    
            })
        }).unwrap_or("-")
    }).collect::<Vec<_>>();

    // todo: pass in thumbnail dir as env var
    let deck_preview_img = format!(
        "https://{0}.s3.amazonaws.com/deck_thumbnails/{1}.png",
        env.s3_bucket_name,
        generate_thumb_name(&deck.deck_data.combat_page_ids)
    );

    let user = get_user(
        env.reqwest_client.as_ref().expect("no http client"),
        &env.discord_secrets.bot_token,
        &deck.author_id
    ).await;

    let avatar_hash = user.as_ref().map(|x| { format!(
        "https://cdn.discordapp.com/avatars/{0}/{1}.png",
        &deck.author_id,
        &x.avatar
    )}).ok();

    let author_name = user.map(|x| {
        format!("@{}", &x.username)
    }).unwrap_or(deck.author_id.clone());

    let key_page_name = deck.deck_data.keypage_id.as_ref().and_then(|x| {
        get_key_page_by_id(x)    
    }).and_then(|x| {
        x.text_id
    }).and_then(|x| {
        get_key_page_locales_by_text_id(x).get(&card_locale).cloned()
    }).map(|x| {
        x.name
    }).unwrap_or("-");

    let passives = deck.deck_data.passive_ids.iter().map(|x| {
        get_passive_locales_by_id(x).get(&card_locale).map(|y| {
            y.name    
        }).unwrap_or("???")
    }).collect::<Vec<_>>();

    let combat_pages = &deck.deck_data.combat_page_ids.iter().map(|x| {
        x.as_ref().and_then(|y| {
            get_combat_page_locales_by_id(y)
                .get(&card_locale)
                .map(|y| y.name)
        }).unwrap_or("???")
    }).collect::<Vec<_>>();

    let combat_page_counts = aggregate_count(combat_pages);
    let mut combat_page_dedup = combat_pages.clone();
    dedup_preserve_order(&mut combat_page_dedup);
    let combat_page_pretty_print = combat_page_dedup.iter().map(|x| {
        env.locales.lookup_with_args(
            &lang_id,
            "read_deck_combat_page_count",
            &HashMap::from([
                ("page_name", FluentValue::from(*x)),
                ("count", FluentValue::from(combat_page_counts.get(x).unwrap())),
            ])
        )
    }).collect::<Vec<_>>();

    let mut fields = vec![
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "read_deck_keypage_header"),
            value: key_page_name.to_string(),
            inline: Some(true),
        }
    ];

    if !passives.is_empty() {
        fields.push(DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "read_deck_passives_header"),
            value: format_to_list(&passives),
            inline: Some(true),
        });
    };

    fields.push(
        DiscordEmbedFields {
            name: env.locales.lookup(&lang_id, "read_deck_combat_pages_header"),
            value: format_to_list(&combat_page_pretty_print),
            inline: Some(true),
        }
    );

    Ok(DiscordEmbed {
        title: Some(deck.name.clone()),
        description: deck.description.clone(),
        color: Some(DiscordEmbedColors::Default as i32),
        image: Some(DiscordEmbedImage { url: deck_preview_img }),
        footer: None,
        author: Some(DiscordEmbedAuthor {
            name: author_name,
            url: None,
            icon_url: avatar_hash,
        }),
        url: tiph_deck_url,
        fields: Some(fields),
    })
}

// todo: move to utils
fn format_to_list<T: AsRef<str>>(v: &[T]) -> String {
    v.iter()
        .map(|x| format!("- {}", x.as_ref()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn aggregate_count<T: AsRef<str> + Eq + std::hash::Hash>(v: &[T]) -> HashMap<&T, i32> {
    let mut ret_val = HashMap::new();

    v.iter().for_each(|x| {
        *ret_val.entry(x).or_insert(0) += 1;
    });

    ret_val
}

fn dedup_preserve_order<T: AsRef<str> + Eq + std::hash::Hash + Clone>(v: &mut Vec<T>){
    let mut set = HashSet::new();
    
    v.retain(|x| set.insert(x.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sanity_parse_deck_name_option() {
        let input = "269925702571130880#Turbo#Nikolai";
        let deck = parse_deck_name_option(input).unwrap();
        assert_eq!("269925702571130880", deck.0);
        assert_eq!("Turbo#Nikolai", deck.1);
    }

    #[tokio::test]
    #[ignore]
    async fn sanity_get_user() {
        let client = reqwest::Client::new();
        let user = get_user(
            &client, "FILL_IN", "269925702571130880"
        ).await.expect("couldn't get user");
        assert_eq!("ghoulean", user.username);
    }

    #[test]
    fn sanity_dedup_preserve_order() {
        let mut vec = vec![
            "a", "b", "d", "a", "c", "b", "c"
        ];
        dedup_preserve_order(&mut vec);
        assert_eq!(4, vec.len());
        assert_eq!(vec!["a", "b", "d", "c"], vec);
    }

}