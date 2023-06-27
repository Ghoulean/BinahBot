import axios from "axios";
import { Context, Callback } from "aws-lambda";
// TODO: move getDiscordSecrets to common package
import {
    IDiscordEventRequest,
    getDiscordSecrets,
} from "discord-bot-cdk-construct";

// TODO: Use Typescript discord library instead of homebaked interfaces
interface DiscordResponse {
    content: string,
}

export async function handler(
    event: IDiscordEventRequest,
    context: Context,
    callback: Callback
): Promise<string> {
    const response = {
        content: "Hello world!",
    };
    
    if (
        event.jsonBody.token && event.jsonBody.id &&
        (await sendResponse(response, event.jsonBody.id, event.jsonBody.token))
    ) {
        console.log("Responded successfully!");
    } else {
        console.log("Failed to send response!");
    }
    return "200";
}
async function sendResponse(
    response: DiscordResponse,
    webhookId: string,
    interactionToken: string,
): Promise<boolean> {
    const discordSecret = await getDiscordSecrets();
    const authConfig = {
        headers: {
            Authorization: `Bot ${discordSecret?.authToken}`,
        },
    };
    try {
        let url = `https://discord.com/api/v10/webhooks/${webhookId}/${interactionToken}`;
        return (await axios.post(url, response, authConfig)).status == 200;
    } catch (exception) {
        console.log(`There was an error posting a response: ${exception}`);
        return false;
    }
}
