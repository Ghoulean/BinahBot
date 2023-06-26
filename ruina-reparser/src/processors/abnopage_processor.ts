/* eslint-disable  @typescript-eslint/no-unsafe-assignment */
/* eslint-disable  @typescript-eslint/no-unsafe-member-access */

import { AbnoPage, AbnoPageSelectType, Chapter, Floor, abnoPageSelectTypeFromStringValue } from '@ghoulean/ruina-common';
import { ABNO_DIR } from '../util/constants';
import { readFile, walkSync } from '../util/file';

export const ABNO_OUTPUT_FILE_NAME = "abno.json";

const ABNO_TO_CHAPTER_MAP: { [key: string]: Chapter } = {
    // Enemy
    "Enemy": Chapter.CANARD,
    // Keter
    "Bloodbath": Chapter.CANARD,
    "HeartofAspiration": Chapter.URBAN_LEGEND,
    "Pinocchio": Chapter.URBAN_NIGHTMARE,
    "TheSnowQueen": Chapter.STAR_OF_THE_CITY,
    "QuietKid": Chapter.IMPURITAS_CIVITATIS, 
    // Malkuth
    "ScorchedGirl": Chapter.CANARD,
    "HappyTeddyBear": Chapter.URBAN_LEGEND,
    "FairyCarnival": Chapter.URBAN_PLAGUE,
    "QueenBee": Chapter.URBAN_NIGHTMARE,
    "SnowWhite": Chapter.URBAN_NIGHTMARE,
    // Yesod
    "ForsakenMurderer": Chapter.URBAN_MYTH,
    "LittleHelper": Chapter.URBAN_LEGEND,
    "SingingMachine": Chapter.URBAN_PLAGUE,
    "Butterfly": Chapter.URBAN_NIGHTMARE,
    "Matan": Chapter.URBAN_NIGHTMARE, 
    // Netzach
    "UniverseZogak": Chapter.URBAN_LEGEND,
    "ChildofGalaxy": Chapter.URBAN_LEGEND,
    "Porccubus": Chapter.URBAN_PLAGUE,
    "Alriune": Chapter.URBAN_NIGHTMARE,
    "SilentOrchestra": Chapter.URBAN_NIGHTMARE,
    // Hod
    "ShyLookToday": Chapter.URBAN_MYTH,
    "RedShoes": Chapter.URBAN_LEGEND,
    "SpiderBud": Chapter.URBAN_PLAGUE,
    "Laetitia": Chapter.URBAN_NIGHTMARE,
    "BlackSwan": Chapter.URBAN_NIGHTMARE, 
    // Tiphereth
    "QueenOfHatred": Chapter.URBAN_PLAGUE,
    "KnightOfDespair": Chapter.URBAN_PLAGUE,
    "Greed": Chapter.URBAN_NIGHTMARE,
    "Angry": Chapter.STAR_OF_THE_CITY,
    "NihilClown": Chapter.STAR_OF_THE_CITY,
    // Gebura
    "Redhood": Chapter.URBAN_NIGHTMARE,
    "BigBadWolf": Chapter.URBAN_NIGHTMARE,
    "Mountain": Chapter.STAR_OF_THE_CITY,
    "Nosferatu": Chapter.STAR_OF_THE_CITY,
    "NothingThere": Chapter.STAR_OF_THE_CITY,    
    // Chesed
    "ScareCrow": Chapter.URBAN_NIGHTMARE,
    "LumberJack": Chapter.URBAN_NIGHTMARE,
    "House": Chapter.STAR_OF_THE_CITY,
    "Ozma": Chapter.STAR_OF_THE_CITY,
    "Oz": Chapter.STAR_OF_THE_CITY,
    // Binah
    "Bigbird": Chapter.STAR_OF_THE_CITY,
    "SmallBird": Chapter.STAR_OF_THE_CITY,
    "LongBird": Chapter.STAR_OF_THE_CITY,
    "ApocalypseBird": Chapter.STAR_OF_THE_CITY,
    // Hokma
    "Bloodytree": Chapter.STAR_OF_THE_CITY,
    "Clock": Chapter.STAR_OF_THE_CITY,
    "BlueStar": Chapter.STAR_OF_THE_CITY,
    "WhiteNight": Chapter.IMPURITAS_CIVITATIS, 
}

export class AbnoPageProcessor {
    public static process(): AbnoPage[] {
        const data: AbnoPage[] = [];
        for (const filePath of walkSync(ABNO_DIR)) {
            // eslint-disable-next-line  @typescript-eslint/no-explicit-any
            const json: any = readFile(filePath);
            for (const card of json['EmotionCardXmlRoot']['EmotionCard']) {
                let emotionRate: number = card['EmotionRate']?.['_text'] ?? 0;
                if (emotionRate === 0 && card['State']?.['_text'] === 'Negative') {
                    emotionRate = -0;
                }
                const emotionLevel: number = card['EmotionLevel']?.['_text'] ?? 0;
                const abnoName: string = card['Name']?.['_text'] ?? '';
                const floorStr: string = (card['Sephirah']?.['_text'] as string ?? Floor.NONE.toString()).toUpperCase();
                const targetTypeStr: string = card['TargetType']?.['_text'] ?? '';

                const abnoPage: AbnoPage = {
                    nameId: abnoName,
                    floor: Floor[floorStr as keyof typeof Floor],
                    chapter: this.determineChapter(abnoName),
                    emoLevel: emotionLevel,
                    emotionRate: emotionRate,
                    targetType: abnoPageSelectTypeFromStringValue(targetTypeStr) ?? AbnoPageSelectType.SELECT_ONE,
                    scriptId: card['Script']?.['_text'] ?? ''
                }
                data.push(abnoPage);
            }
        }
        return data;
    }

    private static determineChapter(abnoPageName: string): Chapter {
        return ABNO_TO_CHAPTER_MAP[abnoPageName.split('_')[0]];
    }
}