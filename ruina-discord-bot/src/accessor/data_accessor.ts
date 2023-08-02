import {
    DecoratedAbnoPage,
    Localization,
    LookupResult,
} from "@ghoulean/ruina-common";
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

export class DataAccessor {
    constructor() {}

    public lookup(query: string, preferredLocale: Localization): LookupResult {
        const lookupResults: LookupResult[] | undefined = LOOKUP_RESULTS[query];
        if (!lookupResults) {
            throw new Error(`Couldn't identify query result for ${query}.`);
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
            if (this.cleanQuery(q).startsWith(cleanQuery)) {
                retVal.push(q);
            }
        }
        return retVal.sort((a, b) => {
            return a.length - b.length || a.localeCompare(b);
        });
    }

    private getLocaledAbnoMapping(
        locale: Localization
    ): QueryToDecoratedAbnopage {
        return LOCALIZATION_TO_DECORATED_ABNO_PAGE[locale];
    }

    private cleanQuery(s: string): string {
        return s.trim().toLowerCase();
    }
}
