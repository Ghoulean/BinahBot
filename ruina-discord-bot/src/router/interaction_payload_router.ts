import { sign } from "tweetnacl";
import { SecretsAccessor } from "../accessor/secrets_accessor";
import { AutocompleteResult, CommandResult } from "../model/command_result";
import {
    DiscordInteraction,
    DiscordInteractionResponse,
} from "../model/discord/discord_interaction";
import { DiscordSecrets } from "../model/discord/discord_secrets";
import { Request } from "../model/request";
import { InteractionResponseBuilder } from "../transformers/interaction_response_builder";
import { RequestTransformer } from "../transformers/request_transformer";
import { DiscordInteractionTypes } from "../util/constants";
import { AutocompleteRouter } from "./autocomplete_router";
import { CommandRouter } from "./command_router";

export class InteractionPayloadRouter {
    private readonly interactionResponseBuilder: InteractionResponseBuilder;
    private readonly requestTransformer: RequestTransformer;
    private readonly commandRouter: CommandRouter;
    private readonly autocompleteRouter: AutocompleteRouter;
    private readonly secretsAccessor: SecretsAccessor;

    constructor(
        interactionResponseBuilder: InteractionResponseBuilder,
        requestTransformer: RequestTransformer,
        commandRouter: CommandRouter,
        autocompleteRouter: AutocompleteRouter,
        secretsAccessor: SecretsAccessor
    ) {
        this.interactionResponseBuilder = interactionResponseBuilder;
        this.requestTransformer = requestTransformer;
        this.commandRouter = commandRouter;
        this.autocompleteRouter = autocompleteRouter;
        this.secretsAccessor = secretsAccessor;
    }

    public async routeInteractionPayload(
        interaction: DiscordInteraction
    ): Promise<DiscordInteractionResponse> {
        const verify: boolean = await this.verifyInteraction(interaction);
        if (!verify) {
            throw new Error("Invalid interaction signature");
        }
        switch (interaction.type) {
            case DiscordInteractionTypes.PING: {
                return this.interactionResponseBuilder.buildPingResponse();
            }
            case DiscordInteractionTypes.APPLICATION_COMMAND: {
                const request: Request =
                    this.requestTransformer.transformInteractionToRequest(
                        interaction
                    );
                const commandResult: CommandResult =
                    this.commandRouter.routeRequest(request);
                return this.interactionResponseBuilder.buildApplicationCommandResponse(
                    commandResult
                );
            }
            case DiscordInteractionTypes.APPLICATION_COMMAND_AUTOCOMPLETE: {
                const request: Request =
                    this.requestTransformer.transformInteractionToRequest(
                        interaction
                    );
                const commandResult: AutocompleteResult =
                    this.autocompleteRouter.routeRequest(request);
                return this.interactionResponseBuilder.buildAutocompleteResponse(
                    commandResult
                );
            }
        }
        throw new Error("Bad request");
    }

    private async verifyInteraction(
        interaction: DiscordInteraction
    ): Promise<boolean> {
        let discordSecrets: DiscordSecrets;
        try {
            discordSecrets = await this.secretsAccessor.getDiscordSecrets();
        } catch (e: unknown) {
            console.log(e);
            return false;
        }
        const isVerified = sign.detached.verify(
            Buffer.from(
                interaction.metadata.timestamp + interaction.metadata.jsonBody
            ),
            Buffer.from(interaction.metadata.signature, "hex"),
            Buffer.from(discordSecrets.publicKey, "hex")
        );
        return isVerified;
    }
}
