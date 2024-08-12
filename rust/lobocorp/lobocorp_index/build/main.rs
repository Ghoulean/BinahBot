use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use lobocorp_index_builder::write_index;

pub fn main() {
    let out_file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(PathBuf::from("out.rs"));
    let mut out_file = File::create(out_file_path).unwrap();

    let index_output = write_index();

    out_file.write_all(index_output.as_bytes()).unwrap();
    dbg!("[lobocorp_index] wrote artifacts");
}
