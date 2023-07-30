import { DiscordEmbed } from "./discord_embed";

/**
 * Object holding interaction data. This only reflects the information that the bot cares about; other fields are discarded.
 *
 * See also: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
 */
export type DiscordInteraction = {
    id: string;
    type: number;
    token: string;
    application_id: string;
    locale?: string;
    guild_locale?: string;
    channel_id?: string;
    data?: {
        id: string;
        name: string;
        options?: DiscordInteractionOptions[];
    };
    metadata: DiscordInteractionMetadata
};

export type DiscordInteractionOptions = {
    name: string;
    value: string;
};

export type DiscordInteractionMetadata = {
    timestamp: string;
    signature: string;
    jsonBody: string;
}

export type DiscordInteractionResponse = {
    type: number;
    data?: DiscordInteractionResponseMessage | DiscordInteractionResponseAutocomplete;
};

export type DiscordInteractionResponseMessage = {
    embeds?: DiscordEmbed[];
};

export type DiscordInteractionResponseAutocomplete = {
    choices?: DiscordInteractionOptions[];
};
