import {
    GetSecretValueCommandOutput,
    SecretsManager,
} from "@aws-sdk/client-secrets-manager";
import axios from "axios";
import { EnvVarRetriever } from "../../src/util/env_var_retriever";
import { DiscordAccessor } from "../../src/accessor/discord_accessor";
import { DiscordEmbed } from "../../src/model/discord/discord_embed";

jest.mock("axios");

const mockSecretsManager = new (<new () => SecretsManager>(
    SecretsManager
))() as jest.Mocked<SecretsManager>;
const mockEnvVarRetriever = new (<new () => EnvVarRetriever>(
    EnvVarRetriever
))() as jest.Mocked<EnvVarRetriever>;
const mockedAxios = axios as jest.Mocked<typeof axios>;

const INTERACTION_TOKEN: string = "interactionToken";
const DISCORD_EMBED: DiscordEmbed = {
    fields: [],
};
const DISCORD_SECRET_ENV_KEY: string = "DISCORD_BOT_API_KEY_NAME";
const DISCORD_SECRET_ENV_VALUE: string = "DISCORD_SECRET_ENV_VALUE";

const SECRET_VALUES: {
    applicationId: string;
    authToken: string;
} = {
    applicationId: "applicationId",
    authToken: "authToken",
};

const SUCCESSFUL_SECRET_RETRIEVAL: GetSecretValueCommandOutput = {
    $metadata: {},
    SecretString: JSON.stringify(SECRET_VALUES),
};
const FAILED_SECRET_RETRIEVAL: GetSecretValueCommandOutput = {
    $metadata: {
        httpStatusCode: 403,
    },
};

const SUCCESSFUL_AXIOS_POST_RESPONSE: { status: number } = {
    status: 204,
};
const TIMEOUT_AXIOS_POST_RESPONSE: { status: number } = {
    status: 408,
};

let discordAccessor: DiscordAccessor;

beforeEach(() => {
    discordAccessor = new DiscordAccessor(
        mockSecretsManager,
        mockEnvVarRetriever
    );
});

test("should successfully post discord response", () => {
    setupSecretValueMocks();
    mockedAxios.post.mockResolvedValueOnce(SUCCESSFUL_AXIOS_POST_RESPONSE);

    discordAccessor.writeResponse(INTERACTION_TOKEN, DISCORD_EMBED).then(() => {
        expect(mockEnvVarRetriever.getRequired.mock.calls.length).toBe(1);
        expect(mockEnvVarRetriever.getRequired.mock.calls[0].length).toBe(1);
        expect(mockEnvVarRetriever.getRequired.mock.calls[0][0]).toEqual(
            DISCORD_SECRET_ENV_KEY
        );
        expect(mockSecretsManager.getSecretValue.mock.calls.length).toBe(1);
        expect(mockSecretsManager.getSecretValue.mock.calls[0].length).toBe(1);
        expect(mockSecretsManager.getSecretValue.mock.calls[0][0]).toEqual({
            SecretId: DISCORD_SECRET_ENV_VALUE,
        });
        expect(mockedAxios.post.mock.calls.length).toBe(1);
        expect(mockedAxios.post.mock.calls[0].length).toBe(3);
        expect(
            mockedAxios.post.mock.calls[0][0].includes(INTERACTION_TOKEN)
        ).toBe(true);
        expect(mockedAxios.post.mock.calls[0][1]).toEqual({
            embeds: [DISCORD_EMBED],
        });
    });
});

test("should retrieve and cache secrets on invocations", () => {
    setupSecretValueMocks();
    mockedAxios.post
        .mockResolvedValueOnce(TIMEOUT_AXIOS_POST_RESPONSE)
        .mockResolvedValueOnce(SUCCESSFUL_AXIOS_POST_RESPONSE);

    discordAccessor
        .writeResponse(INTERACTION_TOKEN, DISCORD_EMBED)
        .then(() => {
            return discordAccessor.writeResponse(
                INTERACTION_TOKEN,
                DISCORD_EMBED
            );
        })
        .then(() => {
            console.log(mockedAxios.post.mock.results[0]);
            expect(mockEnvVarRetriever.getRequired.mock.calls.length).toBe(1);
            expect(mockEnvVarRetriever.getRequired.mock.calls[0].length).toBe(
                1
            );
            expect(mockEnvVarRetriever.getRequired.mock.calls[0][0]).toEqual(
                DISCORD_SECRET_ENV_KEY
            );
            expect(mockSecretsManager.getSecretValue.mock.calls.length).toBe(1);
            expect(mockSecretsManager.getSecretValue.mock.calls[0].length).toBe(
                1
            );
            expect(mockSecretsManager.getSecretValue.mock.calls[0][0]).toEqual({
                SecretId: DISCORD_SECRET_ENV_VALUE,
            });
            expect(mockedAxios.post.mock.calls.length).toBe(2);
            expect(mockedAxios.post.mock.calls[0].length).toBe(3);
            expect(mockedAxios.post.mock.results[0].type).toEqual("return");
            expect(mockedAxios.post.mock.results[1].type).toEqual("return");
            expect(mockedAxios.post.mock.results[0].value).toEqual(
                Promise.resolve(TIMEOUT_AXIOS_POST_RESPONSE)
            );
            expect(mockedAxios.post.mock.results[1].value).toEqual(
                Promise.resolve(SUCCESSFUL_AXIOS_POST_RESPONSE)
            );
        });
});

test("should throw error when axios throws error", () => {
    setupSecretValueMocks();
    const mockedAxios = axios as jest.Mocked<typeof axios>;
    mockedAxios.post.mockImplementationOnce(() => {
        return Promise.reject("error");
    });
    discordAccessor
        .writeResponse(INTERACTION_TOKEN, DISCORD_EMBED)
        .catch((e: unknown) => {
            expect(e).toBeDefined();
        });
});

test("should throw error when secrets manager does not find secret", () => {
    mockSecretsManager.getSecretValue = jest.fn();
    mockEnvVarRetriever.getRequired = jest.fn();
    mockEnvVarRetriever.getRequired.mockImplementationOnce(() => {
        return DISCORD_SECRET_ENV_VALUE;
    });
    mockSecretsManager.getSecretValue.mockImplementationOnce(() => {
        return Promise.resolve(FAILED_SECRET_RETRIEVAL);
    });

    discordAccessor
        .writeResponse(INTERACTION_TOKEN, DISCORD_EMBED)
        .catch((e: unknown) => {
            expect(e).toBeDefined();
        });
});

function setupSecretValueMocks() {
    mockSecretsManager.getSecretValue = jest.fn();
    mockEnvVarRetriever.getRequired = jest.fn();
    mockEnvVarRetriever.getRequired.mockImplementationOnce(() => {
        return DISCORD_SECRET_ENV_VALUE;
    });
    mockSecretsManager.getSecretValue.mockImplementationOnce(() => {
        return Promise.resolve(SUCCESSFUL_SECRET_RETRIEVAL);
    });
}
