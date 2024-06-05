use ruina_common::game_objects::battle_symbol::BattleSymbol;

use crate::PageType;
use crate::TypedId;
use crate::Identifier;

impl Identifier for BattleSymbol<'_> {
    fn get_typed_id(&self) -> TypedId {
        TypedId(PageType::BattleSymbol, self.internal_name.to_owned())
    }
}
