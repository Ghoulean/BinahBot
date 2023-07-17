import {
    DecoratedAbnoPage,
    Localization,
    LookupResult,
} from "@ghoulean/ruina-common";
import * as __CN_ABNO from "../data/cn/abno.json";
import * as __CN_LOOKUP_RESULTS from "../data/cn/queryLookupResults.json";
import * as __EN_ABNO from "../data/en/abno.json";
import * as __EN_LOOKUP_RESULTS from "../data/en/queryLookupResults.json";
import * as __JP_ABNO from "../data/jp/abno.json";
import * as __JP_LOOKUP_RESULTS from "../data/jp/queryLookupResults.json";
import * as __KR_ABNO from "../data/kr/abno.json";
import * as __KR_LOOKUP_RESULTS from "../data/kr/queryLookupResults.json";
import * as __TRCN_ABNO from "../data/trcn/abno.json";
import * as __TRCN_LOOKUP_RESULTS from "../data/trcn/queryLookupResults.json";

type QueryToLookupResult = {
    [key: string]: LookupResult;
};

type LocalizationToQueryLookupResult = {
    [key in Localization]: QueryToLookupResult;
};

type QueryToDecoratedAbnopage = {
    [key: string]: DecoratedAbnoPage;
};

type LocalizationToQueryDecoratedAbnopage = {
    [key in Localization]: QueryToDecoratedAbnopage;
};

const CN_LOOKUP_RESULTS: QueryToLookupResult =
    __CN_LOOKUP_RESULTS as QueryToLookupResult;
const EN_LOOKUP_RESULTS: QueryToLookupResult =
    __EN_LOOKUP_RESULTS as QueryToLookupResult;
const JP_LOOKUP_RESULTS: QueryToLookupResult =
    __JP_LOOKUP_RESULTS as QueryToLookupResult;
const KR_LOOKUP_RESULTS: QueryToLookupResult =
    __KR_LOOKUP_RESULTS as QueryToLookupResult;
const TRCN_LOOKUP_RESULTS: QueryToLookupResult =
    __TRCN_LOOKUP_RESULTS as QueryToLookupResult;

const LOCALIZATION_TO_LOOKUP_RESULTS: LocalizationToQueryLookupResult = {
    [Localization.CHINESE_SIMPLIFIED]: CN_LOOKUP_RESULTS,
    [Localization.CHINESE_TRADITIONAL]: TRCN_LOOKUP_RESULTS,
    [Localization.ENGLISH]: EN_LOOKUP_RESULTS,
    [Localization.JAPANESE]: JP_LOOKUP_RESULTS,
    [Localization.KOREAN]: KR_LOOKUP_RESULTS,
};

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

export class DataAccessor {
    constructor() {}

    public lookup(query: string, locale: Localization): LookupResult {
        const lookupResults: { [key: string]: LookupResult } =
            this.getLocaledLookupResults(locale);
        const lookupResult: LookupResult = lookupResults[query];
        if (!lookupResult) {
            throw new Error(
                `Couldn't identify query result for ${query} in ${locale} locale`
            );
        }
        return lookupResult;
    }

    public getDecoratedAbnoPage(
        pageId: string,
        locale: Localization
    ): DecoratedAbnoPage {
        const decoratedAbnoPage: DecoratedAbnoPage =
            this.getLocaledAbnoMapping(locale)[pageId];
        if (!decoratedAbnoPage) {
            throw new Error(`${pageId} not found in ${locale} locale`);
        }
        return decoratedAbnoPage;
    }

    private getLocaledLookupResults(locale: Localization): QueryToLookupResult {
        return (
            LOCALIZATION_TO_LOOKUP_RESULTS[locale] ??
            LOCALIZATION_TO_LOOKUP_RESULTS[Localization.ENGLISH]
        );
    }

    private getLocaledAbnoMapping(
        locale: Localization
    ): QueryToDecoratedAbnopage {
        return (
            LOCALIZATION_TO_DECORATED_ABNO_PAGE[locale] ??
            LOCALIZATION_TO_DECORATED_ABNO_PAGE[Localization.ENGLISH]
        );
    }
}
