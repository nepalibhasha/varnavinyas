use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Generate header into crate dir (not OUT_DIR) so C consumers have a stable
    // #include path. The directory is .gitignored; CI/release scripts should copy
    // the header into the distribution archive.
    let out_dir = PathBuf::from(&crate_dir).join("include");
    std::fs::create_dir_all(&out_dir).ok();

    let out_file = out_dir.join("varnavinyas.h");
    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(cbindgen::Config::from_file("cbindgen.toml").unwrap())
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(&out_file);

    let mut header = std::fs::read_to_string(&out_file).expect("Unable to read generated header");

    let abi_block = r#"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Word origin classification.
 *
 * Returned by `varnavinyas_classify`. Integer-backed for C ABI safety.
 */
typedef enum Origin {
  Origin_Tatsam = 0,
  Origin_Tadbhav = 1,
  Origin_Deshaj = 2,
  Origin_Aagantuk = 3,
} Origin;

char *varnavinyas_check_text(const char *text);

char *varnavinyas_check_text_with_options(const char *text,
                                          bool grammar,
                                          int punctuation_mode,
                                          bool include_noop_heuristics);

char *varnavinyas_check_word(const char *word);

char *varnavinyas_transliterate(const char *input, int from, int to);

Origin varnavinyas_classify(const char *word);

void varnavinyas_free_string(char *ptr);

char *varnavinyas_version(void);

#ifdef __cplusplus
}  /* extern "C" */
#endif
"#;

    let include_guard_end = "#endif  /* VARNAVINYAS_H */";
    if let Some(idx) = header.rfind(include_guard_end) {
        header.insert_str(idx, abi_block);
    } else {
        header.push_str(abi_block);
    }

    std::fs::write(out_file, header).expect("Unable to write header");
}
