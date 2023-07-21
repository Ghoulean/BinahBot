import {
    DecoratedAbnoPage,
    Localization,
    LookupResult,
    PageType,
} from "@ghoulean/ruina-common";
import { DataAccessor } from "../accessor/data_accessor";
import { DiscordAccessor } from "../accessor/discord_accessor";
import { CommandResult } from "../model/command_result";
import { Request } from "../model/request";
import { AbnoToEmbedTransformer } from "../transformers/abno_to_embed_transformer";
import { DiscordEmbed } from "../model/discord/discord_embed";

const QUERY_COMMAND_ARG = "query";
const LOCALE_COMMAND_ARG = "locale";

// TODO: consider interfacing invoke function
export class LorCommand {
    private readonly dataAccessor: DataAccessor;
    private readonly discordAccessor: DiscordAccessor;
    private readonly abnoToEmbedTransformer: AbnoToEmbedTransformer;

    constructor(
        dataAccessor: DataAccessor,
        discordAccessor: DiscordAccessor,
        abnoToEmbedTransformer: AbnoToEmbedTransformer
    ) {
        this.dataAccessor = dataAccessor;
        this.discordAccessor = discordAccessor;
        this.abnoToEmbedTransformer = abnoToEmbedTransformer;
    }

    public async invoke(request: Request): Promise<CommandResult> {
        const query: string = request.commandArgs[QUERY_COMMAND_ARG] as string;
        // TODO: what class should be responsible for this potentially helper method
        const locale: Localization =
            (request.commandArgs[LOCALE_COMMAND_ARG] as Localization) ??
            request.locale;

        console.log(
            `LorCommand: Received query ${query} with locale ${locale.toString()}`
        );

        const lookupResult: LookupResult = this.dataAccessor.lookup(
            query,
            locale
        );

        console.log(
            `LorCommand: Received LookupResult=${JSON.stringify(lookupResult)}`
        );

        const pageType: PageType = lookupResult.pageType;
        let embed: DiscordEmbed;
        switch (pageType) {
            case PageType.ABNO_PAGE:
                let decoratedAbnoPage: DecoratedAbnoPage;
                try {
                    decoratedAbnoPage = this.dataAccessor.getDecoratedAbnoPage(
                        lookupResult.pageId,
                        locale
                    );
                } catch (e: unknown) {
                    return {
                        success: false,
                        error: `Error occurred while calling getDecoratedAbnoPage: ${e}`,
                    };
                }
                embed =
                    this.abnoToEmbedTransformer.transform(decoratedAbnoPage);
                break;
            default:
                return {
                    success: false,
                    error: `Page type ${pageType} not yet supported`,
                };
        }

        try {
            await this.discordAccessor.writeResponse(
                request.interactionToken,
                embed
            );
            return {
                success: true,
            };
        } catch (e: unknown) {
            return {
                success: false,
                error: `Error occurred while calling discord write response: ${e}`,
            };
        }
    }
}
