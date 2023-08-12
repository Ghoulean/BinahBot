import {
    AbnoPageSelectType,
    Chapter,
    DecoratedAbnoPage,
    DecoratedCombatPage,
    DieType,
    Ego,
    Floor,
    Localization,
    Range,
    Rarity,
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