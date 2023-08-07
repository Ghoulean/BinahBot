import { DieType } from "../enums";

export interface Die {
    type: DieType;
    min: number;
    max: number;
    scriptId: string;
}
