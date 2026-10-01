#[cfg(feature = "grammar-pass")]
use varnavinyas_parikshak::{CheckOptions, DiagnosticKind, check_text_with_options};

#[cfg(feature = "grammar-pass")]
#[test]
fn grammar_pass_emits_variant_or_ambiguous_hints() {
    let text = "सूर्योदय भयो";
    let diags = check_text_with_options(
        text,
        CheckOptions {
            grammar: true,
            ..Default::default()
        },
    );
    assert!(
        diags
            .iter()
            .any(|d| matches!(d.kind, DiagnosticKind::Variant | DiagnosticKind::Ambiguous)),
        "Expected grammar-pass heuristic diagnostics, got: {diags:?}"
    );
}

#[cfg(feature = "grammar-pass")]
#[test]
fn grammar_pass_suppresses_low_confidence_plural_after_quantifier() {
    let text = "धेरै मानिसहरु आए।";
    let diags = check_text_with_options(
        text,
        CheckOptions {
            grammar: true,
            ..Default::default()
        },
    );

    assert!(
        !diags.iter().any(|d| {
            d.rule == varnavinyas_prakriya::Rule::Vyakaran("quantifier-plural-redundancy")
                && matches!(d.kind, DiagnosticKind::Variant)
        }),
        "Low-confidence suffix heuristic should be suppressed, got: {diags:?}"
    );
}

#[cfg(feature = "grammar-pass")]
#[test]
fn grammar_pass_suppresses_low_confidence_ergative_with_intransitive_predicate() {
    let text = "रामले गयो।";
    let diags = check_text_with_options(
        text,
        CheckOptions {
            grammar: true,
            ..Default::default()
        },
    );

    assert!(
        !diags.iter().any(|d| {
            d.rule == varnavinyas_prakriya::Rule::Vyakaran("ergative-le-intransitive")
                && matches!(d.kind, DiagnosticKind::Variant)
        }),
        "Low-confidence suffix heuristic should be suppressed, got: {diags:?}"
    );
}

#[cfg(feature = "grammar-pass")]
#[test]
fn grammar_pass_suppresses_low_confidence_genitive_mismatch_before_plural() {
    let text = "रामको किताबहरु हराए।";
    let diags = check_text_with_options(
        text,
        CheckOptions {
            grammar: true,
            ..Default::default()
        },
    );

    assert!(
        !diags.iter().any(|d| {
            d.rule == varnavinyas_prakriya::Rule::Vyakaran("genitive-mismatch-plural")
                && matches!(d.kind, DiagnosticKind::Variant)
        }),
        "Low-confidence suffix heuristic should be suppressed, got: {diags:?}"
    );
}

#[cfg(feature = "grammar-pass")]
#[test]
fn grammar_pass_does_not_promote_lexical_coincidences_or_derivations() {
    use varnavinyas_parikshak::OrthographyMode;
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        for word in [
            "सवारीमा",
            "दशकमा",
            "आयात",
            "आर्थिक",
            "विकास",
            "यातायात",
            "व्यवस्थापन",
            "रिसाइकल",
        ] {
            let text = format!("🙂 {word} सम्बन्धी जानकारी उपलब्ध छ।");
            let diagnostics = check_text_with_options(
                &text,
                CheckOptions {
                    grammar: true,
                    orthography_mode: mode,
                    ..Default::default()
                },
            );
            assert!(
                !diagnostics
                    .iter()
                    .any(|d| d.rule.code() == "samasa-heuristic"),
                "{word}: {diagnostics:?}"
            );
        }
    }
}

#[cfg(feature = "grammar-pass")]
#[test]
fn grammar_pass_preserves_reviewed_compounds_and_detached_suffixes() {
    use varnavinyas_parikshak::OrthographyMode;
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        for (word, composition) in [
            ("सूर्योदय", "सूर्य + उदय"),
            ("सूर्योदयमा", "सूर्य + उदय + मा"),
            ("मापदण्ड", "माप + दण्ड"),
            ("मापदण्डको", "माप + दण्ड + को"),
            ("पूर्वाधार", "पूर्व + आधार"),
        ] {
            let text = format!("🙂 {word}");
            let diagnostics = check_text_with_options(
                &text,
                CheckOptions {
                    grammar: true,
                    orthography_mode: mode,
                    ..Default::default()
                },
            );
            let diagnostic = diagnostics
                .iter()
                .find(|d| d.rule.code() == "samasa-heuristic")
                .unwrap_or_else(|| panic!("{word}: missing supported analysis: {diagnostics:?}"));
            assert_eq!(diagnostic.incorrect, word);
            assert_eq!(diagnostic.correction, composition);
            assert_eq!(&text[diagnostic.span.0..diagnostic.span.1], word);
            assert_eq!(diagnostic.kind, DiagnosticKind::Variant);
            if word.ends_with("मा") || word.ends_with("को") {
                assert!(diagnostic.explanation.contains("बाहिरी प्रत्यय/निपात:"));
            }
            assert!(
                !check_text_with_options(
                    &text,
                    CheckOptions {
                        grammar: false,
                        orthography_mode: mode,
                        ..Default::default()
                    }
                )
                .iter()
                .any(|d| d.rule.code() == "samasa-heuristic")
            );
        }
    }
}
