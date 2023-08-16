import { Passive, Rarity, rarityFromStringValue } from "@ghoulean/ruina-common";
import { PASSIVE_DIR } from "../util/constants";
import { readFile, walkSync } from "../util/file";

export class PassiveProcessor {
    public static process(): Passive[] {
        const data: Passive[] = [];
        for (const filePath of walkSync(PASSIVE_DIR)) {
            const json: any = readFile(filePath);

            let passiveArray: any = json["PassiveDescRoot"]?.["PassiveDesc"] ?? json["PassiveXmlRoot"]?.["Passive"]

            for (const card of passiveArray) {
                const costStr: string = card["Cost"]?.["_text"] ?? "0";
                const rarityStr: string = card["Rarity"]?.["_text"] ?? Rarity.PAPERBACK

                const passive: Passive = {
                    id: card["_attributes"]["ID"],
                    cost: Number(costStr),
                    rarity: rarityFromStringValue(rarityStr)!,
                    // Since this property only appears to override the default
                    // `true`, simply assume that it is false if it is specified
                    canGivePassive: !!card["CanGivePassive"] ? false : true,
                };

                data.push(passive);
            }
        }

        return data;
    }
}
