import { Localization, PageType } from "@ghoulean/ruina-common";

export type LocalizationMapping = { [key in Localization]: string };

// TODO: Localize
const ABNO_PAGE_LOCALIZATION: LocalizationMapping = {
    [Localization.KOREAN]: "abnormality page",
    [Localization.JAPANESE]: "abnormality page",
    [Localization.ENGLISH]: "abnormality page",
    [Localization.CHINESE_SIMPLIFIED]: "abnormality page",
    [Localization.CHINESE_TRADITIONAL]: "abnormality page",
};

const COMBAT_PAGE_LOCALIZATION: LocalizationMapping = {
    [Localization.KOREAN]: "combat page",
    [Localization.JAPANESE]: "combat page",
    [Localization.ENGLISH]: "combat page",
    [Localization.CHINESE_SIMPLIFIED]: "combat page",
    [Localization.CHINESE_TRADITIONAL]: "combat page",
};

const KEY_PAGE_LOCALIZATION: LocalizationMapping = {
    [Localization.KOREAN]: "key page",
    [Localization.JAPANESE]: "key page",
    [Localization.ENGLISH]: "key page",
    [Localization.CHINESE_SIMPLIFIED]: "key page",
    [Localization.CHINESE_TRADITIONAL]: "key page",
};

const PASSIVE_LOCALIZATION: LocalizationMapping = {
    [Localization.KOREAN]: "passive",
    [Localization.JAPANESE]: "passive",
    [Localization.ENGLISH]: "passive",
    [Localization.CHINESE_SIMPLIFIED]: "passive",
    [Localization.CHINESE_TRADITIONAL]: "passive",
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
