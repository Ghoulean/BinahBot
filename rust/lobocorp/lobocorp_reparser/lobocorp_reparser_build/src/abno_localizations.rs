use std::collections::HashMap;
use std::fs;
use std::iter;

use lobocorp_common::localizations::common::Locale;
use roxmltree::Document;
use strum::IntoEnumIterator;

use crate::abnormalities::PartialBreachingEntity;
use crate::abnormalities::PartialEncyclopediaInfo;
use crate::list::ListEntry;
use crate::path::get_localized_abno_file_path;
use crate::serde::display_serializer;
use crate::serde::serialize_option;
use crate::serde::write_vec;
use crate::xml::find_unique_node_with_name_and_attribute;
use crate::xml::get_nodes;
use crate::xml::get_nodes_text;
use crate::xml::get_unique_node;
use crate::xml::get_unique_node_text;

pub fn write_abno_localizations(list_entries: &[ListEntry], partial_abnos: &HashMap<ListEntry, PartialEncyclopediaInfo>) -> String {
    let mut builder = phf_codegen::Map::new();

    Locale::iter().for_each(|x| {
        let hm = build_abno_localizations(list_entries, partial_abnos, &x);

        for (key, value) in hm {
            let key_fmt = format!("{}#{:?}", key, x);
            builder.entry(key_fmt, &value);
        }
    });
    format!(
        "static ABNO_LOCALIZATIONS: phf::Map<&'static str, EncyclopediaInfoLocalization> = {};",
        builder.build()
    )
}

fn build_abno_localizations(list_entries: &[ListEntry], partial_abnos: &HashMap<ListEntry, PartialEncyclopediaInfo>, locale: &Locale) -> HashMap<u32, String> {
    list_entries.iter().map(|x| {
        let base_name = if x.id == 100009 {
            // inconsistent capitalization
            "Onebadmanygood"
        } else {
            &x.src
        };
        // yggdrasil, il pianto della Luna, clouded monk, ppodae, army in pink cn tr doesn't exist
        let locale = if *locale == Locale::ChineseTraditional && (
            x.id == 100062 ||
            x.id == 100065 ||
            x.id == 100105 ||
            x.id == 100106 ||
            x.id == 100064 ||
            x.id == 300108
        ) {
            &Locale::Chinese
        } else {
            locale
        };

        let path = get_localized_abno_file_path(&locale, base_name);
        let file_str = fs::read_to_string(path.as_path()).expect(&format!("cannot read {:?}", path));
        let doc: Document = Document::parse(&file_str).expect(&format!("failed parsing {:?}", path));

        let partial_info = partial_abnos.get(x).expect("tried to get nonexistent abno");

        (x.id, build_localization(x, &partial_info, &doc))
    }).collect()
}

