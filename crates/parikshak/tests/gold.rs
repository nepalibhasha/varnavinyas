use serde::Deserialize;
use varnavinyas_parikshak::{check_text, check_word};

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
    paragraph_correction: Vec<GoldEntry>,
}

fn load_gold() -> GoldData {
    toml::from_str(include_str!("../../../docs/tests/gold.toml")).expect("parse gold.toml")
}

fn word_entries(data: &GoldData) -> Vec<&GoldEntry> {
    data.shuddha_table
        .iter()
        .chain(&data.hrasva_dirgha)
        .chain(&data.chandrabindu)
        .chain(&data.sha_sha_sa)
        .chain(&data.ri_kri)
        .chain(&data.halanta)
        .chain(&data.ya_e)
        .chain(&data.ksha_chhya)
        .collect()
}

fn paragraph_entries(data: &GoldData) -> Vec<&GoldEntry> {
    data.paragraph_correction.iter().collect()
}

fn is_phrase_like_form(form: &str) -> bool {
    form.contains(' ') || form.contains('-') || form.contains('–') || form.contains('—')
}

/// No false positives: correct forms should NOT produce diagnostics.
#[test]
fn gold_correct_forms_no_false_positives() {
    let data = load_gold();
    let all_entries = word_entries(&data);

    let mut false_positives = Vec::new();
    for entry in &all_entries {
        for correct_form in entry.correct.split('/') {
            if let Some(diag) = check_word(correct_form) {
                false_positives.push(format!(
                    "  {} (flagged as → {})",
                    correct_form, diag.correction
                ));
            }
        }
    }

    assert!(
        false_positives.is_empty(),
        "False positives on correct forms:\n{}",
        false_positives.join("\n")
    );
}

/// Gold incorrect forms must be detected. Enforced floor is 90%; current
/// implementation achieves 100%. Any regression below the floor is a bug
/// in either the correction table or pattern rules.
#[test]
fn gold_incorrect_forms_detected() {
    let data = load_gold();
    let all_entries = word_entries(&data);

    let mut detected = 0;
    let mut missed = Vec::new();
    let total = all_entries.len();

    for entry in &all_entries {
        match check_word(&entry.incorrect) {
            Some(_) => detected += 1,
            None => missed.push(format!("  {} → {}", entry.incorrect, entry.correct)),
        }
    }

    let detection_rate = detected as f64 / total as f64;
    assert!(
        detection_rate >= 0.90,
        "Detection rate {:.1}% ({}/{}) is below 90% threshold.\nMissed:\n{}",
        detection_rate * 100.0,
        detected,
        total,
        missed.join("\n")
    );

    eprintln!(
        "Gold detection: {}/{} ({:.1}%)",
        detected,
        total,
        detection_rate * 100.0
    );
}

/// Regression: misspellings that exist in the lexicon must still be caught.
/// The sabdasakha dictionary contains observed forms including common errors.
/// Academy rules are authoritative and must override lexicon presence.
#[test]
fn lexicon_present_misspellings_still_caught() {
    let cases = [
        ("राजनैतिक", "राजनीतिक"),
        ("अत्याधिक", "अत्यधिक"),
        ("उल्लेखित", "उल्लिखित"),
        ("व्यवहारिक", "व्यावहारिक"),
        ("पुनरावलोकन", "पुनरवलोकन"),
    ];
    for (incorrect, expected) in cases {
        let diag = check_word(incorrect);
        assert!(
            diag.is_some(),
            "'{incorrect}' should be flagged even though it may be in the lexicon"
        );
        let diag = diag.unwrap();
        assert_eq!(
            diag.correction, expected,
            "'{incorrect}' should correct to '{expected}', got '{}'",
            diag.correction
        );
    }
}

#[test]
fn gold_paragraph_correct_forms_no_false_positives() {
    let data = load_gold();
    let entries = paragraph_entries(&data);

    let mut false_positives = Vec::new();
    for entry in &entries {
        if is_phrase_like_form(&entry.correct) {
            let diags = check_text(&entry.correct);
            if !diags.is_empty() {
                false_positives.push(format!("  {} (flagged as {:?})", entry.correct, diags));
            }
        } else if let Some(diag) = check_word(&entry.correct) {
            false_positives.push(format!(
                "  {} (flagged as → {})",
                entry.correct, diag.correction
            ));
        }
    }

    assert!(
        false_positives.is_empty(),
        "False positives on correct paragraph forms:\n{}",
        false_positives.join("\n")
    );
}

#[test]
fn gold_paragraph_incorrect_forms_detected() {
    let data = load_gold();
    let entries = paragraph_entries(&data);

    let mut missed = Vec::new();
    for entry in &entries {
        let text_matched = check_text(&entry.incorrect)
            .iter()
            .any(|d| d.correction == entry.correct);
        let word_matched = if !is_phrase_like_form(&entry.incorrect) {
            check_word(&entry.incorrect)
                .map(|d| d.correction == entry.correct)
                .unwrap_or(false)
        } else {
            false
        };
        let matched = text_matched || word_matched;
        if !matched {
            missed.push(format!("  {} → {}", entry.incorrect, entry.correct));
        }
    }

    assert!(
        missed.is_empty(),
        "Missed paragraph-level corrections:\n{}",
        missed.join("\n")
    );
}
