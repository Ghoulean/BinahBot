use std::collections::HashMap;

use phf::Map;
use ruina_common::game_objects::abno_page::Abno;
use ruina_common::game_objects::abno_page::AbnoPage;
use ruina_common::game_objects::abno_page::AbnoTargetting;
use ruina_common::game_objects::battle_symbol::BattleSymbol;
use ruina_common::game_objects::battle_symbol::BattleSymbolSlot;
use ruina_common::game_objects::combat_page::CombatPage;
use ruina_common::game_objects::combat_page::CombatRange;
use ruina_common::game_objects::combat_page::Die;
use ruina_common::game_objects::combat_page::DieType;
use ruina_common::game_objects::common::Chapter;
use ruina_common::game_objects::common::Collectability;
use ruina_common::game_objects::common::Floor;
use ruina_common::game_objects::common::Rarity;
use ruina_common::game_objects::key_page::KeyPage;
use ruina_common::game_objects::key_page::KeyPageRange;
use ruina_common::game_objects::key_page::KeyPageResists;
use ruina_common::game_objects::key_page::Resistance;
use ruina_common::game_objects::passive::Passive;
use ruina_common::localizations::abno_page_locale::AbnoPageLocale;
use ruina_common::localizations::battle_symbol_locale::BattleSymbolLocale;
use ruina_common::localizations::card_effect_locale::CardEffectLocale;
use ruina_common::localizations::combat_page_locale::CombatPageLocale;
use ruina_common::localizations::common::Locale;
use ruina_common::localizations::key_page_locale::KeyPageLocale;
use ruina_common::localizations::passive_locale::PassiveLocale;
use strum::IntoEnumIterator;

include!(concat!(env!("OUT_DIR"), "/out.rs"));

#[inline(always)]
pub fn get_abno_page_by_internal_name(internal_name: &str) -> Option<&'static AbnoPage<'static>> {
    ABNO_PAGES.get(internal_name)
}

#[inline(always)]
pub fn get_all_abno_pages() -> Vec<&'static AbnoPage<'static>> {
    ABNO_PAGES.values().collect()
}

#[inline(always)]
pub fn get_battle_symbol_by_internal_name(
    internal_name: &str,
) -> Option<&'static BattleSymbol<'static>> {
    BATTLE_SYMBOLS.get(internal_name)
}

#[inline(always)]
pub fn get_all_battle_symbols() -> Vec<&'static BattleSymbol<'static>> {
    BATTLE_SYMBOLS.values().collect()
}

#[inline(always)]
pub fn get_combat_page_by_id(id: &str) -> Option<&'static CombatPage<'static>> {
    COMBAT_PAGES.get(id)
}

#[inline(always)]
pub fn get_all_combat_pages() -> Vec<&'static CombatPage<'static>> {
    COMBAT_PAGES.values().collect()
}

#[inline(always)]
pub fn get_key_page_by_id(id: &str) -> Option<&'static KeyPage<'static>> {
    KEY_PAGES.get(id)
}

#[inline(always)]
pub fn get_all_key_pages() -> Vec<&'static KeyPage<'static>> {
    KEY_PAGES.values().collect()
}

#[inline(always)]
pub fn get_passive_by_id(id: &str) -> Option<&'static Passive<'static>> {
    PASSIVES.get(id)
}

#[inline(always)]
pub fn get_all_passives() -> Vec<&'static Passive<'static>> {
    PASSIVES.values().collect()
}

#[inline(always)]
pub fn get_abno_page_localizations_by_locale(
    locale: Locale,
) -> Option<&'static Map<&'static str, AbnoPageLocale<'static>>> {
    ABNO_PAGE_LOCALES.get(locale.to_string().as_str())
}

#[inline(always)]
pub fn get_abno_page_locales_by_internal_name(
    internal_name: &str,
) -> HashMap<Locale, &AbnoPageLocale> {
    get_locales_by_identifier(&ABNO_PAGE_LOCALES, internal_name)
}

#[inline(always)]
pub fn get_battle_symbol_localizations_by_locale(
    locale: Locale,
) -> Option<&'static Map<&'static str, BattleSymbolLocale<'static>>> {
    BATTLE_SYMBOL_LOCALES.get(locale.to_string().as_str())
}

#[inline(always)]
pub fn get_battle_symbol_locales_by_internal_name(
    internal_name: &str,
) -> HashMap<Locale, &BattleSymbolLocale> {
    get_locales_by_identifier(&BATTLE_SYMBOL_LOCALES, internal_name)
}

#[inline(always)]
pub fn get_card_effect_localizations_by_locale(
    locale: Locale,
) -> Option<&'static Map<&'static str, CardEffectLocale<'static>>> {
    CARD_EFFECT_LOCALES.get(locale.to_string().as_str())
}

#[inline(always)]
pub fn get_card_effect_locales_by_id(id: &str) -> HashMap<Locale, &CardEffectLocale> {
    get_locales_by_identifier(&CARD_EFFECT_LOCALES, id)
}

#[inline(always)]
pub fn get_combat_page_localizations_by_locale(
    locale: Locale,
) -> Option<&'static Map<&'static str, CombatPageLocale<'static>>> {
    COMBAT_PAGE_LOCALES.get(locale.to_string().as_str())
}

#[inline(always)]
pub fn get_combat_page_locales_by_id(id: &str) -> HashMap<Locale, &CombatPageLocale> {
    get_locales_by_identifier(&COMBAT_PAGE_LOCALES, id)
}

#[inline(always)]
pub fn get_key_page_localizations_by_locale(
    locale: Locale,
) -> Option<&'static Map<&'static str, KeyPageLocale<'static>>> {
    KEY_PAGE_LOCALES.get(locale.to_string().as_str())
}

