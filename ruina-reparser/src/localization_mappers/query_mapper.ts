import {
    ALL_LOCALIZATIONS,
    Chapter,
    DecoratedAbnoPage,
    Localization,
    LookupResult,
    PageType,
} from "@ghoulean/ruina-common";

type QueryMapperLocaleResults = {
    abnoPages?: {
        [key: string]: DecoratedAbnoPage;
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
            if (!localedResults.abnoPages) {
                throw new Error(`Abno pages not found for locale:${locale}`);
            }
            for (const abnoPageId in localedResults.abnoPages) {
                const abnoPage: DecoratedAbnoPage =
                    localedResults.abnoPages[abnoPageId];
                const query: string = this.disambiguate(abnoPage.name, abnoPage.locale, PageType.ABNO_PAGE);
                const abnoLookupResult: LookupResult =
                    this.constructAbnoLookupResult(abnoPage, query);
                if (retVal[query]) {
                    retVal[query].push(abnoLookupResult);
                } else {
                    retVal[query] = [abnoLookupResult];
                }
            }

            // TODO: Combat pages
            // TODO: Key pages
            // TODO: Passives
        }

        // overlap detection
        for (const query in retVal) {
            const lookupResults: LookupResult[] = retVal[query]
            const pageTypes: PageType[] = this.getPageTypes(lookupResults);
            if (pageTypes.length != 1) {
                console.error(`Query "${query}" has mixed page types: ${pageTypes.map((p) => { return PageType[p]; })}`);
            }
        }

        return retVal;
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

    private static getPageTypes(lookupResults: LookupResult[]): PageType[] {
        const set: {[key in PageType]?: 1} = {};
        for (const lookupResult of lookupResults) {
            set[lookupResult.pageType] = 1;
        }
        return Object.keys(set).map((str) => { return Number(str) }) as PageType[];
    }

    private static disambiguate(query: string, _locale: Localization, _pageType: PageType): string {
        return query;
    }
}
