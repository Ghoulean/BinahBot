import { DiscordInteraction } from "../../src/model/discord/discord_interaction";
import { InteractionToRequestTransformer } from "../../src/transformers/interaction_to_request_transformer";

// TODO: reconsider testing strategy for spoiler channels
const DEBUG_SPOILER_CHANNEL = "270097753177194498";

const INTERACTION: DiscordInteraction = {
    id: "",
    token: "",
    application_id: "",
    data: {
        id: "",
        name: "",
    },
};

let interactionToRequestTransformer: InteractionToRequestTransformer;

beforeEach(() => {
    interactionToRequestTransformer = new InteractionToRequestTransformer();
});

test("should transform interaction to request", () => {
    expect(
        interactionToRequestTransformer.transform(INTERACTION)
    ).toMatchSnapshot();
});

test("should transform interaction options to command args", () => {
    const optionsInteraction: DiscordInteraction = {
        ...INTERACTION,
        data: {
            ...INTERACTION.data!,
            options: [
                {
                    name: "name",
                    value: "value",
                },
            ],
        },
    };
    expect(
        interactionToRequestTransformer.transform(optionsInteraction)
    ).toMatchSnapshot();
});

test("should prioritize locale correctly", () => {
    const guildLocaleInteraction: DiscordInteraction = {
        ...INTERACTION,
        guild_locale: "ko",
    };
    const localeInteraction: DiscordInteraction = {
        ...guildLocaleInteraction,
        locale: "ja",
    };
    expect(
        interactionToRequestTransformer.transform(guildLocaleInteraction)
    ).toMatchSnapshot();
    expect(
        interactionToRequestTransformer.transform(localeInteraction)
    ).toMatchSnapshot();
});

test("should set chapter level from spoiler config", () => {
    const interaction: DiscordInteraction = {
        ...INTERACTION,
        channel_id: DEBUG_SPOILER_CHANNEL
    };
    expect(
        interactionToRequestTransformer.transform(interaction)
    ).toMatchSnapshot();
});
