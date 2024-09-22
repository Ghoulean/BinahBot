use ruina_common::game_objects::abno_page::AbnoPage;

use crate::Identifier;
use crate::PageType;
use crate::TypedId;

impl Identifier for &AbnoPage<'_> {
    fn get_typed_id(&self) -> TypedId {
        TypedId(PageType::AbnoPage, self.internal_name.to_owned())
    }
}

impl Identifier for AbnoPage<'_> {
    fn get_typed_id(&self) -> TypedId {
        (&self).get_typed_id()
    }
}
