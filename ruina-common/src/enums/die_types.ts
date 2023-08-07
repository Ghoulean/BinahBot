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