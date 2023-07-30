import { APIGatewayProxyEvent, APIGatewayProxyResult } from "aws-lambda";
import { DiscordInteraction } from "../model/discord/discord_interaction";

const TIMESTAMP_HEADER = "x-signature-timestamp";
const SIGNATURE_HEADER = "x-signature-ed25519";

export class ApiTransformer {
    constructor() {}

    public transformApiRequestToInteractionPayload(
        event: APIGatewayProxyEvent
    ): DiscordInteraction {
        return {
            ...JSON.parse(event.body!) as DiscordInteraction,
            metadata: {
                timestamp: event.headers[TIMESTAMP_HEADER]!,
                signature: event.headers[SIGNATURE_HEADER]!,
                jsonBody: event.body!,
            },
        };
    }

    public transformDataBlobToApiResponse(blob: any): APIGatewayProxyResult {
        return {
            statusCode: 200,
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(blob),
        };
    }
}
