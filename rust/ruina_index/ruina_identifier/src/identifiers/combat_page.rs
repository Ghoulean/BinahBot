use ruina_common::game_objects::combat_page::CombatPage;

use crate::PageType;
use crate::TypedId;
use crate::Identifier;

impl Identifier for &CombatPage<'_> {
    fn get_typed_id(&self) -> TypedId {
        TypedId(PageType::CombatPage, self.id.to_owned())
    }
}

impl Identifier for CombatPage<'_> {
    fn get_typed_id(&self) -> TypedId {
        (&self).get_typed_id()
    }
}