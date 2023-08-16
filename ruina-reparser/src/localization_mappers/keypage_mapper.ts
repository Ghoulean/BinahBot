import {
    DecoratedKeyPage,
    DecoratedPassive,
    KeyPage,
    Localization,
} from "@ghoulean/ruina-common";
import { LOCALIZE_DIR } from "../util/constants";
import { readFile, walkSync } from "../util/file";
import path = require("path");
import { Util } from "../util/util";

export class KeyPageMapper {
    public static map(
        keyPages: KeyPage[],
        decoratedPassives: { [key: string]: DecoratedPassive },
        locale: Localization
    ): { [key: string]: DecoratedKeyPage } {
        const retVal: { [key: string]: DecoratedKeyPage } = {};
        const keyPageOwnerDict: any = this.buildNameDict(locale);

        for (const keyPage of keyPages) {
            const id: string = keyPage.id;
            const bookName: string | undefined =
                keyPageOwnerDict[id]?.["BookName"]?.["_text"];
            if (!bookName) {
                console.log(
                    `Keypage ${id} with locale ${locale} has no localization found; skipping`
                );
                continue;
            }

            const remappedPassiveNames: string[] =
                keyPage.defaultPassiveIds.map((passiveId: string) => {
                    return decoratedPassives[passiveId]?.name ?? "";
                });

            const decoratedKeyPage: DecoratedKeyPage = {
                ...keyPage,
                name: Util.cleanString(bookName),
                passiveNames: remappedPassiveNames,
                locale: locale
            };
            retVal[id] = decoratedKeyPage;
        }

        return retVal;
    }

    private static buildNameDict(locale: Localization): any {
        const cardBlobs: any = {};

        const localizedCardDir = path.resolve(
            LOCALIZE_DIR,
            locale.toString(),
            "Books"
        );

        for (const filePath of walkSync(localizedCardDir)) {
            const json: any = readFile(filePath);
            for (const blob of json["BookDescRoot"]["bookDescList"][
                "BookDesc"
            ]) {
                cardBlobs[blob["_attributes"]["BookID"]] = blob;
            }
        }

        return cardBlobs;
    }
}
