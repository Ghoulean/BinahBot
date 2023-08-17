import { Chapter, Localization, PageType } from "../enums";

export interface LookupResult {
    // What one should type to get this
    query: string;
    // What one sees when this is returned, if it
    // differs from query. e.g. diacritics
    displayQuery?: string;
    chapter: Chapter;
    locale: Localization;
    pageType: PageType;
    pageId: string;
}
