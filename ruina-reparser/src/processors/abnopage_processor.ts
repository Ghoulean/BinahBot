import { AbnoPage, AbnoPageSelectType, Chapter, Floor, abnoPageSelectTypeFromStringValue } from '@ghoulean/ruina-common';
import { ABNO_DIR } from '../util/constants';
import { readFile, walkSync, writeDataFile } from '../util/file';

const ABNO_OUTPUT_FILE_NAME = "abno.json";

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
    public static process() {
        const data: AbnoPage[] = [];
        for (const filePath of walkSync(ABNO_DIR)) {
            const json: any = readFile(filePath);
            for (const card of json['EmotionCardXmlRoot']['EmotionCard']) {

                let emotionRate: number | undefined = card['EmotionRate']?.['_text'];
                if (emotionRate === undefined) {
                    emotionRate = 0;
                } else if (emotionRate === 0 && card['State']['_text'] === 'Negative') {
                    emotionRate = -0;
                }
                let emotionLevel: number | undefined = card['EmotionLevel']?.['_text'];
                if (emotionLevel === undefined) {
                    emotionLevel = 0;
                }
                const abnoName: string = card['Name']['_text'];

                const abnoPage: AbnoPage = {
                    nameId: abnoName,
                    floor: Floor[card['Sephirah']['_text'].toUpperCase() as keyof typeof Floor],
                    chapter: this.determineChapter(abnoName),
                    emoLevel: emotionLevel!,
                    emotionRate: emotionRate!,
                    targetType: abnoPageSelectTypeFromStringValue(card['TargetType']['_text'])!,
                    scriptId: card['Script']['_text']
                }
                data.push(abnoPage);
            }
        }
        writeDataFile(ABNO_OUTPUT_FILE_NAME, data);
    }

    private static determineChapter(abnoPageName: string): Chapter {
        return ABNO_TO_CHAPTER_MAP[abnoPageName.split('_')[0]];
    }
}