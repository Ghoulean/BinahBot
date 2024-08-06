use std::str::FromStr;

use lobocorp::lobocorp_common::localizations::common::Locale;

use crate::lc::button::build_buttons;
use crate::lc::button::Code;
use crate::lc::transformers::transform_encyclopedia_page;
use crate::models::binahbot::BinahBotEnvironment;
use crate::models::binahbot::BinahBotLocale;
use crate::models::discord::AllowedMentions;
use crate::models::discord::DiscordEmbed;
use crate::models::discord::DiscordInteraction;
use crate::models::discord::DiscordInteractionData;
use crate::models::discord::DiscordInteractionOptionValue;
use crate::models::discord::DiscordInteractionResponseMessage;
use crate::models::discord::DiscordInteractionResponseType;
use crate::models::discord::DiscordMessageFlag;
use crate::models::discord::MessageResponse;
use crate::utils::get_binahbot_locale;
use crate::utils::get_option_value;

pub fn lc_command(interaction: &DiscordInteraction, env: &BinahBotEnvironment) -> MessageResponse {
    let binding = match interaction.data.as_ref().expect("no data") {
        DiscordInteractionData::ApplicationCommand(x) => x,
        _ => unreachable!()
    };
    let command_args = binding.options.as_ref().unwrap();

    tracing::info!("Lc command: command args: {:#?}", command_args);

    let binah_locale: BinahBotLocale = get_binahbot_locale(interaction);

    let locale: Locale = get_option_value("locale", command_args).map(|x| match x {
        DiscordInteractionOptionValue::String(y) => y,
        _ => unreachable!()
    }).and_then(|x| Locale::from_str(x.as_str()).ok()).unwrap_or(Locale::from(binah_locale.clone()));

    let query = get_option_value("query", command_args).map(|x| match x {
        DiscordInteractionOptionValue::String(y) => y,
        _ => unreachable!()
    }).and_then(|x| x.parse::<u32>().ok()).expect("couldn't parse input");

    let embed: DiscordEmbed = transform_encyclopedia_page(
        &query,
        &locale,
        &binah_locale,
        env
    );

    let is_private = get_option_value("private", command_args).map(|x| match x {
        DiscordInteractionOptionValue::Bool(y) => y,
        _ => unreachable!()
    }).is_some_and(|x| *x);

    let flags = is_private.then_some(DiscordMessageFlag::EphemeralMessage as i32);

    let components = build_buttons(
        query, &locale, &binah_locale, &(Code::Encyclopedia, 0), env
    );

    MessageResponse {
        r#type: DiscordInteractionResponseType::ChannelMessageWithSource,
        data: Some(DiscordInteractionResponseMessage {
            allowed_mentions: Some(AllowedMentions { parse: Vec::new() }),
            content: None,
            embeds: Some(vec![embed]),
            flags: flags,
            components: Some(components),
        }),
    }
}
