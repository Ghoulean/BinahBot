import axios, { AxiosError } from "axios";
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
    // TODO: typing
    event: any,
    context: Context,
    callback: Callback
): Promise<string> {
    console.log(JSON.stringify(event, null, 4));
    const jsonBody = JSON.parse(event.body);
    const response = {
        content: "Hello world!",
    };
    const applicationId: string = jsonBody.application_id!;
    const interactionToken: string = jsonBody.token!;
    if (await sendResponse(response, applicationId, interactionToken)) {
        console.log("Responded successfully!");
    } else {
        console.log("Failed to send response!");
    }
    return "200";
}
async function sendResponse(
    // TODO: typing
    response: any,
    applicationId: string,
    interactionToken: string
): Promise<boolean> {
    console.log("Trying to send response. Getting secret...");
    const discordSecret = await getDiscordSecrets();
    console.log("Got secret! Now sending response message");
    const authConfig = {
        headers: {
            Authorization: `Bot ${discordSecret?.authToken}`,
        },
    };
    try {
        let url = `https://discord.com/api/v10/webhooks/${applicationId}/${interactionToken}`;
        return (await axios.post(url, response, authConfig)).status == 200;
    } catch (e: any | AxiosError) {
        console.log(`There was an error posting a response: ${e}`);
        if (axios.isAxiosError(e)) {
            console.log(e.message);
            console.log(e.response?.data);
        } else {
            console.log("It's not an AxiosError");
        }
        return false;
    }
}
