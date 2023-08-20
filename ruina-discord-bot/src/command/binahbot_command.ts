import { CommandResult } from "../model/command_result";
import { DiscordEmbed } from "../model/discord/discord_embed";
import { Request } from "../model/request";

// Very slightly tinted purple
const BINAH_EMBED_COLOR: number = 0x12001f;

const GITHUB_REPO = "https://github.com/Ghoulean/Ruina/tree/main";
const INVITE_LINK_INDEX = 1;

const ABOUT_BINAHBOT_BASE_EMBED: DiscordEmbed = {
    color: BINAH_EMBED_COLOR,
    title: "About BinahBot:",
    fields: [
        {
            name: "Github repo",
            value: `[Link](${GITHUB_REPO})`,
            inline: true,
        },
        {
            name: "Invite BinahBot to your server:",
            value: "Replace this value",
            inline: true,
        },
    ],
};

// TODO: consider command interface
export class BinahBotCommand {
    private readonly discordEmbed: DiscordEmbed;

    constructor(clientId: string) {
        this.discordEmbed = this.constructDiscordEmbed(clientId);
    }

    public invoke(_request: Request): CommandResult {
        return {
            success: true,
            payload: this.discordEmbed,
        };
    }

    private constructDiscordEmbed(clientId: string): DiscordEmbed {
        const inviteLink: string = `https://discord.com/api/oauth2/authorize?client_id=${clientId}&permissions=0&scope=applications.commands%20bot`;
        const displayLink: string = `[Link](${inviteLink})`
        const retVal: DiscordEmbed = {
            ...ABOUT_BINAHBOT_BASE_EMBED,
        };
        retVal.fields[INVITE_LINK_INDEX].value = displayLink;
        return retVal;
    }
}
