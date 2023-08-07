export enum Rarity {
    PAPERBACK = "Common",
    HARDCOVER = "Uncommon",
    LIMITED = "Rare",
    OBJET_D_ART = "Unique",
}

export function rarityFromStringValue(value: string): Rarity | undefined {
    return (Object.values(Rarity) as unknown as string[]).includes(value)
        ? (value as unknown as Rarity)
        : undefined;
}
