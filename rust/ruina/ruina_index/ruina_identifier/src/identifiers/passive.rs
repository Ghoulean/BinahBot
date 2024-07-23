use ruina_common::game_objects::passive::Passive;

use crate::PageType;
use crate::TypedId;
use crate::Identifier;

impl Identifier for &Passive<'_> {
    fn get_typed_id(&self) -> TypedId {
        TypedId(PageType::Passive, self.id.to_owned())
    }
}

impl Identifier for Passive<'_> {
    fn get_typed_id(&self) -> TypedId {
        (&self).get_typed_id()
    }
}