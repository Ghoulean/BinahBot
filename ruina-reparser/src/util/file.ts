import * as fs from 'fs';
import * as path from 'path';
import * as xmljs from'xml-js';
import { BASEMOD_DIR, DATA_DIR, OUTPUT_DIR } from './constants';

const CONVERT_JSON_OPTIONS = {
    compact: true,
    ignoreDeclaration: true,
    ignoreInstruction: true,
    ignoreComment: true,
    ignoreCdata: true,
    ignoreDoctype: true,
};

export function hasFileExtension(filePath: string, ext: string) {
    return filePath.endsWith(`.${ext}`);
}

export function renameFileExtension(filePath: string, newExt: string) {
    return path.format({ ...path.parse(filePath), base: "", ext: `.${newExt}` });
}

export function convertToJson(inputPath: string, outputPath: string) {
    const xml = fs.readFileSync(inputPath, 'utf8');
    const result = xmljs.xml2json(xml, CONVERT_JSON_OPTIONS);
    fs.mkdirSync(path.dirname(outputPath), { recursive: true });
    try {
        fs.writeFileSync(outputPath, result, { "flag": "wx" });
    } catch (err) {
        return;
    }
}

export function readFile(inputPath: string): unknown {
    return JSON.parse(fs.readFileSync(inputPath, 'utf8'));
}

export function writeDataFile(outputFile: string, data: unknown) {
    const outputPath = path.resolve(DATA_DIR, outputFile);
    try {
        fs.mkdirSync(path.dirname(outputPath));
    } catch (e) {
        // do nothing
    }
    fs.writeFileSync(outputPath, JSON.stringify(data));
}

export function* walkSync(dir: string): Generator<string, unknown, unknown> {
    const files = fs.readdirSync(dir, { withFileTypes: true });
    for (const file of files) {
        if (file.isDirectory()) {
            yield* walkSync(path.join(dir, file.name));
        } else {
            yield path.join(dir, file.name);
        }
    }
}

export function fileSanityCheck() {
    if (!fs.existsSync(BASEMOD_DIR)){
        throw new Error(`BaseMod sanity check failed: did not find ${BASEMOD_DIR}`);
    }
}

export function setupJsonFiles() {
    try {
        fs.mkdirSync(OUTPUT_DIR);
    } catch (e) {
        // do nothing
    }
    try {
        fs.mkdirSync(DATA_DIR);
    } catch (e) {
        // do nothing
    }
    for (const filePath of walkSync(BASEMOD_DIR)) {
        if (hasFileExtension(filePath, "txt") || hasFileExtension(filePath, "xml")) {
            const outputPath = path.resolve(OUTPUT_DIR, path.relative(BASEMOD_DIR, renameFileExtension(filePath, "json")));
            convertToJson(filePath, path.resolve(OUTPUT_DIR, outputPath));
        }
    }
}
