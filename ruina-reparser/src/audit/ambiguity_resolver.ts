import {
    ALL_LOCALIZATIONS,
    Localization,
    LookupResult,
    PageType,
} from "@ghoulean/ruina-common";
import { QueryMapperLookupTable } from "../localization_mappers/query_mapper";
import { CARD_LOCALIZATIONS } from "../util/localize_constants";
import { Util } from "../util/util";
import { BookDrops } from "./book_drops";

export type AmbiguousResults = {
    id: string;
    locale: Localization;
    query: string;
    lookupResults: LookupResult[];
};

export class AmbiguityResolver {
    /**
     * Detect ambiguity in the generated query mapper lookup table.
     * Ambiguity occurs when a query maps to several pages of the same locale
     */
    public static detect(
        queryMapperLookupTable: QueryMapperLookupTable
    ): AmbiguousResults[] {
        const retVal: AmbiguousResults[] = [];

        for (const query in queryMapperLookupTable) {
            const lookupResults: LookupResult[] = queryMapperLookupTable[query];
            for (const locale of ALL_LOCALIZATIONS) {
                const filteredLookupResults: LookupResult[] =
                    this.getLookupResultByLocale(lookupResults, locale);
                if (filteredLookupResults.length > 1) {
                    retVal.push({
                        id: this.generateDisambiguationPageId(),
                        locale: locale,
                        query: query,
                        lookupResults: filteredLookupResults,
                    });
                }
            }
        }

        return retVal;
    }

    /**
     * Modifies both queryMapperLookupTable and ambiguousResults in-place to add disambiguation entries.
     * Assume queryMapperLookupTable[ambiguousResults.query] does not yet contain a disambiguation entry
     * Done in two-step process to expose AmbiguousResults.
     *
     * Disambigution strategy:
     * 1. Check if some cards can be differentiated by card type.
     * 2. Check if card can be differentiated by collectability status.
     * 2a. [TODO] Check if card can be differentiated by non-collectability status. e.g. "Enemy page"
     * 3. Give up and append some arbitrary ID
     */
    public static disambiguate(
        queryMapperLookupTable: QueryMapperLookupTable,
        ambiguousResults: AmbiguousResults
    ) {
        for (const disambiguatedLookupResult of ambiguousResults.lookupResults) {
            // Check by page type
            const lookupResultsByPageType: LookupResult[] =
                this.getLookupResultByPageType(
                    ambiguousResults.lookupResults,
                    disambiguatedLookupResult.pageType
                );
            if (lookupResultsByPageType.length == 1) {
                console.log(
                    `Disambiguating "${ambiguousResults.query}" in locale ${ambiguousResults.locale} using pageType ${disambiguatedLookupResult.pageType}`
                );

                // TODO: Move disambiguating to helper
                const disambiguator: string =
                    CARD_LOCALIZATIONS[disambiguatedLookupResult.pageType][
                        disambiguatedLookupResult.locale
                    ];
                const disambiguatedName: string = this.appendDisambiguatorStr(
                    disambiguatedLookupResult.query,
                    disambiguator
                );
                this.appendDisambiguator(
                    disambiguatedLookupResult,
                    disambiguator
                );
                queryMapperLookupTable[disambiguatedName] = [
                    disambiguatedLookupResult,
                ];
                continue;
            }

            // Check if disambiguation by collectability
            const disambiguatedPageType: PageType =
                disambiguatedLookupResult.pageType;
            let collectableIds: string[];
            if (disambiguatedPageType == PageType.COMBAT_PAGE) {
                collectableIds = BookDrops.getAllCollectableCombatPages();
            } else if (disambiguatedPageType == PageType.KEY_PAGE) {
                collectableIds = BookDrops.getAllCollectableKeyPages();
            } else {
                collectableIds = [];
            }
            if (collectableIds.indexOf(disambiguatedLookupResult.pageId) >= 0) {
                console.log(
                    `Disambiguating "${ambiguousResults.query}" with pageType ${disambiguatedLookupResult.pageType} by collectability status`
                );
                // TODO: Move disambiguating to helper
                // TODO: localize
                const disambiguator: string = "collectable";
                const disambiguatedName: string = this.appendDisambiguatorStr(
                    disambiguatedLookupResult.query,
                    disambiguator
                );
                this.appendDisambiguator(
                    disambiguatedLookupResult,
                    disambiguator
                );
                queryMapperLookupTable[disambiguatedName] = [
                    disambiguatedLookupResult,
                ];
                continue;
            }

            // Ran out of disambiguation strategies
            console.log(
                `Ran out of options; appending arbitrary identifier to ${ambiguousResults.query} with pageId=${disambiguatedLookupResult.pageId}`
            );

            // TODO: Move disambiguating to helper
            const disambiguator: string = disambiguatedLookupResult.pageId;
            const randomDisambiguatedName: string = this.appendDisambiguatorStr(
                disambiguatedLookupResult.query,
                disambiguator
            );
            this.appendDisambiguator(disambiguatedLookupResult, disambiguator);
            disambiguatedLookupResult.query = randomDisambiguatedName;
            queryMapperLookupTable[randomDisambiguatedName] = [
                disambiguatedLookupResult,
            ];
        }

        // Clear entry
        queryMapperLookupTable[ambiguousResults.query] = [];
    }

    private static appendDisambiguatorStr(
        query: string,
        disambiguator: string
    ): string {
        return `${query} (${disambiguator})`;
    }

    private static appendDisambiguator(
        lookupResult: LookupResult,
        disambiguator: string
    ) {
        lookupResult.query = this.appendDisambiguatorStr(
            lookupResult.query,
            disambiguator
        );
        if (lookupResult.displayQuery) {
            lookupResult.displayQuery = this.appendDisambiguatorStr(
                lookupResult.displayQuery,
                disambiguator
            );
        }
    }

    private static getLookupResultByPageType(
        lookupResults: LookupResult[],
        pageType: PageType
    ): LookupResult[] {
        return lookupResults.filter((result: LookupResult) => {
            return result.pageType == pageType;
        });
    }

    private static removeLookupResult(
        lookupResults: LookupResult[],
        removeThis: LookupResult
    ): LookupResult[] {
        return lookupResults.filter((result: LookupResult) => {
            return !Util.areEqualShallow(result, removeThis);
        });
    }

    private static getLookupResultByLocale(
        lookupResults: LookupResult[],
        locale: Localization
    ): LookupResult[] {
        return lookupResults.filter((result) => {
            return result.locale == locale;
        });
    }

    private static generateDisambiguationPageId(): string {
        return (
            Math.random().toString(16).slice(2) +
            Math.random().toString(16).slice(2)
        );
    }
}
