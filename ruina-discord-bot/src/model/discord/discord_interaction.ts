/**
 * Object holding interaction data. This only reflects the information that the bot cares about; other fields are discarded.
 * 
 * See also: https://discord.com/developers/docs/interactions/receiving-and-responding#interaction-object
 */
export type DiscordInteraction = {
    id: string;
    token: string;
    locale?: string;
    guild_locale?: string;
    application_id: string;
    channel_id?: string;
    data?: {
        id: string;
        name: string;
        options?: DiscordInteractionOptions[]
    }
}

export type DiscordInteractionOptions = {
    name: string;
    value: string;
}
