//! Reviewed compound winners and negative evidence cases.
use serde::Deserialize;
use varnavinyas_samasa::analyze_compound;

#[derive(Debug, Deserialize)]
struct SamasaGold {
    samasa: Vec<SamasaEntry>,
    not_samasa: Vec<NegativeEntry>,
}

#[derive(Debug, Deserialize)]
struct SamasaEntry {
    word: String,
    left: String,
    right: String,
    expected_type: String,
    vigraha: String,
}

#[derive(Debug, Deserialize)]
struct NegativeEntry {
    word: String,
}

#[test]
fn samasa_gold_winners_and_negative_cases() {
    let gold: SamasaGold = toml::from_str(include_str!("../../../docs/tests/samasa_gold.toml"))
        .expect("samasa_gold.toml must parse");
    assert!(!gold.samasa.is_empty() && !gold.not_samasa.is_empty());
    for entry in &gold.samasa {
        let candidates = analyze_compound(&entry.word);
        let winner = candidates
            .first()
            .unwrap_or_else(|| panic!("{}: no supported compound", entry.word));
        assert_eq!(winner.left, entry.left, "{}: wrong winner", entry.word);
        assert_eq!(winner.right, entry.right, "{}: wrong winner", entry.word);
        assert_eq!(
            format!("{:?}", winner.samasa_type),
            entry.expected_type,
            "{}",
            entry.word
        );
        assert_eq!(winner.vigraha, entry.vigraha, "{}", entry.word);
        assert_eq!(
            candidates.len(),
            1,
            "{}: unreviewed alternatives",
            entry.word
        );
        println!(
            "✓ {} → {} + {} [{:?}]",
            entry.word, winner.left, winner.right, winner.samasa_type
        );
    }
    for entry in &gold.not_samasa {
        let candidates = analyze_compound(&entry.word);
        assert!(
            candidates.is_empty(),
            "{}: unsupported compound {candidates:?}",
            entry.word
        );
    }
    println!(
        "{} reviewed winners; {} negative evidence cases",
        gold.samasa.len(),
        gold.not_samasa.len()
    );
}
