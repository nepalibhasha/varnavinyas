use crate::engine;
use crate::rule::Rule;
use varnavinyas_shabda::{Origin, OriginSource, classify_with_provenance, source_language};

/// Analysis of a word's orthography with origin-based explanations.
#[derive(Debug, Clone)]
pub struct WordAnalysis {
    /// The input word.
    pub word: String,
    /// The word's origin classification.
    pub origin: Origin,
    /// Provenance for origin classification (`override`, `kosha`, `heuristic`).
    pub origin_source: OriginSource,
    /// Confidence score for origin classification (0.0–1.0).
    pub origin_confidence: f32,
    /// Source language name (e.g., "फारसी", "अरबी", "संस्कृत"), if known.
    pub source_language: Option<String>,
    /// Whether the word's orthography is correct.
    pub is_correct: bool,
    /// Suggested correction, if any.
    pub correction: Option<String>,
    /// Explanatory notes citing Academy rules.
    pub rule_notes: Vec<RuleNote>,
}

/// An explanatory note about why a word's orthography is correct or incorrect.
#[derive(Debug, Clone)]
pub struct RuleNote {
    /// The Academy rule being cited.
    pub rule: Rule,
    /// Human-readable explanation in Nepali.
    pub explanation: String,
}

/// Analyze a word: derive its correction (if any) and generate explanatory rule notes.
///
/// Unlike `derive()`, this function also explains *why* a correct word is correct,
/// based on its origin classification and applicable Academy rules.
pub fn analyze(input: &str) -> WordAnalysis {
    if input.is_empty() {
        return WordAnalysis {
            word: String::new(),
            origin: Origin::Deshaj,
            origin_source: OriginSource::Heuristic,
            origin_confidence: 0.0,
            source_language: None,
            is_correct: true,
            correction: None,
            rule_notes: Vec::new(),
        };
    }

    let origin_decision = classify_with_provenance(input);
    let origin = origin_decision.origin;
    let source_lang = source_language(input).map(String::from);
    let prakriya = engine::derive(input);
    let mut rule_notes = Vec::new();

    if prakriya.is_correct {
        // Generate explanatory notes for why the word is correct
        generate_correct_notes(input, origin, &mut rule_notes);
    } else {
        // Generate notes explaining why the word is incorrect
        for step in &prakriya.steps {
            rule_notes.push(RuleNote {
                rule: step.rule,
                explanation: step.description.clone(),
            });
        }
    }

    WordAnalysis {
        word: input.to_string(),
        origin,
        origin_source: origin_decision.source,
        origin_confidence: origin_decision.confidence,
        source_language: source_lang,
        is_correct: prakriya.is_correct,
        correction: if prakriya.is_correct {
            None
        } else {
            Some(prakriya.output)
        },
        rule_notes,
    }
}

