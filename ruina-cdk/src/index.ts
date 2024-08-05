import { App } from "aws-cdk-lib";
import "dotenv/config";
import { DiscordStack } from "./discord_stack";

const app = new App();

new DiscordStack(app, "DiscordStack", {
    clientId: process.env["CLIENT_ID"]!,
    emojis: {
        SLASH_EMOJI_ID: process.env["SLASH_EMOJI_ID"],
        PIERCE_EMOJI_ID: process.env["PIERCE_EMOJI_ID"],
        BLUNT_EMOJI_ID: process.env["BLUNT_EMOJI_ID"],
        BLOCK_EMOJI_ID: process.env["BLOCK_EMOJI_ID"],
        EVADE_EMOJI_ID: process.env["EVADE_EMOJI_ID"],
        C_SLASH_EMOJI_ID: process.env["C_SLASH_EMOJI_ID"],
        C_PIERCE_EMOJI_ID: process.env["C_PIERCE_EMOJI_ID"],
        C_BLUNT_EMOJI_ID: process.env["C_BLUNT_EMOJI_ID"],
        C_BLOCK_EMOJI_ID: process.env["C_BLOCK_EMOJI_ID"],
        C_EVADE_EMOJI_ID: process.env["C_EVADE_EMOJI_ID"],
        INSTINCT_EMOJI_ID: process.env["INSTINCT_EMOJI_ID"],
        INSIGHT_EMOJI_ID: process.env["INSIGHT_EMOJI_ID"],
        ATTACHMENT_EMOJI_ID: process.env["ATTACHMENT_EMOJI_ID"],
        REPRESSION_EMOJI_ID: process.env["REPRESSION_EMOJI_ID"],
        RED_DAMAGE_EMOJI_ID: process.env["RED_DAMAGE_EMOJI_ID"],
        WHITE_DAMAGE_EMOJI_ID: process.env["WHITE_DAMAGE_EMOJI_ID"],
        BLACK_DAMAGE_EMOJI_ID: process.env["BLACK_DAMAGE_EMOJI_ID"],
        PALE_DAMAGE_EMOJI_ID: process.env["PALE_DAMAGE_EMOJI_ID"],
        GOOD_MOOD_EMOJI_ID: process.env["GOOD_MOOD_EMOJI_ID"],
        NORMAL_MOOD_EMOJI_ID: process.env["NORMAL_MOOD_EMOJI_ID"],
        BAD_EMOJI_ID: process.env["BAD_EMOJI_ID"],
    },
});

app.synth();
