import {
    GetSecretValueCommandOutput,
    SecretsManager,
} from "@aws-sdk/client-secrets-manager";
import axios from "axios";
import { EnvVarRetriever } from "../util/env_var_retriever";

const BASE_DISCORD_URL: string = "https://discord.com/api/v10";
// TODO: reconsider where this value goes
// TODO: rename
const DISCORD_SECRET_ENV_KEY: string = "DISCORD_BOT_API_KEY_NAME";

// TODO: reconsider where this type should go
type DiscordSecretsValues = {
    applicationId: string;
    authToken: string;
};

export class DiscordAccessor {
    private readonly secretsManager: SecretsManager;
    private readonly envVarRetriever: EnvVarRetriever;
    private applicationId: string;
    private botAuthToken: string;

    constructor(
        secretsManager: SecretsManager,
        envVarRetriever: EnvVarRetriever
    ) {
        this.secretsManager = secretsManager;
        this.envVarRetriever = envVarRetriever;
    }

    public async writeResponse(
        interactionToken: string,
        // TODO: strongly typed embed
        embed: any
    ): Promise<void> {
        console.log(`Retrieving secrets (if not done so already)`);
        await this.retrieveSecrets();
        console.log(`Got secrets`);
        try {
            const discordPayload: any = {
                embeds: [embed],
            };
            console.log(
                `DiscordAccessor: writing response with applicationId=${this.applicationId} interactionToken=${interactionToken}`
            );
            console.log(
                `DiscordAccessor: writing response with discordPayload=${JSON.stringify(
                    discordPayload
                )}`
            );
            const { status } = await axios.post(
                `${BASE_DISCORD_URL}/webhooks/${this.applicationId}/${interactionToken}`,
                discordPayload,
                {
                    headers: {
                        Authorization: `Bot ${this.botAuthToken}`,
                        "Content-Type": "application/json",
                    },
                }
            );
            if (status === 204) {
                console.log("Successfully returned Discord response");
            }
        } catch (e) {
            throw e;
        }
    }

    private async retrieveSecrets(): Promise<void> {
        if (this.botAuthToken) {
            return;
        }
        const getSecretValueCommandOutput: GetSecretValueCommandOutput =
            await this.secretsManager.getSecretValue({
                SecretId: this.envVarRetriever.getRequired(
                    DISCORD_SECRET_ENV_KEY
                ),
            });
        if (getSecretValueCommandOutput.SecretString) {
            const secrets: DiscordSecretsValues = JSON.parse(
                getSecretValueCommandOutput.SecretString
            ) as DiscordSecretsValues;
            this.applicationId = secrets.applicationId;
            this.botAuthToken = secrets.authToken;
        }
    }
}
