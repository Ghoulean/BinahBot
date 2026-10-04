use std::collections::HashMap;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use fluent_templates::fluent_bundle::FluentValue;
use fluent_templates::Loader;
use unic_langid::LanguageIdentifier;

use crate::ddb::get_listdecks_interaction;
use crate::ddb::put_listdecks_interaction;
use crate::deck::deck_search::search;
use crate::deck::deck_search::search_prefer_cache;
use crate::deck::deck_search::try_get_search_cache;
use crate::deck::deck_search::CACHE_PAGE_COUNT;
use crate::deck::deck_search::CACHE_PAGE_SIZE;
use crate::deck::deck_utils::DeckKey;
use crate::deck::read_deck::build_deck_embed;
use crate::deck::read_deck::DeckEmbedError;
use crate::macros::cast_enum_variant;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::DiscordEmbedColors;
use crate::models::deck::DeckMetadata;
use crate::models::deck::DeckSearchParams;
use crate::models::deck::ListDecksInteraction;
use crate::models::discord::ActionRowComponent;
use crate::models::discord::AllowedMentions;
use crate::models::discord::ButtonComponent;
use crate::models::discord::ButtonStyle;
use crate::models::discord::DiscordComponent;
use crate::models::discord::DiscordComponentType;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordEmbedFooter;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionData;
use crate::models::discord::DiscordInteractionMetadata;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionOptions;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordMessageFlag;
use crate::models::discord::MessageResponse;
use crate::utils::build_delete_button_component;
use crate::utils::build_error_message_response;
use crate::utils::get_binahbot_locale;
use crate::utils::get_option_value;

// ld#<display page>
pub const LISTDECK_PAGE_BUTTON_PREFIX: &str = "ld#";
// ldd#<deck index>
pub const LISTDECK_GET_DECK_BUTTON_PREFIX: &str = "ldd#";

const MAX_BUTTONS_PER_ROW: usize = 5;

const LISTDECKS_INTERACTION_TTL_SECONDS: u64 = 15 * 60;

pub async fn listdeck_command(
    interaction: &DiscordInteraction,
    env: &BinahBotEnvironment,
) -> MessageResponse {
    let command_args = interaction
        .data
        .as_ref()
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionData::ApplicationCommand))
        .and_then(|x| x.options.as_ref());

    tracing::info!("Listdeck command: command args: {:#?}", command_args);

    let lang_id = LanguageIdentifier::from(&get_binahbot_locale(interaction));

    fn get_string_option<'a>(
        name: &'a str,
        command_args: Option<&'a Vec<DiscordInteractionOptions>>,
    ) -> Option<&'a str> {
        command_args
            .and_then(|x| get_option_value(name, x))
            .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::String))
            .map(|x| x.as_str())
    }
    let search_params = DeckSearchParams::new(
        get_string_option("name", command_args),
        get_string_option("author", command_args),
        get_string_option("keypage", command_args),
    );
    let is_private = command_args
        .and_then(|x| get_option_value("private", x))
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionOptionValue::Bool))
        .is_some_and(|x| *x);

    let listdecks_interaction = ListDecksInteraction {
        interaction_id: interaction.id.clone(),
        search_params,
        ttl: get_listdecks_interaction_ttl(),
    };

    // always recompute on a fresh command so new searches reflect the latest decks
    let (entry, put_listdecks_interaction_result) = tokio::join!(
        search(&listdecks_interaction.search_params, 0, env),
        put_listdecks_interaction(
            env.ddb_client.as_ref().expect("no ddb client"),
            &env.ddb_interaction_ttl_table_name,
            &listdecks_interaction,
        )
    );
    if let Err(e) = put_listdecks_interaction_result {
        tracing::error!("couldn't write listdecks interaction: {:?}", e);
    }
    let entry = match entry {
        Ok(x) => x,
        Err(e) => {
            tracing::error!("deck search failed: {:?}", e);
            return build_error_message_response(&lang_id, "generic_error_message", env);
        }
    };

    MessageResponse {
        r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
        data: Some(DiscordInteractionResponseMessage {
            allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
            content: None,
            embeds: Some(vec![build_embed(&entry.decks, 0, &lang_id, env)]),
            flags: is_private.then_some(DiscordMessageFlag::EphemeralMessage as i32),
            components: Some(build_buttons(
                0,
                &entry.decks,
                entry.result_count,
                &lang_id,
                env,
            )),
        }),
    }
}

