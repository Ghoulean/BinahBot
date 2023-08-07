import {
    ALL_LOCALIZATIONS,
    AbnoPage,
    CombatPage,
    DecoratedAbnoPage,
    DecoratedCombatPage,
    Localization,
} from "@ghoulean/ruina-common";
import { AbnoPageMapper } from "./localization_mappers/abno_page_mapper";
import { CombatPageMapper } from "./localization_mappers/combat_page_mapper";
import {
    QueryMapper,
    QueryMapperLookupTable,
    QueryMapperPropType,
} from "./localization_mappers/query_mapper";
import { AbnoPageProcessor } from "./processors/abnopage_processor";
import { CombatPageProcessor } from "./processors/combatpage_processor";
import { fileSanityCheck, setupJsonFiles, writeDataFile } from "./util/file";
import { AmbiguousResults, AmbiguityResolver } from "./audit/ambiguity_resolver";

fileSanityCheck();
setupJsonFiles();

const queryMapperProps: QueryMapperPropType = {
    [Localization.KOREAN]: {},
    [Localization.JAPANESE]: {},
    [Localization.ENGLISH]: {},
    [Localization.CHINESE_SIMPLIFIED]: {},
    [Localization.CHINESE_TRADITIONAL]: {},
};

const abnoPages: AbnoPage[] = AbnoPageProcessor.process();
const combatPages: CombatPage[] = CombatPageProcessor.process();

for (const locale of ALL_LOCALIZATIONS) {
    // Abno
    const decoratedAbnoPages: { [key: string]: DecoratedAbnoPage } =
        AbnoPageMapper.map(abnoPages, locale);
    queryMapperProps[locale].abnoPages = decoratedAbnoPages;
    writeDataFile(`${locale}/abno.json`, decoratedAbnoPages);

    // TODO: Combat pages
    const decoratedCombatPages: { [key: string]: DecoratedCombatPage } =
        CombatPageMapper.map(combatPages, locale);
    queryMapperProps[locale].combatPages = decoratedCombatPages;
    writeDataFile(`${locale}/combat.json`, decoratedCombatPages);

    // TODO: Key pages
    // TODO: Passives
}

// Query mapper
const lookupResults: QueryMapperLookupTable = QueryMapper.map(queryMapperProps);

// Disambiguate
const ambiguousResults: AmbiguousResults[] = AmbiguityResolver.detect(lookupResults);
writeDataFile(`ambiguousResults.json`, ambiguousResults);

// Remap with disambiguator
const disambiguatedLookupResults: QueryMapperLookupTable = QueryMapper.remap(lookupResults, ambiguousResults);

// Write query map
writeDataFile(`queryLookupResults.json`, disambiguatedLookupResults);

// Autocomplete
writeDataFile(`autocomplete.json`, {
    data: Object.keys(disambiguatedLookupResults),
});

console.log("Done");
