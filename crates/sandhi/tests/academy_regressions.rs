//! Regression tests for Academy standard sandhi examples (Section 3(ख), lines 539-552).
//!
//! These test both forward sandhi (apply) and reverse sandhi (split) against
//! the explicit examples given in the Nepal Academy orthography standard.

use varnavinyas_sandhi::{apply, split};

/// Academy examples for inherent vowel sandhi (Gap #1).
/// Morphemes ending in a bare consonant carry an implicit अ that must
/// participate in दीर्घ/गुण sandhi.
#[test]
fn inherent_vowel_sandhi_apply() {
    let cases = [
        ("प्र", "अध्यापक", "प्राध्यापक", "दीर्घ: प्र(अ) + अ → आ"),
        ("प्र", "ईक्षा", "प्रेक्षा", "गुण: प्र(अ) + ई → ए"),
        ("अप", "अङ्ग", "अपाङ्ग", "दीर्घ: अप(अ) + अ → आ"),
        ("स", "अङ्ग", "साङ्ग", "दीर्घ: स(अ) + अ → आ"),
    ];

    for (left, right, expected, label) in &cases {
        let res = apply(left, right).unwrap_or_else(|e| panic!("{left} + {right}: {e}"));
        assert_eq!(res.output, *expected, "{label}");
    }
}

/// Academy examples for Guna/Vriddhi sandhi splitting (Gap #2).
/// The splitter must reconstruct the original vowel from a merged matra.
#[test]
fn guna_vriddhi_split() {
    let cases = [
        ("सूर्योदय", "सूर्य", "उदय"),
        ("देवेन्द्र", "देव", "इन्द्र"),
        ("हिमालय", "हिम", "आलय"),
        ("महोत्सव", "मह", "उत्सव"),
        ("नरेन्द्र", "नर", "इन्द्र"),
    ];

    for (word, exp_left, exp_right) in &cases {
        let results = split(word);
        assert!(
            results
                .iter()
                .any(|(l, r, _)| l == exp_left && r == exp_right),
            "{word}: expected {exp_left} + {exp_right}, got {:?}",
            results
                .iter()
                .map(|(l, r, _)| format!("{l} + {r}"))
                .collect::<Vec<_>>()
        );
    }
}
