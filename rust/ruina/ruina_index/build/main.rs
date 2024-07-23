use std::{env, fs::File, io::Write, path::PathBuf};

use ruina_index_builder::{precompute_disambiguations_map, precompute_index, write_to_string};

fn main() {
    let out_file_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(PathBuf::from("out.rs"));
    let mut out_file = File::create(out_file_path).unwrap();

    let disambiguations_output = format!(
        "static DISAMBIGUATIONS_MAP: {}",
        write_to_string(&precompute_disambiguations_map())
    );
    let index_output = precompute_index();

    let output = [
        disambiguations_output,
        index_output
    ].join("\n");

    out_file.write_all(output.as_bytes()).unwrap();
    dbg!("[index] wrote artifacts");
}
