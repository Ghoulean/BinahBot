import { PageType } from "@ghoulean/ruina-common";
import {
    CARD_DROP_TABLE_DIR,
    DROP_BOOK_DIR,
    FINAL_BAND_REWARD_DIR,
    STAGE_INFO_DIR,
} from "../util/constants";
import { readFile, walkSync } from "../util/file";

type CollectableCardIds = {
    type: PageType;
    id: string;
};

export class BookDrops {
    private static collectableKeyPageCache: string[];
    private static collectableCombatPageCache: string[];

    public static getAllCollectableKeyPages(): string[] {
        if (!BookDrops.collectableKeyPageCache) {
            BookDrops.collectableKeyPageCache = this.getAllCollectableCardIds()
                .filter((oci) => {
                    return oci.type == PageType.KEY_PAGE;
                })
                .map((oci) => {
                    return oci.id;
                });
        }
        return BookDrops.collectableKeyPageCache;
    }
    public static getAllCollectableCombatPages(): string[] {
        if (!BookDrops.collectableCombatPageCache) {
            BookDrops.collectableCombatPageCache =
                this.getAllCollectableCardIds()
                    .filter((oci) => {
                        return oci.type == PageType.COMBAT_PAGE;
                    })
                    .map((oci) => {
                        return oci.id;
                    });
        }
        return BookDrops.collectableCombatPageCache;
    }

    private static getAllCollectableCardIds(): CollectableCardIds[] {
        return this.getAllCollectableCardDropTables().concat(
            this.getAllCollectableDropBookTables(),
            this.getAllReverbEnsembleRewardCards(),
            this.getAllBlackSilenceAndDistortedEnsembleRewardCards()
        );
    }

    private static getAllCollectableCardDropTables(): CollectableCardIds[] {
        const retVal: CollectableCardIds[] = [];
        for (const filePath of walkSync(CARD_DROP_TABLE_DIR)) {
            // eslint-disable-next-line  @typescript-eslint/no-explicit-any
            const json: any = readFile(filePath);
            for (const cardDropTable of json["CardDropTableXmlRoot"][
                "DropTable"
            ]) {
                const cardsArr: any[] = cardDropTable["Card"];
                if (!cardsArr) {
                    continue;
                }
                for (const card of cardsArr) {
                    retVal.push({
                        type: PageType.COMBAT_PAGE,
                        id: card["_text"]!,
                    });
                }
            }
        }
        return retVal;
    }

    private static getAllCollectableDropBookTables(): CollectableCardIds[] {
        const retVal: CollectableCardIds[] = [];
        for (const filePath of walkSync(DROP_BOOK_DIR)) {
            // eslint-disable-next-line  @typescript-eslint/no-explicit-any
            const json: any = readFile(filePath);
            for (const dropBookList of json["BookUseXmlRoot"]["BookUse"]) {
                const drops: any = dropBookList["DropItem"];
                let dropsArr: any[];
                if (!drops) {
                    continue;
                }
                if (Array.isArray(drops)) {
                    dropsArr = drops;
                } else {
                    dropsArr = [drops];
                }
                for (const drop of dropsArr) {
                    retVal.push({
                        type:
                            drop["_attributes"]?.["Type"] == "Equip"
                                ? PageType.KEY_PAGE
                                : PageType.COMBAT_PAGE,
                        id: drop["_text"]!,
                    });
                }
            }
        }
        return retVal;
    }

    private static getAllReverbEnsembleRewardCards(): CollectableCardIds[] {
        const retValMap: { [key: string]: CollectableCardIds } = {};
        for (const filePath of walkSync(STAGE_INFO_DIR)) {
            // eslint-disable-next-line  @typescript-eslint/no-explicit-any
            const json: any = readFile(filePath);
            for (const stageInfo of json["StageXmlRoot"]["Stage"]) {
                const rewards: any[] = stageInfo["RewardItems"];
                if (!rewards) {
                    continue;
                }
                for (const reward of rewards) {
                    const id: string = reward["_text"];
                    retValMap[id] = {
                        type:
                            reward["_attributes"]?.["Type"] == "Equip"
                                ? PageType.KEY_PAGE
                                : PageType.COMBAT_PAGE,
                        id: id,
                    };
                }
            }
        }
        return Object.values(retValMap);
    }

    private static getAllBlackSilenceAndDistortedEnsembleRewardCards(): CollectableCardIds[] {
        const retVal: CollectableCardIds[] = [];
        for (const filePath of walkSync(FINAL_BAND_REWARD_DIR)) {
            // eslint-disable-next-line  @typescript-eslint/no-explicit-any
            const json: any = readFile(filePath);
            for (const reward of json["RewardXmlRoot"]["Card"]) {
                retVal.push({
                    type: PageType.COMBAT_PAGE,
                    id: reward["_text"],
                });
            }
        }
        return retVal;
    }
}
