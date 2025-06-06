use serde::Deserialize;
use serde::Serialize;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;
use std::collections::HashMap;

/**
 * Object holding embed data.
 *
 * See also: https://discord.com/developers/docs/resources/channel#embed-object
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordEmbed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub color: Option<i32>,
    pub image: Option<DiscordEmbedImage>,
    pub thumbnail: Option<DiscordEmbedThumbnail>,
    pub footer: Option<DiscordEmbedFooter>,
    pub author: Option<DiscordEmbedAuthor>,
    pub url: Option<String>,
    pub fields: Option<Vec<DiscordEmbedFields>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordEmbedImage {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordEmbedThumbnail {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordEmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordEmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordEmbedFields {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

/**
 * Object holding interaction data. This only reflects the information that the bot cares about; other fields are discarded.
 *
 * See also: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordInteraction {
    pub id: String,
    pub application_id: String,
    pub r#type: DiscordInteractionType,
    pub data: Option<DiscordInteractionData>,
    pub channel_id: Option<String>,
    pub member: Option<DiscordGuildMember>,
    pub user: Option<DiscordUser>,
    pub token: String,
    pub locale: Option<String>,
    pub guild_locale: Option<String>,
    pub message: Option<DiscordMessage>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i32)]
pub enum DiscordInteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Clone, Debug)]
#[repr(i32)]
pub enum DiscordComponentType {
    ActionRow = 1,
    Button = 2,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DiscordInteractionData {
    ApplicationCommand(DiscordApplicationCommandInteractionData),
    MessageComponent(DiscordMessageComponentInteractionData),
    ModalSubmit(),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordApplicationCommandInteractionData {
    pub id: String,
    pub name: String,
    pub options: Option<Vec<DiscordInteractionOptions>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordMessageComponentInteractionData {
    pub custom_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum DiscordInteractionOptionValue {
    Bool(bool),
    String(String),
    Integer(i32),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordInteractionOptions {
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub value: DiscordInteractionOptionValue,
    pub focused: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordInteractionValidationData {
    pub timestamp: String,
    pub signature: String,
    pub json_body: String,
}

// Workaround: https://github.com/serde-rs/serde/issues/745
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DiscordInteractionResponse {
    Message(MessageResponse),
    Autocomplete(AutocompleteResponse),
    Ping(PingResponse),
    DeferredUpdateMessage(DeferredUpdateResponse),
    UpdateMessage(MessageResponse),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    pub r#type: DiscordInteractionResponseType,
    pub data: Option<DiscordInteractionResponseMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AutocompleteResponse {
    pub r#type: DiscordInteractionResponseType,
    pub data: Option<DiscordInteractionResponseAutocomplete>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingResponse {
    pub r#type: DiscordInteractionResponseType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeferredUpdateResponse {
    pub r#type: DiscordInteractionResponseType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum DiscordComponent {
    ActionRow(ActionRowComponent),
    Button(ButtonComponent),
    // remaining are not used
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionRowComponent {
    pub r#type: DiscordComponentType,
    // action row cannot contain another action row
    pub components: Vec<DiscordComponent>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Clone, Debug)]
#[repr(i32)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
    Premium = 6,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ButtonComponent {
    pub r#type: DiscordComponentType,
    pub style: ButtonStyle,
    pub label: Option<String>,
    pub custom_id: Option<String>,
    pub disabled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllowedMentions {
    pub parse: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordInteractionResponseMessage {
    pub allowed_mentions: Option<AllowedMentions>,
    pub content: Option<String>,
    pub embeds: Option<Vec<DiscordEmbed>>,
    pub flags: Option<i32>,
    pub components: Option<Vec<DiscordComponent>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordInteractionResponseAutocomplete {
    pub choices: Option<Vec<DiscordInteractionOptions>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DiscordUser {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordGuildMember {
    pub user: Option<DiscordUser>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i32)]
pub enum DiscordInteractionResponseType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DeferredUpdateMessage = 6,
    UpdateMessage = 7,
    ApplicationCommandAutocompleteResult = 8,
    Modal = 9,
    PremiumRequired = 10,
}

#[repr(i32)]
pub enum DiscordMessageFlag {
    #[allow(dead_code)]
    SuppressEmbeds = 4,
    EphemeralMessage = 64,
    #[allow(dead_code)]
    SuppressNotifications = 4096,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordMessage {
    pub interaction_metadata: Option<DiscordInteractionMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordInteractionMetadata {
    pub id: String,
    pub r#type: DiscordInteractionType,
    pub user: DiscordUser,
    pub original_response_message_id: Option<String>,
    pub interacted_message_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity_discord_response_deser() {
        let ping_response = DiscordInteractionResponse::Ping(PingResponse {
            r#type: DiscordInteractionResponseType::Pong,
        });
        assert_eq!(
            serde_json::to_string(&ping_response).expect("serialize failed"),
            "{\"type\":1}"
        );
    }
}
