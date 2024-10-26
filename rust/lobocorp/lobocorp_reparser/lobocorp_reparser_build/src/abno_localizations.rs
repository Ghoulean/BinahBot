use std::collections::HashMap;
use std::fs;
use std::iter;

use lobocorp_common::localizations::common::Locale;
use roxmltree::Document;
use roxmltree::Node;
use strum::IntoEnumIterator;

use crate::abnormalities::PartialBreachingEntity;
use crate::abnormalities::PartialEncyclopediaInfo;
use crate::hardcoded_names::get_apostle_names;
use crate::hardcoded_names::get_nothing_there_names;
use crate::list::ListEntry;
use crate::path::get_localized_abno_file_path;
use crate::serde::display_serializer;
use crate::serde::serialize_option;
use crate::serde::str_serializer;
use crate::serde::write_vec;
use crate::xml::find_unique_node_with_name_and_attribute;
use crate::xml::get_nodes;
use crate::xml::get_nodes_text;
use crate::xml::get_unique_node;
use crate::xml::get_unique_node_text;

#[derive(Clone, Debug, strum_macros::Display)]
enum PartialNarrationType {
    Move,
    Start,
    Death,
    Panic,
    Finish,
    Mid(i32),
    Special(String)
}

pub fn write_abno_localizations(
    list_entries: &[ListEntry],
    partial_abnos: &HashMap<ListEntry, PartialEncyclopediaInfo>,
) -> String {
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

fn build_abno_localizations(
    list_entries: &[ListEntry],
    partial_abnos: &HashMap<ListEntry, PartialEncyclopediaInfo>,
    locale: &Locale,
) -> HashMap<u32, String> {
    list_entries
        .iter()
        .map(|x| {
            let base_name = if x.id == 100009 {
                // inconsistent capitalization
                "Onebadmanygood"
            } else {
                &x.src
            };
            // yggdrasil, il pianto della Luna, clouded monk, ppodae, army in pink cn tr doesn't exist
            let locale = if *locale == Locale::ChineseTraditional
                && (x.id == 100062
                    || x.id == 100065
                    || x.id == 100105
                    || x.id == 100106
                    || x.id == 100064
                    || x.id == 300108)
            {
                &Locale::Chinese
            } else {
                locale
            };

            let path = get_localized_abno_file_path(&locale, base_name);
            let file_str =
                fs::read_to_string(path.as_path()).expect(&format!("cannot read {:?}", path));
            let doc: Document =
                Document::parse(&file_str).expect(&format!("failed parsing {:?}", path));

            let partial_info = partial_abnos.get(x).expect("tried to get nonexistent abno");

            (x.id, build_localization(x, &partial_info, &doc, &locale))
        })
        .collect()
}

fn build_localization(
    entry: &ListEntry,
    partial_info: &PartialEncyclopediaInfo,
    doc: &Document,
    locale: &Locale,
) -> String {
    let creature_node = get_unique_node(&doc.root(), "creature").expect("no creature node");
    let observe_node = get_unique_node(&creature_node, "observe").expect("no observe node");
    let collection_node = get_unique_node(&observe_node, "collection").expect("no collection node");

    let id = entry.id;
    let name = get_nodes_text(&collection_node, "name")
        .last()
        .map(|x| format_story_data(x.trim()))
        .expect("couldn't get name");
    let other_names = get_nodes_text(&collection_node, "name")
        .iter()
        .rev()
        .skip(1)
        .rev()
        .map(|x| x.trim())
        .collect::<Vec<_>>();
    let code = get_unique_node_text(&collection_node, "codeNo")
        .map(|x| x.trim())
        .expect("couldn't get code");
    let selection_text = serialize_option(
        &get_unique_node_text(&collection_node, "openText")
            .map(|x| format_story_data(x.trim()).to_string())
            .ok(),
        display_serializer,
    );
    let special_tips = get_unique_node(&observe_node, "specialTipSize")
        .map(|x| get_nodes(&x, "specialTip"))
        .expect("couldn't get special tips");
    let managerial_guidances = write_vec(
        &special_tips
            .iter()
            .map(|x| format_story_data(x.text().expect("no text")))
            .collect::<Vec<_>>(),
    );

    // index: observation level. usize: id
    let mut story: Vec<Vec<(usize, String)>> = Vec::new();
    get_nodes(&observe_node, "desc")
            .iter()
            .for_each(|x| {
                let observation_level = x.attribute("openLevel").and_then(|y| y.parse::<usize>().ok());
                if observation_level.is_none() {
                    return;
                }
                let observation_level = observation_level.unwrap();
                let id = x.attribute("id").and_then(|y| y.parse::<usize>().ok()).unwrap();
                if story.len() < observation_level + 1 {
                    story.resize(observation_level + 1, Vec::new());
                }
                let v = story.get_mut(observation_level).unwrap();

                let inside_text = x.children()
                        .filter(|y| y.is_text())
                        .collect::<Vec<_>>()
                        .last()
                        .expect("no text nodes")
                        .text()
                        .clone()
                        .expect("no text");
                v.push((id, format_story_data2(inside_text)));
            });
    story.iter_mut().for_each(|v| {
        v.sort_by(|a, b| a.0.cmp(&b.0));
    });
    let story = story.iter().map(|v| {
        let str = &v.iter().map(|(_, t)| {
            t.replace("\r", "").split("\n").map(|x| x.trim()).collect::<Vec<_>>().join("\n")
        }).collect::<Vec<_>>().join("\n\n");
        wrap_raw_str(&str)
    })
    .collect::<Vec<_>>();
    let story = write_vec(&story);

    let breaching_entities = match partial_info {
        PartialEncyclopediaInfo::Normal(x) => Some(&x.breaching_entities),
        PartialEncyclopediaInfo::Tool(x) => Some(&x.breaching_entities),
        PartialEncyclopediaInfo::DontTouchMe(_) => None,
    };

    let default_name_vec: Vec<String> = if id == 100038 {
        // apo bird egg localizations not in standard location
        let etc_node = get_unique_node(&creature_node, "etc").expect("no etc node");
        let binding = vec!["gateway", "bigBirdEgg", "longBirdEgg", "smallBirdEgg"];
        let eggs = binding
            .iter()
            .map(|x| {
                find_unique_node_with_name_and_attribute(&etc_node, "param", "key", x)
                    .ok()
                    .and_then(|x| x.text())
                    .map(|x| x.trim())
                    .map(|x| format_story_data(x))
                    .expect(&format!("no name for {}", x))
                    .to_string()
            })
            .map(|x| x);
        iter::once(name.clone()).chain(eggs).collect::<Vec<_>>()
    } else if id == 100015 {
        // whitenight apostle names are only nicknames
        iter::once(name.clone())
            .chain(get_apostle_names(locale).map(|x| format_story_data(&x)))
            .collect::<Vec<_>>()
    } else if id == 100005 {
        // Nothing There names are also only nicknames
        get_nothing_there_names(locale)
            .iter()
            .map(|x| format_story_data(&x))
            .collect::<Vec<_>>()
    } else {
        let len = breaching_entities.map(|x| x.len()).unwrap_or(0);
        vec![name.clone()].into_iter().cycle().take(len).collect()
    };

    let breaching_entity_localizations = breaching_entities
        .map(|x| {
            x.iter()
                .enumerate()
                .map(|(i, x)| {
                    let default_name = default_name_vec.get(i).unwrap();
                    build_breaching_entity_localization(x, &default_name, doc)
                })
                .collect::<Vec<_>>()
        })
        .map(|x| write_vec(&x))
        .unwrap_or("[]".to_string());

    let info_node = get_unique_node(&doc.root(), "info").expect("no info node");
    let narration_entries = build_narration_entries(&info_node);
    let narration_str = write_narration_entries(&narration_entries);

    format!(
        "EncyclopediaInfoLocalization {{
        id: \"{id}\",
        name: {name},
        other_names: &{other_names:?},
        code: \"{code}\",
        selection_text: {selection_text},
        managerial_guidances: &{managerial_guidances},
        story: &{story},
        breaching_entity_localizations: &{breaching_entity_localizations},
        narration_map: &{narration_str}
    }}"
    )
}

