import { Chapter, Ego, Floor, Localization, Range, Rarity } from "../enums";
import { Die } from "./die";

export interface CombatPage {
    id: string;
    ego: Ego;
    egoFloor: Floor;
    scriptId: string;
    rarity: Rarity;
    dice: Die[];
    chapter: Chapter;
    cost: number;
    range: Range;
    imagePath: string;
}

export interface DecoratedCombatPage extends CombatPage {
    locale: Localization;
    name: string;
    description: string;
    diceDescriptions: string[];
}
