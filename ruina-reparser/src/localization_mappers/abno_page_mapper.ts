import {
    AbnoPage,
    DecoratedAbnoPage,
    Floor,
    Localization,
} from "@ghoulean/ruina-common";
import * as path from "path";
import { LOCALIZE_DIR } from "../util/constants";
import { readFile } from "../util/file";

/**
 * Generates localized maps from id to decorated abno pages
 */
export class AbnoPageMapper {
    public static map(
        abnoPages: AbnoPage[],
        locale: Localization
    ): { [key: string]: DecoratedAbnoPage } {
        const retVal: { [key: string]: DecoratedAbnoPage } = {};
        const localInfo = readFile(
            path.resolve(
                LOCALIZE_DIR,
                locale.toString(),
                "AbnormalityCards",
                "AbnormalityCards.json"
            )
        );
        for (const abnoPage of abnoPages) {
            const floor: Floor = abnoPage.floor;

            // Ignore non-obtainable abno pages for now -- revisit later
            if (floor === Floor.NONE) {
                continue;
            }

            const id: string = abnoPage.nameId;
            const rawData: any = this.findRawData(localInfo, floor, id);
            const decoratedAbnoPage: DecoratedAbnoPage = {
                ...abnoPage,
                locale: locale,
                name: rawData["CardName"]["_text"],
                description: rawData["AbilityDesc"]["_text"],
                flavorText: rawData["FlaborText"]["_text"], // sic
            };
            retVal[id] = decoratedAbnoPage;
        }
        return retVal;
    }

    private static findRawData(data: any, floor: Floor, nameId: string): any {
        let sephirahData;
        for (const d of data["AbnormalityCardsRoot"]["Sephirah"]) {
            if (d["_attributes"]["SephirahType"] === floor.toString()) {
                sephirahData = d;
                break;
            }
        }
        if (!sephirahData) {
            throw new Error(`Could not find floor ${floor}`);
        }
        for (const d of sephirahData["AbnormalityCard"]) {
            if (d["_attributes"]["ID"] === nameId) {
                return d;
            }
        }
        throw new Error(`Found floor ${floor} but not abno with id ${nameId}`);
    }
}
