import { fileSanityCheck, setupJsonFiles, writeDataFile } from './util/file';
import { AbnoPageProcessor } from './processors/abnopage_processor';
import { ALL_LOCALIZATIONS, AbnoPage, DecoratedAbnoPage, Localization } from '@ghoulean/ruina-common';
import { AbnoPageMapper } from './localization_mappers/abno_page_mapper';
import { QueryMapper, QueryMapperPropType } from './localization_mappers/query_mapper';

fileSanityCheck();
setupJsonFiles();

const queryMapperProps: {[key in Localization]?: QueryMapperPropType} = {};

const abnoPages: AbnoPage[] = AbnoPageProcessor.process();

for (const locale of ALL_LOCALIZATIONS) {
    queryMapperProps[locale] = {
        locale: locale
    };

    // Abno
    const decoratedAbnoPages: {[key: string]: DecoratedAbnoPage} = AbnoPageMapper.map(abnoPages, locale);
    writeDataFile(`${locale}/abno.json`, decoratedAbnoPages);
    queryMapperProps[locale]!.abnoPages = decoratedAbnoPages;

    // TODO: Combat pages
    // TODO: Key pages
    // TODO: Passives

    // Query mapper
    const lookupResults = QueryMapper.map(queryMapperProps[locale]!);
    writeDataFile(`${locale}/queryLookupResults.json`, lookupResults);
}

console.log("Done");
