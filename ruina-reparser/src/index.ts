import {
    ALL_LOCALIZATIONS,
    AbnoPage,
    CombatPage,
    DecoratedAbnoPage,
    DecoratedCombatPage,
    DecoratedKeyPage,
    DecoratedPassive,
    KeyPage,
    Localization,
    Passive,
} from "@ghoulean/ruina-common";
import {
    AmbiguityResolver,
    AmbiguousResults,
} from "./audit/ambiguity_resolver";
import { AbnoPageMapper } from "./localization_mappers/abno_page_mapper";
import { CombatPageMapper } from "./localization_mappers/combat_page_mapper";
import { KeyPageMapper } from "./localization_mappers/keypage_mapper";
import { PassiveMapper } from "./localization_mappers/passive_mapper";
import {
    QueryMapper,
    QueryMapperLookupTable,
    QueryMapperPropType,
} from "./localization_mappers/query_mapper";
import { AbnoPageProcessor } from "./processors/abnopage_processor";
import { CombatPageProcessor } from "./processors/combatpage_processor";
import { KeyPageProcessor } from "./processors/keypage_processor";
import { PassiveProcessor } from "./processors/passive_processor";
import { fileSanityCheck, setupJsonFiles, writeDataFile } from "./util/file";

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
const keyPages: KeyPage[] = KeyPageProcessor.process();
const passives: Passive[] = PassiveProcessor.process();

for (const locale of ALL_LOCALIZATIONS) {
    // Abno
    const decoratedAbnoPages: { [key: string]: DecoratedAbnoPage } =
        AbnoPageMapper.map(abnoPages, locale);
    queryMapperProps[locale].abnoPages = decoratedAbnoPages;
    writeDataFile(`${locale}/abno.json`, decoratedAbnoPages);

    // Combat pages
    const decoratedCombatPages: { [key: string]: DecoratedCombatPage } =
        CombatPageMapper.map(combatPages, locale);
    queryMapperProps[locale].combatPages = decoratedCombatPages;
    writeDataFile(`${locale}/combat.json`, decoratedCombatPages);

    // Passives
    const decoratedPassives: { [key: string]: DecoratedPassive } =
        PassiveMapper.map(passives, locale);
    queryMapperProps[locale].passives = decoratedPassives;
    writeDataFile(`${locale}/passive.json`, decoratedPassives);

    // Key pages
    const decoratedKeyPages: { [key: string]: DecoratedKeyPage } =
        KeyPageMapper.map(keyPages, decoratedPassives, locale);
    queryMapperProps[locale].keyPages = decoratedKeyPages;
    writeDataFile(`${locale}/keypages.json`, decoratedKeyPages);
}

// Query mapper
const lookupResults: QueryMapperLookupTable = QueryMapper.map(queryMapperProps);

// Disambiguate
const ambiguousResults: AmbiguousResults[] =
    AmbiguityResolver.detect(lookupResults);

// Remap with disambiguator
// Side effects: this modifies both lookupResults and ambiguousResults in place
QueryMapper.remap(lookupResults, ambiguousResults);

// Write query map
writeDataFile(`queryLookupResults.json`, lookupResults);

// Write disambiguation data
const ambiguousResultsDataBlob = Object.fromEntries(
    ambiguousResults.map((ambiguousResults) => {
        return [ambiguousResults.id, ambiguousResults];
    })
);
writeDataFile(`ambiguousResults.json`, ambiguousResultsDataBlob);

// Autocomplete
writeDataFile(`autocomplete.json`, {
    data: Object.keys(lookupResults),
});

console.log("Done");
