import { Chapter, DieType, Resistance } from "../enums";

export interface KeyPage {
    id: string;
    hp: number;
    stagger: number;
    hpResistances: { [key in DieType]? : Resistance };
    staggerResistances: { [key in DieType]? : Resistance };
    minBaseSpeed: number;
    maxBaseSpeed: number;
    baseLight: number;
    baseSpeedDieSlots: number;
    defaultPassiveIds: string[];
    exclusiveCardsIds: string[];
    chapter: Chapter;
}
