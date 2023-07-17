import { SecretsManager } from "@aws-sdk/client-secrets-manager";
import {
    APIGatewayProxyEvent,
    APIGatewayProxyResult,
    Callback,
    Context,
} from "aws-lambda";
import { DataAccessor } from "./accessor/data_accessor";
import { DiscordAccessor } from "./accessor/discord_accessor";
import { CommandManager } from "./command/command_manager";
import { LorCommand } from "./command/lor_command";
import { CommandResult } from "./model/command_result";
import { DiscordInteraction } from "./model/discord/discord_interaction";
import { Request } from "./model/request";
import { AbnoToEmbedTransformer } from "./transformers/abno_to_embed_transformer";
import { InteractionToRequestTransformer } from "./transformers/interaction_to_request_transformer";
import { EnvVarRetriever } from "./util/env_var_retriever";

// TODO: reconsider where this value goes
const S3_BUCKET_NAME_KEY: string = "S3_BUCKET_NAME";

const envVarRetriever: EnvVarRetriever = new EnvVarRetriever();
const dataAccessor: DataAccessor = new DataAccessor();
const secretsManager: SecretsManager = new SecretsManager({});
const discordAccessor: DiscordAccessor = new DiscordAccessor(
    secretsManager,
    envVarRetriever
);

const interactionToRequestTransformer: InteractionToRequestTransformer =
    new InteractionToRequestTransformer();
const abnoToEmbedTransformer: AbnoToEmbedTransformer =
    new AbnoToEmbedTransformer(envVarRetriever.getRequired(S3_BUCKET_NAME_KEY));

const lorCommand: LorCommand = new LorCommand(
    dataAccessor,
    discordAccessor,
    abnoToEmbedTransformer
);
const commandManager: CommandManager = new CommandManager(lorCommand);

export async function handler(
    event: APIGatewayProxyEvent,
    _context: Context,
    _callback: Callback
): Promise<APIGatewayProxyResult> {
    console.log(`Received event: ${event}`);

    const body: DiscordInteraction = JSON.parse(event.body!);
    console.log(`Received raw discord interaction: ${body}`);

    const request: Request = interactionToRequestTransformer.transform(body);
    console.log(`Received parsed request: ${request}`);

    try {
        const result: CommandResult = await commandManager.routeRequest(
            request
        );
        console.log(
            `Result from CommandManager: success=${result.success} and error=${result.error}`
        );
    } catch (e: unknown) {
        console.log(`Error occured during command execution: ${e}`);
    } finally {
        return {
            statusCode: 200,
            body: "",
        };
    }
}