pub async fn listdeck_page_button(
    interaction: &DiscordInteraction,
    env: &BinahBotEnvironment,
) -> Result<MessageResponse, ()> {
    tracing::info!("Listdeck page button: interaction={:#?}", interaction);

    let (display_page, listdecks_interaction) =
        parse_button_interaction(interaction, LISTDECK_PAGE_BUTTON_PREFIX, env).await?;
    let lang_id = LanguageIdentifier::from(&get_binahbot_locale(interaction));

    let entry = search_prefer_cache(&listdecks_interaction.search_params, display_page, env)
        .await
        .map_err(|e| tracing::error!("deck search failed: {:?}", e))?;

    Ok(MessageResponse {
        r#type: DiscordInteractionResponseType::UpdateMessage,
        data: Some(DiscordInteractionResponseMessage {
            allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
            content: None,
            embeds: Some(vec![build_embed(&entry.decks, display_page, &lang_id, env)]),
            flags: None,
            components: Some(build_buttons(
                display_page,
                &entry.decks,
                entry.result_count,
                &lang_id,
                env,
            )),
        }),
    })
}

pub async fn listdeck_get_deck_button(
    interaction: &DiscordInteraction,
    env: &BinahBotEnvironment,
) -> Result<MessageResponse, ()> {
    tracing::info!("Listdeck get deck button: interaction={:#?}", interaction);

    let (deck_index, listdecks_interaction) =
        parse_button_interaction(interaction, LISTDECK_GET_DECK_BUTTON_PREFIX, env).await?;

    let entry = try_get_search_cache(
        &listdecks_interaction.search_params,
        deck_index / CACHE_PAGE_SIZE,
        env,
    )
    .await
    .map_err(|e| tracing::info!("deck search cache unavailable: {:?}", e))?;
    let deck = entry
        .decks
        .get(deck_index % CACHE_PAGE_SIZE)
        .ok_or_else(|| tracing::error!("no deck at index {}", deck_index))?;

    let message = interaction.message.as_ref().ok_or(())?;
    let components = message
        .components
        .clone()
        .map(serde_json::from_value::<Vec<DiscordComponent>>)
        .transpose()
        .map_err(|e| tracing::error!("couldn't parse message components: {:?}", e))?
        .ok_or(())?;
    let is_private = message
        .flags
        .is_some_and(|x| x & DiscordMessageFlag::EphemeralMessage as i32 != 0);

    let request_locale = get_binahbot_locale(interaction);
    let lang_id = LanguageIdentifier::from(&request_locale);

    let embed = match build_deck_embed(
        &DeckKey(deck.author_id.clone(), deck.name.clone()),
        interaction.channel_id.as_ref(),
        is_private,
        &request_locale,
        env,
    )
    .await
    {
        Ok(x) => x,
        Err(DeckEmbedError::NotFound) => return Err(()),
        Err(DeckEmbedError::Spoiler(x)) => return Ok(x),
    };

    Ok(MessageResponse {
        r#type: DiscordInteractionResponseType::UpdateMessage,
        data: Some(DiscordInteractionResponseMessage {
            allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
            content: None,
            embeds: Some(vec![embed]),
            flags: None,
            components: Some(select_get_deck_button(
                components,
                &build_get_deck_button_custom_id(deck_index),
                &lang_id,
                env,
            )),
        }),
    })
}

fn select_get_deck_button(
    components: Vec<DiscordComponent>,
    clicked_custom_id: &str,
    lang_id: &LanguageIdentifier,
    env: &BinahBotEnvironment,
) -> Vec<DiscordComponent> {
    let mut has_page_button = false;
    let mut rows = components
        .into_iter()
        .map(|row| match row {
            DiscordComponent::ActionRow(mut x) => {
                for component in x.components.iter_mut() {
                    if let DiscordComponent::Button(button) = component {
                        let custom_id = button.custom_id.as_deref().unwrap_or("");
                        if custom_id.starts_with(LISTDECK_GET_DECK_BUTTON_PREFIX) {
                            button.disabled = Some(custom_id == clicked_custom_id);
                        } else if custom_id.starts_with(LISTDECK_PAGE_BUTTON_PREFIX) {
                            has_page_button = true;
                            button.disabled = Some(false);
                        }
                    }
                }
                DiscordComponent::ActionRow(x)
            }
            x => x,
        })
        .collect::<Vec<_>>();

    if !has_page_button {
        let page_row = DiscordComponent::ActionRow(ActionRowComponent {
            r#type: DiscordComponentType::ActionRow,
            components: vec![build_page_button(0, false, lang_id, env)],
        });
        rows.insert(rows.len().saturating_sub(1), page_row);
    }
    rows
}

