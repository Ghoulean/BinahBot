import {
    ALL_LOCALIZATIONS,
    Localization,
    LookupResult,
    PageType,
} from "@ghoulean/ruina-common";
import { QueryMapperLookupTable } from "../localization_mappers/query_mapper";
import { CARD_LOCALIZATIONS } from "../util/localize_constants";
import { Util } from "../util/util";

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
     * 1. Check if some cards can be differentiated by card type. If yes, do so
     * 2. Give up and append some arbitrary ID
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
                const disambiguatedName: string = this.appendDisambiguator(
                    disambiguatedLookupResult.query,
                    CARD_LOCALIZATIONS[disambiguatedLookupResult.pageType][
                        disambiguatedLookupResult.locale
                    ]
                );
                disambiguatedLookupResult.query = disambiguatedName;
                queryMapperLookupTable[disambiguatedName] = [
                    disambiguatedLookupResult,
                ];
                continue;
            }

            // Ran out of disambiguation strategies
            console.log(
                `Ran out of options; appending arbitrary identifier to ${ambiguousResults.query} with pageId=${disambiguatedLookupResult.pageId}`
            );
            const randomDisambiguatedName: string = this.appendDisambiguator(
                disambiguatedLookupResult.query,
                disambiguatedLookupResult.pageId
            );
            disambiguatedLookupResult.query = randomDisambiguatedName;
            queryMapperLookupTable[randomDisambiguatedName] = [
                disambiguatedLookupResult,
            ];
        }

        // Clear entry
        queryMapperLookupTable[ambiguousResults.query] = [];
    }

    private static appendDisambiguator(
        query: string,
        disambiguator: string
    ): string {
        return `${query} (${disambiguator})`;
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
