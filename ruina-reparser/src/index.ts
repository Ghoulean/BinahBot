import { fileSanityCheck, setupJsonFiles } from './util/file';
import { AbnoPageProcessor } from './processors/abnopage_processor';
import { AbnoPage } from '@ghoulean/ruina-common';

fileSanityCheck();
setupJsonFiles();

const abnopages: AbnoPage[] = AbnoPageProcessor.process();
