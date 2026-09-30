use varnavinyas_parikshak::{
    CheckOptions, DiagnosticCategory, OrthographyMode, check_text_with_options, check_word,
};
use varnavinyas_prakriya::{DiagnosticKind, derive};

#[test]
fn reviewed_final_ii_classes_are_errors_in_both_modes_and_keep_valid_forms_clean() {
    let data = include_str!("../../../data/rule_inventories/final_ii_semantic_classes.tsv");
    for row in data.lines().skip(1) {
        let correct = row.split('\t').next().unwrap();
        let wrong = format!("{}ि", correct.strip_suffix('ी').unwrap());
        let expected = derive(&wrong);
        let word = check_word(&wrong).expect(&wrong);
        assert_eq!(word.correction, correct, "{wrong}");
        assert_eq!(word.rule, expected.steps[0].rule, "{wrong}");
        for mode in [
            OrthographyMode::AcademyStrict,
            OrthographyMode::CommonEditorial,
        ] {
            let options = CheckOptions {
                orthography_mode: mode,
                ..CheckOptions::default()
            };
            let text = format!("🙂 {wrong}।");
            let diagnostics = check_text_with_options(&text, options);
            assert_eq!(diagnostics.len(), 1, "{text}: {diagnostics:?}");
            let d = &diagnostics[0];
            assert_eq!(d.correction, correct);
            assert_eq!(d.rule, word.rule);
            assert_eq!(d.kind, DiagnosticKind::Error);
            assert_eq!(&text[d.span.0..d.span.1], wrong);
            assert!(
                check_text_with_options(correct, options).is_empty(),
                "{correct}"
            );
        }
        assert!(
            check_word(correct).is_none(),
            "{correct}: {:?}",
            check_word(correct)
        );
    }
    for word in [
        "फर्सीको",
        "गाडीमा",
        "कोदालीले",
        "गोरीलाई",
        "कागतीहरू",
        "गाडीकोपनि",
    ] {
        assert!(check_word(word).is_none(), "{word}: {:?}", check_word(word));
    }
}

#[test]
fn whole_word_correction_survives_a_speculative_suffix_stack() {
    let expected = derive("मीलेको");
    assert_eq!(expected.output, "मिलेको");
    let word = check_word("मीलेको").expect("whole-word rule must not be bypassed");
    assert_eq!(word.correction, expected.output);
    assert_eq!(word.rule, expected.steps[0].rule);
    assert_eq!(word.kind, DiagnosticKind::Error);
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        let text = "🙂 मीलेको।";
        let diagnostics = check_text_with_options(
            text,
            CheckOptions {
                orthography_mode: mode,
                ..CheckOptions::default()
            },
        );
        assert_eq!(diagnostics.len(), 1, "{diagnostics:?}");
        assert_eq!(diagnostics[0].correction, "मिलेको");
        assert_eq!(&text[diagnostics[0].span.0..diagnostics[0].span.1], "मीलेको");
    }
}

#[test]
fn correct_words_and_supported_affix_stacks_remain_clean() {
    for word in [
        "मिलेको",
        "संसदमा",
        "विद्यालयमा",
        "रामकोपनि",
        "रामसम्मपनि",
        "तीन",
        "जून",
        "फूल",
    ] {
        assert!(check_word(word).is_none(), "{word}: {:?}", check_word(word));
        assert!(
            check_text_with_options(word, CheckOptions::default()).is_empty(),
            "{word}"
        );
    }
}

#[test]
fn school_grammar_final_u_examples_have_source_specific_corrections() {
    for (wrong, correct) in [("प्रभू", "प्रभु"), ("साधू", "साधु"), ("श्रद्धालू", "श्रद्धालु")]
    {
        let word = check_word(wrong).expect(wrong);
        assert_eq!(word.correction, correct);
        assert_eq!(word.rule.code(), "3(क)(इ)-PS-Saisanik-(छ)");
        assert!(check_word(correct).is_none(), "{correct}");
        let text = format!("🙂 {wrong}।");
        let diagnostics = check_text_with_options(&text, CheckOptions::default());
        assert_eq!(diagnostics.len(), 1, "{diagnostics:?}");
        assert_eq!(diagnostics[0].correction, correct);
        assert_eq!(&text[diagnostics[0].span.0..diagnostics[0].span.1], wrong);
    }
}

#[test]
fn reviewed_vadhu_conflict_is_an_error_in_both_modes() {
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        let diagnostics = check_text_with_options(
            "बधू",
            CheckOptions {
                orthography_mode: mode,
                ..CheckOptions::default()
            },
        );
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].correction, "वधू");
        assert_eq!(diagnostics[0].kind, DiagnosticKind::Error);
    }
}

#[test]
fn word_joining_preserves_intervening_punctuation() {
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        for text in [
            "तीर्थ, व्रत, सत्सङ्ग",
            "आख्यान साहित्य; कथा",
            "राम, सँग",
            "राम; को",
            "राम — सँग",
        ] {
            let diagnostics = check_text_with_options(
                text,
                CheckOptions {
                    orthography_mode: mode,
                    ..CheckOptions::default()
                },
            );
            assert!(
                !diagnostics
                    .iter()
                    .any(|d| d.category == DiagnosticCategory::ShuddhaTable
                        && d.incorrect.chars().any(|c| matches!(c, ',' | ';' | '—'))),
                "{text}: {diagnostics:?}"
            );
        }
        for text in ["राम सँग", "राम  सँग", "राम\tसँग"] {
            let diagnostics = check_text_with_options(
                text,
                CheckOptions {
                    orthography_mode: mode,
                    ..CheckOptions::default()
                },
            );
            assert!(
                diagnostics.iter().any(|d| d.correction == "रामसँग"),
                "{text}: {diagnostics:?}"
            );
        }
    }
}
