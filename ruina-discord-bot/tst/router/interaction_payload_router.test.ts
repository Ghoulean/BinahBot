import { Chapter, Localization } from "@ghoulean/ruina-common";
import { sign } from "tweetnacl";
import { SecretsAccessor } from "../../src/accessor/secrets_accessor";
import { AutocompleteResult, CommandResult } from "../../src/model/command_result";
import {
    DiscordInteraction,
    DiscordInteractionResponse,
} from "../../src/model/discord/discord_interaction";
import { DiscordSecrets } from "../../src/model/discord/discord_secrets";
import { Request } from "../../src/model/request";
import { InteractionPayloadRouter } from "../../src/router/interaction_payload_router";
import { RequestRouter } from "../../src/router/request_router";
import { InteractionResponseBuilder } from "../../src/transformers/interaction_response_builder";
import { RequestTransformer } from "../../src/transformers/request_transformer";
import { DiscordInteractionTypes } from "../../src/util/constants";
import { CommandRouter } from "../../src/router/command_router";
import { AutocompleteRouter } from "../../src/router/autocomplete_router";

jest.mock("tweetnacl");

const BASE_INTERACTION: DiscordInteraction = {
    id: "",
    type: 0,
    token: "",
    application_id: "",
    metadata: {
        timestamp: "",
        signature: "",
        jsonBody: "",
    },
};

const BAD_INTERACTION_TYPE: number = 9999;

const DISCORD_SECRETS: DiscordSecrets = {
    applicationId: "",
    authToken: "",
    publicKey: "",
};

const REQUEST: Request = {
    command: "",
    commandArgs: {},
    interactionToken: "",
    locale: Localization.ENGLISH,
    chapter: Chapter.IMPURITAS_CIVITATIS,
};

const COMMAND_RESULT: CommandResult = {
    success: true,
};
const AUTOCOMPLETE_RESULT: AutocompleteResult = {
    success: true,
};

const DISCORD_INTERACTION_RESPONSE: DiscordInteractionResponse = {
    type: 0,
};

const mockInteractionResponseBuilder = new (<
    new () => InteractionResponseBuilder
>InteractionResponseBuilder)() as jest.Mocked<InteractionResponseBuilder>;
const mockRequestTransformer = new (<new () => RequestTransformer>(
    RequestTransformer
))() as jest.Mocked<RequestTransformer>;
const mockCommandRouter = new (<new () => CommandRouter>(
    CommandRouter
))() as jest.Mocked<CommandRouter>;
const mockAutocompleteRouter = new (<new () => AutocompleteRouter>(
    AutocompleteRouter
))() as jest.Mocked<AutocompleteRouter>;
const mockSecretsAccessor = new (<new () => SecretsAccessor>(
    SecretsAccessor
))() as jest.Mocked<SecretsAccessor>;
const mockSignDetached = sign.detached as jest.Mocked<typeof sign.detached>;

let interactionPayloadRouter: InteractionPayloadRouter;

beforeEach(() => {
    interactionPayloadRouter = new InteractionPayloadRouter(
        mockInteractionResponseBuilder,
        mockRequestTransformer,
        mockCommandRouter,
        mockAutocompleteRouter,
        mockSecretsAccessor
    );
});

test("should fast return on ping interactions", () => {
    const interaction: DiscordInteraction = {
        ...BASE_INTERACTION,
        type: DiscordInteractionTypes.PING,
    };

    setupVerifyMocks();
    mockInteractionResponseBuilder.buildPingResponse = jest.fn();

    mockInteractionResponseBuilder.buildPingResponse.mockReturnValueOnce(
        DISCORD_INTERACTION_RESPONSE
    );

    expect(
        interactionPayloadRouter.routeInteractionPayload(interaction)
    ).resolves.toEqual(DISCORD_INTERACTION_RESPONSE);
});

test.each([
    {
        descriptor: "command",
        
    },
    {
        descriptor: "autocomplete",
        interaction: {
            ...BASE_INTERACTION,
            type: DiscordInteractionTypes.APPLICATION_COMMAND_AUTOCOMPLETE,
        },
    },
])

