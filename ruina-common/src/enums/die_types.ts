import { Localization } from "./localization";

export enum DieType {
    BLUNT = "Atk, Hit",
    PIERCE = "Atk, Penetrate",
    SLASH = "Atk, Slash",
    BLOCK = "Def, Guard",
    EVADE = "Def, Evasion",
    BLUNT_COUNTER = "Standby, Hit",
    PIERCE_COUNTER = "Standby, Penetrate",
    SLASH_COUNTER = "Standby, Slash",
    BLOCK_COUNTER = "Standby, Guard",
    EVADE_COUNTER = "Standby, Evasion",
}

export function dieTypeFromStrings(
    type: string,
    detail: string
): DieType | undefined {
    const value: string = `${type}, ${detail}`;
    return (Object.values(DieType) as unknown as string[]).includes(value)
        ? (value as unknown as DieType)
        : undefined;
}

export const COUNTER_DIE: DieType[] = [
    DieType.BLUNT_COUNTER,
    DieType.PIERCE_COUNTER,
    DieType.SLASH_COUNTER,
    DieType.BLOCK_COUNTER,
    DieType.EVADE_COUNTER,
];

const localizationMap: { [key in DieType]: { [key in Localization]: string } } =
    {
        [DieType.BLUNT]: {
            [Localization.CHINESE_SIMPLIFIED]: "",
            [Localization.CHINESE_TRADITIONAL]: "",
            [Localization.ENGLISH]: "Blunt",
            [Localization.JAPANESE]: "",
            [Localization.KOREAN]: "",
        },
        [DieType.PIERCE]: {
            [Localization.CHINESE_SIMPLIFIED]: "",
            [Localization.CHINESE_TRADITIONAL]: "",
            [Localization.ENGLISH]: "Pierce",
            [Localization.JAPANESE]: "",
            [Localization.KOREAN]: "",
        },
        [DieType.SLASH]: {
            [Localization.CHINESE_SIMPLIFIED]: "",
            [Localization.CHINESE_TRADITIONAL]: "",
            [Localization.ENGLISH]: "Slash",
            [Localization.JAPANESE]: "",
            [Localization.KOREAN]: "",
        },
        [DieType.BLOCK]: {
            [Localization.CHINESE_SIMPLIFIED]: "",
            [Localization.CHINESE_TRADITIONAL]: "",
            [Localization.ENGLISH]: "Block",
            [Localization.JAPANESE]: "",
            [Localization.KOREAN]: "",
        },
        [DieType.EVADE]: {
            [Localization.CHINESE_SIMPLIFIED]: "",
            [Localization.CHINESE_TRADITIONAL]: "",
            [Localization.ENGLISH]: "Evade",
            [Localization.JAPANESE]: "",
            [Localization.KOREAN]: "",
        },
        [DieType.BLUNT_COUNTER]: {
            [Localization.CHINESE_SIMPLIFIED]: "",
            [Localization.CHINESE_TRADITIONAL]: "",
            [Localization.ENGLISH]: "C.Blunt",
            [Localization.JAPANESE]: "",
            [Localization.KOREAN]: "",
        },
        [DieType.PIERCE_COUNTER]: {
            [Localization.CHINESE_SIMPLIFIED]: "",
            [Localization.CHINESE_TRADITIONAL]: "",
            [Localization.ENGLISH]: "C.Pierce",
            [Localization.JAPANESE]: "",
            [Localization.KOREAN]: "",
        },
        [DieType.SLASH_COUNTER]: {
            [Localization.CHINESE_SIMPLIFIED]: "",
            [Localization.CHINESE_TRADITIONAL]: "",
            [Localization.ENGLISH]: "C.Slash",
            [Localization.JAPANESE]: "",
            [Localization.KOREAN]: "",
        },
        [DieType.BLOCK_COUNTER]: {
            [Localization.CHINESE_SIMPLIFIED]: "",
            [Localization.CHINESE_TRADITIONAL]: "",
            [Localization.ENGLISH]: "C.Block",
            [Localization.JAPANESE]: "",
            [Localization.KOREAN]: "",
        },
        [DieType.EVADE_COUNTER]: {
            [Localization.CHINESE_SIMPLIFIED]: "",
            [Localization.CHINESE_TRADITIONAL]: "",
            [Localization.ENGLISH]: "C.Evade",
            [Localization.JAPANESE]: "",
            [Localization.KOREAN]: "",
        },
    };

export function dieTypeToShortString(
    type: DieType,
    locale: Localization
): string {
    return localizationMap[type][locale];
}
