import {
    Chapter,
    Localization,
    LookupResult,
    PageType,
} from "@ghoulean/ruina-common";
import { DisambiguationResults } from "../../src/model/disambiguation_result";

export const ABNO_LOOKUP_RESULT: LookupResult = {
    query: "query",
    chapter: Chapter.LIBRARY_OF_RUINA,
    locale: Localization.ENGLISH,
    pageType: PageType.ABNO_PAGE,
    pageId: "pageId",
};

export const COMBAT_LOOKUP_RESULT: LookupResult = {
    ...ABNO_LOOKUP_RESULT,
    pageType: PageType.COMBAT_PAGE,
};

export const KEYPAGE_LOOKUP_RESULT: LookupResult = {
    ...ABNO_LOOKUP_RESULT,
    pageType: PageType.KEY_PAGE,
};

export const PASSIVE_LOOKUP_RESULT: LookupResult = {
    ...ABNO_LOOKUP_RESULT,
    pageType: PageType.PASSIVE,
};

export const DISAMBIGUATION_LOOKUP_RESULT: LookupResult = {
    ...ABNO_LOOKUP_RESULT,
    pageType: PageType.DISAMBIGUATION,
};

export const DISAMBIGUATION_RESULTS: DisambiguationResults = {
    id: "disambiguation_id",
    locale: Localization.ENGLISH,
    query: "disambiguation",
    lookupResults: [ABNO_LOOKUP_RESULT, COMBAT_LOOKUP_RESULT],
};