test("should route command interactions to command router",
    async () => {
        setupVerifyMocks();
        mockRequestTransformer.transformInteractionToRequest = jest.fn();
        mockCommandRouter.routeRequest = jest.fn();
        mockInteractionResponseBuilder.buildApplicationCommandResponse = jest.fn();

        mockRequestTransformer.transformInteractionToRequest.mockReturnValueOnce(
            REQUEST
        );
        mockCommandRouter.routeRequest.mockReturnValueOnce(COMMAND_RESULT);
        mockInteractionResponseBuilder.buildApplicationCommandResponse.mockReturnValueOnce(
            DISCORD_INTERACTION_RESPONSE
        );

        const interaction: DiscordInteraction = {
            ...BASE_INTERACTION,
            type: DiscordInteractionTypes.APPLICATION_COMMAND,
        };
        const interactionResponse: DiscordInteractionResponse =
            await interactionPayloadRouter.routeInteractionPayload(interaction);

        expect(interactionResponse).toEqual(DISCORD_INTERACTION_RESPONSE);

        expect(
            mockRequestTransformer.transformInteractionToRequest.mock.calls
                .length
        ).toBe(1);
        expect(
            mockRequestTransformer.transformInteractionToRequest.mock.calls[0]
                .length
        ).toBe(1);
        expect(
            mockRequestTransformer.transformInteractionToRequest.mock
                .calls[0][0]
        ).toEqual(interaction);
        expect(mockCommandRouter.routeRequest.mock.calls.length).toBe(1);
        expect(mockCommandRouter.routeRequest.mock.calls[0].length).toBe(1);
        expect(mockCommandRouter.routeRequest.mock.calls[0][0]).toEqual(
            REQUEST
        );
        expect(
            mockInteractionResponseBuilder.buildApplicationCommandResponse.mock.calls.length
        ).toBe(1);
        expect(
            mockInteractionResponseBuilder.buildApplicationCommandResponse.mock.calls[0].length
        ).toBe(1);
        expect(
            mockInteractionResponseBuilder.buildApplicationCommandResponse.mock.calls[0][0]
        ).toEqual(COMMAND_RESULT);
    }
);


test("should route autocomplete interactions to autocomplete router",
    async () => {
        setupVerifyMocks();
        mockRequestTransformer.transformInteractionToRequest = jest.fn();
        mockAutocompleteRouter.routeRequest = jest.fn();
        mockInteractionResponseBuilder.buildAutocompleteResponse = jest.fn();

        mockRequestTransformer.transformInteractionToRequest.mockReturnValueOnce(
            REQUEST
        );
        mockAutocompleteRouter.routeRequest.mockReturnValueOnce(AUTOCOMPLETE_RESULT);
        mockInteractionResponseBuilder.buildAutocompleteResponse.mockReturnValueOnce(
            DISCORD_INTERACTION_RESPONSE
        );

        const interaction: DiscordInteraction = {
            ...BASE_INTERACTION,
            type: DiscordInteractionTypes.APPLICATION_COMMAND_AUTOCOMPLETE,
        };
        const interactionResponse: DiscordInteractionResponse =
            await interactionPayloadRouter.routeInteractionPayload(interaction);

        expect(interactionResponse).toEqual(DISCORD_INTERACTION_RESPONSE);

        expect(
            mockRequestTransformer.transformInteractionToRequest.mock.calls
                .length
        ).toBe(1);
        expect(
            mockRequestTransformer.transformInteractionToRequest.mock.calls[0]
                .length
        ).toBe(1);
        expect(
            mockRequestTransformer.transformInteractionToRequest.mock
                .calls[0][0]
        ).toEqual(interaction);
        expect(mockAutocompleteRouter.routeRequest.mock.calls.length).toBe(1);
        expect(mockAutocompleteRouter.routeRequest.mock.calls[0].length).toBe(1);
        expect(mockAutocompleteRouter.routeRequest.mock.calls[0][0]).toEqual(
            REQUEST
        );
        expect(
            mockInteractionResponseBuilder.buildAutocompleteResponse.mock.calls.length
        ).toBe(1);
        expect(
            mockInteractionResponseBuilder.buildAutocompleteResponse.mock.calls[0].length
        ).toBe(1);
        expect(
            mockInteractionResponseBuilder.buildAutocompleteResponse.mock.calls[0][0]
        ).toEqual(AUTOCOMPLETE_RESULT);
    }
);

test("should throw error if bad interaction type", () => {
    const interaction: DiscordInteraction = {
        ...BASE_INTERACTION,
        type: BAD_INTERACTION_TYPE,
    };

    setupVerifyMocks();

    expect(
        interactionPayloadRouter.routeInteractionPayload(interaction)
    ).rejects.toBeDefined();
});

test("should throw error if can't get discord secrets", () => {
    mockSecretsAccessor.getDiscordSecrets = jest.fn();

    mockSecretsAccessor.getDiscordSecrets.mockRejectedValueOnce("error");

    expect(
        interactionPayloadRouter.routeInteractionPayload(BASE_INTERACTION)
    ).rejects.toBeDefined();
});

test("should throw error if signature fails", () => {
    mockSecretsAccessor.getDiscordSecrets = jest.fn();
    mockSignDetached.verify = jest.fn();

    mockSecretsAccessor.getDiscordSecrets.mockResolvedValueOnce(
        DISCORD_SECRETS
    );
    mockSignDetached.verify.mockImplementationOnce(() => {
        return false;
    });

    expect(
        interactionPayloadRouter.routeInteractionPayload(BASE_INTERACTION)
    ).rejects.toBeDefined();
});

function setupVerifyMocks() {
    mockSecretsAccessor.getDiscordSecrets = jest.fn();
    mockSignDetached.verify = jest.fn();

    mockSecretsAccessor.getDiscordSecrets.mockResolvedValueOnce(
        DISCORD_SECRETS
    );
    mockSignDetached.verify.mockImplementationOnce(() => {
        return true;
    });
}
