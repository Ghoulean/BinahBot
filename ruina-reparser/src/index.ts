import {
    ALL_LOCALIZATIONS,
    AbnoPage,
    DecoratedAbnoPage,
    Localization,
} from "@ghoulean/ruina-common";
import { AbnoPageMapper } from "./localization_mappers/abno_page_mapper";
import {
    QueryMapper,
    QueryMapperLookupTable,
    QueryMapperPropType,
} from "./localization_mappers/query_mapper";
import { AbnoPageProcessor } from "./processors/abnopage_processor";
import { fileSanityCheck, setupJsonFiles, writeDataFile } from "./util/file";

fileSanityCheck();
setupJsonFiles();

const queryMapperProps: QueryMapperPropType = {
    [Localization.KOREAN]: {},
    [Localization.JAPANESE]: {},
    [Localization.ENGLISH]: {},
    [Localization.CHINESE_SIMPLIFIED]: {},
    [Localization.CHINESE_TRADITIONAL]: {}
}

const abnoPages: AbnoPage[] = AbnoPageProcessor.process();

for (const locale of ALL_LOCALIZATIONS) {
    // Abno
    const decoratedAbnoPages: { [key: string]: DecoratedAbnoPage } =
        AbnoPageMapper.map(abnoPages, locale);
    queryMapperProps[locale].abnoPages = decoratedAbnoPages;
    writeDataFile(`${locale}/abno.json`, decoratedAbnoPages);

    // TODO: Combat pages
    // TODO: Key pages
    // TODO: Passives
}

// Query mapper
const lookupResults: QueryMapperLookupTable = QueryMapper.map(queryMapperProps);
writeDataFile(`queryLookupResults.json`, lookupResults);

// Autocomplete
writeDataFile(`autocomplete.json`, {
    data: Object.keys(lookupResults),
});

console.log("Done");
