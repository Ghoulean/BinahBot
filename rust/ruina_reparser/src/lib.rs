extern crate ruina_common;

use phf::Map;
use ruina_common::game_objects::abno_page::{Abno, AbnoPage, AbnoTargetting};
use ruina_common::game_objects::battle_symbol::{BattleSymbol, BattleSymbolSlot};
use ruina_common::game_objects::combat_page::{CombatPage, CombatRange, Die, DieType};
use ruina_common::game_objects::common::{Chapter, Floor, Rarity};
use ruina_common::game_objects::key_page::{KeyPage, KeyPageRange, KeyPageResists, Resistance};
use ruina_common::game_objects::passive::Passive;
use ruina_common::localizations::abno_page_locale::AbnoPageLocale;
use ruina_common::localizations::battle_symbol_locale::BattleSymbolLocale;
use ruina_common::localizations::card_effect_locale::CardEffectLocale;
use ruina_common::localizations::combat_page_locale::CombatPageLocale;
use ruina_common::localizations::common::Locale;
use ruina_common::localizations::key_page_locale::KeyPageLocale;
use ruina_common::localizations::passive_locale::PassiveLocale;

include!(concat!(env!("OUT_DIR"), "/out.rs"));

#[inline(always)]
pub fn get_abno_page_localizations_by_locale(locale: Locale) -> Option<&'static Map<&'static str, AbnoPageLocale<'static>>> {
    ABNO_PAGE_LOCALES.get(locale.to_string().as_str())
}

#[inline(always)]
pub fn get_card_effect_localizations_by_locale(locale: Locale) -> Option<&'static Map<&'static str, CardEffectLocale<'static>>> {
    CARD_EFFECT_LOCALES.get(locale.to_string().as_str())
}
 
#[inline(always)]
pub fn get_combat_page_localizations_by_locale(locale: Locale) -> Option<&'static Map<&'static str, CombatPageLocale<'static>>> {
    COMBAT_PAGE_LOCALES.get(locale.to_string().as_str())
}

#[inline(always)]
pub fn get_key_page_localizations_by_locale(locale: Locale) -> Option<&'static Map<&'static str, KeyPageLocale<'static>>> {
    KEY_PAGE_LOCALES.get(locale.to_string().as_str())
}

#[inline(always)]
pub fn get_passive_localizations_by_locale(locale: Locale) -> Option<&'static Map<&'static str, PassiveLocale<'static>>> {
    PASSIVE_LOCALES.get(locale.to_string().as_str())
}


#[inline(always)]
pub fn get_battle_symbol_localizations_by_locale(locale: Locale) -> Option<&'static Map<&'static str, BattleSymbolLocale<'static>>> {
    BATTLE_SYMBOL_LOCALES.get(locale.to_string().as_str())
}