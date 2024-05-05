use std::collections::HashMap;

use crate::taggers::tagger::Tag;
use crate::taggers::tagger::TypedId;

#[derive(Clone, Debug)]
pub struct Index(pub HashMap<TypedId, Vec<Tag>>);

impl Index {
    pub fn merge(&self, other: Index) -> Index {
        Index(self.0.clone().into_iter().chain(other.0).collect())
    }
}