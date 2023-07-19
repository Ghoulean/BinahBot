/**
 * Object holding embed data.
 * 
 * See also: https://discord.com/developers/docs/resources/channel#embed-object
 */
export type DiscordEmbed = {
    title?: string;
    description?: string;
    color?: number;
    image?: {
        url: string;
    },
    footer?: {
        text: string;
        icon_url?: string;
    },
    author?: {
        name: string;
        url?: string;
        icon_url?: string;
    },
    fields: DiscordEmbedFields[];
}

export type DiscordEmbedFields = {
    name: string;
    value: string;
    inline?: boolean;
}
