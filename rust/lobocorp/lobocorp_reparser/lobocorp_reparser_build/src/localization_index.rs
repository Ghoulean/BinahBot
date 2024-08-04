use std::collections::HashMap;
use std::fs;

use lobocorp_common::localizations::common::Locale;

use roxmltree::Document;
use strum::IntoEnumIterator;

use crate::path::get_localized_equipment_file_path;
use crate::xml::get_nodes;
use crate::xml::get_unique_node;

#[derive(Debug, Eq, PartialEq, Hash)]
struct OwnedLocalizationKey(pub String, pub Locale);
 
pub fn write_localization_index() -> String {
    let hm = build_localization_index();
    let mut builder = phf_codegen::Map::new();
    for (key, value) in hm {
        let key_fmt = format!("{}#{:?}", key.0, key.1);
        builder.entry(key_fmt, &value);
    }
    format!(
        "static LOCALIZATION_INDEX: phf::Map<&'static str, &'static str> = {};",
        builder.build()
    )
}

fn build_localization_index() -> HashMap<OwnedLocalizationKey, String> {
    let mut hm: HashMap<OwnedLocalizationKey, String> = HashMap::new();

    Locale::iter().for_each(|locale| {
        let path = get_localized_equipment_file_path(&locale);
        let file_str = fs::read_to_string(path.as_path()).expect(&format!("cannot read {:?}", path));
        let doc: Document = Document::parse(&file_str).expect(&format!("failed parsing {:?}", path));

        let localize_root = get_unique_node(&doc.root(), "localize").expect("couldn't find localize");
        get_nodes(&localize_root, "text").iter().for_each(|x| {
            let id = x.attribute("id").expect("no id");

            // workaround to the fact that roxmltree parses &lt;i&gt; as a valid node
            let start = x.first_child().unwrap().range().start;
            let end = x.last_child().unwrap().range().end;
            let binding = doc.input_text()[start..end].to_string();
            let text = binding.trim();

            let localization_key = OwnedLocalizationKey(id.to_string(), locale.clone());
            hm.entry(localization_key).or_insert(format!("r#\"{}\"#", text));
        });
    });

    hm
}