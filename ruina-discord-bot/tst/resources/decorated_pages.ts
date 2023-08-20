import {
    AbnoPageSelectType,
    Chapter,
    DecoratedAbnoPage,
    DecoratedCombatPage,
    DecoratedKeyPage,
    DecoratedPassive,
    DieType,
    Ego,
    Floor,
    Localization,
    Range,
    Rarity,
    Resistance,
} from "@ghoulean/ruina-common";

export const DECORATED_ABNO_PAGE: DecoratedAbnoPage = {
    locale: Localization.ENGLISH,
    name: "TestAbnoPage",
    description: "TestDescription",
    flavorText: "TestFlavorText",
    imagePath: "test.png",
    nameId: "test",
    floor: Floor.NONE,
    chapter: Chapter.LIBRARY_OF_RUINA,
    emoLevel: 1,
    emotionRate: 1,
    emotionSign: 1,
    targetType: AbnoPageSelectType.SELECT_ONE,
    scriptId: "testScriptId",
};

export const BASE_DECORATED_COMBAT_PAGE: DecoratedCombatPage = {
    locale: Localization.ENGLISH,
    name: "name",
    description: "description",
    diceDescriptions: ["description1"],
    id: "id",
    ego: Ego.NOT_EGO,
    egoFloor: Floor.BINAH,
    scriptId: "scriptId",
    rarity: Rarity.PAPERBACK,
    dice: [
        {
            type: DieType.BLUNT,
            min: 1,
            max: 2,
            scriptId: "scriptId",
        },
    ],
    chapter: Chapter.LIBRARY_OF_RUINA,
    cost: 0,
    range: Range.MELEE,
    imagePath: "image.png",
};

export const BASE_DECORATED_KEY_PAGE: DecoratedKeyPage = {
    name: "Keypage Name",
    passiveNames: ["passiveName1"],
    locale: Localization.ENGLISH,
    id: "pageId",
    textId: "textId",
    hp: 1,
    stagger: 2,
    hpResistances: {
        [DieType.SLASH]: Resistance.FATAL,
        [DieType.PIERCE]: Resistance.WEAK,
        [DieType.BLUNT]: Resistance.NORMAL
    },
    staggerResistances: {
        [DieType.SLASH]: Resistance.ENDURED,
        [DieType.PIERCE]: Resistance.INEFFECTIVE,
        [DieType.BLUNT]: Resistance.IMMUNE
    },
    minBaseSpeed: 1,
    maxBaseSpeed: 2,
    baseLight: 3,
    chapter: Chapter.LIBRARY_OF_RUINA,
    rarity: Rarity.PAPERBACK,
    defaultPassiveIds: ["passiveId1", "passiveId2"],
    exclusiveCardsIds: ["cardId1"]
}

export const BASE_DECORATED_PASSIVE: DecoratedPassive = {
    name: "Passive Name",
    description: "Description of passive",
    locale: Localization.ENGLISH,
    id: "passiveId",
    cost: 2,
    rarity: Rarity.PAPERBACK,
    canGivePassive: false
}