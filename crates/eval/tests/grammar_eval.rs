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

        if count >= entry.expect_variant_or_ambiguous_min {
            println!(
                "  ✓ '{}' => {} grammar hints (expected >= {})",
                entry.text, count, entry.expect_variant_or_ambiguous_min
            );
        } else {
            println!(
                "  ✗ '{}' => {} grammar hints (expected >= {})",
                entry.text, count, entry.expect_variant_or_ambiguous_min
            );
            failures.push((
                entry.text.clone(),
                count,
                entry.expect_variant_or_ambiguous_min,
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "Grammar sentence expectations failed: {:?}",
        failures
    );
}
