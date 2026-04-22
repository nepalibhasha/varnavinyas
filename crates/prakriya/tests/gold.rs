use serde::Deserialize;
use varnavinyas_prakriya::derive;

#[derive(Debug, Deserialize)]
struct GoldEntry {
    incorrect: String,
    correct: String,
    #[serde(rename = "rule")]
    _rule: String,
    #[serde(rename = "section")]
    _section: String,
    #[serde(rename = "page")]
    _page: u32,
}

#[derive(Debug, Deserialize)]
struct GoldData {
    #[serde(default)]
    shuddha_table: Vec<GoldEntry>,
    #[serde(default)]
    hrasva_dirgha: Vec<GoldEntry>,
    #[serde(default)]
    chandrabindu: Vec<GoldEntry>,
    #[serde(default)]
    sha_sha_sa: Vec<GoldEntry>,
    #[serde(default)]
    ri_kri: Vec<GoldEntry>,
    #[serde(default)]
    halanta: Vec<GoldEntry>,
    #[serde(default)]
    ya_e: Vec<GoldEntry>,
    #[serde(default)]
    ksha_chhya: Vec<GoldEntry>,
    #[serde(default)]
    _paragraph_correction: Vec<GoldEntry>,
}

/// P5: All gold.toml entries produce correct output with non-empty rule citation.
#[test]
fn p5_all_gold_entries() {
    let gold_toml = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/tests/gold.toml"
    ))
    .expect("Failed to read gold.toml");

    let gold: GoldData = toml::from_str(&gold_toml).expect("Failed to parse gold.toml");

    let mut all_entries: Vec<(&str, &GoldEntry)> = Vec::new();
    for e in &gold.shuddha_table {
        all_entries.push(("shuddha_table", e));
    }
    for e in &gold.hrasva_dirgha {
        all_entries.push(("hrasva_dirgha", e));
    }
    for e in &gold.chandrabindu {
        all_entries.push(("chandrabindu", e));
    }
    for e in &gold.sha_sha_sa {
        all_entries.push(("sha_sha_sa", e));
    }
    for e in &gold.ri_kri {
        all_entries.push(("ri_kri", e));
    }
    for e in &gold.halanta {
        all_entries.push(("halanta", e));
    }
    for e in &gold.ya_e {
        all_entries.push(("ya_e", e));
    }
    for e in &gold.ksha_chhya {
        all_entries.push(("ksha_chhya", e));
    }
    assert!(
        !all_entries.is_empty(),
        "gold.toml should contain at least one regression entry"
    );

    let mut failures = Vec::new();

    for (category, entry) in &all_entries {
        let p = derive(&entry.incorrect);

        // Check that the output matches one of the correct alternatives
        let correct_alternatives: Vec<&str> = entry.correct.split('/').collect();
        let output_matches = correct_alternatives.contains(&p.output.as_str());

        if !output_matches {
            failures.push(format!(
                "[{category}] '{}' → expected '{}', got '{}'",
                entry.incorrect, entry.correct, p.output,
            ));
        }

        // Check that the derivation has steps (P6: non-empty trace)
        if p.steps.is_empty() {
            failures.push(format!(
                "[{category}] '{}' → '{}' has empty step trace",
                entry.incorrect, p.output,
            ));
        }
    }

    if !failures.is_empty() {
        panic!(
            "Gold test failures ({}/{}):\n{}",
            failures.len(),
            all_entries.len(),
            failures.join("\n"),
        );
    }
}
