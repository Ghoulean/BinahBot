import { ALL_LOCALIZATIONS, DecoratedAbnoPage, Localization, LookupResult, PageType } from "@ghoulean/ruina-common";

export type QueryMapperPropType = {
    locale: Localization,
    abnoPages?: {
        [key: string]: DecoratedAbnoPage
    }
}

/**
 * Generates maps from localized search queries to (localization, page type, page id) lookup results
 * Localized names may include disambiguators
 */
export class QueryMapper {
    public static map(props: QueryMapperPropType): { [key: string]: LookupResult } {
        const retVal: { [key: string]: LookupResult } = {};

        // Abno
        if (!props.abnoPages) {
            throw new Error(`Abno pages not found for locale:${props.locale}`);
        }
        for (const abnoPageId of Object.keys(props.abnoPages!)) {
            const abnoPage: DecoratedAbnoPage = props.abnoPages![abnoPageId];
            // Use this helper method to inject disambiguators of different types
            const abnoPageName: string = this.transformName(abnoPage.name);
            if (retVal[abnoPageName]) {
                const otherId: string = retVal[abnoPageName]!.pageId;
                console.log(Object.keys(props.abnoPages!));
                console.log(retVal[abnoPageName]!);
                throw new Error(`Overlapping name ${abnoPageName} discovered for id:${abnoPageId} and id:${otherId}`);
            } else {
                retVal[abnoPageName] = {
                    query: abnoPageName,
                    chapter: abnoPage.chapter,
                    locale: props.locale,
                    pageType: PageType.ABNO_PAGE,
                    pageId: abnoPageId
                }
            }
        }
        
        // TODO: Combat pages
        // TODO: Key pages
        // TODO: Passives

        return retVal;
    }

    private static transformName(displayName: string): string {
        return displayName;
    }
}