use ruina_common::game_objects::battle_symbol::BattleSymbol;

use crate::Identifier;
use crate::PageType;
use crate::TypedId;

impl Identifier for &BattleSymbol<'_> {
    fn get_typed_id(&self) -> TypedId {
        TypedId(PageType::BattleSymbol, self.internal_name.to_owned())
    }
}

impl Identifier for BattleSymbol<'_> {
    fn get_typed_id(&self) -> TypedId {
        (&self).get_typed_id()
    }
}
