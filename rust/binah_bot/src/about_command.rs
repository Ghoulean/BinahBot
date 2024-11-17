use std::str::FromStr;

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
use crate::models::discord::DiscordEmbedAuthor;
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

pub fn about_command(
    interaction: &DiscordInteraction,
    env: &BinahBotEnvironment,
) -> MessageResponse {
    let binding = vec![];
    let command_args = interaction
        .data
        .as_ref()
        .and_then(|x| cast_enum_variant!(x, DiscordInteractionData::ApplicationCommand))
        .and_then(|x| x.options.as_ref())
        .unwrap_or(&binding);

    tracing::info!("About command: command args: {:#?}", command_args);

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

    let fields = vec![DiscordEmbedFields {
        name: env.locales.lookup(&lang_id, "about_binahbot_github_header"),
        value: "https://github.com/Ghoulean/BinahBot".to_string(),
        inline: Some(true),
    }];

    let embed = DiscordEmbed {
        title: Some(env.locales.lookup(&lang_id, "about_binahbot_header")),
        description: None,
        color: Some(DiscordEmbedColors::Default as i32),
        image: None,
        thumbnail: None,
        footer: None,
        author: Some(DiscordEmbedAuthor {
            name: "ghoulean".to_string(),
            url: None,
            icon_url: Some("https://cdn.discordapp.com/avatars/269925702571130880/9a755534741a4774a53b093f440845ba.png".to_string()),
        }),
        url: None,
        fields: Some(fields),
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

#[cfg(test)]
mod tests {
    use crate::models::discord::DiscordApplicationCommandInteractionData;
    use crate::models::discord::DiscordInteractionData;
    use crate::models::discord::DiscordInteractionOptions;
    use crate::models::discord::DiscordInteractionType;
    use crate::models::discord::DiscordUser;
    use crate::test_utils::build_mocked_binahbot_env;

    use super::*;

    #[test]
    fn sanity() {
        let interaction = DiscordInteraction {
            id: "id".to_string(),
            application_id: "app_id".to_string(),
            r#type: DiscordInteractionType::ApplicationCommand,
            data: Some(DiscordInteractionData::ApplicationCommand(
                DiscordApplicationCommandInteractionData {
                    id: "id".to_string(),
                    name: "lor".to_string(),
                    options: Some(vec![DiscordInteractionOptions {
                        name: "private".to_string(),
                        name_localizations: None,
                        value: DiscordInteractionOptionValue::Bool(false),
                        focused: None,
                    }]),
                },
            )),
            channel_id: None,
            token: "token".to_string(),
            locale: None,
            guild_locale: None,
            user: Some(DiscordUser {
                id: "snowflake".to_string(),
                username: "username".to_string(),
                avatar: Some("hash".to_string()),
            }),
            member: None,
            message: None,
        };

        let _does_not_crash = about_command(&interaction, &build_mocked_binahbot_env());
    }
}
