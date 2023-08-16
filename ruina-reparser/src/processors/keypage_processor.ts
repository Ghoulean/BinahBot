import {
    Chapter,
    DieType,
    KeyPage,
    KeyPageResistance,
    resistanceFromString,
} from "@ghoulean/ruina-common";
import { KEY_PAGE_DIR } from "../util/constants";
import { readFile, walkSync } from "../util/file";

type EquipEffectTxt = {
    _text: string;
};

// I made this to make my life easier but it didn't actually do that
// but now I've committed too far to back out
type EquipEffect = {
    HP: EquipEffectTxt;
    Break: EquipEffectTxt;
    SpeedMin: EquipEffectTxt;
    Speed: EquipEffectTxt;
    SResist: EquipEffectTxt; // slash
    PResist: EquipEffectTxt; // pierce
    HResist: EquipEffectTxt; // blunt
    SBResist: EquipEffectTxt;
    PBResist: EquipEffectTxt;
    HBResist: EquipEffectTxt;
    StartPlayPoint?: EquipEffectTxt;
    MaxPlayPoint?: EquipEffectTxt;
    Passive?: EquipEffectTxt[];
    OnlyCard?: EquipEffectTxt[];
};

const EQUIP_EFFECT_KEYS: string[] = [
    "HP",
    "Break",
    "SpeedMin",
    "Speed",
    "SResist",
    "PResist",
    "HResist",
    "SBResist",
    "PBResist",
    "HBResist",
];

export class KeyPageProcessor {
    public static process(): KeyPage[] {
        const data: KeyPage[] = [];
        for (const filePath of walkSync(KEY_PAGE_DIR)) {
            const json: any = readFile(filePath);
            for (const keypage of json["BookXmlRoot"]["Book"]) {
                const id: string = keypage["_attributes"]["ID"];
                const equipEffect: EquipEffect = keypage[
                    "EquipEffect"
                ] as EquipEffect;
                if (!this.verifyEquipEffect(equipEffect)) {
                    console.log(`Keypage ${id} has bad props; skipping`);
                    continue;
                }
                const hpResistances: KeyPageResistance = {
                    [DieType.SLASH]: resistanceFromString(
                        equipEffect.SResist["_text"]
                    )!,
                    [DieType.PIERCE]: resistanceFromString(
                        equipEffect.PResist["_text"]
                    )!,
                    [DieType.BLUNT]: resistanceFromString(
                        equipEffect.HResist["_text"]
                    )!,
                };
                const staggerResistances: KeyPageResistance = {
                    [DieType.SLASH]: resistanceFromString(
                        equipEffect.SBResist["_text"]
                    )!,
                    [DieType.PIERCE]: resistanceFromString(
                        equipEffect.PBResist["_text"]
                    )!,
                    [DieType.BLUNT]: resistanceFromString(
                        equipEffect.HBResist["_text"]
                    )!,
                };

                let passives: EquipEffectTxt[];
                if (!equipEffect.Passive) {
                    passives = [];
                } else if (Array.isArray(equipEffect.Passive)) {
                    passives = equipEffect.Passive;
                } else {
                    passives = [equipEffect.Passive];
                }
                let exclusiveCardIds: EquipEffectTxt[];
                if (!equipEffect.OnlyCard) {
                    exclusiveCardIds = [];
                } else if (Array.isArray(equipEffect.OnlyCard)) {
                    exclusiveCardIds = equipEffect.OnlyCard;
                } else {
                    exclusiveCardIds = [equipEffect.OnlyCard];
                }

                const keyPage: KeyPage = {
                    id: id,
                    hp: Number(equipEffect.HP["_text"]),
                    stagger: Number(equipEffect.Break["_text"]),
                    hpResistances: hpResistances,
                    staggerResistances: staggerResistances,
                    minBaseSpeed: Number(equipEffect.SpeedMin["_text"]),
                    maxBaseSpeed: Number(equipEffect.Speed["_text"]),
                    baseLight: Number(
                        equipEffect.StartPlayPoint?.["_text"] ?? "3"
                    ),
                    defaultPassiveIds: passives.map((txt: EquipEffectTxt) => {
                        return txt["_text"];
                    }),
                    exclusiveCardsIds: exclusiveCardIds.map((txt) => {
                        return txt["_text"];
                    }),
                    chapter: Number(
                        keypage["Chapter"]?.["_text"] ??
                            Chapter.LIBRARY_OF_RUINA
                    ),
                };
                data.push(keyPage);
            }
        }

        return data;
    }

    private static verifyEquipEffect(e: EquipEffect): boolean {
        for (const k of EQUIP_EFFECT_KEYS) {
            if (!(k in e)) {
                return false;
            }
        }
        return true;
    }
}
