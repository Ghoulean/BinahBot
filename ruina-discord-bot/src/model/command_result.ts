import { DiscordEmbed } from "./discord/discord_embed";

// TODO: The term "command" is overloaded to refer to a registered Discord
// slash command, and to the execution of such command. Note that "autocomplete"
// is a subset of the first, but not of the second.
export interface BaseCommandResult {
    success: boolean;
    error?: string;
};

export interface CommandResult extends BaseCommandResult {
    embed?: DiscordEmbed
}

export interface AutocompleteResult extends BaseCommandResult {
    choices?: string[]
}