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
    use std::collections::HashSet;

    use strum::IntoEnumIterator;
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
            "<i>Behold: you stood at the door and knocked, and it was opened to you.</i>\n<i>I come from the end, and I am here to stay for but a moment.</i>\n<i>At the same time, I am the one who kindled the light to face the world.</i>\n<i>My loved ones, who now eagerly desire the greater gifts; I will show you the most excellent way.</i>",
            *result
        );
    }

    #[test]
    #[ignore]
    fn sanity_all_child_abnos_have_name() {
        Locale::iter().for_each(|locale| {
            get_all_encyclopedia_ids().iter().for_each(|x| {
                dbg!(&x);
                let binding = get_abno_localization(&x, &locale).expect("bad id-locale pair");
                dbg!(&binding.name);

                let mut hs = HashSet::new();
                let mut count = 0;

                binding.breaching_entity_localizations.iter().for_each(|x| {
                    count += 1;
                    hs.insert(format!("{}#{}", x.name, x.code));
                });

                assert!(hs.len() == count);
            });
        });
    }


    #[test]
    fn correct_breachability_display() {
        let non_breachable = [
            100053, 100009, 100028, 100014, 100007, 100013, 100027,
            100037, 100059, 100103, 100002, 100041, 100017, 100045
        ];
        let breaching_no_defenses = [
            100005, 100019
        ];
        let breaching_with_defenses = [
            100015, 100018, 100036, 100054, 100043, 100011, 100057,
            100029, 100033, 100008, 100035, 100055, 100061, 100038,
            100058, 100064, 100056, 100063, 100042
        ];

        non_breachable.iter().for_each(|x| {
            let encyclopedia_info = get_encyclopedia_info(&x).expect("invalid id");
            let info = match encyclopedia_info {
                EncyclopediaInfo::Normal(x) => x,
                _ => panic!()
            };
            assert!(!info.is_breachable);
            assert!(info.defenses.is_none());
        });
        breaching_no_defenses.iter().for_each(|x| {
            let encyclopedia_info = get_encyclopedia_info(&x).expect("invalid id");
            let info = match encyclopedia_info {
                EncyclopediaInfo::Normal(x) => x,
                _ => panic!()
            };
            assert!(info.is_breachable);
            assert!(info.defenses.is_none());
        });
        breaching_with_defenses.iter().for_each(|x| {
            let encyclopedia_info = get_encyclopedia_info(&x).expect("invalid id");
            let info = match encyclopedia_info {
                EncyclopediaInfo::Normal(x) => x,
                _ => panic!()
            };
            assert!(info.is_breachable);
            assert!(info.defenses.is_some());
        })
    }
}