fn build_narration_entries(
    info_node: &Node,
) -> Vec<(PartialNarrationType, String)> {
    let narration_nodes = get_nodes(&info_node, "narration");
    narration_nodes.into_iter().flat_map(|x| {
        let binding = "";
        let inner_text = format_narration_data(x.text().unwrap_or(&binding));
        let narration_type = PartialNarrationType::from(x.attribute("action").unwrap());
        inner_text.into_iter().map(|x| {
            (narration_type.clone(), x)
        }).collect::<Vec<_>>()
    }).collect()
}

fn write_narration_entries(narration_entries: &[(PartialNarrationType, String)]) -> String {
    format!("[{}]", narration_entries.into_iter().map(|x| {
        let narration_serder = match &x.0 {
            PartialNarrationType::Mid(y) => format!("Mid({})", y),
            PartialNarrationType::Special(y) => format!("Special(\"{}\")", y),
            _ => x.0.to_string()
        };
        format!("(NarrationType::{}, {})", narration_serder, x.1)
    }).collect::<Vec<_>>().join(","))
}

fn build_breaching_entity_localization(
    breaching_entity: &PartialBreachingEntity,
    parent_name: &str,
    doc: &Document,
) -> String {
    let creature_node = get_unique_node(&doc.root(), "creature").expect("no creature node");
    let child_node = get_unique_node(&creature_node, "child").ok();

    let id = &breaching_entity.id;
    let binding = child_node
        .as_ref()
        .and_then(|x| get_unique_node_text(&x, "name").ok())
        .map(|x| format_story_data(x))
        .unwrap_or(parent_name.to_string());
    let name = binding.trim();
    let code = child_node
        .as_ref()
        .and_then(|x| get_unique_node_text(&x, "codeId").ok())
        .map(|x| x.trim().to_string());
    let code = serialize_option(&code, str_serializer);

    format!(
        "BreachingEntityLocalization {{
        id: \"{id}\",
        name: {name},
        code: {code}
    }}"
    )
}

