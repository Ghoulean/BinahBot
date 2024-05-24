use std::{env, fs::File, io::Write, path::PathBuf};

use ruina_reparser_build::build_reparser;

fn main() {
    let out_file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(PathBuf::from("out.rs"));
    let mut out_file = File::create(out_file_path).unwrap();

    let output = build_reparser();

    out_file.write_all(output.as_bytes()).unwrap();
    dbg!("[reparser] wrote artifacts");
}
