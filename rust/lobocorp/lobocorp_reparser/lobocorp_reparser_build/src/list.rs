use std::fs;
use std::iter;
use std::path::PathBuf;

use roxmltree::Document;

use crate::path::BASE_LIST_PATH_STR;
use crate::xml::get_nodes;
use crate::xml::get_unique_node;
use crate::xml::get_unique_node_text;

#[derive(Debug)]
pub struct ListEntry {
    // pub name: String, // used with equipment, but ignored for our purposes
    pub src: String, // {src}_{locale_id}.xml for localization file
    pub id: u32, // numerical id
    pub stat: String // encyclopedia + breaching info file
}

pub fn load_encyclopedia_list() -> Vec<ListEntry> {
    let binding = PathBuf::from(BASE_LIST_PATH_STR);
    let abno_list_str = fs::read_to_string(binding.as_path()).expect("cannot read BaseList.xml");

    let doc: Document = Document::parse(&abno_list_str).expect("failed parsing BaseList.xml");

    let creature_list_node = get_unique_node(&doc.root(), "creature_list").expect("couldn't find creature_list");
    
    let apo_bird = iter::once(ListEntry {
        src: "BossBird".to_string(),
        id: 100038,
        stat: "ChildCreatures/BossBird_stat".to_string(),
    });

    get_nodes(&creature_list_node, "creature").iter().map(|x| {
        ListEntry {
            src: x.attribute("src").expect("no src attribute").to_string(),
            id: x.attribute("id").expect("no id attribute").parse().expect("failed parse into u32"),
            stat: get_unique_node_text(x, "stat").expect("no stat text").to_string(),
        }
    }).chain(apo_bird).filter(|x| {
        // todo: filter out ordeals; figure out how to parse later
        x.id <= 200000 || x.id > 299999
    }).filter(|x| {
        // todo: filter out gebura and binah; figure out how to parse later
        x.id <= 400001 || x.id > 499999
    }).filter(|x| {
        // filter out apo bird gate and eggs
        x.id < 1000350 || x.id > 1000353
    })
    .collect()
}