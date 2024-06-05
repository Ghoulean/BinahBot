use std::{env, fs::File, io::Write, path::PathBuf};

use ruina_index_builder::precompute_index;

fn main() {
    let out_file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(PathBuf::from("out.rs"));
    let mut out_file = File::create(out_file_path).unwrap();

    let output = precompute_index();

    out_file.write_all(output.as_bytes()).unwrap();
    dbg!("[index] wrote artifacts");
}
