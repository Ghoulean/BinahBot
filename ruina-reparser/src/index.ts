import {
    ALL_LOCALIZATIONS,
    AbnoPage,
    DecoratedAbnoPage,
    Localization,
    LookupResult,
} from "@ghoulean/ruina-common";
import { AbnoPageMapper } from "./localization_mappers/abno_page_mapper";
import {
    QueryMapper,
    QueryMapperPropType,
} from "./localization_mappers/query_mapper";
import { AbnoPageProcessor } from "./processors/abnopage_processor";
import { fileSanityCheck, setupJsonFiles, writeDataFile } from "./util/file";

fileSanityCheck();
setupJsonFiles();

const queryMapperProps: { [key in Localization]?: QueryMapperPropType } = {};

const abnoPages: AbnoPage[] = AbnoPageProcessor.process();

const autocomplete: { [key: string]: 1 } = {};

for (const locale of ALL_LOCALIZATIONS) {
    queryMapperProps[locale] = {
        locale: locale,
    };

    // Abno
    const decoratedAbnoPages: { [key: string]: DecoratedAbnoPage } =
        AbnoPageMapper.map(abnoPages, locale);
    writeDataFile(`${locale}/abno.json`, decoratedAbnoPages);
    queryMapperProps[locale]!.abnoPages = decoratedAbnoPages;

    // TODO: Combat pages
    // TODO: Key pages
    // TODO: Passives

    // Query mapper
    const lookupResults: { [key: string]: LookupResult } = QueryMapper.map(
        queryMapperProps[locale]!
    );
    writeDataFile(`${locale}/queryLookupResults.json`, lookupResults);

    // Add object keys to autocomplete
    for (const key in lookupResults) {
        autocomplete[key] = 1;
    }
}

writeDataFile(`autocomplete.json`, {
    data: Object.keys(autocomplete),
});

console.log("Done");