async fn parse_button_interaction(
    interaction: &DiscordInteraction,
    prefix: &str,
    env: &BinahBotEnvironment,
) -> Result<(usize, ListDecksInteraction), ()> {
    let data = interaction
        .data
        .as_ref()
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionData::MessageComponent))
        .ok_or(())?;
    let number = data
        .custom_id
        .strip_prefix(prefix)
        .and_then(|x| x.parse::<usize>().ok())
        .ok_or_else(|| {
            tracing::error!("couldn't parse custom_id={}", data.custom_id);
        })?;

    let interaction_metadata = check_original_user(interaction)?;

    let listdecks_interaction = get_listdecks_interaction(
        env.ddb_client.as_ref().expect("no ddb client"),
        &env.ddb_interaction_ttl_table_name,
        &interaction_metadata.id,
    )
    .await
    .map_err(|e| tracing::error!("couldn't get listdecks interaction: {:?}", e))?;

    if listdecks_interaction.ttl <= now() {
        tracing::info!(
            "listdecks interaction expired: ttl={}",
            listdecks_interaction.ttl
        );
        return Err(());
    }

    Ok((number, listdecks_interaction))
}

fn check_original_user(
    interaction: &DiscordInteraction,
) -> Result<&DiscordInteractionMetadata, ()> {
    let interaction_metadata = interaction
        .message
        .as_ref()
        .and_then(|x| x.interaction_metadata.as_ref())
        .ok_or(())?;
    let user_id = interaction.user.as_ref().map(|x| &x.id).unwrap_or_else(|| {
        &interaction
            .member
            .as_ref()
            .and_then(|x| x.user.as_ref())
            .map(|x| &x.id)
            .unwrap()
    });
    if &interaction_metadata.user.id != user_id {
        tracing::error!(
            "interaction author differs: user_id={}; original_user_id={}",
            user_id,
            interaction_metadata.user.id
        );
        return Err(());
    }

    Ok(interaction_metadata)
}

