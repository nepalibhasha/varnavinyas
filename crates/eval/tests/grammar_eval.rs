//! Grammar-pass evaluation against curated sentence fixtures.
//!
//! Run:
//! `cargo test -p varnavinyas-eval --test grammar_eval -- --nocapture`

use std::collections::{BTreeMap, HashSet};

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
    expected_rule_codes: Option<Vec<String>>,
}

#[test]
fn grammar_pass_sentence_expectations() {
    let data = include_str!("../../../docs/tests/grammar_sentences.toml");
    let gold: GrammarGold = toml::from_str(data).expect("grammar_sentences.toml must parse");

    let mut failures = Vec::new();
    let mut expected_by_rule: BTreeMap<String, usize> = BTreeMap::new();
    let mut hits_by_rule: BTreeMap<String, usize> = BTreeMap::new();
    let mut unexpected_by_rule: BTreeMap<String, usize> = BTreeMap::new();

    println!("\n=== Grammar Pass Evaluation ===");

    for entry in &gold.sentence {
        let diags = check_text_with_options(&entry.text, CheckOptions { grammar: true });

        let grammar_diags: Vec<_> = diags
            .iter()
            .filter(|d| matches!(d.kind, DiagnosticKind::Variant | DiagnosticKind::Ambiguous))
            .collect();
        let count = grammar_diags.len();

        let observed_rules: HashSet<String> = grammar_diags
            .iter()
            .map(|d| d.rule.code().to_string())
            .collect();

        let min_ok = count >= entry.expect_variant_or_ambiguous_min;
        let max_ok = entry
            .expect_variant_or_ambiguous_max
            .is_none_or(|max| count <= max);

        let mut expected_rules_ok = true;
        if let Some(expected_rules) = &entry.expected_rule_codes {
            for code in expected_rules {
                *expected_by_rule.entry(code.clone()).or_default() += 1;
                if observed_rules.contains(code) {
                    *hits_by_rule.entry(code.clone()).or_default() += 1;
                } else {
                    expected_rules_ok = false;
                }
            }

            let expected_set: HashSet<&str> = expected_rules.iter().map(String::as_str).collect();
            for observed in &observed_rules {
                if !expected_set.contains(observed.as_str()) {
                    *unexpected_by_rule.entry(observed.clone()).or_default() += 1;
                }
            }
        } else {
            for observed in &observed_rules {
                *unexpected_by_rule.entry(observed.clone()).or_default() += 1;
            }
        }

        if min_ok && max_ok && expected_rules_ok {
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
                "  ✗ '{}' => {} grammar hints (expected >= {} and <= {:?}; rules {:?})",
                entry.text,
                count,
                entry.expect_variant_or_ambiguous_min,
                entry.expect_variant_or_ambiguous_max,
                entry.expected_rule_codes
            );
            failures.push((
                entry.text.clone(),
                count,
                entry.expect_variant_or_ambiguous_min,
                entry.expect_variant_or_ambiguous_max,
                entry.expected_rule_codes.clone(),
                observed_rules,
            ));
        }
    }

    println!("\n=== Per-rule Summary ===");
    if expected_by_rule.is_empty() {
        println!("  (no expected_rule_codes provided)");
    } else {
        for (rule, expected) in &expected_by_rule {
            let hits = hits_by_rule.get(rule).copied().unwrap_or(0);
            let recall = if *expected == 0 {
                1.0
            } else {
                hits as f64 / *expected as f64
            };
            println!(
                "  {}: hits {}/{} ({:.1}%)",
                rule,
                hits,
                expected,
                recall * 100.0
            );
        }
    }

    if !unexpected_by_rule.is_empty() {
        println!("\n=== Unexpected rule hits ===");
        for (rule, count) in &unexpected_by_rule {
            println!("  {}: {}", rule, count);
        }
    }

    assert!(
        failures.is_empty(),
        "Grammar sentence expectations failed: {:?}",
        failures
    );
}
