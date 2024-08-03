use lobocorp_common::game_objects::abnormality::BreachingEntity;
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

include!(concat!(env!("OUT_DIR"), "/out.rs"));

#[inline(always)]
pub fn get_encyclopedia_info(id: &u32) -> Option<&EncyclopediaInfo> {
    ENCYCLOPEDIA.get(id)
}
