use std::collections::HashMap;

use crate::taggers::tagger::TypedId;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AmbiguousAutocomplete(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DisambiguationDisplay(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DisambiguatedAutocomplete(pub AmbiguousAutocomplete, pub Option<DisambiguationDisplay>);

#[derive(Clone)]
pub struct AmbiguousAutocompleteMap(pub HashMap<TypedId, DisambiguatedAutocomplete>);

#[derive(Clone)]
pub struct IncompleteAutocompleteMap(pub HashMap<AmbiguousAutocomplete, AmbiguousAutocompleteMap>);
