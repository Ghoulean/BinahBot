import {
    DecoratedAbnoPage,
    Localization,
    LookupResult,
} from "@ghoulean/ruina-common";
import * as fastestLevenshtein from "fastest-levenshtein";
import * as __AUTOCOMPLETE from "../data/autocomplete.json";
import * as __CN_ABNO from "../data/cn/abno.json";
import * as __EN_ABNO from "../data/en/abno.json";
import * as __JP_ABNO from "../data/jp/abno.json";
import * as __KR_ABNO from "../data/kr/abno.json";
import * as __LOOKUP_RESULTS from "../data/queryLookupResults.json";
import * as __TRCN_ABNO from "../data/trcn/abno.json";

type QueryToLookupResult = {
    [key: string]: LookupResult[];
};

type QueryToDecoratedAbnopage = {
    [key: string]: DecoratedAbnoPage;
};

type LocalizationToQueryDecoratedAbnopage = {
    [key in Localization]: QueryToDecoratedAbnopage;
};

type LevenshteinResults = {
    best: string;
    score: number;
};

const LOOKUP_RESULTS: QueryToLookupResult =
    __LOOKUP_RESULTS as QueryToLookupResult;

const CN_ABNO: QueryToDecoratedAbnopage = __CN_ABNO as QueryToDecoratedAbnopage;
const EN_ABNO: QueryToDecoratedAbnopage = __EN_ABNO as QueryToDecoratedAbnopage;
const JP_ABNO: QueryToDecoratedAbnopage = __JP_ABNO as QueryToDecoratedAbnopage;
const KR_ABNO: QueryToDecoratedAbnopage = __KR_ABNO as QueryToDecoratedAbnopage;
const TRCN_ABNO: QueryToDecoratedAbnopage =
    __TRCN_ABNO as QueryToDecoratedAbnopage;

const LOCALIZATION_TO_DECORATED_ABNO_PAGE: LocalizationToQueryDecoratedAbnopage =
    {
        [Localization.CHINESE_SIMPLIFIED]: CN_ABNO,
        [Localization.CHINESE_TRADITIONAL]: TRCN_ABNO,
        [Localization.ENGLISH]: EN_ABNO,
        [Localization.JAPANESE]: JP_ABNO,
        [Localization.KOREAN]: KR_ABNO,
    };

const AUTOCOMPLETE: string[] = __AUTOCOMPLETE.data;

const FUZZY_MATCHING_DISTANCE = 2;

export class DataAccessor {
    constructor() {}

    public lookup(query: string, preferredLocale: Localization): LookupResult {
        let lookupResults: LookupResult[] | undefined = LOOKUP_RESULTS[query];
        if (!lookupResults) {
            const levenshteinResults: LevenshteinResults =
                this.levenshteinLookup(query);
            if (levenshteinResults.score > FUZZY_MATCHING_DISTANCE) {
                throw new Error(`Couldn't identify query result for ${query}.`);
            }
            lookupResults = LOOKUP_RESULTS[levenshteinResults.best];
        }
        for (const lookupResult of lookupResults) {
            if (lookupResult.locale == preferredLocale) {
                return lookupResult;
            }
        }
        return lookupResults[0];
    }

    public getDecoratedAbnoPage(
        pageId: string,
        locale: Localization
    ): DecoratedAbnoPage {
        const decoratedAbnoPage: DecoratedAbnoPage | undefined =
            this.getLocaledAbnoMapping(locale)[pageId];
        if (!decoratedAbnoPage) {
            throw new Error(`${pageId} not found in ${locale} locale`);
        }
        return decoratedAbnoPage;
    }

    public autocomplete(query: string): string[] {
        const retVal: string[] = [];
        const cleanQuery = this.cleanQuery(query);
        for (const q of AUTOCOMPLETE) {
            if (
                this.autocompleteDistance(cleanQuery, q) <=
                FUZZY_MATCHING_DISTANCE
            ) {
                retVal.push(q);
            }
        }
        return retVal.sort((a, b) => {
            return (
                this.autocompleteDistance(cleanQuery, a) -
                    this.autocompleteDistance(cleanQuery, b) ||
                a.length - b.length ||
                a.localeCompare(b)
            );
        });
    }

    private getLocaledAbnoMapping(
        locale: Localization
    ): QueryToDecoratedAbnopage {
        return LOCALIZATION_TO_DECORATED_ABNO_PAGE[locale];
    }

    private cleanQuery(s: string): string {
        return s.replace(/\s/g, "").toLowerCase();
    }

    private autocompleteDistance(query: string, lookup: string): number {
        return fastestLevenshtein.distance(
            query,
            this.cleanQuery(lookup).substring(0, query.length)
        );
    }

    private levenshteinLookup(s: string): LevenshteinResults {
        const closest: string = fastestLevenshtein.closest(s, AUTOCOMPLETE);
        return {
            best: closest,
            score: fastestLevenshtein.distance(
                this.cleanQuery(s),
                this.cleanQuery(closest)
            ),
        };
    }
}
