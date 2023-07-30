import {
    GetSecretValueCommand,
    GetSecretValueCommandOutput,
    SecretsManager,
} from "@aws-sdk/client-secrets-manager";
import { SecretsAccessor } from "../../src/accessor/secrets_accessor";
import { DiscordSecrets } from "../../src/model/discord/discord_secrets";

const mockSecretsManager = new (<new () => SecretsManager>(
    SecretsManager
))() as jest.Mocked<SecretsManager>;
const mockSecretsId: string = "mockSecretsId";

const SECRET_VALUES: DiscordSecrets = {
    applicationId: "applicationId",
    authToken: "authToken",
    publicKey: "publicKey",
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

let secretsAccessor: SecretsAccessor;

beforeEach(() => {
    secretsAccessor = new SecretsAccessor(mockSecretsId, mockSecretsManager);
});

test("should successfully retrieve secrets value, cache or otherwise", async () => {
    mockSecretsManager.send = jest.fn();
    mockSecretsManager.send.mockImplementationOnce(() => {
        return Promise.resolve(SUCCESSFUL_SECRET_RETRIEVAL);
    });

    const discordSecrets: DiscordSecrets =
        await secretsAccessor.getDiscordSecrets();

    expect(discordSecrets).toEqual(SECRET_VALUES);

    expect(mockSecretsManager.send.mock.calls.length).toBe(1);
    expect(mockSecretsManager.send.mock.calls[0].length).toBe(1);
    // toMatchObject doesn't work; one serializes to GetSecretValueCommand and the other to Object
    expect(JSON.stringify(mockSecretsManager.send.mock.calls[0][0])).toEqual(
        JSON.stringify(new GetSecretValueCommand({
            SecretId: mockSecretsId,
        }))
    );

    const secondDiscordSecrets: DiscordSecrets =
        await secretsAccessor.getDiscordSecrets();
    expect(secondDiscordSecrets).toEqual(SECRET_VALUES);
    expect(mockSecretsManager.send.mock.calls.length).toBe(1);
});

test("should error if secrets manager errors", () => {
    mockSecretsManager.send = jest.fn();
    mockSecretsManager.send.mockImplementationOnce(() => {
        return Promise.reject("error");
    });

    expect(secretsAccessor.getDiscordSecrets()).rejects.toBeDefined();
});

test("should error if secrets manager returns failed retrieval", () => {
    mockSecretsManager.send = jest.fn();
    mockSecretsManager.send.mockImplementationOnce(() => {
        return Promise.resolve(FAILED_SECRET_RETRIEVAL);
    });

    expect(secretsAccessor.getDiscordSecrets()).rejects.toBeDefined();
});
