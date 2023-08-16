import {
    ALL_FLOORS,
    Chapter,
    CombatPage,
    Die,
    DieType,
    Ego,
    Floor,
    dieTypeFromStrings,
    rangeFromStringValue,
    rarityFromStringValue,
} from "@ghoulean/ruina-common";
import { COMBAT_PAGE_DIR } from "../util/constants";
import { readFile, walkSync } from "../util/file";
import path = require("path");

export class CombatPageProcessor {
    public static process(): CombatPage[] {
        const data: CombatPage[] = [];
        for (const filePath of walkSync(COMBAT_PAGE_DIR)) {
            const json: any = readFile(filePath);
            for (const card of json["DiceCardXmlRoot"]["Card"]) {
                const id: string = card["_attributes"]["ID"];
                const imageId: string | undefined = card["Artwork"]?.["_text"];
                if (!imageId) {
                    console.log(
                        `No image found for combat page ${id}; skipping`
                    );
                }
                const ego: Ego = this.determineEgo(card);

                let rawDiceArray: unknown = card["BehaviourList"]["Behaviour"];
                if (!rawDiceArray) {
                    rawDiceArray = [];
                } else if (!Array.isArray(rawDiceArray)) {
                    rawDiceArray = [rawDiceArray];
                }
                const dice: Die[] = (rawDiceArray as any[]).map(
                    (rawDieBlob: any) => {
                        return this.parseDie(rawDieBlob["_attributes"]);
                    }
                );

                const chapter: Chapter = this.determineChapter(filePath, card);

                const combatPage: CombatPage = {
                    id: id,
                    ego: ego,
                    egoFloor:
                        ego == Ego.ABNO_EGO
                            ? this.determineEgoFloor(Number(id))
                            : Floor.NONE,
                    scriptId: card["Script"]["_text"] ?? "",
                    rarity: rarityFromStringValue(card["Rarity"]["_text"])!,
                    dice: dice,
                    chapter: chapter,
                    cost: Number(card["Spec"]["_attributes"]["Cost"]) || 0,
                    range: rangeFromStringValue(
                        card["Spec"]["_attributes"]["Range"]
                    )!,
                    imagePath: `/${imageId}.png`,
                };
                data.push(combatPage);
            }
        }
        return data;
    }

    private static parseDie(blob: any): Die {
        return {
            // Some cards, like Crescendo (both of them) and Deluge of Brachial Quietuses
            // have typo'd "Detail" as "Detil". This causes the die type to default to SLASH
            type: dieTypeFromStrings(blob["Type"], blob["Detail"]) ?? DieType.SLASH,
            min: Number(blob["Min"]) || -1,
            max: Number(blob["Dice"]) || -1,
            scriptId: blob["Script"],
        };
    }

    private static determineChapter(filePath: string, card: any): Chapter {
        return (
            Number(card["Chapter"]?.["_text"]) ||
            Number(path.basename(filePath).replace(/[^0-9]/gi, "")) ||
            Chapter.LIBRARY_OF_RUINA
        );
    }

    private static determineEgo(blob: any): Ego {
        if (blob["Option"]?.["_text"] == "EGO") {
            return Ego.ABNO_EGO;
        } else if (blob["Option"]?.["_text"] == "EgoPersonal") {
            return Ego.ABNO_EGO;
        } else {
            return Ego.NOT_EGO;
        }
    }

    private static determineEgoFloor(id: number): Floor {
        const BASE_EGO_NUM: number = 910000;
        const egoId: number = id - BASE_EGO_NUM;
        if (egoId <= 0) {
            return Floor.NONE;
        } else if (egoId <= 5) {
            return Floor.MALKUTH;
        } else if (egoId <= 10) {
            return Floor.KETER;
        } else if (egoId <= 50) {
            return ALL_FLOORS[Math.floor((egoId - 1) / 5)];
        } else {
            return Floor.NONE;
        }
    }
}
