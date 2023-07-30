import { Chapter, Localization } from "@ghoulean/ruina-common";
import { sign } from "tweetnacl";
import { SecretsAccessor } from "../../src/accessor/secrets_accessor";
import { CommandResult } from "../../src/model/command_result";
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

const DISCORD_INTERACTION_RESPONSE: DiscordInteractionResponse = {
    type: 0,
};

const mockInteractionResponseBuilder = new (<
    new () => InteractionResponseBuilder
>InteractionResponseBuilder)() as jest.Mocked<InteractionResponseBuilder>;
const mockRequestTransformer = new (<new () => RequestTransformer>(
    RequestTransformer
))() as jest.Mocked<RequestTransformer>;
const mockRequestRouter = new (<new () => RequestRouter>(
    RequestRouter
))() as jest.Mocked<RequestRouter>;
const mockSecretsAccessor = new (<new () => SecretsAccessor>(
    SecretsAccessor
))() as jest.Mocked<SecretsAccessor>;
const mockSignDetached = sign.detached as jest.Mocked<typeof sign.detached>;

let interactionPayloadRouter: InteractionPayloadRouter;

beforeEach(() => {
    interactionPayloadRouter = new InteractionPayloadRouter(
        mockInteractionResponseBuilder,
        mockRequestTransformer,
        mockRequestRouter,
        mockSecretsAccessor
    );
});

test("should fast return on ping interactions", () => {
    const interaction: DiscordInteraction = {
        ...BASE_INTERACTION,
        type: DiscordInteractionTypes.PING,
    };

    setupVerifyMocks();
    mockInteractionResponseBuilder.buildResponse = jest.fn();

    mockInteractionResponseBuilder.buildResponse.mockReturnValueOnce(
        DISCORD_INTERACTION_RESPONSE
    );

    expect(
        interactionPayloadRouter.routeInteractionPayload(interaction)
    ).resolves.toEqual(DISCORD_INTERACTION_RESPONSE);
});

test.each([
    {
        descriptor: "command",
        interaction: {
            ...BASE_INTERACTION,
            type: DiscordInteractionTypes.APPLICATION_COMMAND,
        },
    },
    {
        descriptor: "autocomplete",
        interaction: {
            ...BASE_INTERACTION,
            type: DiscordInteractionTypes.APPLICATION_COMMAND_AUTOCOMPLETE,
        },
    },
])(
    "should route $descriptor interactions to request router",
    async ({ interaction }) => {
        setupVerifyMocks();
        mockRequestTransformer.transformInteractionToRequest = jest.fn();
        mockRequestRouter.routeRequest = jest.fn();
        mockInteractionResponseBuilder.buildResponse = jest.fn();

        mockRequestTransformer.transformInteractionToRequest.mockReturnValueOnce(
            REQUEST
        );
        mockRequestRouter.routeRequest.mockReturnValueOnce(COMMAND_RESULT);
        mockInteractionResponseBuilder.buildResponse.mockReturnValueOnce(
            DISCORD_INTERACTION_RESPONSE
        );

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
        expect(mockRequestRouter.routeRequest.mock.calls.length).toBe(1);
        expect(mockRequestRouter.routeRequest.mock.calls[0].length).toBe(1);
        expect(mockRequestRouter.routeRequest.mock.calls[0][0]).toEqual(
            REQUEST
        );
        expect(
            mockInteractionResponseBuilder.buildResponse.mock.calls.length
        ).toBe(1);
        expect(
            mockInteractionResponseBuilder.buildResponse.mock.calls[0].length
        ).toBe(2);
        expect(
            mockInteractionResponseBuilder.buildResponse.mock.calls[0][0]
        ).toEqual(interaction.type);
        expect(
            mockInteractionResponseBuilder.buildResponse.mock.calls[0][1]
        ).toEqual(COMMAND_RESULT.payload);
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
