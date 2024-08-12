use std::{env, fs::File, io::Write, path::PathBuf};

use binah_bot_spoiler::build_spoiler_channel_hashmap;

fn main() {
    let out_file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(PathBuf::from("out.rs"));
    let mut out_file = File::create(out_file_path).unwrap();

    let output = build_spoiler_channel_hashmap();

    out_file.write_all(output.as_bytes()).unwrap();
    dbg!("[binah_bot] wrote artifacts");
}
