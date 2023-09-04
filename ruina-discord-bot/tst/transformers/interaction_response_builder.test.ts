import {
    AutocompleteResult,
    CommandResult,
} from "../../src/model/command_result";
import { DiscordEmbed } from "../../src/model/discord/discord_embed";
import { InteractionResponseBuilder } from "../../src/transformers/interaction_response_builder";

const DISCORD_EMBED: DiscordEmbed = {
    fields: [],
};

const DISCORD_AUTOCOMPLETE_LENGTH: number = 3;

const DISCORD_AUTOCOMPLETE: string[] = Array(DISCORD_AUTOCOMPLETE_LENGTH).fill(
    "autocomplete option"
);

const COMMAND_RESULT: CommandResult = {
    success: true,
    embed: DISCORD_EMBED,
};

const AUTOCOMPLETE_RESULT: AutocompleteResult = {
    success: true,
    choices: DISCORD_AUTOCOMPLETE,
};

let interactionResponseBuilder: InteractionResponseBuilder;

beforeEach(() => {
    interactionResponseBuilder = new InteractionResponseBuilder();
});

test("should built response for pings", () => {
    expect(interactionResponseBuilder.buildPingResponse()).toMatchSnapshot();
});

test("should built response for commands", () => {
    expect(
        interactionResponseBuilder.buildApplicationCommandResponse(
            COMMAND_RESULT
        )
    ).toMatchSnapshot();
});

test("should built response for autocomplete", () => {
    expect(
        interactionResponseBuilder.buildAutocompleteResponse(
            AUTOCOMPLETE_RESULT
        )
    ).toMatchSnapshot();
});
