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
use lobocorp_common::game_objects::equipment::WeaponDamageType;
use lobocorp_common::game_objects::equipment::WeaponRange;
use lobocorp_common::localizations::abnormality::BreachingEntityLocalization;
use lobocorp_common::localizations::abnormality::EncyclopediaInfoLocalization;
use lobocorp_common::localizations::common::Locale;
use lobocorp_common::localizations::equipment::LocalizationKey;

include!(concat!(env!("OUT_DIR"), "/out.rs"));

pub fn get_all_encyclopedia_ids() -> Vec<&'static u32> {
    ENCYCLOPEDIA.keys().collect()
}

pub fn get_encyclopedia_info(id: &u32) -> Option<&EncyclopediaInfo> {
    ENCYCLOPEDIA.get(id)
}

pub fn get_localization<'a>(key: &'a LocalizationKey<'a>) -> Option<&'a &'static str> {
    LOCALIZATION_INDEX.get(&&format!("{}#{:?}", key.0, key.1))
}

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
            "*Behold: you stood at the door and knocked, and it was opened to you.*\n*I come from the end, and I am here to stay for but a moment.*\n*At the same time, I am the one who kindled the light to face the world.*\n*My loved ones, who now eagerly desire the greater gifts; I will show you the most excellent way.*",
            *result
        );
    }

    #[test]
    fn sanity_whitenight_display_names() {
        let whitenight = get_abno_localization(&100015, &Locale::English).unwrap();
        let names = whitenight.breaching_entity_localizations.into_iter().map(|x| {
            x.name    
        });
        let expected = ["WhiteNight", "Scythe Apostle", "Guardian Apostle", "Staff Apostle", "Spear Apostle"];
        names.enumerate().for_each(|(i, x)| {
            assert_eq!(expected[i], x);
        });
    }

    #[test]
    fn sanity_censored_correct_breaching_number() {
        let censored = get_encyclopedia_info(&100056).unwrap();
        let censored = match censored {
            EncyclopediaInfo::Normal(x) => x,
            _ => unreachable!()
        };
        assert_eq!(2, censored.breaching_entities.len());
    }

    #[test]
    fn sanity_nothing_there_correct_breaching_number() {
        let nothing_there = get_encyclopedia_info(&100005).unwrap();
        let nothing_there = match nothing_there {
            EncyclopediaInfo::Normal(x) => x,
            _ => unreachable!()
        };
        // dog, egg, man
        assert_eq!(3, nothing_there.breaching_entities.len());
    }

    #[test]
    fn sanity_yang_correct_breaching_number() {
        let yang = get_encyclopedia_info(&300109).unwrap();
        let yang = match yang {
            EncyclopediaInfo::Tool(x) => x,
            _ => unreachable!()
        };
        assert_eq!(1, yang.breaching_entities.len());
    }

    #[test]
    fn sanity_shelter_has_all_guidances() {
        let shelter = get_abno_localization(&300006, &Locale::English).unwrap();
        shelter.managerial_guidances.iter().for_each(|x| {
            assert!(!x.trim().is_empty());    
        });
        shelter.story.iter().for_each(|x| {
            assert!(!x.trim().is_empty());    
        });
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
                    dbg!(&x.name);
                    hs.insert(x.name);
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

    #[test]
    fn special_obtain_equipment_have_correct_cost_and_observability() {
        let firebird = get_encyclopedia_info(&100061).expect("no firebird entry");
        let firebird = match firebird {
            EncyclopediaInfo::Normal(x) => x,
            _ => panic!()
        };
        let feather_of_honor = firebird.weapon.as_ref().expect("no firebird weapon");
        assert_eq!(None, feather_of_honor.cost);
        assert_eq!(None, feather_of_honor.observation_level);

        let wn = get_encyclopedia_info(&100015).expect("no whitenight entry");
        let wn = match wn {
            EncyclopediaInfo::Normal(x) => x,
            _ => panic!()
        };
        let paradise_lost = wn.weapon.as_ref().expect("no whitenight weapon");
        assert_eq!(None, paradise_lost.cost);
        assert_eq!(None, paradise_lost.observation_level);

        let snow_queen = get_encyclopedia_info(&100102).expect("no snow queen entry");
        let snow_queen = match snow_queen {
            EncyclopediaInfo::Normal(x) => x,
            _ => panic!()
        };
        let kiss = snow_queen.gifts.get(0).expect("no snow queen gift");
        assert_eq!(None, kiss.obtain_probability);
        assert_eq!(None, kiss.observation_level);

        let bbw = get_encyclopedia_info(&100033).expect("no bigbadwolf entry");
        let bbw = match bbw {
            EncyclopediaInfo::Normal(x) => x,
            _ => panic!()
        };
        let sheepskin = bbw.gifts.get(1).expect("no bigbadwolf gift");
        assert_eq!(None, sheepskin.obtain_probability);
        assert_eq!(None, sheepskin.observation_level);

        let apobird = get_encyclopedia_info(&100038).expect("no apo bird entry");
        let apobird = match apobird {
            EncyclopediaInfo::Normal(x) => x,
            _ => panic!()
        };
        let wing = apobird.gifts.get(0).expect("no apo bird gift");
        assert_eq!(None, wing.obtain_probability);
        assert_eq!(None, wing.observation_level);
    }
}