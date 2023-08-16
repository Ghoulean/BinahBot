export enum Resistance {
    FATAL = 2.0,
    WEAK = 1.5,
    NORMAL = 1.0,
    ENDURED = 0.5,
    INEFFECTIVE = 0.25,
    IMMUNE = 0,
}

export function resistanceFromString(str: string): Resistance | undefined {
    switch (str) {
        case "Weak":
            return Resistance.FATAL;
        case "Vulnerable":
            return Resistance.WEAK;
        case "Normal":
            return Resistance.NORMAL;
        case "Endure":
            return Resistance.ENDURED;
        case "Resist":
            return Resistance.INEFFECTIVE;
        case "Immune":
            return Resistance.IMMUNE;
        default:
            return undefined;
    }
}
