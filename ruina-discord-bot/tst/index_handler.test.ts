import { APIGatewayProxyEvent, APIGatewayProxyResult } from "aws-lambda";
import { IndexHandler } from "../src/index_handler";
import {
    DiscordInteraction,
    DiscordInteractionResponse,
} from "../src/model/discord/discord_interaction";
import { InteractionPayloadRouter } from "../src/router/interaction_payload_router";
import { ApiTransformer } from "../src/transformers/api_transformer";

const EVENT: APIGatewayProxyEvent = {
    body: "{}",
    headers: {},
    multiValueHeaders: {},
    httpMethod: "",
    isBase64Encoded: false,
    path: "",
    pathParameters: null,
    queryStringParameters: null,
    multiValueQueryStringParameters: null,
    stageVariables: null,
    requestContext: {
        accountId: "",
        apiId: "",
        authorizer: undefined,
        protocol: "",
        httpMethod: "",
        identity: {
            accessKey: null,
            accountId: null,
            apiKey: null,
            apiKeyId: null,
            caller: null,
            clientCert: null,
            cognitoAuthenticationProvider: null,
            cognitoAuthenticationType: null,
            cognitoIdentityId: null,
            cognitoIdentityPoolId: null,
            principalOrgId: null,
            sourceIp: "",
            user: null,
            userAgent: null,
            userArn: null,
        },
        path: "",
        stage: "",
        requestId: "",
        requestTimeEpoch: 0,
        resourceId: "",
        resourcePath: "",
    },
    resource: "",
};

const INTERACTION: DiscordInteraction = {
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

const INTERACTION_RESPONSE: DiscordInteractionResponse = {
    type: 0,
};

const API_GATEWAY_PROXY_RESULT: APIGatewayProxyResult = {
    statusCode: 0,
    body: "",
};

const mockApiTransformer = new (<new () => ApiTransformer>(
    ApiTransformer
))() as jest.Mocked<ApiTransformer>;
const mockInteractionPayloadRouter = new (<new () => InteractionPayloadRouter>(
    InteractionPayloadRouter
))() as jest.Mocked<InteractionPayloadRouter>;

let indexHandler: IndexHandler;

beforeEach(() => {
    indexHandler = new IndexHandler(
        mockApiTransformer,
        mockInteractionPayloadRouter
    );
});

test("should handle request", async () => {
    mockApiTransformer.transformApiRequestToInteractionPayload = jest.fn();
    mockApiTransformer.transformDataBlobToApiResponse = jest.fn();
    mockInteractionPayloadRouter.routeInteractionPayload = jest.fn();

    mockApiTransformer.transformApiRequestToInteractionPayload.mockReturnValueOnce(
        INTERACTION
    );
    mockInteractionPayloadRouter.routeInteractionPayload.mockResolvedValueOnce(
        INTERACTION_RESPONSE
    );
    mockApiTransformer.transformDataBlobToApiResponse.mockReturnValueOnce(
        API_GATEWAY_PROXY_RESULT
    );

    const result: APIGatewayProxyResult = await indexHandler.handle(EVENT);

    expect(result).toEqual(API_GATEWAY_PROXY_RESULT);
    expect(
        mockApiTransformer.transformApiRequestToInteractionPayload.mock.calls
            .length
    ).toBe(1);
    expect(
        mockApiTransformer.transformApiRequestToInteractionPayload.mock.calls[0]
            .length
    ).toBe(1);
    expect(
        mockApiTransformer.transformApiRequestToInteractionPayload.mock
            .calls[0][0]
    ).toEqual(EVENT);
    expect(
        mockInteractionPayloadRouter.routeInteractionPayload.mock.calls.length
    ).toBe(1);
    expect(
        mockInteractionPayloadRouter.routeInteractionPayload.mock.calls[0]
            .length
    ).toBe(1);
    expect(
        mockInteractionPayloadRouter.routeInteractionPayload.mock.calls[0][0]
    ).toEqual(INTERACTION);
    expect(
        mockApiTransformer.transformDataBlobToApiResponse.mock.calls.length
    ).toBe(1);
    expect(
        mockApiTransformer.transformDataBlobToApiResponse.mock.calls[0].length
    ).toBe(1);
    expect(
        mockApiTransformer.transformDataBlobToApiResponse.mock.calls[0][0]
    ).toEqual(INTERACTION_RESPONSE);
});
