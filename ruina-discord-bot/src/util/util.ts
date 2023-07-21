import { Localization } from "@ghoulean/ruina-common";

const DISCORD_LOCALE_TO_LOCALIZATION_MAP: { [key: string]: Localization } = {
    ko: Localization.KOREAN,
    ja: Localization.JAPANESE,
    "zh-CN": Localization.CHINESE_SIMPLIFIED,
    "zh-TW": Localization.CHINESE_TRADITIONAL,
    "en-US": Localization.ENGLISH,
    "en-GB": Localization.ENGLISH,
};

export abstract class Util {
    public static deserializeDiscordLocale(
        discordLocale: string
    ): Localization {
        return (
            DISCORD_LOCALE_TO_LOCALIZATION_MAP[discordLocale] ??
            Localization.ENGLISH
        );
    }
}
