use std::fmt;

use ruina_common::game_objects::common::PageType;

pub mod identifiers;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypedId(pub PageType, pub String);

pub trait Identifier {
    fn get_typed_id(&self) -> TypedId;
}

impl fmt::Display for TypedId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}{}", self.0, self.1))
    }
}
