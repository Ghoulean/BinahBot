import { App } from "aws-cdk-lib";
import "dotenv/config";
import { DiscordStack } from "./discord_stack";

const app = new App();

new DiscordStack(app, "DiscordStack", { 
    clientId: process.env["CLIENT_ID"]!,
    emojis: {
        "SLASH_EMOJI_ID": process.env["SLASH_EMOJI_ID"],
        "PIERCE_EMOJI_ID": process.env["PIERCE_EMOJI_ID"],
        "BLUNT_EMOJI_ID": process.env["BLUNT_EMOJI_ID"],
        "BLOCK_EMOJI_ID": process.env["BLOCK_EMOJI_ID"],
        "EVADE_EMOJI_ID": process.env["EVADE_EMOJI_ID"],
        "C_SLASH_EMOJI_ID": process.env["C_SLASH_EMOJI_ID"],
        "C_PIERCE_EMOJI_ID": process.env["C_PIERCE_EMOJI_ID"],
        "C_BLUNT_EMOJI_ID": process.env["C_BLUNT_EMOJI_ID"],
        "C_BLOCK_EMOJI_ID": process.env["C_BLOCK_EMOJI_ID"],
        "C_EVADE_EMOJI_ID": process.env["C_EVADE_EMOJI_ID"]
    }
});

app.synth();
