import { APIGatewayProxyEvent, APIGatewayProxyResult } from "aws-lambda";
import {
    DiscordInteraction,
    DiscordInteractionResponse,
} from "./model/discord/discord_interaction";
import { InteractionPayloadRouter } from "./router/interaction_payload_router";
import { ApiTransformer } from "./transformers/api_transformer";

export class IndexHandler {
    private readonly apiTransformer: ApiTransformer;
    private readonly interactionPayloadRouter: InteractionPayloadRouter;

    constructor(
        apiTransformer: ApiTransformer,
        interactionPayloadRouter: InteractionPayloadRouter
    ) {
        this.apiTransformer = apiTransformer;
        this.interactionPayloadRouter = interactionPayloadRouter;
    }

    public async handle(
        event: APIGatewayProxyEvent
    ): Promise<APIGatewayProxyResult> {
        console.log(`Received event: ${JSON.stringify(event)}`);

        const interaction: DiscordInteraction =
            this.apiTransformer.transformApiRequestToInteractionPayload(event);

        console.log(
            `Transformed into interaction: ${JSON.stringify(interaction)}`
        );

        const interactionResponse: DiscordInteractionResponse =
            await this.interactionPayloadRouter.routeInteractionPayload(
                interaction
            );

        console.log(`Responding with: ${JSON.stringify(interactionResponse)}`);

        return this.apiTransformer.transformDataBlobToApiResponse(
            interactionResponse
        );
    }
}
