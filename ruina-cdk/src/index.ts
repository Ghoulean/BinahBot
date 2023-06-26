import { App } from "aws-cdk-lib";
import { DiscordStack } from "./discord_stack";

const app = new App();

new DiscordStack(app, "DiscordStack", {});

app.synth();
