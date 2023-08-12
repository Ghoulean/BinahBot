import {
    ALL_LOCALIZATIONS,
    Chapter,
    DecoratedAbnoPage,
    DecoratedCombatPage,
    Localization,
    LookupResult,
    PageType,
} from "@ghoulean/ruina-common";
import {
    AmbiguityResolver,
    AmbiguousResults,
} from "../audit/ambiguity_resolver";

type QueryMapperLocaleResults = {
    abnoPages?: {
        [key: string]: DecoratedAbnoPage;
    };
    combatPages?: {
        [key: string]: DecoratedCombatPage;
    };
};

export type QueryMapperPropType = {
    [key in Localization]: QueryMapperLocaleResults;
};

export type QueryMapperLookupTable = {
    [key: string]: LookupResult[];
};

/**
 * Generates maps from search queries to (localization, page type, page id) lookup results
 */
export class QueryMapper {
    public static map(props: QueryMapperPropType): QueryMapperLookupTable {
        const retVal: QueryMapperLookupTable = {};

        for (const locale of ALL_LOCALIZATIONS) {
            const localedResults: QueryMapperLocaleResults = props[locale];
            if (!localedResults.abnoPages) {
                throw new Error(`Abno pages not found for locale:${locale}`);
            }
            if (!localedResults.combatPages) {
                throw new Error(`Combat pages not found for locale:${locale}`);
            }

            // Abno
            for (const abnoPageId in localedResults.abnoPages) {
                const abnoPage: DecoratedAbnoPage =
                    localedResults.abnoPages[abnoPageId];
                const query: string = abnoPage.name;
                const abnoLookupResult: LookupResult =
                    this.constructAbnoLookupResult(abnoPage, query);
                if (retVal[query]) {
                    retVal[query].push(abnoLookupResult);
                } else {
                    retVal[query] = [abnoLookupResult];
                }
            }

            // Combat pages
            for (const combatPageId in localedResults.combatPages) {
                const combatPage: DecoratedCombatPage =
                    localedResults.combatPages[combatPageId];
                const query: string = combatPage.name;
                const combatLookupResult: LookupResult =
                    this.constructCombatLookupResult(combatPage, query);
                if (retVal[query]) {
                    retVal[query].push(combatLookupResult);
                } else {
                    retVal[query] = [combatLookupResult];
                }
            }

            // TODO: Key pages
            // TODO: Passives
        }

        return retVal;
    }

    // This OOP is getting really murky now
    public static remap(
        queryMapperLookupTable: QueryMapperLookupTable,
        ambiguousResults: AmbiguousResults[]
    ): QueryMapperLookupTable {
        for (const result of ambiguousResults) {
            const query: string = result.query;
            const disambiguateLookupResult: LookupResult =
                this.constructDisambiguateLookupResult(result);
            queryMapperLookupTable = AmbiguityResolver.disambiguate(
                queryMapperLookupTable,
                result
            );
            queryMapperLookupTable[query].push(disambiguateLookupResult);
        }

        return queryMapperLookupTable;
    }

    private static constructAbnoLookupResult(
        decoratedAbnoPage: DecoratedAbnoPage,
        query: string
    ): LookupResult {
        return {
            query: query,
            chapter: decoratedAbnoPage.chapter,
            locale: decoratedAbnoPage.locale,
            pageType: PageType.ABNO_PAGE,
            pageId: decoratedAbnoPage.nameId,
        };
    }

    private static constructCombatLookupResult(
        decoratedCombatPage: DecoratedCombatPage,
        query: string
    ): LookupResult {
        return {
            query: query,
            chapter: decoratedCombatPage.chapter,
            locale: decoratedCombatPage.locale,
            pageType: PageType.COMBAT_PAGE,
            pageId: decoratedCombatPage.id,
        };
    }

    private static constructDisambiguateLookupResult(
        results: AmbiguousResults
    ) {
        // Get the second-soonest chapter in results, if nonnull
        const sortedChapters: Chapter[] = [
            ...new Set(
                results.lookupResults.map((lr: LookupResult) => {
                    return lr.chapter;
                })
            ),
        ].sort((a: number, b: number) => {
            return a - b;
        });
        const chapter: Chapter =
            (sortedChapters.length >= 2 ? sortedChapters[1] : null) ??
            sortedChapters[0] ??
            Chapter.LIBRARY_OF_RUINA;
        return {
            query: results.query,
            chapter: chapter,
            locale: results.locale,
            pageType: PageType.DISAMBIGUATION,
            pageId: results.id,
        };
    }
}
