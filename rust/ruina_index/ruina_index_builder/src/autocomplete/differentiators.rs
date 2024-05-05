
use crate::taggers::tagger::TypedId;

use super::models::DisambiguationDisplay;

pub fn typed_id_disambiguator(typed_id: &TypedId) -> Option<DisambiguationDisplay> {
    Some(DisambiguationDisplay(format!("{}", typed_id.clone())))
}
