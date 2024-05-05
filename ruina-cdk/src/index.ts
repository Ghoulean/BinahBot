import { App } from "aws-cdk-lib";
import "dotenv/config";
import { DiscordStack } from "./discord_stack";

const app = new App();

new DiscordStack(app, "DiscordStack", { clientId: process.env["CLIENT_ID"]! });

app.synth();
