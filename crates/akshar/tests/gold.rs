use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use varnavinyas_akshar::{normalize, split_aksharas};

#[derive(Debug, Deserialize)]
struct GoldData {
    shuddha_table: Vec<Entry>,
    hrasva_dirgha: Vec<Entry>,
    chandrabindu: Vec<Entry>,
    sha_sha_sa: Vec<Entry>,
    ri_kri: Vec<Entry>,
    halanta: Vec<Entry>,
    ya_e: Vec<Entry>,
    ksha_chhya: Vec<Entry>,
    paragraph_correction: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    incorrect: String,
    correct: String,
}

fn load_gold() -> GoldData {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = PathBuf::from(manifest_dir).join("../../docs/tests/gold.toml");
    
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read gold.toml from {:?}: {}", path, e));
    
    toml::from_str(&content).expect("Failed to parse gold.toml")
}

#[test]
fn test_gold_dataset_smoke() {
    let gold = load_gold();

    check_entries(&gold.shuddha_table, "shuddha_table");
    check_entries(&gold.hrasva_dirgha, "hrasva_dirgha");
    check_entries(&gold.chandrabindu, "chandrabindu");
    check_entries(&gold.sha_sha_sa, "sha_sha_sa");
    check_entries(&gold.ri_kri, "ri_kri");
    check_entries(&gold.halanta, "halanta");
    check_entries(&gold.ya_e, "ya_e");
    check_entries(&gold.ksha_chhya, "ksha_chhya");
    check_entries(&gold.paragraph_correction, "paragraph_correction");
}

fn check_entries(entries: &[Entry], section_name: &str) {
    for (idx, entry) in entries.iter().enumerate() {
        // 1. Verify normalization idempotence
        let n_corr = normalize(&entry.correct);
        let n_incorr = normalize(&entry.incorrect);
        
        let n_corr_2 = normalize(&n_corr);
        let n_incorr_2 = normalize(&n_incorr);
        
        assert_eq!(n_corr, n_corr_2, "Normalization not idempotent for {} ({})", entry.correct, section_name);
        assert_eq!(n_incorr, n_incorr_2, "Normalization not idempotent for {} ({})", entry.incorrect, section_name);

        // 2. Verify segmentation succeeds and reconstructs perfectly
        let aks_corr = split_aksharas(&entry.correct);
        let reconstructed_corr: String = aks_corr.iter().map(|a| a.text.as_str()).collect();
        assert_eq!(
            reconstructed_corr, entry.correct,
            "Segmentation failed reconstruction for {} (section {}, entry {})",
            entry.correct, section_name, idx
        );
        assert!(!aks_corr.is_empty(), "Empty aksharas for {}", entry.correct);

        let aks_incorr = split_aksharas(&entry.incorrect);
        let reconstructed_incorr: String = aks_incorr.iter().map(|a| a.text.as_str()).collect();
        assert_eq!(
            reconstructed_incorr, entry.incorrect,
            "Segmentation failed reconstruction for {} (section {}, entry {})",
            entry.incorrect, section_name, idx
        );
        assert!(!aks_incorr.is_empty(), "Empty aksharas for {}", entry.incorrect);
    }
}