fn build_embed(
    decks: &[DeckMetadata],
    display_page: usize,
    lang_id: &LanguageIdentifier,
    env: &BinahBotEnvironment,
) -> DiscordEmbed {
    let description = if decks.is_empty() {
        env.locales.lookup(lang_id, "list_deck_empty")
    } else {
        decks
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let name_author = env.locales.lookup_with_args(
                    lang_id,
                    "list_deck_name_author",
                    &HashMap::from([
                        ("deck_name", FluentValue::from(&x.name)),
                        ("author", FluentValue::from(&x.author_name)),
                    ]),
                );
                format!(
                    "{}. {}",
                    display_page * CACHE_PAGE_SIZE + i + 1,
                    name_author
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    DiscordEmbed {
        title: Some(env.locales.lookup(lang_id, "list_deck_title")),
        description: Some(description),
        color: Some(DiscordEmbedColors::Default as i32),
        image: None,
        thumbnail: None,
        footer: Some(DiscordEmbedFooter {
            text: env.locales.lookup_with_args(
                lang_id,
                "list_deck_page_footer",
                &HashMap::from([("page", FluentValue::from(display_page + 1))]),
            ),
            icon_url: None,
        }),
        author: None,
        url: None,
        fields: None,
    }
}

fn build_buttons(
    display_page: usize,
    decks: &[DeckMetadata],
    result_count: usize,
    lang_id: &LanguageIdentifier,
    env: &BinahBotEnvironment,
) -> Vec<DiscordComponent> {
    let page_count = result_count.div_ceil(CACHE_PAGE_SIZE).min(CACHE_PAGE_COUNT);

    let first_deck = display_page * CACHE_PAGE_SIZE;
    let deck_buttons = (first_deck..first_deck + decks.len())
        .map(|i| {
            DiscordComponent::Button(ButtonComponent {
                r#type: DiscordComponentType::Button,
                style: ButtonStyle::Secondary,
                label: Some((i + 1).to_string()),
                custom_id: Some(build_get_deck_button_custom_id(i)),
                disabled: Some(false),
            })
        })
        .collect::<Vec<_>>();
    let page_buttons = (0..page_count)
        .map(|i| build_page_button(i, i == display_page, lang_id, env))
        .collect::<Vec<_>>();
    let delete_button = DiscordComponent::Button(build_delete_button_component(lang_id, env));

    let action_row = |components| {
        DiscordComponent::ActionRow(ActionRowComponent {
            r#type: DiscordComponentType::ActionRow,
            components,
        })
    };

    let mut rows = deck_buttons
        .chunks(MAX_BUTTONS_PER_ROW)
        .map(|x| action_row(x.to_vec()))
        .collect::<Vec<_>>();
    if page_count > 1 {
        rows.push(action_row(page_buttons));
    }
    rows.push(action_row(vec![delete_button]));
    rows
}

fn build_page_button(
    display_page: usize,
    disabled: bool,
    lang_id: &LanguageIdentifier,
    env: &BinahBotEnvironment,
) -> DiscordComponent {
    DiscordComponent::Button(ButtonComponent {
        r#type: DiscordComponentType::Button,
        style: ButtonStyle::Primary,
        label: Some(env.locales.lookup_with_args(
            lang_id,
            "list_deck_page_button",
            &HashMap::from([("page", FluentValue::from(display_page + 1))]),
        )),
        custom_id: Some(format!("{}{}", LISTDECK_PAGE_BUTTON_PREFIX, display_page)),
        disabled: Some(disabled),
    })
}

fn build_get_deck_button_custom_id(deck_index: usize) -> String {
    format!("{}{}", LISTDECK_GET_DECK_BUTTON_PREFIX, deck_index)
}

fn get_listdecks_interaction_ttl() -> u64 {
    now() + LISTDECKS_INTERACTION_TTL_SECONDS
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("couldn't calculate epoch time")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use crate::test_utils::build_mocked_binahbot_env;

    use super::*;

    fn button_rows(components: &[DiscordComponent]) -> Vec<Vec<&ButtonComponent>> {
        components
            .iter()
            .map(|row| match row {
                DiscordComponent::ActionRow(x) => x
                    .components
                    .iter()
                    .map(|b| match b {
                        DiscordComponent::Button(b) => b,
                        _ => panic!("expected button"),
                    })
                    .collect(),
                _ => panic!("expected action row"),
            })
            .collect()
    }

    fn labels<'a>(row: &[&'a ButtonComponent]) -> Vec<&'a str> {
        row.iter().map(|b| b.label.as_deref().unwrap()).collect()
    }

    fn custom_ids<'a>(row: &[&'a ButtonComponent]) -> Vec<&'a str> {
        row.iter()
            .map(|b| b.custom_id.as_deref().unwrap())
            .collect()
    }

    fn disabled(row: &[&ButtonComponent]) -> Vec<bool> {
        row.iter().map(|b| b.disabled.unwrap()).collect()
    }

    fn decks(count: usize) -> Vec<DeckMetadata> {
        (0..count)
            .map(|i| DeckMetadata {
                name: format!("Deck {}", i),
                author_id: (269925702571130880u64 + i as u64).to_string(),
                author_name: format!("author {}", i),
            })
            .collect()
    }

    fn lang_id() -> LanguageIdentifier {
        LanguageIdentifier::from(&crate::models::binahbot::BinahBotLocale::EnglishUS)
    }

    #[test]
    fn deck_and_page_buttons_up_to_five_per_row() {
        let env = &build_mocked_binahbot_env();
        let decks = decks(CACHE_PAGE_SIZE);

        let components = build_buttons(2, &decks, 1000, &lang_id(), env);
        let rows = button_rows(&components);

        assert_eq!(4, rows.len());
        assert_eq!(vec!["21", "22", "23", "24", "25"], labels(&rows[0]));
        assert_eq!(vec!["26", "27", "28", "29", "30"], labels(&rows[1]));
        assert_eq!(
            vec!["ldd#20", "ldd#21", "ldd#22", "ldd#23", "ldd#24"],
            custom_ids(&rows[0])
        );
        assert!(disabled(&rows[0]).iter().all(|x| !x));
        assert_eq!(
            vec![
                "Page \u{2068}1\u{2069}",
                "Page \u{2068}2\u{2069}",
                "Page \u{2068}3\u{2069}",
                "Page \u{2068}4\u{2069}",
                "Page \u{2068}5\u{2069}"
            ],
            labels(&rows[2])
        );
        assert_eq!(
            vec!["ld#0", "ld#1", "ld#2", "ld#3", "ld#4"],
            custom_ids(&rows[2])
        );
        assert_eq!(vec![false, false, true, false, false], disabled(&rows[2]));
        assert_eq!(vec!["delete"], custom_ids(&rows[3]));
    }

    #[test]
    fn only_pages_with_results_get_buttons() {
        let env = &build_mocked_binahbot_env();

        let components = build_buttons(2, &decks(1), 21, &lang_id(), env);
        let rows = button_rows(&components);
        assert_eq!(vec!["21"], labels(&rows[0]));
        assert_eq!(3, rows[1].len());
    }

    #[test]
    fn single_page_has_no_page_buttons() {
        let env = &build_mocked_binahbot_env();

        let components = build_buttons(0, &[], 0, &lang_id(), env);
        let rows = button_rows(&components);
        assert_eq!(1, rows.len());
        assert_eq!(vec!["delete"], custom_ids(&rows[0]));

        let components =
            build_buttons(0, &decks(CACHE_PAGE_SIZE), CACHE_PAGE_SIZE, &lang_id(), env);
        let rows = button_rows(&components);
        assert_eq!(3, rows.len());
        assert_eq!(vec!["delete"], custom_ids(&rows[2]));
    }

    #[test]
    fn selecting_a_deck_enables_every_page_button() {
        let env = &build_mocked_binahbot_env();
        let decks = decks(CACHE_PAGE_SIZE);
        let components = build_buttons(1, &decks, 30, &lang_id(), env);

        let selected = select_get_deck_button(
            components,
            &build_get_deck_button_custom_id(12),
            &lang_id(),
            env,
        );
        let rows = button_rows(&selected);
        assert_eq!(vec![false, false, true, false, false], disabled(&rows[0]));
        assert_eq!(vec![false, false, false, false, false], disabled(&rows[1]));
        assert_eq!(vec![false, false, false], disabled(&rows[2]));

        // selecting another deck re-enables the previous one
        let selected = select_get_deck_button(
            selected,
            &build_get_deck_button_custom_id(17),
            &lang_id(),
            env,
        );
        let rows = button_rows(&selected);
        assert_eq!(vec![false, false, false, false, false], disabled(&rows[0]));
        assert_eq!(vec![false, false, true, false, false], disabled(&rows[1]));
    }

    #[test]
    fn selecting_a_deck_on_a_single_page_adds_a_page_button() {
        let env = &build_mocked_binahbot_env();
        let decks = decks(3);
        let components = build_buttons(0, &decks, 3, &lang_id(), env);

        let selected = select_get_deck_button(
            components,
            &build_get_deck_button_custom_id(0),
            &lang_id(),
            env,
        );
        let rows = button_rows(&selected);
        assert_eq!(3, rows.len());
        assert_eq!(vec!["Page \u{2068}1\u{2069}"], labels(&rows[1]));
        assert_eq!(vec![false], disabled(&rows[1]));
        assert_eq!(vec!["delete"], custom_ids(&rows[2]));
    }

    #[test]
    fn components_survive_a_discord_round_trip() {
        let env = &build_mocked_binahbot_env();
        let components = build_buttons(0, &decks(3), 3, &lang_id(), env);

        let json = serde_json::to_value(&components).unwrap();
        let parsed = serde_json::from_value::<Vec<DiscordComponent>>(json).unwrap();
        assert_eq!(
            button_rows(&components)
                .iter()
                .map(|x| custom_ids(x))
                .collect::<Vec<_>>(),
            button_rows(&parsed)
                .iter()
                .map(|x| custom_ids(x))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn page_and_deck_prefixes_are_distinct() {
        assert!(!LISTDECK_GET_DECK_BUTTON_PREFIX.starts_with(LISTDECK_PAGE_BUTTON_PREFIX));
        assert!(!LISTDECK_PAGE_BUTTON_PREFIX.starts_with(LISTDECK_GET_DECK_BUTTON_PREFIX));
    }

    #[test]
    fn list_is_numbered_across_pages() {
        let env = &build_mocked_binahbot_env();

        let description = build_embed(&decks(2), 1, &lang_id(), env)
            .description
            .unwrap();
        let lines = description.lines().collect::<Vec<_>>();
        assert!(lines[0].starts_with("11. \u{2068}Deck 0\u{2069}"));
        assert!(lines[1].starts_with("12. \u{2068}Deck 1\u{2069}"));
    }
}
