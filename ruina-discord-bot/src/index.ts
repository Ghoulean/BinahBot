import { SecretsManager } from "@aws-sdk/client-secrets-manager";
import {
    APIGatewayProxyEvent,
    APIGatewayProxyResult,
    Callback,
    Context,
} from "aws-lambda";
import { DataAccessor } from "./accessor/data_accessor";
import { SecretsAccessor } from "./accessor/secrets_accessor";
import { LorAutocomplete } from "./command/lor_autocomplete";
import { LorCommand } from "./command/lor_command";
import {
    DiscordInteraction,
    DiscordInteractionResponse,
} from "./model/discord/discord_interaction";
import { InteractionPayloadRouter } from "./router/interaction_payload_router";
import { RequestRouter } from "./router/request_router";
import { ApiTransformer } from "./transformers/api_transformer";
import { EmbedTransformer } from "./transformers/embed_transformer";
import { InteractionResponseBuilder } from "./transformers/interaction_response_builder";
import { RequestTransformer } from "./transformers/request_transformer";
import { EnvVarRetriever } from "./util/env_var_retriever";

// TODO: reconsider where these values goes
// (probably in constants)
const S3_BUCKET_NAME_KEY = "S3_BUCKET_NAME";
const SECRETS_ID_KEY = "SECRETS_ID";

const envVarRetriever: EnvVarRetriever = new EnvVarRetriever();
const dataAccessor: DataAccessor = new DataAccessor();

const embedTransformer: EmbedTransformer = new EmbedTransformer(
    envVarRetriever.getRequired(S3_BUCKET_NAME_KEY)
);

const secretsManager: SecretsManager = new SecretsManager({});

const secretsAccessor: SecretsAccessor = new SecretsAccessor(
    envVarRetriever.getRequired(SECRETS_ID_KEY),
    secretsManager
);

const apiTransformer: ApiTransformer = new ApiTransformer();
const interactionResponseBuilder: InteractionResponseBuilder =
    new InteractionResponseBuilder();
const requestTransformer: RequestTransformer = new RequestTransformer();

const lorCommand: LorCommand = new LorCommand(dataAccessor, embedTransformer);
const lorAutocomplete: LorAutocomplete = new LorAutocomplete(dataAccessor);
const requestRouter: RequestRouter = new RequestRouter(
    lorCommand,
    lorAutocomplete
);

const interactionPayloadRouter: InteractionPayloadRouter =
    new InteractionPayloadRouter(
        interactionResponseBuilder,
        requestTransformer,
        requestRouter,
        secretsAccessor
    );

// TODO: testing strategy
export async function handler(
    event: APIGatewayProxyEvent,
    _context: Context,
    _callback: Callback
): Promise<APIGatewayProxyResult> {
    console.log(`Received event: ${JSON.stringify(event)}`);

    const interaction: DiscordInteraction =
        apiTransformer.transformApiRequestToInteractionPayload(event);

    console.log(`Transformed into interaction: ${JSON.stringify(interaction)}`);

    const interactionResponse: DiscordInteractionResponse =
        await interactionPayloadRouter.routeInteractionPayload(interaction);

    console.log(`Responding with: ${JSON.stringify(interactionResponse)}`);

    return apiTransformer.transformDataBlobToApiResponse(interactionResponse);
}
