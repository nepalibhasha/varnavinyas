//! Samasa analyzer evaluation against curated samasa_gold.toml.
//!
//! Run:
//! `cargo test -p varnavinyas-eval --test samasa_eval -- --nocapture`

use serde::Deserialize;
use varnavinyas_samasa::{SamasaType, analyze_compound};

#[derive(Debug, Deserialize)]
struct SamasaGold {
    samasa: Vec<SamasaEntry>,
}

#[derive(Debug, Deserialize)]
struct SamasaEntry {
    word: String,
    left: String,
    right: String,
    expected_type: String,
}

#[test]
fn samasa_gold_pair_and_type_coverage() {
    let data = include_str!("../../../docs/tests/samasa_gold.toml");
    let gold: SamasaGold = toml::from_str(data).expect("samasa_gold.toml must parse");

    let total = gold.samasa.len();
    let mut pair_found = 0usize;
    let mut type_matched = 0usize;
    let mut misses: Vec<String> = Vec::new();

    println!("\n=== Samasa Gold Evaluation ===");

    for entry in &gold.samasa {
        let expected_type = parse_type(&entry.expected_type)
            .unwrap_or_else(|| panic!("unknown samasa type '{}'", entry.expected_type));

        let candidates = analyze_compound(&entry.word);
        let pair = candidates
            .iter()
            .find(|c| c.left == entry.left && c.right == entry.right);

        if let Some(c) = pair {
            pair_found += 1;
            if c.samasa_type == expected_type {
                type_matched += 1;
                println!(
                    "  ✓ {} → {} + {} [{:?}]",
                    entry.word, entry.left, entry.right, c.samasa_type
                );
            } else {
                println!(
                    "  ~ {} → pair found, type {:?} (expected {:?})",
                    entry.word, c.samasa_type, expected_type
                );
            }
        } else {
            let summary = if candidates.is_empty() {
                "[]".to_string()
            } else {
                candidates
                    .iter()
                    .take(3)
                    .map(|c| format!("{}+{}:{:?}", c.left, c.right, c.samasa_type))
                    .collect::<Vec<_>>()
                    .join(" | ")
            };
            println!(
                "  ✗ {} → missing expected pair {} + {} (got: {})",
                entry.word, entry.left, entry.right, summary
            );
            misses.push(entry.word.clone());
        }
    }

    let pair_recall = pair_found as f64 / total as f64;
    let type_accuracy_on_found = if pair_found == 0 {
        0.0
    } else {
        type_matched as f64 / pair_found as f64
    };

    println!("\nTotal:                {}", total);
    println!(
        "Expected pair found:  {} ({:.1}%)",
        pair_found,
        pair_recall * 100.0
    );
    println!(
        "Type match (on found): {} ({:.1}%)",
        type_matched,
        type_accuracy_on_found * 100.0
    );

    // MVP thresholds: ensure useful split recall while allowing heuristic type drift.
    assert!(
        pair_recall >= 0.66,
        "Pair recall too low ({:.1}%). Misses: {:?}",
        pair_recall * 100.0,
        misses
    );
    assert!(
        type_accuracy_on_found >= 0.33,
        "Type accuracy too low ({:.1}%)",
        type_accuracy_on_found * 100.0
    );
}

fn parse_type(s: &str) -> Option<SamasaType> {
    match s {
        "Tatpurusha" => Some(SamasaType::Tatpurusha),
        "Karmadharaya" => Some(SamasaType::Karmadharaya),
        "Dvigu" => Some(SamasaType::Dvigu),
        "Bahuvrihi" => Some(SamasaType::Bahuvrihi),
        "Dvandva" => Some(SamasaType::Dvandva),
        "Avyayibhava" => Some(SamasaType::Avyayibhava),
        "Unknown" => Some(SamasaType::Unknown),
        _ => None,
    }
}
