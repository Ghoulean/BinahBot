export enum Range {
    MELEE = "Near",
    RANGE = "Far",
    MASS_INDIVIDUAL = "FarAreaEach",
    MASS_SUMMATION = "FarArea",
    ON_PLAY = "Immediate",
    SPECIAL = "Special"
}

export function rangeFromStringValue(value: string): Range | undefined {
    return (Object.values(Range) as unknown as string[]).includes(value)
        ? (value as unknown as Range)
        : undefined;
}
