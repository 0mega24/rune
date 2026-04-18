use std::{env, fs, path::PathBuf};

fn main() {
    let seed =
        env::var("SEED").expect("SEED must be set to a file path (e.g. SEED=/path/to/words.txt)");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("entries.txt");
    fs::copy(&seed, &out).unwrap_or_else(|e| panic!("failed to copy SEED file '{seed}': {e}"));
    println!("cargo:rerun-if-env-changed=SEED");
    println!("cargo:rerun-if-changed={seed}");
}
