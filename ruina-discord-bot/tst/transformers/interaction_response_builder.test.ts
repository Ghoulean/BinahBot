import { DiscordEmbed } from "../../src/model/discord/discord_embed";
import { DiscordInteractionOptions } from "../../src/model/discord/discord_interaction";
import { InteractionResponseBuilder } from "../../src/transformers/interaction_response_builder";
import { DiscordInteractionTypes } from "../../src/util/constants";

const DISCORD_EMBED: DiscordEmbed = {
    fields: [],
};

const DISCORD_AUTOCOMPLETE_LENGTH: number = 3;

const DISCORD_AUTOCOMPLETE: DiscordInteractionOptions[] = Array(
    DISCORD_AUTOCOMPLETE_LENGTH
).fill({ name: "Name", value: "value" });

const BAD_REQUEST_TYPE: number = 9999;

let interactionResponseBuilder: InteractionResponseBuilder;

beforeEach(() => {
    interactionResponseBuilder = new InteractionResponseBuilder();
});

test("should built response for pings", () => {
    expect(
        interactionResponseBuilder.buildResponse(
            DiscordInteractionTypes.PING,
            null
        )
    ).toMatchSnapshot();
});

test("should built response for commands", () => {
    expect(
        interactionResponseBuilder.buildResponse(
            DiscordInteractionTypes.APPLICATION_COMMAND,
            DISCORD_EMBED
        )
    ).toMatchSnapshot();
});

test("should built response for autocomplete", () => {
    expect(
        interactionResponseBuilder.buildResponse(
            DiscordInteractionTypes.APPLICATION_COMMAND_AUTOCOMPLETE,
            DISCORD_AUTOCOMPLETE
        )
    ).toMatchSnapshot();
});

test("should error on bad request type", () => {
    expect(() => {
        interactionResponseBuilder.buildResponse(BAD_REQUEST_TYPE, null);
    }).toThrow();
});
