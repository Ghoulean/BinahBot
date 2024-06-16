export enum CommandType {
    ChatInput = 1,
    User,
    Message
}

export enum IntegrationType {
    GuildInstall = 0,
    UserInstall = 1
}

export enum InteractionContextType {
    Guild = 0,
    BotDm,
    PrivateChannel
}

export interface DiscordApplicationCommand {
    id: string;
    type?: CommandType;
    application_id: string;
    guild_id?: string;
    name: string;
    name_localizations?: { [key: string]: string };
    description: string;
    description_localizations?: { [key: string]: string };
    options?: DiscordApplicationOption[];
    default_member_permissions: string;
    nsfw?: boolean;
    integration_types?: IntegrationType[];
    contexts?: InteractionContextType[];
    version: string;
}

export enum CommandOptionType {
    SubCommand = 1,
    SubCommandGroup,
    String,
    Integer,
    Boolean,
    User,
    Channel,
    Role,
    Mentionable,
    Number,
    Attachment
}

export enum ChannelType {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildAnnouncement = 5,
    AnnouncementThread = 10,
    PublicThread = 11,
    PrivateThread = 12,
    GuildStageVoice = 13,
    GuildDirectory = 14,
    GuildForum = 15,
    GuildMedia = 16
}

export interface DiscordApplicationOption {
    type: CommandOptionType;
    name: string;
    name_localizations?: { [key: string]: string };
    description: string;
    description_localizations?: { [key: string]: string };
    required?: boolean;
    choices?: DiscordApplicationOptionChoice[];
    options?: DiscordApplicationOption[];
    channel_types?: ChannelType

}

export interface DiscordApplicationOptionChoice {
    name: string;
    name_localizations?: { [key: string]: string };
    value: string | number;
}