fn build_localization(entry: &ListEntry, partial_info: &PartialEncyclopediaInfo, doc: &Document) -> String {
    let creature_node = get_unique_node(&doc.root(), "creature").expect("no creature node");
    let observe_node = get_unique_node(&creature_node, "observe").expect("no observe node");
    let collection_node = get_unique_node(&observe_node, "collection").expect("no collection node");

    let id = entry.id;
    let name = get_nodes_text(&collection_node, "name").last().map(|x| format_story_data(x.trim())).expect("couldn't get name");
    let other_names = get_nodes_text(&collection_node, "name").iter().rev().skip(1).rev().map(|x| format_story_data(x.trim())).collect::<Vec<_>>();
    let code = get_unique_node_text(&collection_node, "codeNo").map(|x| x.trim()).expect("couldn't get code");
    let selection_text = serialize_option(
        &get_unique_node_text(&collection_node, "openText").map(|x| format_story_data(x.trim()).to_string()).ok(),
        display_serializer
    );
    let special_tips = get_unique_node(&observe_node, "specialTipSize")
        .map(|x| get_nodes(&x, "specialTip"))
        .expect("couldn't get special tips");
    let managerial_guidances = write_vec(
        &special_tips.iter()
            .map(|x| format_story_data(x.text().expect("no text")))
            .collect::<Vec<_>>());
    let story = write_vec(
        &get_nodes(&observe_node, "desc").iter()
            .map(|x| format_story_data(x.text().expect("no text")))
            .collect::<Vec<_>>()
    );

    let breaching_entities = match partial_info {
        PartialEncyclopediaInfo::Normal(x) => Some(&x.breaching_entities),
        PartialEncyclopediaInfo::Tool(_) => None,
        PartialEncyclopediaInfo::DontTouchMe(_) => None,
    };

    // apo bird egg localizations not in standard location
    // todo: override for whitenight apostles
    let default_name_code_vec: Vec<(String, String)> = if id == 100038 {
        let etc_node = get_unique_node(&creature_node, "etc").expect("no etc node");
        let binding = vec!["gateway", "bigBirdEgg", "longBirdEgg", "smallBirdEgg"];
        let eggs = binding.iter().map(|x| {
            find_unique_node_with_name_and_attribute(&etc_node, "param", "key", x).ok()
                .and_then(|x| x.text())
                .map(|x| x.trim())
                .map(|x| format_story_data(x))
                .expect(&format!("no name for {}", x))
                .to_string()
        }).map(|x| {
            (x, "".to_string())  
        });
        iter::once((name.clone(), code.to_string())).chain(eggs).collect::<Vec<_>>()
    } else {
        let len = breaching_entities.map(|x| x.len()).unwrap_or(0);
        vec![(name.clone(), code.to_string())].into_iter().cycle().take(len).collect()
    };

    let breaching_entity_localizations = breaching_entities.map(|x| x.iter().enumerate().map(|(i, x)| {
        let (default_name, default_code) = default_name_code_vec.get(i).unwrap();
        build_breaching_entity_localization(x, &default_name, &default_code, doc)
    }).collect::<Vec<_>>()).map(|x| {
        write_vec(&x)  
    }).unwrap_or("[]".to_string());

    format!("EncyclopediaInfoLocalization {{
        id: \"{id}\",
        name: {name},
        other_names: &{other_names:?},
        code: \"{code}\",
        selection_text: {selection_text},
        managerial_guidances: &{managerial_guidances},
        story: &{story},
        breaching_entity_localizations: &{breaching_entity_localizations},
    }}")
}

fn build_breaching_entity_localization(
    breaching_entity: &PartialBreachingEntity,
    parent_name: &str,
    parent_code: &str,
    doc: &Document
) -> String {
    let creature_node = get_unique_node(&doc.root(), "creature").expect("no creature node");
    let child_node = get_unique_node(&creature_node, "child").ok();

    let id = &breaching_entity.id;
    let binding = child_node.as_ref()
        .and_then(|x| get_unique_node_text(&x, "name").ok())
        .map(|x| format_story_data(x))
        .unwrap_or(parent_name.to_string());
    let name = binding.trim();
    let code = child_node.as_ref().and_then(|x| get_unique_node_text(&x, "codeId").ok()).unwrap_or(parent_code).trim().to_string();

    format!("BreachingEntityLocalization {{
        id: \"{id}\",
        name: {name},
        code: \"{code}\"
    }}")
}

fn format_story_data(s: &str) -> String {
    // todo: properly deal with lobocorp's custom serialization format beyond this broken hack
    let start = s.find("{");
    let end = s.rfind("}");

    let x = if let (Some(a), Some(b)) = (start, end) {
        &s[(a + "{".len())..b]
    } else {
        s
    };

    let x = x.replace("&#13;&#10;", "\n");

    if !x.contains('\"') {
        format!("r\"{x}\"")
    } else if x.contains("##") {
        format!("r###\"{x}\"###")
    } else if x.contains('#') {
        format!("r##\"{x}\"##")
    } else {
        format!("r#\"{x}\"#")
    }
}
