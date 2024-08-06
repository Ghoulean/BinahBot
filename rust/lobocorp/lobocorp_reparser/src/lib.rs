use lobocorp_common::game_objects::abnormality::BreachingEntity;
use lobocorp_common::game_objects::abnormality::DontTouchMeInfo;
use lobocorp_common::game_objects::abnormality::EncyclopediaInfo;
use lobocorp_common::game_objects::abnormality::NormalInfo;
use lobocorp_common::game_objects::abnormality::ToolInfo;
use lobocorp_common::game_objects::abnormality::ToolType;
use lobocorp_common::game_objects::abnormality::WorkProbabilities;
use lobocorp_common::game_objects::common::DamageRange;
use lobocorp_common::game_objects::common::DamageType;
use lobocorp_common::game_objects::common::Defenses;
use lobocorp_common::game_objects::common::Resistance;
use lobocorp_common::game_objects::common::RiskLevel;
use lobocorp_common::game_objects::common::Stat;
use lobocorp_common::game_objects::common::StatBonus;
use lobocorp_common::game_objects::equipment::EquipRequirement;
use lobocorp_common::game_objects::equipment::EquipRequirementKey;
use lobocorp_common::game_objects::equipment::Gift;
use lobocorp_common::game_objects::equipment::Slot;
use lobocorp_common::game_objects::equipment::Suit;
use lobocorp_common::game_objects::equipment::Weapon;
use lobocorp_common::game_objects::equipment::WeaponAttackSpeed;
use lobocorp_common::game_objects::equipment::WeaponRange;
use lobocorp_common::localizations::abnormality::BreachingEntityLocalization;
use lobocorp_common::localizations::abnormality::EncyclopediaInfoLocalization;
use lobocorp_common::localizations::common::Locale;
use lobocorp_common::localizations::equipment::LocalizationKey;

include!(concat!(env!("OUT_DIR"), "/out.rs"));

#[inline(always)]
pub fn get_all_encyclopedia_ids() -> Vec<&'static u32> {
    ENCYCLOPEDIA.keys().collect()
}

#[inline(always)]
pub fn get_encyclopedia_info(id: &u32) -> Option<&EncyclopediaInfo> {
    ENCYCLOPEDIA.get(id)
}

#[inline(always)]
pub fn get_localization<'a>(key: &'a LocalizationKey<'a>) -> Option<&'a &'static str> {
    LOCALIZATION_INDEX.get(&&format!("{}#{:?}", key.0, key.1))
}

#[inline(always)]
pub fn get_abno_localization<'a>(id: &'a u32, locale: &'a Locale) -> Option<&'a EncyclopediaInfoLocalization<'a>> {
    ABNO_LOCALIZATIONS.get(format!("{}#{:?}", id, locale).as_str())
}

#[cfg(test)]
mod tests {
    use lobocorp_common::localizations::common::Locale;

    use super::*;

    #[test]
    fn sanity_get_encyclopedia_info() {
        let _does_not_crash = get_encyclopedia_info(&100038);
    }

    #[test]
    fn sanity_get_localization() {
        let paradise_lost_desc = LocalizationKey("DeathAngel_weapon_desc", Locale::English);
        let result = get_localization(&paradise_lost_desc).expect("couldn't find");

        assert_eq!(
            "&lt;i&gt;Behold: you stood at the door and knocked, and it was opened to you.&lt;/i&gt;&#13;&#10;&lt;i&gt;I come from the end, and I am here to stay for but a moment.&lt;/i&gt;&#13;&#10;&lt;i&gt;At the same time, I am the one who kindled the light to face the world.&lt;/i&gt;&#13;&#10;&lt;i&gt;My loved ones, who now eagerly desire the greater gifts; I will show you the most excellent way.&lt;/i&gt;",
            *result
        );
    }
}