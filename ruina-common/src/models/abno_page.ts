import { AbnoPageSelectType, Chapter, Floor } from "../enums";

export interface AbnoPage {
    nameId: string;
    floor: Floor;
    chapter: Chapter;
    emoLevel: number;
    emotionRate: number; // Negative are breakdown pages. -0 is valid and distinct from +0.
    targetType: AbnoPageSelectType;
    scriptId: string;
}