/// Generate explanatory notes for a word that is already correct.
fn generate_correct_notes(word: &str, origin: Origin, notes: &mut Vec<RuleNote>) {
    match origin {
        Origin::Tatsam => {
            notes.push(RuleNote {
                rule: Rule::VarnaVinyasNiyam("3(क)"),
                explanation: "तत्सम शब्द: संस्कृतको मूल वर्णविन्यास कायम राख्नुपर्छ".into(),
            });

            // Tatsam-specific observations
            if word.contains('ऋ') || word.contains('ृ') {
                notes.push(RuleNote {
                    rule: Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
                    explanation: "तत्सम शब्दमा ऋ/ृ संस्कृतबाट कायम".into(),
                });
            }
            if word.contains('ष') {
                notes.push(RuleNote {
                    rule: Rule::VarnaVinyasNiyam("3(ग)(अ)"),
                    explanation: "तत्सम शब्दमा मूर्धन्य ष कायम".into(),
                });
            }
            if word.contains('श') {
                notes.push(RuleNote {
                    rule: Rule::VarnaVinyasNiyam("3(ग)(अ)"),
                    explanation: "तत्सम शब्दमा तालव्य श कायम".into(),
                });
            }
            if word.contains('ी') || word.contains('ई') {
                notes.push(RuleNote {
                    rule: Rule::VarnaVinyasNiyam("3(क)(ई)"),
                    explanation: "तत्सम शब्दमा दीर्घ ई/ी संस्कृतबाट कायम".into(),
                });
            }
            if word.contains('ू') || word.contains('ऊ') {
                notes.push(RuleNote {
                    rule: Rule::VarnaVinyasNiyam("3(क)(ऊ)"),
                    explanation: "तत्सम शब्दमा दीर्घ ऊ/ू संस्कृतबाट कायम".into(),
                });
            }
            if word.contains("क्ष") {
                notes.push(RuleNote {
                    rule: Rule::VarnaVinyasNiyam("3(छ)-क्ष"),
                    explanation: "तत्सम शब्दमा क्ष संयुक्त व्यञ्जन कायम".into(),
                });
            }
            if word.contains("ज्ञ") {
                notes.push(RuleNote {
                    rule: Rule::VarnaVinyasNiyam("3(छ)-ज्ञ"),
                    explanation: "तत्सम शब्दमा ज्ञ संयुक्त व्यञ्जन कायम".into(),
                });
            }
            // Panchham varna in correct tatsam words
            if word.contains('ङ') || word.contains('ञ') || word.contains('ण') {
                notes.push(RuleNote {
                    rule: Rule::VarnaVinyasNiyam("3(ख)-पञ्चम"),
                    explanation: "तत्सम शब्दमा स्पर्श व्यञ्जन अघि पञ्चम वर्ण प्रयोग (Academy 3(ख)(अ))"
                        .into(),
                });
            }
            // Halanta in correct tatsam words
            if word.ends_with('्') {
                notes.push(RuleNote {
                    rule: Rule::VarnaVinyasNiyam("3(ङ)"),
                    explanation: "तत्सम शब्दमा हलन्त चिह्न आवश्यक".into(),
                });
            }
        }
        Origin::Tadbhav => {
            notes.push(RuleNote {
                rule: Rule::VarnaVinyasNiyam("3(क)"),
                explanation: "तद्भव शब्द: संस्कृतबाट परिवर्तित, नेपाली ध्वनि नियम लागू".into(),
            });

            if word.contains('ि') || word.contains('ु') {
                notes.push(RuleNote {
                    rule: Rule::VarnaVinyasNiyam("3(क)-12"),
                    explanation: "तद्भव शब्दमा ह्रस्व स्वर प्रयोग हुन्छ".into(),
                });
            }
            if word.contains('ँ') {
                notes.push(RuleNote {
                    rule: Rule::VarnaVinyasNiyam("3(ख)"),
                    explanation: "तद्भव शब्दमा चन्द्रबिन्दु (ँ) प्रयोग हुन्छ".into(),
                });
            }
        }
        Origin::Deshaj => {
            notes.push(RuleNote {
                rule: Rule::VarnaVinyasNiyam("3(क)"),
                explanation: "देशज शब्द: मूल नेपाली शब्द, ह्रस्व नियम लागू".into(),
            });
        }
        Origin::Aagantuk => {
            notes.push(RuleNote {
                rule: Rule::VarnaVinyasNiyam("3(ग)(अ)-9"),
                explanation: "आगन्तुक शब्द: विदेशी शब्दमा 'स' मात्र प्रयोग हुन्छ".into(),
            });

            if word.contains('स') && !word.contains('श') && !word.contains('ष') {
                notes.push(RuleNote {
                    rule: Rule::VarnaVinyasNiyam("3(ग)(अ)-9"),
                    explanation: "आगन्तुक शब्दमा दन्त्य स को शुद्ध प्रयोग".into(),
                });
            }
        }
    }
}
