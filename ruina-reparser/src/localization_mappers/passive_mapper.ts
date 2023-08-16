import {
    DecoratedPassive,
    Localization,
    Passive,
} from "@ghoulean/ruina-common";
import { LOCALIZE_DIR } from "../util/constants";
import { readFile, walkSync } from "../util/file";
import path = require("path");

export class PassiveMapper {
    public static map(
        passives: Passive[],
        locale: Localization
    ): { [key: string]: DecoratedPassive } {
        const retVal: { [key: string]: DecoratedPassive } = {};
        const passiveDict: any = this.buildPassiveInfo(locale);

        for (const passive of passives) {
            const id: string = passive.id;
            const localedInfo: any = passiveDict[id];
            if (!localedInfo) {
                console.log(
                    `No localization found for passive id=${id}; skipping`
                );
                continue;
            }

            const decoratedPassive: DecoratedPassive = {
                ...passive,
                name: localedInfo["Name"]["_text"],
                description: localedInfo["Desc"]["_text"],
            };
            retVal[id] = decoratedPassive;
        }

        return retVal;
    }

    private static buildPassiveInfo(locale: Localization): any {
        const cardBlobs: any = {};

        const localizedCardDir = path.resolve(
            LOCALIZE_DIR,
            locale.toString(),
            "PassiveDesc"
        );

        for (const filePath of walkSync(localizedCardDir)) {
            const json: any = readFile(filePath);
            for (const blob of json["PassiveDescRoot"]["PassiveDesc"]) {
                cardBlobs[blob["_attributes"]["ID"]] = blob;
            }
        }

        return cardBlobs;
    }
}
