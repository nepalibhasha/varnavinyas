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
    for template in NOTE_TEMPLATES {
        if template.origin == origin && marker_matches(word, template.marker) {
            notes.push(RuleNote {
                rule: template.rule,
                explanation: template.explanation.to_string(),
            });
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum NoteMarker {
    Always,
    ContainsChar(char),
    ContainsAny(&'static [char]),
    ContainsStr(&'static str),
    EndsWith(char),
    DentyaSaOnly,
}

#[derive(Debug, Clone, Copy)]
struct NoteTemplate {
    origin: Origin,
    marker: NoteMarker,
    rule: Rule,
    explanation: &'static str,
}

const NOTE_TEMPLATES: &[NoteTemplate] = &[
    // Tatsam
    NoteTemplate {
        origin: Origin::Tatsam,
        marker: NoteMarker::Always,
        rule: Rule::VarnaVinyasNiyam("3(क)"),
        explanation: "तत्सम शब्द: संस्कृतको मूल वर्णविन्यास कायम राख्नुपर्छ",
    },
    NoteTemplate {
        origin: Origin::Tatsam,
        marker: NoteMarker::ContainsAny(&['ऋ', 'ृ']),
        rule: Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
        explanation: "तत्सम शब्दमा ऋ/ृ संस्कृतबाट कायम",
    },
    NoteTemplate {
        origin: Origin::Tatsam,
        marker: NoteMarker::ContainsChar('ष'),
        rule: Rule::VarnaVinyasNiyam("3(ग)(अ)"),
        explanation: "तत्सम शब्दमा मूर्धन्य ष कायम",
    },
    NoteTemplate {
        origin: Origin::Tatsam,
        marker: NoteMarker::ContainsChar('श'),
        rule: Rule::VarnaVinyasNiyam("3(ग)(अ)"),
        explanation: "तत्सम शब्दमा तालव्य श कायम",
    },
    NoteTemplate {
        origin: Origin::Tatsam,
        marker: NoteMarker::ContainsAny(&['ी', 'ई']),
        rule: Rule::VarnaVinyasNiyam("3(क)(ई)"),
        explanation: "तत्सम शब्दमा दीर्घ ई/ी संस्कृतबाट कायम",
    },
    NoteTemplate {
        origin: Origin::Tatsam,
        marker: NoteMarker::ContainsAny(&['ू', 'ऊ']),
        rule: Rule::VarnaVinyasNiyam("3(क)(ऊ)"),
        explanation: "तत्सम शब्दमा दीर्घ ऊ/ू संस्कृतबाट कायम",
    },
    NoteTemplate {
        origin: Origin::Tatsam,
        marker: NoteMarker::ContainsStr("क्ष"),
        rule: Rule::VarnaVinyasNiyam("3(छ)-क्ष"),
        explanation: "तत्सम शब्दमा क्ष संयुक्त व्यञ्जन कायम",
    },
    NoteTemplate {
        origin: Origin::Tatsam,
        marker: NoteMarker::ContainsStr("ज्ञ"),
        rule: Rule::VarnaVinyasNiyam("3(छ)-ज्ञ"),
        explanation: "तत्सम शब्दमा ज्ञ संयुक्त व्यञ्जन कायम",
    },
    NoteTemplate {
        origin: Origin::Tatsam,
        marker: NoteMarker::ContainsAny(&['ङ', 'ञ', 'ण']),
        rule: Rule::VarnaVinyasNiyam("3(ख)-पञ्चम"),
        explanation: "तत्सम शब्दमा स्पर्श व्यञ्जन अघि पञ्चम वर्ण प्रयोग (Academy 3(ख)(अ))",
    },
    NoteTemplate {
        origin: Origin::Tatsam,
        marker: NoteMarker::EndsWith('्'),
        rule: Rule::VarnaVinyasNiyam("3(ङ)"),
        explanation: "तत्सम शब्दमा हलन्त चिह्न आवश्यक",
    },
    // Tadbhav
    NoteTemplate {
        origin: Origin::Tadbhav,
        marker: NoteMarker::Always,
        rule: Rule::VarnaVinyasNiyam("3(क)"),
        explanation: "तद्भव शब्द: संस्कृतबाट परिवर्तित, नेपाली ध्वनि नियम लागू",
    },
    NoteTemplate {
        origin: Origin::Tadbhav,
        marker: NoteMarker::ContainsAny(&['ि', 'ु']),
        rule: Rule::VarnaVinyasNiyam("3(क)-12"),
        explanation: "तद्भव शब्दमा ह्रस्व स्वर प्रयोग हुन्छ",
    },
    NoteTemplate {
        origin: Origin::Tadbhav,
        marker: NoteMarker::ContainsChar('ँ'),
        rule: Rule::VarnaVinyasNiyam("3(ख)"),
        explanation: "तद्भव शब्दमा चन्द्रबिन्दु (ँ) प्रयोग हुन्छ",
    },
    // Deshaj
    NoteTemplate {
        origin: Origin::Deshaj,
        marker: NoteMarker::Always,
        rule: Rule::VarnaVinyasNiyam("3(क)"),
        explanation: "देशज शब्द: मूल नेपाली शब्द, ह्रस्व नियम लागू",
    },
    // Aagantuk
    NoteTemplate {
        origin: Origin::Aagantuk,
        marker: NoteMarker::Always,
        rule: Rule::VarnaVinyasNiyam("3(ग)(अ)-9"),
        explanation: "आगन्तुक शब्द: विदेशी शब्दमा 'स' मात्र प्रयोग हुन्छ",
    },
    NoteTemplate {
        origin: Origin::Aagantuk,
        marker: NoteMarker::DentyaSaOnly,
        rule: Rule::VarnaVinyasNiyam("3(ग)(अ)-9"),
        explanation: "आगन्तुक शब्दमा दन्त्य स को शुद्ध प्रयोग",
    },
];

fn marker_matches(word: &str, marker: NoteMarker) -> bool {
    match marker {
        NoteMarker::Always => true,
        NoteMarker::ContainsChar(c) => word.contains(c),
        NoteMarker::ContainsAny(chars) => chars.iter().any(|c| word.contains(*c)),
        NoteMarker::ContainsStr(s) => word.contains(s),
        NoteMarker::EndsWith(c) => word.ends_with(c),
        NoteMarker::DentyaSaOnly => {
            word.contains('स') && !word.contains('श') && !word.contains('ष')
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tatsam_templates_emit_expected_notes() {
        let mut notes = Vec::new();
        generate_correct_notes("कृतीषशक्षज्ञण्", Origin::Tatsam, &mut notes);

        assert_eq!(notes.len(), 9);
        assert_eq!(notes[0].rule, Rule::VarnaVinyasNiyam("3(क)"));
        assert_eq!(notes[1].rule, Rule::VarnaVinyasNiyam("3(ग)-ऋ"));
        assert_eq!(notes[2].rule, Rule::VarnaVinyasNiyam("3(ग)(अ)"));
        assert_eq!(notes[3].rule, Rule::VarnaVinyasNiyam("3(ग)(अ)"));
        assert_eq!(notes[4].rule, Rule::VarnaVinyasNiyam("3(क)(ई)"));
        assert_eq!(notes[5].rule, Rule::VarnaVinyasNiyam("3(छ)-क्ष"));
        assert_eq!(notes[6].rule, Rule::VarnaVinyasNiyam("3(छ)-ज्ञ"));
        assert_eq!(notes[7].rule, Rule::VarnaVinyasNiyam("3(ख)-पञ्चम"));
        assert_eq!(notes[8].rule, Rule::VarnaVinyasNiyam("3(ङ)"));
    }

    #[test]
    fn tadbhav_templates_emit_expected_notes() {
        let mut notes = Vec::new();
        generate_correct_notes("हिँड्नु", Origin::Tadbhav, &mut notes);

        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].rule, Rule::VarnaVinyasNiyam("3(क)"));
        assert_eq!(notes[1].rule, Rule::VarnaVinyasNiyam("3(क)-12"));
        assert_eq!(notes[2].rule, Rule::VarnaVinyasNiyam("3(ख)"));
    }

    #[test]
    fn aagantuk_sa_note_requires_no_sha_or_ssa() {
        let mut notes = Vec::new();
        generate_correct_notes("साबुन", Origin::Aagantuk, &mut notes);
        assert_eq!(notes.len(), 2);

        let mut notes_with_sha = Vec::new();
        generate_correct_notes("शहर", Origin::Aagantuk, &mut notes_with_sha);
        assert_eq!(notes_with_sha.len(), 1);
    }
}
