import { Localization } from "@ghoulean/ruina-common";

export abstract class TransformerUtil {
    public static deserializeDiscordLocale(discordLocale: string): Localization {
        switch (discordLocale) {
            case "ko":
                return Localization.KOREAN;
            case "ja":
                return Localization.JAPANESE;
            case "zh-CN":
                return Localization.CHINESE_SIMPLIFIED;
            case "zh-TW":
                return Localization.CHINESE_TRADITIONAL;
            case "en-US":
            case "en-GB":
            default:
                return Localization.ENGLISH;
        }
    }
}
