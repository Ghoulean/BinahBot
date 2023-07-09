import { AbnoPageSelectType, Chapter, Floor, Localization } from "../enums";

export interface AbnoPage {
    nameId: string;
    floor: Floor;
    chapter: Chapter;
    emoLevel: number;
    emotionRate: number; // Negative are breakdown pages. -0 is valid and distinct from +0.
    targetType: AbnoPageSelectType;
    scriptId: string;
}

export interface DecoratedAbnoPage extends AbnoPage {
    locale: Localization;
    name: string;
    description: string;
    flavorText: string;
    imagePath: string;
}