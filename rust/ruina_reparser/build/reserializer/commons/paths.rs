use std::fs::{self, DirEntry};
use std::io::{self};
use std::path::PathBuf;
use std::sync::OnceLock;

use ruina_common::localizations::common::Locale;

// todo: macro-ify
pub fn game_obj_path() -> &'static PathBuf {
    static GAME_OBJ_PATH: OnceLock<PathBuf> = OnceLock::new();
    GAME_OBJ_PATH.get_or_init(|| PathBuf::from("./BaseMod/StaticInfo"))
}

// todo: hashmap not a vec of entries wtf
pub fn localize_paths() -> &'static Vec<(Locale, PathBuf)> {
    static LOCALIZE_PATHS: OnceLock<Vec<(Locale, PathBuf)>> = OnceLock::new();
    LOCALIZE_PATHS.get_or_init(||
        vec![
            (Locale::English, PathBuf::from("./BaseMod/Localize/en")),
            (Locale::Korean, PathBuf::from("./BaseMod/Localize/kr")),
            (Locale::Japanese, PathBuf::from("./BaseMod/Localize/jp")),
            (Locale::Chinese, PathBuf::from("./BaseMod/Localize/cn")),
            (Locale::TraditionalChinese, PathBuf::from("./BaseMod/Localize/trcn"))
        ]
    )
}

// TODO: env var this
pub fn _outfile_path() -> &'static PathBuf {
    static OUTFILE_PATH: OnceLock<PathBuf> = OnceLock::new();
    OUTFILE_PATH.get_or_init(|| PathBuf::from("./build/out.rs"))
}

pub fn read_xml_files_in_dir(dir: &PathBuf) -> Vec<(PathBuf, String)> {
    let pathbufs: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .filter(|x: &Result<DirEntry, io::Error>| filter_xml_files_only(x.as_ref().unwrap()))
        .map(|x: Result<DirEntry, io::Error>| x.unwrap().path())
        .collect();
    let raw_file_text: Vec<_> = pathbufs
        .clone()
        .iter()
        .map(|x| fs::read_to_string(x.as_path()).unwrap())
        .collect();
    Iterator::zip(pathbufs.into_iter(), raw_file_text).collect()
}

fn filter_xml_files_only(dir_entry: &DirEntry) -> bool {
    dir_entry.path().is_file() && dir_entry.path().extension().unwrap() == "xml"
}
