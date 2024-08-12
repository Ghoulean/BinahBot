use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use lobocorp_reparser_build::build_reparser;

pub fn main() {
    let out_file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(PathBuf::from("out.rs"));
    let mut out_file = File::create(out_file_path).unwrap();

    let output = build_reparser();

    out_file.write_all(output.as_bytes()).unwrap();
    dbg!("[lobocorp_reparser] wrote artifacts");
}
