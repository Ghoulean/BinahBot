import { Chapter, Localization, PageType } from "../enums";

export interface LookupResult {
    query: string;
    chapter: Chapter;
    locale: Localization;
    pageType: PageType;
    pageId: string;
}
