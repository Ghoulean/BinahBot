export enum AbnoPageSelectType {
    SELECT_ONE = "SelectOne",
    ALL = "All",
    ALL_INCLUDING_ENEMY = "AllIncludingEnemy"
}

export function abnoPageSelectTypeFromStringValue(
    value: string
): AbnoPageSelectType | undefined {
    return (Object.values(AbnoPageSelectType) as unknown as string[]).includes(
        value
    )
        ? (value as unknown as AbnoPageSelectType)
        : undefined;
}
