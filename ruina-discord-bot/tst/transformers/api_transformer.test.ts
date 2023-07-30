import { APIGatewayProxyEvent } from "aws-lambda";
import { DiscordInteractionResponse } from "../../src/model/discord/discord_interaction";
import { ApiTransformer } from "../../src/transformers/api_transformer";

const APIG_PROXY_EVENT: APIGatewayProxyEvent = {
    body: "{}",
    headers: {
        "x-signature-timestamp": "timestamp",
        "x-signature-ed25519": "signature",
    },
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

const DISCORD_INTERACTION_RESPONSE: DiscordInteractionResponse = {
    type: 1,
};

let apiTransformer: ApiTransformer;

beforeEach(() => {
    apiTransformer = new ApiTransformer();
});

test("should transform apigw event into interaction payload", () => {
    expect(
        apiTransformer.transformApiRequestToInteractionPayload(APIG_PROXY_EVENT)
    ).toMatchSnapshot();
});

test("should transform response into apigw response", () => {
    expect(
        apiTransformer.transformDataBlobToApiResponse(
            DISCORD_INTERACTION_RESPONSE
        )
    ).toMatchSnapshot();
});
