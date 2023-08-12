import {
    ALL_LOCALIZATIONS,
    Localization,
    LookupResult,
    PageType,
    QUERYABLE_PAGE_TYPES,
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
                        lookupResults: lookupResults,
                    });
                }
            }
        }

        return retVal;
    }

    /**
     * Modify query mapper lookup table in-place to add disambiguation entries.
     * Done in two-step process to expose AmbiguousResults.
     *
     * Disambigution strategy:
     * 1. Check if some cards can be differentiated by card type. If yes, do so
     * 2. Give up and append some arbitrary ID
     */
    public static disambiguate(
        queryMapperLookupTable: QueryMapperLookupTable,
        ambiguousResults: AmbiguousResults
    ): QueryMapperLookupTable {
        for (const pageType of QUERYABLE_PAGE_TYPES) {
            const lookupResultsByPageType: LookupResult[] =
                this.getLookupResultByPageType(
                    ambiguousResults.lookupResults,
                    pageType
                );

            if (lookupResultsByPageType.length == 0) {
                continue;
            }
            if (lookupResultsByPageType.length > 1) {
                console.log(
                    `Can't disambiguate "${ambiguousResults.query}" in locale ${ambiguousResults.locale} with pageType ${pageType}`
                );
                continue;
            }
            const disambiguatedLookupResult: LookupResult =
                lookupResultsByPageType[0];
            const disambiguatedName: string = this.appendDisambiguator(
                disambiguatedLookupResult.query,
                CARD_LOCALIZATIONS[disambiguatedLookupResult.pageType][
                    disambiguatedLookupResult.locale
                ]
            );
            this.disambiguateEntry(
                queryMapperLookupTable,
                ambiguousResults,
                disambiguatedLookupResult,
                disambiguatedName
            );
        }
        while (ambiguousResults.lookupResults.length > 0) {
            const disambiguatedLookupResult: LookupResult =
                ambiguousResults.lookupResults[
                    ambiguousResults.lookupResults.length - 1
                ];
            console.log(
                `Appending arbitrary identifier to ${ambiguousResults.query} with pageId=${disambiguatedLookupResult.pageId}`
            );
            const randomDisambiguatedName: string = this.appendDisambiguator(
                disambiguatedLookupResult.query,
                disambiguatedLookupResult.pageId
            );
            this.disambiguateEntry(
                queryMapperLookupTable,
                ambiguousResults,
                disambiguatedLookupResult,
                randomDisambiguatedName
            );
        }

        // Map original query to only disambiguation entry
        queryMapperLookupTable[ambiguousResults.query] = queryMapperLookupTable[
            ambiguousResults.query
        ].filter((lookupResult) => {
            return lookupResult.pageType == PageType.DISAMBIGUATION;
        });

        return queryMapperLookupTable;
    }

    // In-place modification
    private static disambiguateEntry(
        queryMapperLookupTable: QueryMapperLookupTable,
        ambiguousResults: AmbiguousResults,
        disambiguatedLookupResult: LookupResult,
        disambiguatedName: string
    ) {
        ambiguousResults.lookupResults = this.removeLookupResult(
            ambiguousResults.lookupResults,
            disambiguatedLookupResult
        );
        queryMapperLookupTable[disambiguatedName] = [
            disambiguatedLookupResult,
        ];
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
