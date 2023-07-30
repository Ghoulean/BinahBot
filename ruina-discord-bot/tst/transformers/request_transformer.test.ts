import { DiscordInteraction } from "../../src/model/discord/discord_interaction";
import { RequestTransformer } from "../../src/transformers/request_transformer";

// TODO: reconsider testing strategy for spoiler channels.
// Possibly add abstraction layer over this
const DEBUG_SPOILER_CHANNEL = "270097753177194498";

const BASE_INTERACTION: DiscordInteraction = {
    id: "",
    token: "",
    application_id: "",
    data: {
        id: "",
        name: "",
    },
    type: 1,
    metadata: {
        timestamp: "",
        signature: "",
        jsonBody: "",
    },
};

let requestTransformer: RequestTransformer;

beforeEach(() => {
    requestTransformer = new RequestTransformer();
});

test("should transform interaction into request", () => {
    const optionsInteraction: DiscordInteraction = {
        ...BASE_INTERACTION,
        data: {
            ...BASE_INTERACTION.data!,
            options: [
                {
                    name: "name",
                    value: "value",
                },
            ],
        },
    };
    expect(
        requestTransformer.transformInteractionToRequest(optionsInteraction)
    ).toMatchSnapshot();
});

test("should prioritize locale correctly", () => {
    const guildLocaleInteraction: DiscordInteraction = {
        ...BASE_INTERACTION,
        guild_locale: "ko",
    };
    const localeInteraction: DiscordInteraction = {
        ...guildLocaleInteraction,
        locale: "ja",
    };
    expect(
        requestTransformer.transformInteractionToRequest(guildLocaleInteraction)
    ).toMatchSnapshot();
    expect(
        requestTransformer.transformInteractionToRequest(localeInteraction)
    ).toMatchSnapshot();
});

test("should set chapter level from spoiler config", () => {
    const interaction: DiscordInteraction = {
        ...BASE_INTERACTION,
        channel_id: DEBUG_SPOILER_CHANNEL,
    };
    expect(
        requestTransformer.transformInteractionToRequest(interaction)
    ).toMatchSnapshot();
});

test("should throw error if interaction has no data", () => {
    const interaction: DiscordInteraction = {
        ...BASE_INTERACTION,
        data: undefined,
    };
    expect(() => {
        requestTransformer.transformInteractionToRequest(interaction);
    }).toThrow();
});
