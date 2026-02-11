use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Generate header into crate dir (not OUT_DIR) so C consumers have a stable
    // #include path. The directory is .gitignored; CI/release scripts should copy
    // the header into the distribution archive.
    let out_dir = PathBuf::from(&crate_dir).join("include");
    std::fs::create_dir_all(&out_dir).ok();

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(cbindgen::Config::from_file("cbindgen.toml").unwrap())
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(out_dir.join("varnavinyas.h"));
}
