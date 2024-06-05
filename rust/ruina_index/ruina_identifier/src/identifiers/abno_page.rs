use ruina_common::game_objects::abno_page::AbnoPage;

use crate::PageType;
use crate::TypedId;
use crate::Identifier;

impl Identifier for AbnoPage<'_> {
    fn get_typed_id(&self) -> TypedId {
        TypedId(PageType::AbnoPage, self.internal_name.to_owned())
    }
}
