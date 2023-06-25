import { fileSanityCheck, setupJsonFiles } from './util/file';
import { AbnoPageProcessor } from './processors/abnopage_processor';

fileSanityCheck();
setupJsonFiles();

AbnoPageProcessor.process();
