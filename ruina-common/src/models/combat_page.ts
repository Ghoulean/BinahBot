import { Chapter, Ego, Floor, Rarity } from "../enums";
import { Die } from "./die";

export interface CombatPage {
    id: string;
    ego: Ego;
    egoFloor: Floor;
    scriptId: string;
    rarity: Rarity;
    dice: Die[];
    chapter: Chapter;
}