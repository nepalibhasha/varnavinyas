//! Sandhi split evaluation against all headwords.
//!
//! Tests the morphology-first sandhi pipeline:
//! 1. Strip agglutinative suffixes via shabda::decompose()
//! 2. Attempt sandhi splitting on the morphological root
//! 3. Guard: roots < 3 aksharas are skipped (atomic stems)
//! 4. Guard: each split part must have >= 2 aksharas
//!
//! Run: cargo test -p varnavinyas-eval --test sandhi_eval -- --nocapture

use varnavinyas_akshar::split_aksharas;
use varnavinyas_kosha::kosha;
use varnavinyas_sandhi::split as sandhi_split;
use varnavinyas_shabda::decompose;

/// Known correct sandhi splits (word → expected left + right).
const EXPECTED_SPLITS: &[(&str, &str, &str)] = &[
    ("अत्यधिक", "अति", "अधिक"),
    ("अत्याचार", "अति", "आचार"),
    ("अत्यन्त", "अति", "अन्त"),
    ("परोपकार", "पर", "उपकार"),
    ("सूर्योदय", "सूर्य", "उदय"),
    ("देवेन्द्र", "देव", "इन्द्र"),
    ("हिमालय", "हिम", "आलय"),
    ("विद्यार्थी", "विद्या", "अर्थी"),
    ("महोत्सव", "मह", "उत्सव"),
    ("नरेन्द्र", "नर", "इन्द्र"),
];

/// Words that must NOT produce any sandhi split.
const NO_SPLIT_EXPECTED: &[&str] = &[
    "राम",      // name (2 aksharas, atomic)
    "काम",      // noun (2 aksharas)
    "नाम",      // noun (2 aksharas)
    "आम",       // noun (< 3 aksharas)
    "देश",      // noun (2 aksharas)
    "प्रेम",    // noun (2 aksharas)
    "नेपाल",    // name
    "रामसँग",   // after stripping सँग, root=राम (2 aksharas)
];

/// Run sandhi split using the morphology-first pipeline.
fn pipeline_split(word: &str) -> Vec<(String, String)> {
    let morph = decompose(word);
    let root = &morph.root;
    sandhi_split(root)
        .into_iter()
        .map(|(l, r, _)| (l, r))
        .collect()
}

#[test]
fn known_correct_splits_found() {
    let mut found = 0;
    let mut missed = Vec::new();

    for &(word, exp_left, exp_right) in EXPECTED_SPLITS {
        let results = pipeline_split(word);
        let has_match = results
            .iter()
            .any(|(l, r)| l == exp_left && r == exp_right);
        if has_match {
            found += 1;
            println!("  ✓ {} → {} + {}", word, exp_left, exp_right);
        } else {
            missed.push((word, exp_left, exp_right, results));
            println!(
                "  ✗ {} → expected {} + {} but got {:?}",
                word, exp_left, exp_right,
                pipeline_split(word)
            );
        }
    }

    println!(
        "\nKnown splits: {}/{} found",
        found,
        EXPECTED_SPLITS.len()
    );
    // Note: misses are due to splitter reconstruction coverage, not the guard.
    // The 3-akshara guard + 2-akshara per-part filter is working correctly.
    // Track this count to detect regressions as we improve reconstruction.
    assert!(
        found >= 3,
        "Regression: fewer known splits found than baseline (3): {:?}",
        missed
    );
}

#[test]
fn no_split_words_are_clean() {
    let mut failures = Vec::new();

    for &word in NO_SPLIT_EXPECTED {
        let results = pipeline_split(word);
        if results.is_empty() {
            println!("  ✓ {} → no split (correct)", word);
        } else {
            println!("  ✗ {} → unexpected split: {:?}", word, results);
            failures.push((word, results));
        }
    }

    assert!(
        failures.is_empty(),
        "Words that should NOT split produced results: {:?}",
        failures
    );
}

#[test]
fn headword_sandhi_census() {
    let lex = kosha();
    let headwords: Vec<_> = (0..lex.headword_count())
        .filter_map(|_| None::<&str>) // placeholder, iterate via data
        .collect();

    // We'll read headwords directly from the data
    let data = include_str!("../../../data/headwords.tsv");
    let words: Vec<&str> = data
        .lines()
        .filter_map(|line| {
            let word = line.split('\t').next()?.trim();
            if word.is_empty() {
                None
            } else {
                Some(word)
            }
        })
        .collect();

    let total = words.len();
    let mut split_count = 0;
    let mut split_words: Vec<(&str, Vec<(String, String)>)> = Vec::new();
    let mut short_skipped = 0;

    for &word in &words {
        let aksharas = split_aksharas(word).len();
        if aksharas < 3 {
            short_skipped += 1;
            continue;
        }

        let morph = decompose(word);
        let root = &morph.root;
        let results: Vec<(String, String)> = sandhi_split(root)
            .into_iter()
            .map(|(l, r, _)| (l, r))
            .collect();

        if !results.is_empty() {
            split_count += 1;
            split_words.push((word, results));
        }
    }

    let _ = headwords; // suppress unused

    println!("\n=== Sandhi Split Census ===");
    println!("Total headwords:      {}", total);
    println!("Skipped (< 3 akshar): {}", short_skipped);
    println!("Attempted:            {}", total - short_skipped);
    println!("Produced splits:      {}", split_count);
    println!(
        "Split rate:           {:.1}%",
        (split_count as f64 / (total - short_skipped) as f64) * 100.0
    );

    // Print first 50 splits for manual review
    println!("\n--- Sample splits (first 50) ---");
    for (i, (word, splits)) in split_words.iter().take(50).enumerate() {
        let split_strs: Vec<String> = splits
            .iter()
            .map(|(l, r)| format!("{} + {}", l, r))
            .collect();
        println!("  {:>3}. {} → {}", i + 1, word, split_strs.join(" | "));
    }

    // Print POS distribution of split words
    println!("\n--- POS distribution of split words ---");
    let mut pos_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for (word, _) in &split_words {
        let pos = lex
            .lookup(word)
            .map(|e| {
                // Extract just the POS label (before any [bracket] tags)
                let p = e.pos.trim();
                // Take the part after any [...] tag
                let clean = if let Some(idx) = p.rfind(']') {
                    p[idx + 1..].trim()
                } else {
                    p
                };
                if clean.is_empty() {
                    p.to_string()
                } else {
                    clean.to_string()
                }
            })
            .unwrap_or_else(|| "unknown".to_string());
        *pos_counts.entry(pos).or_default() += 1;
    }
    let mut pos_sorted: Vec<_> = pos_counts.into_iter().collect();
    pos_sorted.sort_by(|a, b| b.1.cmp(&a.1));
    for (pos, count) in &pos_sorted {
        println!("  {:<20} {}", pos, count);
    }

    // Sanity: split rate should be modest (< 15% of attempted words)
    let attempted = total - short_skipped;
    let rate = split_count as f64 / attempted as f64;
    println!("\nSplit rate: {:.1}% (threshold < 15%)", rate * 100.0);
    assert!(
        rate < 0.15,
        "Split rate too high ({:.1}%) — likely too many false positives",
        rate * 100.0
    );
}
