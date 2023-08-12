import { Localization, LookupResult } from "@ghoulean/ruina-common";

// TODO: This model is duplicated between the reparser and this package.
// Reconsider where it should be placed
export type DisambiguationResults = {
    id: string;
    locale: Localization;
    query: string;
    lookupResults: LookupResult[];
};