use ruina_common::localizations::common::Locale;
use ruina_reparser::get_combat_page_by_id;
use ruina_reparser::get_combat_page_locales_by_id;
use unic_langid::LanguageIdentifier;

use crate::ddb::get_deck;
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
use crate::utils::get_binahbot_locale;
use crate::utils::get_option_value;

use super::deck_utils::build_generic_error_message_response;

struct DeckKey(String, String);

static NOT_FOUND_IMAGE_NAME: &str = "404_Not_Found.png";

pub async fn read_deck(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> MessageResponse {
    let command_args = &interaction.data.as_ref().unwrap().options;

    let name_option = match get_option_value("name", &command_args).expect("couldn't find required arg") {
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
        &env.ddb_client.as_ref().unwrap(),
        &env.ddb_table_name,
        &deck_key.1,
        &deck_key.0
    ).await;

    let request_locale = get_binahbot_locale(interaction);
    let lang_id = LanguageIdentifier::from(&request_locale);

    match deck_result {
        Ok(x) => {
            MessageResponse {
                r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
                data: Some(DiscordInteractionResponseMessage {
                    allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
                    content: None,
                    embeds: Some(transform_deck(&x, &request_locale, env)),
                    flags: None
                })
            }
        },
        Err(_) => {
            // todo: check for error type
            build_generic_error_message_response(&lang_id, env)
        }
    }
            // get user id+deck name from input
            // getitem ddb
            // check ddb result
            // resource not found exception -> does not exist soz (ephemeral)
            // other exception -> crash or something
            // success -> transform  (NOT ephemeral)
}

fn parse_deck_name_option(name_option: &str) -> Result<DeckKey, ()> {
    let mut split: Vec<_> = name_option.split('#').collect();
    if split.len() >= 2 {
        let split2 = split.split_off(1);
        Ok(DeckKey(split.first().unwrap().to_string(), split2.join("#")))
    } else {
        Err(())
    }
}

fn transform_deck(
    deck: &Deck,
    request_locale: &BinahBotLocale,
    env: &BinahBotEnvironment,
) -> Vec<DiscordEmbed> {
    let _lang_id = LanguageIdentifier::from(request_locale);

    let tiph_deck_url = deck.tiph_deck.as_ref().map(|x| {
        format!("https://tiphereth.zasz.su/u/decks/{}/", x.0)
    }).unwrap_or("https://tiphereth.zasz.su/u/deck_editor/".to_string());

    let combat_page_images = deck.deck_data.combat_page_ids.iter().map(|x| {
        let image_path = x.as_ref().map(|y| {
            get_combat_page_by_id(&y).unwrap().artwork
        }).flatten().unwrap_or(NOT_FOUND_IMAGE_NAME);
        format!(
            "https://{0}.s3.amazonaws.com/{1}.png",
            env.s3_bucket_name,
            image_path
        )
    }).collect::<Vec<_>>();

    let _combat_page_names = deck.deck_data.combat_page_ids.iter().map(|x| {
        x.as_ref().map(|y| {
            get_combat_page_locales_by_id(&y).get(&Locale::from(request_locale)).map(|z| {
                z.name    
            })
        }).flatten().unwrap_or("-")
    }).collect::<Vec<_>>();

    let mut embeds: Vec<_> = combat_page_images.iter().map(|image_url| {
        DiscordEmbed {
            title: None,
            description: None,
            color: None,
            image: Some(DiscordEmbedImage { url: image_url.to_string() }),
            footer: None,
            author: None,
            url: Some(tiph_deck_url.clone()),
            fields: None,
        }
    }).collect();

    embeds[0] = DiscordEmbed {
        title: Some(deck.name.clone()),
        description: deck.description.clone(),
        color: Some(DiscordEmbedColors::Default as i32),
        image: Some(DiscordEmbedImage { url: combat_page_images.first().unwrap().to_string() }),
        footer: None,
        author: None,
        url: Some(tiph_deck_url),
        fields: Some(vec![
                
        ]),
    };

    embeds
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

}