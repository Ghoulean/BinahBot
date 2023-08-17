import {
    ALL_LOCALIZATIONS,
    Chapter,
    DecoratedAbnoPage,
    DecoratedCombatPage,
    DecoratedKeyPage,
    DecoratedPassive,
    Localization,
    LookupResult,
    PageType,
} from "@ghoulean/ruina-common";
import {
    AmbiguityResolver,
    AmbiguousResults,
} from "../audit/ambiguity_resolver";
import { Util } from "../util/util";

type QueryMapperLocaleResults = {
    abnoPages?: {
        [key: string]: DecoratedAbnoPage;
    };
    combatPages?: {
        [key: string]: DecoratedCombatPage;
    };
    passives?: {
        [key: string]: DecoratedPassive;
    };
    keyPages?: {
        [key: string]: DecoratedKeyPage;
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

            // Abno
            for (const abnoPageId in localedResults.abnoPages) {
                const abnoPage: DecoratedAbnoPage =
                    localedResults.abnoPages[abnoPageId];
                const query: string = Util.cleanString(abnoPage.name);
                const abnoLookupResult: LookupResult =
                    this.constructAbnoLookupResult(abnoPage);
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
                const query: string = Util.cleanString(combatPage.name);
                const combatLookupResult: LookupResult =
                    this.constructCombatLookupResult(combatPage);
                if (retVal[query]) {
                    retVal[query].push(combatLookupResult);
                } else {
                    retVal[query] = [combatLookupResult];
                }
            }

            // Key pages
            for (const keyPageId in localedResults.keyPages) {
                const keyPage: DecoratedKeyPage =
                    localedResults.keyPages[keyPageId];
                const query: string = Util.cleanString(keyPage.name);
                const keypageLookupResult: LookupResult =
                    this.constructKeyPageLookupResult(keyPage);
                if (retVal[query]) {
                    retVal[query].push(keypageLookupResult);
                } else {
                    retVal[query] = [keypageLookupResult];
                }
            }

            // Passives
            for (const passiveId in localedResults.passives) {
                const passive: DecoratedPassive =
                    localedResults.passives[passiveId];
                const query: string = Util.cleanString(passive.name);
                const passiveLookupResult: LookupResult =
                    this.constructPassiveLookupResult(passive);
                if (retVal[query]) {
                    retVal[query].push(passiveLookupResult);
                } else {
                    retVal[query] = [passiveLookupResult];
                }
            }
        }

        return retVal;
    }

    // This OOP is getting really murky now
    // Modifies queryMapperLookupTable and ambiguousResults in-place
    public static remap(
        queryMapperLookupTable: QueryMapperLookupTable,
        ambiguousResults: AmbiguousResults[]
    ) {
        for (const result of ambiguousResults) {
            const query: string = result.query;
            const disambiguateLookupResult: LookupResult =
                this.constructDisambiguateLookupResult(result);
            AmbiguityResolver.disambiguate(queryMapperLookupTable, result);
            queryMapperLookupTable[query].push(disambiguateLookupResult);
        }
    }

    private static constructAbnoLookupResult(
        decoratedAbnoPage: DecoratedAbnoPage
    ): LookupResult {
        return {
            ...this.resolveDisplayQuery(decoratedAbnoPage.name),
            chapter: decoratedAbnoPage.chapter,
            locale: decoratedAbnoPage.locale,
            pageType: PageType.ABNO_PAGE,
            pageId: decoratedAbnoPage.nameId,
        };
    }

    private static constructCombatLookupResult(
        decoratedCombatPage: DecoratedCombatPage
    ): LookupResult {
        return {
            ...this.resolveDisplayQuery(decoratedCombatPage.name),
            chapter: decoratedCombatPage.chapter,
            locale: decoratedCombatPage.locale,
            pageType: PageType.COMBAT_PAGE,
            pageId: decoratedCombatPage.id,
        };
    }

    private static constructKeyPageLookupResult(
        decoratedKeyPage: DecoratedKeyPage
    ): LookupResult {
        return {
            ...this.resolveDisplayQuery(decoratedKeyPage.name),
            chapter: decoratedKeyPage.chapter,
            locale: decoratedKeyPage.locale,
            pageType: PageType.KEY_PAGE,
            pageId: decoratedKeyPage.id,
        };
    }

    private static constructPassiveLookupResult(
        decoratedPassive: DecoratedPassive
    ): LookupResult {
        return {
            ...this.resolveDisplayQuery(decoratedPassive.name),
            chapter: Chapter.CANARD, // TODO: possibly calculate this based on keypage chapters?
            locale: decoratedPassive.locale,
            pageType: PageType.PASSIVE,
            pageId: decoratedPassive.id,
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

    private static resolveDisplayQuery(name: string): {
        query: string;
        displayQuery?: string;
    } {
        const cleanedQuery: string = Util.cleanString(name);
        return {
            query: cleanedQuery,
            displayQuery: cleanedQuery == name ? undefined : name,
        };
    }
}
