import { Chapter, DieType, Resistance } from "../enums";

export interface KeyPage {
    id: string;
    hp: number;
    stagger: number;
    hpResistances: KeyPageResistance;
    staggerResistances: KeyPageResistance;
    minBaseSpeed: number;
    maxBaseSpeed: number;
    baseLight: number;
    defaultPassiveIds: string[];
    exclusiveCardsIds: string[];
    chapter: Chapter;
}

export interface KeyPageResistance {
    [DieType.SLASH]: Resistance;
    [DieType.PIERCE]: Resistance;
    [DieType.BLUNT]: Resistance;
}

export interface DecoratedKeyPage extends KeyPage {
    name: string;
    passiveNames: string[];
}