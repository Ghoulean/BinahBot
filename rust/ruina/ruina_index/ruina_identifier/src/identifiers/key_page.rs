use ruina_common::game_objects::key_page::KeyPage;

use crate::Identifier;
use crate::PageType;
use crate::TypedId;

impl Identifier for &KeyPage<'_> {
    fn get_typed_id(&self) -> TypedId {
        TypedId(PageType::KeyPage, self.id.to_owned())
    }
}

impl Identifier for KeyPage<'_> {
    fn get_typed_id(&self) -> TypedId {
        (&self).get_typed_id()
    }
}
