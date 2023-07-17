import { AbnoPageSelectType, Chapter, Floor, Localization } from "../enums";

export interface AbnoPage {
    nameId: string;
    floor: Floor;
    chapter: Chapter;
    emoLevel: number;
    emotionSign: -1 | 1; // -1 for breakdown, 1 for awakening
    emotionRate: number; // Negative are breakdown pages. -0 is encoded as 0
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
