import {
    DecoratedAbnoPage,
    DecoratedCombatPage,
    DecoratedKeyPage,
    DecoratedPassive,
    Localization,
    LookupResult,
    PageType,
} from "@ghoulean/ruina-common";
import { DataAccessor } from "../accessor/data_accessor";
import { CommandResult } from "../model/command_result";
import { DisambiguationResults } from "../model/disambiguation_result";
import { DiscordEmbed } from "../model/discord/discord_embed";
import { Request } from "../model/request";
import { EmbedTransformer } from "../transformers/embed_transformer";

const QUERY_COMMAND_ARG = "query";
const LOCALE_COMMAND_ARG = "locale";

// TODO: consider interfacing invoke function
export class LorCommand {
    private readonly dataAccessor: DataAccessor;
    private readonly embedTransformer: EmbedTransformer;

    constructor(
        dataAccessor: DataAccessor,
        embedTransformer: EmbedTransformer
    ) {
        this.dataAccessor = dataAccessor;
        this.embedTransformer = embedTransformer;
    }

    public invoke(request: Request): CommandResult {
        const query: string = request.commandArgs[QUERY_COMMAND_ARG] as string;
        const cardLocale: Localization =
            (request.commandArgs[LOCALE_COMMAND_ARG] as Localization) ??
            request.locale;

        console.log(
            `LorCommand: Received query ${query} with cardLocale ${cardLocale.toString()}`
        );

        let lookupResult: LookupResult;
        try {
            lookupResult = this.dataAccessor.lookup(query, cardLocale);
        } catch (e: unknown) {
            return {
                success: true,
                embed: this.embedTransformer.noResultsFoundEmbed(
                    query,
                    request.locale
                ),
            };
        }

        console.log(
            `LorCommand: Received LookupResult=${JSON.stringify(lookupResult)}`
        );

        const pageType: PageType = lookupResult.pageType;
        let embed: DiscordEmbed;
        switch (pageType) {
            case PageType.ABNO_PAGE: {
                let decoratedAbnoPage: DecoratedAbnoPage;
                try {
                    decoratedAbnoPage = this.dataAccessor.getDecoratedAbnoPage(
                        lookupResult.pageId,
                        cardLocale
                    );
                } catch (e: unknown) {
                    return {
                        success: false,
                        error: `Error occurred while calling getDecoratedAbnoPage: ${JSON.stringify(
                            e
                        )}`,
                    };
                }
                embed = this.embedTransformer.transformAbnoPage(
                    decoratedAbnoPage,
                    request.locale
                );
                break;
            }
            case PageType.COMBAT_PAGE: {
                let decoratedCombatPage: DecoratedCombatPage;
                try {
                    decoratedCombatPage =
                        this.dataAccessor.getDecoratedCombatPage(
                            lookupResult.pageId,
                            cardLocale
                        );
                } catch (e: unknown) {
                    return {
                        success: false,
                        error: `Error occurred while calling getDecoratedCombatPage: ${JSON.stringify(
                            e
                        )}`,
                    };
                }
                embed = this.embedTransformer.transformCombatPage(
                    decoratedCombatPage,
                    request.locale
                );
                break;
            }
            case PageType.KEY_PAGE: {
                let decoratedKeyPage: DecoratedKeyPage;
                try {
                    decoratedKeyPage = this.dataAccessor.getDecoratedKeyPage(
                        lookupResult.pageId,
                        cardLocale
                    );
                } catch (e: unknown) {
                    return {
                        success: false,
                        error: `Error occurred while calling getDecoratedKeyPage: ${JSON.stringify(
                            e
                        )}`,
                    };
                }
                embed = this.embedTransformer.transformKeyPage(
                    decoratedKeyPage,
                    request.locale
                );
                break;
            }
            case PageType.PASSIVE: {
                let decoratedPassive: DecoratedPassive;
                try {
                    decoratedPassive = this.dataAccessor.getDecoratedPassive(
                        lookupResult.pageId,
                        cardLocale
                    );
                } catch (e: unknown) {
                    return {
                        success: false,
                        error: `Error occurred while calling getDecoratedPassive: ${JSON.stringify(
                            e
                        )}`,
                    };
                }
                embed = this.embedTransformer.transformPassive(
                    decoratedPassive,
                    request.locale
                );
                break;
            }
            case PageType.DISAMBIGUATION: {
                let disambiguation: DisambiguationResults;
                try {
                    disambiguation = this.dataAccessor.getDisambiguationResult(
                        lookupResult.pageId
                    );
                } catch (e: unknown) {
                    return {
                        success: false,
                        error: `Error occurred while calling getDisambiguationResult: ${JSON.stringify(
                            e
                        )}`,
                    };
                }
                embed = this.embedTransformer.transformDisambiguationPage(
                    disambiguation,
                    request.locale
                );
                break;
            }
        }

        return {
            success: true,
            embed: embed,
        };
    }
}
