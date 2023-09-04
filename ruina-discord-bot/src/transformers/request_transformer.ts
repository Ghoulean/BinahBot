import { Chapter } from "@ghoulean/ruina-common";
import * as __SPOILER_CONFIG from "../config/spoiler_config.json";
import {
    DiscordInteraction,
    DiscordInteractionOptions,
} from "../model/discord/discord_interaction";
import { Request, RequestCommandArgs } from "../model/request";
import { DiscordInteractionTypes } from "../util/constants";
import { Util } from "../util/util";

// TODO: consider spoiler config accessor?
const SPOILER_CONFIG = __SPOILER_CONFIG as { [key: string]: Chapter };

const DEFAULT_DISCORD_LOCALE = "en-US";

export class RequestTransformer {
    constructor() {}

    public transformInteractionToRequest(
        interaction: DiscordInteraction
    ): Request {
        const locale: string =
            interaction.locale ??
            interaction.guild_locale ??
            DEFAULT_DISCORD_LOCALE;
        if (!interaction.data) {
            throw new Error("Interaction data not found");
        }
        const commandArgs: RequestCommandArgs = interaction.data.options
            ? this.transformArgs(interaction.data.options)
            : {};
        return {
            command: interaction.data.name,
            commandArgs: commandArgs,
            interactionToken: interaction.token,
            locale: Util.deserializeDiscordLocale(locale),
            chapter: this.getChapter(interaction.channel_id)
        };
    }

    private transformArgs(
        discordInteractionOptions: DiscordInteractionOptions[]
    ): RequestCommandArgs {
        const retVal: RequestCommandArgs = {};
        for (const option of discordInteractionOptions) {
            retVal[option.name] = option.value;
        }
        return retVal;
    }

    private getChapter(channelId?: string): Chapter {
        return (
            (channelId && SPOILER_CONFIG[channelId]) ||
            Chapter.IMPURITAS_CIVITATIS
        );
    }
}
