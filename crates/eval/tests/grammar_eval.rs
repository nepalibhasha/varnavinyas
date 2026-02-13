//! Grammar-pass evaluation against curated sentence fixtures.
//!
//! Run:
//! `cargo test -p varnavinyas-eval --test grammar_eval -- --nocapture`

use serde::Deserialize;
use varnavinyas_parikshak::{CheckOptions, DiagnosticKind, check_text_with_options};

#[derive(Debug, Deserialize)]
struct GrammarGold {
    sentence: Vec<SentenceEntry>,
}

#[derive(Debug, Deserialize)]
struct SentenceEntry {
    text: String,
    expect_variant_or_ambiguous_min: usize,
    expect_variant_or_ambiguous_max: Option<usize>,
}

#[test]
fn grammar_pass_sentence_expectations() {
    let data = include_str!("../../../docs/tests/grammar_sentences.toml");
    let gold: GrammarGold = toml::from_str(data).expect("grammar_sentences.toml must parse");

    let mut failures = Vec::new();

    println!("\n=== Grammar Pass Evaluation ===");

    for entry in &gold.sentence {
        let diags = check_text_with_options(&entry.text, CheckOptions { grammar: true });
        let count = diags
            .iter()
            .filter(|d| matches!(d.kind, DiagnosticKind::Variant | DiagnosticKind::Ambiguous))
            .count();

        let min_ok = count >= entry.expect_variant_or_ambiguous_min;
        let max_ok = entry
            .expect_variant_or_ambiguous_max
            .is_none_or(|max| count <= max);

        if min_ok && max_ok {
            let max_desc = entry
                .expect_variant_or_ambiguous_max
                .map(|n| n.to_string())
                .unwrap_or_else(|| "∞".to_string());
            println!(
                "  ✓ '{}' => {} grammar hints (expected {}..={})",
                entry.text, count, entry.expect_variant_or_ambiguous_min, max_desc
            );
        } else {
            println!(
                "  ✗ '{}' => {} grammar hints (expected >= {} and <= {:?})",
                entry.text,
                count,
                entry.expect_variant_or_ambiguous_min,
                entry.expect_variant_or_ambiguous_max
            );
            failures.push((
                entry.text.clone(),
                count,
                entry.expect_variant_or_ambiguous_min,
                entry.expect_variant_or_ambiguous_max,
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "Grammar sentence expectations failed: {:?}",
        failures
    );
}