#[inline(always)]
pub fn get_key_page_locales_by_text_id(text_id: &str) -> HashMap<Locale, &KeyPageLocale> {
    get_locales_by_identifier(&KEY_PAGE_LOCALES, text_id)
}

#[inline(always)]
pub fn get_passive_localizations_by_locale(
    locale: Locale,
) -> Option<&'static Map<&'static str, PassiveLocale<'static>>> {
    PASSIVE_LOCALES.get(locale.to_string().as_str())
}

#[inline(always)]
pub fn get_passive_locales_by_id(id: &str) -> HashMap<Locale, &PassiveLocale> {
    get_locales_by_identifier(&PASSIVE_LOCALES, id)
}

#[inline(always)]
fn get_locales_by_identifier<T>(
    locale_mapping: &'static phf::Map<&'static str, phf::Map<&'static str, T>>,
    id: &str,
) -> HashMap<Locale, &'static T> {
    Locale::iter()
        .map(|locale| {
            (
                locale.clone(),
                locale_mapping
                    .get(locale.to_string().as_str())
                    .unwrap(),
            )
        })
        .filter(|locale_map| locale_map.1.get(id).is_some())
        .map(|locale_map| (locale_map.0, locale_map.1.get(id).unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUM_LOCALES: usize = 5;

    #[test]
    fn game_objects_count_sanity_check() {
        assert_eq!(ABNO_PAGES.len(), 156);
        assert_eq!(BATTLE_SYMBOLS.len(), 200);
        assert_eq!(COMBAT_PAGES.len(), 1613);
        assert_eq!(KEY_PAGES.len(), 625);
        assert_eq!(PASSIVES.len(), 808);
    }

    #[test]
    fn abno_page_locales_map_sanity_check() {
        ABNO_PAGES.values().for_each(|abno_page| {
            let locales = get_abno_page_locales_by_internal_name(abno_page.internal_name);
            let locale_count = locales.values().len();
            assert!(locale_count == NUM_LOCALES || locale_count == 0);
        });
    }

    #[test]
    fn battle_symbol_locales_map_sanity_check() {
        BATTLE_SYMBOLS.values().for_each(|battle_symbol| {
            let locales = get_battle_symbol_locales_by_internal_name(battle_symbol.internal_name);
            let locale_count = locales.values().len();
            assert!(locale_count == NUM_LOCALES || locale_count == 0);
        });
    }

    #[test]
    fn combat_page_locales_map_sanity_check() {
        let exceptions = HashMap::from([
            ("513001", 1),  // Unused 7 association card
            ("610007", 2),  // Unused Xiao(?) card
            ("703526", 4),  // Unused(?) copy of Horrendous Pitch (Ms Mermaid) but no JP locale
            ("703923", 4),  // Unused Shade card (Pluto)
            ("1100008", 4), // Unused(?) Nihil card in Tiph realization
        ]);

        COMBAT_PAGES.values().for_each(|combat_page| {
            let locales = get_combat_page_locales_by_id(combat_page.id);
            let locale_count = locales.values().len();
            if let Some(exception_count) = exceptions.get(combat_page.id) {
                assert_eq!(exception_count.clone(), locale_count);
            } else {
                assert!(locale_count == NUM_LOCALES || locale_count == 0);
            }
        });
    }

    #[test]
    fn key_page_locales_map_sanity_check() {
        // None of the units in Keter realization have a CN nor TRCN localization
        let exceptions = HashMap::from([
            ("9100511", 3), // Wrist Cutter
            ("9100521", 3), // Aspiration
            ("9100522", 3), // Lung of Craving
            ("9100531", 3), // Marionette
            ("9100532", 3), // Pinnochio
            ("9100541", 3), // Frost Splinter
            ("9100542", 3), // Prison of Ice
            ("9100551", 3), // Remorse
            ("9100552", 3), // Nail
            ("9100553", 3), // Hammer
        ]);

        KEY_PAGES
            .values()
            .filter(|x| x.text_id.is_some())
            .for_each(|key_page| {
                let locales = get_key_page_locales_by_text_id(key_page.text_id.unwrap());
                let locale_count = locales.values().len();
                if let Some(exception_count) = exceptions.get(key_page.id) {
                    assert_eq!(exception_count.clone(), locale_count);
                } else {
                    assert!(locale_count == NUM_LOCALES || locale_count == 0);
                }
            });
    }

    #[test]
    fn passive_locales_map_sanity_check() {
        let exceptions = HashMap::from([
            ("240010", 4),  // Acrobatic (Noah) has no TRCN localization
            ("180001", 4),  // An Arbiter (Zena) has no TRCN localization
            ("180002", 4),  // A Claw (Barel) has no TRCN
            ("402013", 4), // Parting Tears (Child of the Galaxy's slime from Netz 2) has no TRCN localization
            ("1005416", 1), // Used but empty passive from Frost Splinter (Keter realization)
        ]);

        PASSIVES.values().for_each(|passive| {
            let locales = get_passive_locales_by_id(passive.id);
            let locale_count = locales.values().len();
            if let Some(exception_count) = exceptions.get(passive.id) {
                assert_eq!(exception_count.clone(), locale_count);
            } else {
                assert!(locale_count == NUM_LOCALES || locale_count == 0);
            }
        });
    }

    #[test]
    fn test_get_script_without_locale() {
        let script_id = "final_scorchedgirl_fourthfire";
        let result = get_card_effect_locales_by_id(script_id);

        assert_eq!(result.values().len(), 0);
    }

}
