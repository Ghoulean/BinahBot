import { Rarity } from "../enums";

export interface Passive {
    id: string;
    cost: number;
    scriptId: string;
    rarity: Rarity;
    canGivePassive: boolean;
}