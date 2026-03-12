//! Build script for varnavinyas-kosha.
//!
//! Pre-builds the FST from `data/words.txt` at compile time so the WASM
//! artifact embeds compact FST bytes (~1-3 MB) rather than the raw word list
//! (~4.9 MB). The FST is reconstructed at runtime with near-zero cost.

use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let words_path = manifest_dir.join("../../data/words.txt");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let fst_out = out_dir.join("words.fst");

    // Trigger rebuild only when the word list changes.
    println!("cargo:rerun-if-changed={}", words_path.display());

    let words_data = std::fs::read_to_string(&words_path).expect("data/words.txt must exist");

    let words: Vec<&str> = words_data.lines().filter(|l| !l.is_empty()).collect();

    // words.txt is pre-sorted by UTF-8 bytes (LC_ALL=C sort).
    let fst_bytes = build_fst_set(&words);

    std::fs::write(&fst_out, &fst_bytes).expect("failed to write pre-built FST");
}

fn build_fst_set(words: &[&str]) -> Vec<u8> {
    // Mirrors crates/kosha/src/builder.rs. Duplicated here because build
    // scripts can only use build-dependencies, not the crate itself.
    let mut builder = fst::SetBuilder::memory();
    for word in words {
        builder.insert(word).expect("words must be sorted");
    }
    builder.into_inner().expect("FST build should succeed")
}