fn format_story_data(s: &str) -> String {
    // todo: properly deal with lobocorp's custom serialization format beyond this broken hack
    let start = s.find("{");
    let end = s.rfind("}");

    let x = if let (Some(a), Some(b)) = (start, end) {
        &s[(a + "{".len())..b]
    } else {
        s
    }.trim();

    wrap_raw_str(x)
}

fn format_story_data2(s: &str) -> String {
    // todo: properly deal with lobocorp's custom serialization format beyond this even more broken hack
    let start = s.find("{");
    let end = s.rfind("}");

    if let (Some(a), Some(b)) = (start, end) {
        &s[(a + "{".len())..b]
    } else {
        s
    }.trim().to_string()
}

fn format_narration_data(s: &str) -> Vec<String> {
    if s.find("[").is_none() {
        vec![format_story_data(s)]
    } else {
        s.split("{").flat_map(|x| {
            x.find("}").map(|y| &x[..y]).map(|y| wrap_raw_str(y.trim()))
        }).collect()
    }
}

fn wrap_raw_str(x: &str) -> String {
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

impl From<&str> for PartialNarrationType {
    fn from(value: &str) -> Self {
        match value {
            "move" => PartialNarrationType::Move,
            "start" => PartialNarrationType::Start,
            "death" => PartialNarrationType::Death,
            "panic" => PartialNarrationType::Panic,
            "finish" => PartialNarrationType::Finish,
            _ => {
                if value.starts_with("mid") {
                let num = (&value[("mid".len())..]).parse::<i32>().unwrap();
                    PartialNarrationType::Mid(num)
                } else {
                    PartialNarrationType::Special(value.to_string())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use roxmltree::Document;
    use roxmltree::NodeType;

    use super::format_narration_data;

    #[test]
    fn sanity_xml_parsing() {
        let doc = Document::parse(r#"<desc id="3" openLevel="2">[ {Company P } ]</desc>"#).unwrap();

        assert_eq!(
            Some("[ {Company P } ]"),
            doc.root().first_child().unwrap().text()
        );
        assert_eq!(
            Some("[ {Company P } ]"),
            doc.root()
                .first_child()
                .unwrap()
                .last_child()
                .unwrap()
                .text()
        );
        assert_eq!(
            NodeType::Element,
            doc.root().first_child().unwrap().node_type()
        );
        assert_eq!(
            1,
            doc.root()
                .first_child()
                .unwrap()
                .children()
                .collect::<Vec<_>>()
                .len()
        );

        let doc = roxmltree::Document::parse(r#"<desc id="4" openLevel="3">
<!-- Translation has been fixed here ;) -->
[ {Thanks to this shelter, the selected refugees were safely shielded from the ocean of endless screams and bloodshed.} ]
</desc>"#).unwrap();

        assert_eq!(
            NodeType::Element,
            doc.root().first_child().unwrap().node_type()
        );
        assert_eq!(
            3,
            doc.root()
                .first_child()
                .unwrap()
                .children()
                .collect::<Vec<_>>()
                .len()
        );
        assert_eq!(
            2,
            doc.root()
                .first_child()
                .unwrap()
                .children()
                .filter(|x| x.is_text())
                .collect::<Vec<_>>()
                .len()
        );
        assert_eq!(Some("\n[ {Thanks to this shelter, the selected refugees were safely shielded from the ocean of endless screams and bloodshed.} ]\n"), doc.root().first_child().unwrap().last_child().unwrap().text());
    }

    #[test]
    fn sanity_format_narration_data() {
        let str = "      [
      {#0 turns their head away to avoid eye contact, and is met with a burning lamp.},
      {A dazzlingly bright lamp is held in front of #0, who turned away to avoid looking towards $0.},
      {#0 sees $0 holding its illuminating lamp.},
      {It was a very good decision for #0 to face away from $0.}
      ]";
        let format_narration_data = format_narration_data(&str);
        assert_eq!(4, format_narration_data.len());
        assert_eq!("r\"#0 turns their head away to avoid eye contact, and is met with a burning lamp.\"", format_narration_data.get(0).unwrap());
        assert_eq!("r\"It was a very good decision for #0 to face away from $0.\"", format_narration_data.get(3).unwrap());
    }
}
