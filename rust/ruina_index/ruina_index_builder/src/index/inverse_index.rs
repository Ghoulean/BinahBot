use std::collections::HashMap;

use crate::taggers::tagger::Tag;
use crate::taggers::tagger::TypedId;

use super::index::Index;

pub struct InverseIndex(pub HashMap<Tag, Vec<TypedId>>);

impl InverseIndex {
    pub fn from_index(index: Index) -> InverseIndex {
        let Index(index_map) = index;
        let mut inverse_index: HashMap<Tag, Vec<TypedId>> = HashMap::new();
        index_map.iter().for_each(|(key, tags)| {
            dbg!("inverting index for key=", key);
            tags.iter().for_each(|tag| {
                let inverse_index_entry = inverse_index.entry(tag.clone()).or_insert(Vec::new());
                if !inverse_index_entry.contains(key) {
                    inverse_index_entry.push(key.clone());
                }
            })
        });
        InverseIndex(inverse_index)
    }

    pub fn to_serialized_phf_map(&self, var_name: &str) -> String {
        let mut builder = phf_codegen::Map::new();
        for (key, vec) in &self.0 {
            builder.entry(
                key.0.clone(),
                format!(
                    "&[{}]",
                    vec.iter()
                        .map(|x| format!("{:?}", x))
                        .collect::<Vec<String>>()
                        .join(",")
                )
                .as_str(),
            );
        }
        format!(
            "static {}: phf::Map<&'static str, &[TypedId<'_>]> = {};",
            var_name,
            builder.build()
        )
    }
}
