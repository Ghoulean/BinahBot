import { AutocompleteResult, CommandResult } from "../model/command_result";
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
    public buildPingResponse(): DiscordInteractionResponse {
        return PING_RESPONSE;
    }

    public buildApplicationCommandResponse(
        commandResult: CommandResult
    ): DiscordInteractionResponse {
        // TODO: handle success/fail here
        const discordPayload: DiscordInteractionResponseMessage = {
            embeds: [commandResult.embed!],
        };
        return {
            type: MESSAGE_DISCORD_INTERACTION_TYPE,
            data: discordPayload,
        };
    }

    public buildAutocompleteResponse(
        autocompleteResult: AutocompleteResult
    ): DiscordInteractionResponse {
        // TODO: handle success/fail here
        const discordPayload: DiscordInteractionResponseAutocomplete = {
            choices: autocompleteResult.choices!.map((s: string) => {
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
