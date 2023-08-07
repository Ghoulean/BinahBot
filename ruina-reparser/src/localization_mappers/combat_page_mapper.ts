import {
    CombatPage,
    DecoratedCombatPage,
    Localization,
} from "@ghoulean/ruina-common";
import { LOCALIZE_DIR } from "../util/constants";
import { readFile, walkSync } from "../util/file";
import path = require("path");

export class CombatPageMapper {
    public static map(
        combatPages: CombatPage[],
        locale: Localization
    ): { [key: string]: DecoratedCombatPage } {
        const retVal: { [key: string]: DecoratedCombatPage } = {};
        const cardDict: any = this.buildLocalInfo(locale);

        for (const page of combatPages) {
            const id: string = page.id;
            const localeInfo: any = cardDict[id];
            if (!localeInfo) {
                console.log(`No localization found for combat page id=${id}; skipping`);
                continue;
            }
            const diceDescriptions: string[] = new Array(page.dice.length).fill(
                ""
            );

            let rawDiceArray: unknown = localeInfo["Behaviour"];
            if (!rawDiceArray) {
                rawDiceArray = [];
            } else if (!Array.isArray(rawDiceArray)) {
                rawDiceArray = [rawDiceArray];
            }
            (rawDiceArray as any[]).forEach((rawDieBlob: any) => {
                const index: number = rawDieBlob["_attributes"]["ID"];
                const str: string = rawDieBlob["_text"];
                diceDescriptions[index] = str;
            });

            const decoratedCombatPage: DecoratedCombatPage = {
                ...page,
                locale: locale,
                name: localeInfo["LocalizedName"]?.["_text"] ?? "",
                description: localeInfo["Ability"]?.["_text"] ?? "",
                diceDescriptions: diceDescriptions,
            };
            retVal[id] = decoratedCombatPage;
        }

        return retVal;
    }

    private static buildLocalInfo(locale: Localization): any {
        const cardBlobs: any = {};

        const localizedCardDir = path.resolve(
            LOCALIZE_DIR,
            locale.toString(),
            "BattlesCards"
        );

        for (const filePath of walkSync(localizedCardDir)) {
            const json: any = readFile(filePath);
            for (const blob of json["BattleCardDescRoot"]["cardDescList"][
                "BattleCardDesc"
            ]) {
                cardBlobs[blob["_attributes"]["ID"]] = blob;
            }
        }

        return cardBlobs;
    }
}
