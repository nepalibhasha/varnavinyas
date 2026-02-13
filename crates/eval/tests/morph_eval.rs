//! Morphology evaluation against curated morph_gold.toml.
//!
//! Run:
//! `cargo test -p varnavinyas-eval --test morph_eval -- --nocapture`

use serde::Deserialize;
use varnavinyas_vyakaran::{Case, MorphAnalyzer, Number, Person, RuleBasedAnalyzer, Tense};

#[derive(Debug, Deserialize)]
struct MorphGold {
    morph: Vec<MorphEntry>,
}

#[derive(Debug, Deserialize)]
struct MorphEntry {
    word: String,
    lemma: Option<String>,
    prefix: Option<String>,
    suffix: Option<String>,
    case: Option<String>,
    number: Option<String>,
    tense: Option<String>,
    person: Option<String>,
}

#[test]
fn morph_gold_coverage() {
    let data = include_str!("../../../docs/tests/morph_gold.toml");
    let gold: MorphGold = toml::from_str(data).expect("morph_gold.toml must parse");

    let analyzer = RuleBasedAnalyzer;
    let total = gold.morph.len();
    let mut matched = 0usize;
    let mut misses: Vec<String> = Vec::new();

    println!("\n=== Morph Gold Evaluation ===");

    for entry in &gold.morph {
        let exp_case = entry.case.as_deref().and_then(parse_case);
        let exp_number = entry.number.as_deref().and_then(parse_number);
        let exp_tense = entry.tense.as_deref().and_then(parse_tense);
        let exp_person = entry.person.as_deref().and_then(parse_person);

        let analyses = analyzer
            .analyze(&entry.word)
            .unwrap_or_else(|e| panic!("analysis failed for '{}': {}", entry.word, e));

        let ok = analyses.iter().any(|a| {
            entry.lemma.as_deref().is_none_or(|v| a.lemma == v)
                && entry
                    .prefix
                    .as_deref()
                    .is_none_or(|v| a.prefix.as_deref() == Some(v))
                && entry
                    .suffix
                    .as_deref()
                    .is_none_or(|v| a.suffix.as_deref() == Some(v))
                && exp_case.is_none_or(|v| a.features.case == Some(v))
                && exp_number.is_none_or(|v| a.features.number == Some(v))
                && exp_tense.is_none_or(|v| a.features.tense == Some(v))
                && exp_person.is_none_or(|v| a.features.person == Some(v))
        });

        if ok {
            matched += 1;
            println!("  ✓ {}", entry.word);
        } else {
            let sample = analyses
                .iter()
                .take(3)
                .map(|a| {
                    format!(
                        "lemma={} prefix={:?} suffix={:?} case={:?} number={:?} tense={:?} person={:?}",
                        a.lemma,
                        a.prefix,
                        a.suffix,
                        a.features.case,
                        a.features.number,
                        a.features.tense,
                        a.features.person
                    )
                })
                .collect::<Vec<_>>()
                .join(" | ");
            println!("  ✗ {} (got: {})", entry.word, sample);
            misses.push(entry.word.clone());
        }
    }

    let coverage = matched as f64 / total as f64;
    println!("\nTotal:    {}", total);
    println!("Matched:  {} ({:.1}%)", matched, coverage * 100.0);

    assert!(
        coverage >= 0.88,
        "Morph coverage too low ({:.1}%). Misses: {:?}",
        coverage * 100.0,
        misses
    );
}

fn parse_case(s: &str) -> Option<Case> {
    match s {
        "Nominative" => Some(Case::Nominative),
        "Accusative" => Some(Case::Accusative),
        "Instrumental" => Some(Case::Instrumental),
        "Dative" => Some(Case::Dative),
        "Ablative" => Some(Case::Ablative),
        "Genitive" => Some(Case::Genitive),
        "Locative" => Some(Case::Locative),
        "Vocative" => Some(Case::Vocative),
        _ => None,
    }
}

fn parse_number(s: &str) -> Option<Number> {
    match s {
        "Singular" => Some(Number::Singular),
        "Plural" => Some(Number::Plural),
        _ => None,
    }
}

fn parse_tense(s: &str) -> Option<Tense> {
    match s {
        "Present" => Some(Tense::Present),
        "Past" => Some(Tense::Past),
        "Future" => Some(Tense::Future),
        "Unknown" => Some(Tense::Unknown),
        _ => None,
    }
}

fn parse_person(s: &str) -> Option<Person> {
    match s {
        "First" => Some(Person::First),
        "Second" => Some(Person::Second),
        "Third" => Some(Person::Third),
        _ => None,
    }
}
