import { Rarity } from "../enums";

export interface Passive {
    id: string;
    cost: number;
    rarity: Rarity;
    canGivePassive: boolean;
}

export interface DecoratedPassive extends Passive {
    name: string;
    description: string;
}