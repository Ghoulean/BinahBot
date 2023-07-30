import {
    GetSecretValueCommand,
    GetSecretValueCommandInput,
    GetSecretValueCommandOutput,
    SecretsManager,
} from "@aws-sdk/client-secrets-manager";
import { DiscordSecrets } from "../model/discord/discord_secrets";

export class SecretsAccessor {
    private readonly secretsId: string;
    private readonly secretsManager: SecretsManager;
    private discordSecrets: DiscordSecrets;

    constructor(secretsId: string, secretsManager: SecretsManager) {
        this.secretsId = secretsId;
        this.secretsManager = secretsManager;
    }

    public async getDiscordSecrets(): Promise<DiscordSecrets> {
        return this.discordSecrets ?? await this.cacheDiscordSecrets();
    }

    private async cacheDiscordSecrets(): Promise<DiscordSecrets> {
        const input: GetSecretValueCommandInput = {
            SecretId: this.secretsId,
        };
        const command: GetSecretValueCommand = new GetSecretValueCommand(input);
        let discordApiKeys: GetSecretValueCommandOutput;
        try {
            discordApiKeys = await this.secretsManager.send(command);
        } catch (e: unknown) {
            throw new Error(`Unable to get Discord secrets: ${JSON.stringify(e)}`);
        }
        if (discordApiKeys.SecretString) {
            this.discordSecrets = JSON.parse(
                discordApiKeys.SecretString
            ) as DiscordSecrets;
        } else {
            throw new Error(
                `Got Discord secrets, but can't parse SecretString in ${JSON.stringify(discordApiKeys)}`
            );
        }
        return this.discordSecrets;
    }
}
