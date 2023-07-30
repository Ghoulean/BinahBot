import { DiscordEmbed } from "../model/discord/discord_embed";
import {
    DiscordInteractionResponse,
    DiscordInteractionResponseAutocomplete,
    DiscordInteractionResponseMessage
} from "../model/discord/discord_interaction";
import { DiscordInteractionTypes } from "../util/constants";

const PING_DISCORD_INTERACTION_TYPE = 1;
const MESSAGE_DISCORD_INTERACTION_TYPE = 4;
const AUTOCOMPLETE_RESULT_DISCORD_INTERACTION_TYPE = 8;

const PING_RESPONSE: DiscordInteractionResponse = {
    type: PING_DISCORD_INTERACTION_TYPE,
};

export class InteractionResponseBuilder {
    public buildResponse(
        requestType: number,
        data: unknown
    ): DiscordInteractionResponse {
        switch (requestType) {
            case DiscordInteractionTypes.PING:
                return PING_RESPONSE;
            case DiscordInteractionTypes.APPLICATION_COMMAND:
                return this.buildCommandResponse(data as DiscordEmbed);
            case DiscordInteractionTypes.APPLICATION_COMMAND_AUTOCOMPLETE:
                return this.buildAutocompleteResponse(data as string[]);
            default:
                throw new Error(
                    `Unrecognized or unsupported request type ${requestType}`
                );
        }
    }

    private buildCommandResponse(
        embed: DiscordEmbed
    ): DiscordInteractionResponse {
        const discordPayload: DiscordInteractionResponseMessage = {
            embeds: [embed],
        };
        return {
            type: MESSAGE_DISCORD_INTERACTION_TYPE,
            data: discordPayload,
        };
    }

    private buildAutocompleteResponse(
        choices: string[]
    ): DiscordInteractionResponse {
        const discordPayload: DiscordInteractionResponseAutocomplete = {
            choices: choices.map((s: string) => {
                return {
                    name: s,
                    value: s,
                };
            }),
        };
        return {
            type: AUTOCOMPLETE_RESULT_DISCORD_INTERACTION_TYPE,
            data: discordPayload,
        };
    }
}
