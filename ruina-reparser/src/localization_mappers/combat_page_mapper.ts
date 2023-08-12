import {
    CombatPage,
    DecoratedCombatPage,
    Die,
    Localization,
} from "@ghoulean/ruina-common";
import { LOCALIZE_DIR } from "../util/constants";
import { readFile, walkSync } from "../util/file";
import { Util } from "../util/util";
import path = require("path");

export class CombatPageMapper {
    public static map(
        combatPages: CombatPage[],
        locale: Localization
    ): { [key: string]: DecoratedCombatPage } {
        const retVal: { [key: string]: DecoratedCombatPage } = {};
        const cardDict: any = this.buildBattleCardLocalInfo(locale);
        const cardAbilityDict: any =
            this.buildBattleCardAbilityLocalInfo(locale);

        for (const page of combatPages) {
            const id: string = page.id;
            const localeInfo: any = cardDict[id];
            if (!localeInfo) {
                console.log(
                    `No localization found for combat page id=${id}; skipping`
                );
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
                const index: number = Number(rawDieBlob["_attributes"]["ID"]);
                if (Number.isNaN(index)) {
                    return;
                }
                const str: string = rawDieBlob["_text"] ?? "";
                diceDescriptions[index] = Util.cleanString(str);
            });
            page.dice.forEach((die: Die, index: number) => {
                const scriptId: string = die.scriptId;
                const scriptDesc: string | undefined =
                    cardAbilityDict[scriptId];
                if (scriptDesc) {
                    diceDescriptions[index] = Util.cleanString(scriptDesc);
                }
            });

            let cardDesc: string | undefined = localeInfo["Ability"]?.["_text"];
            if (!cardDesc) {
                const scriptId: string = page.scriptId ?? "";
                const scriptDesc: string | undefined =
                    cardAbilityDict[scriptId];
                if (scriptDesc) {
                    cardDesc = Util.cleanString(scriptDesc);
                }
            }

            const decoratedCombatPage: DecoratedCombatPage = {
                ...page,
                locale: locale,
                name: Util.cleanString(
                    localeInfo["LocalizedName"]?.["_text"] ?? ""
                ),
                description: cardDesc ?? "",
                diceDescriptions: diceDescriptions,
            };
            retVal[id] = decoratedCombatPage;
        }

        return retVal;
    }

    private static buildBattleCardLocalInfo(locale: Localization): any {
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

    private static buildBattleCardAbilityLocalInfo(locale: Localization): any {
        const cardBlobs: any = {};

        const localizedCardDir = path.resolve(
            LOCALIZE_DIR,
            locale.toString(),
            "BattleCardAbilities"
        );

        for (const filePath of walkSync(localizedCardDir)) {
            const json: any = readFile(filePath);
            for (const blob of json["BattleCardAbilityDescRoot"][
                "BattleCardAbility"
            ]) {
                let desc: any = blob["Desc"];
                if (!desc) {
                    continue;
                }
                if (!Array.isArray(desc)) {
                    desc = [desc];
                }
                desc = desc
                    .map((descBlob: any) => {
                        return descBlob["_text"];
                    })
                    .join("\n");

                cardBlobs[blob["_attributes"]["ID"]] = desc;
            }
        }

        return cardBlobs;
    }
}
