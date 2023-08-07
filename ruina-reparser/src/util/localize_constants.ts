import { Localization, PageType } from "@ghoulean/ruina-common";

export type LocalizationMapping = { [key in Localization]: string };

const ABNO_PAGE_LOCALIZATION: LocalizationMapping = {
    [Localization.KOREAN]: "",
    [Localization.JAPANESE]: "",
    [Localization.ENGLISH]: "abnormality page",
    [Localization.CHINESE_SIMPLIFIED]: "",
    [Localization.CHINESE_TRADITIONAL]: "",
};

const COMBAT_PAGE_LOCALIZATION: LocalizationMapping = {
    [Localization.KOREAN]: "",
    [Localization.JAPANESE]: "",
    [Localization.ENGLISH]: "combat page",
    [Localization.CHINESE_SIMPLIFIED]: "",
    [Localization.CHINESE_TRADITIONAL]: "",
};

const KEY_PAGE_LOCALIZATION: LocalizationMapping = {
    [Localization.KOREAN]: "",
    [Localization.JAPANESE]: "",
    [Localization.ENGLISH]: "key page",
    [Localization.CHINESE_SIMPLIFIED]: "",
    [Localization.CHINESE_TRADITIONAL]: "",
};

const PASSIVE_LOCALIZATION: LocalizationMapping = {
    [Localization.KOREAN]: "",
    [Localization.JAPANESE]: "",
    [Localization.ENGLISH]: "passive",
    [Localization.CHINESE_SIMPLIFIED]: "",
    [Localization.CHINESE_TRADITIONAL]: "",
};

export const CARD_LOCALIZATIONS: {
    [key in PageType]: LocalizationMapping;
} = {
    [PageType.ABNO_PAGE]: ABNO_PAGE_LOCALIZATION,
    [PageType.COMBAT_PAGE]: COMBAT_PAGE_LOCALIZATION,
    [PageType.KEY_PAGE]: KEY_PAGE_LOCALIZATION,
    [PageType.PASSIVE]: PASSIVE_LOCALIZATION,
    // Don't use this PageType, this is an ugly hack
    [PageType.DISAMBIGUATION]: ABNO_PAGE_LOCALIZATION
};
