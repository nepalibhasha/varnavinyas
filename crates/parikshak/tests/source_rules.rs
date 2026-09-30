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

#[test]
fn suffix_splits_require_evidence_and_preserve_whole_word_corrections() {
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        let options = CheckOptions {
            orthography_mode: mode,
            ..CheckOptions::default()
        };
        for word in ["नापो", "टुप्पो", "झुप्पो", "पगरी", "नगरी", "ढोकामा", "भएकामा"]
        {
            let diagnostics = check_text_with_options(word, options);
            assert!(
                !diagnostics.iter().any(|d| d.correction.contains(' ')),
                "{word}: {diagnostics:?}"
            );
        }
        for (wrong, correct) in [("गित", "गीत"), ("बहिनि", "बहिनी")]
        {
            assert_eq!(check_word(wrong).unwrap().correction, correct);
            let text = format!("🙂 {wrong}।");
            let diagnostics = check_text_with_options(&text, options);
            let d = diagnostics
                .iter()
                .find(|d| d.incorrect == wrong)
                .expect(wrong);
            assert_eq!(d.correction, correct);
            assert_eq!(&text[d.span.0..d.span.1], wrong);
        }
        for (wrong, correct) in [
            ("ऊनि", "ऊ नि"),
            ("रामपो", "राम पो"),
            ("रामत", "राम त"),
            ("बुझिनेगरी", "बुझिने गरी"),
            ("ढिलोगरी", "ढिलो गरी"),
            ("रामकामा", "रामका मा"),
        ] {
            let ds = check_text_with_options(wrong, options);
            assert!(
                ds.iter().any(|d| d.correction == correct),
                "{wrong}: {ds:?}"
            );
        }
    }
}

#[test]
fn nasal_rules_keep_native_suffixes_and_require_supported_targets() {
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        let options = CheckOptions {
            orthography_mode: mode,
            ..CheckOptions::default()
        };
        for word in [
            "एवं",
            "गुरुसँग",
            "समाजसँग",
            "विज्ञानसुहाउँदो",
            "आँख्लाद्वारा",
            "आँखा",
            "पाँच",
            "भुईं",
        ] {
            assert!(derive(word).is_correct, "{word}: {:?}", derive(word));
            let ds = check_text_with_options(word, options);
            assert!(
                !ds.iter().any(|d| d.kind == DiagnosticKind::Error),
                "{word}: {ds:?}"
            );
        }
        for (wrong, correct) in [
            ("आंखा", "आँखा"),
            ("पांच", "पाँच"),
            ("सिँह", "सिंह"),
            ("सँवाद", "संवाद"),
            ("आउंदा", "आउँदा"),
            ("जान्छौं", "जान्छौँ"),
        ] {
            let p = derive(wrong);
            assert_eq!(p.output, correct, "{wrong}");
            let text = format!("🙂 {wrong}।");
            let ds = check_text_with_options(&text, options);
            let d = ds.iter().find(|d| d.incorrect == wrong).expect(wrong);
            assert_eq!(d.correction, correct, "{wrong}");
            assert_eq!(d.kind, DiagnosticKind::Error);
            assert_eq!(&text[d.span.0..d.span.1], wrong);
        }
    }
}

#[test]
fn initial_o_rule_preserves_distinct_u_verbs() {
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        let options = CheckOptions {
            orthography_mode: mode,
            ..CheckOptions::default()
        };
        for word in [
            "उड्ने",
            "उड्न",
            "उडे",
            "उर्लने",
            "उर्लन",
            "उड्दै",
            "उड्नु",
            "ओड्ने",
            "ओर्लने",
        ] {
            assert!(derive(word).is_correct, "{word}: {:?}", derive(word));
            assert!(
                !check_text_with_options(word, options)
                    .iter()
                    .any(|d| d.kind == DiagnosticKind::Error),
                "{word}"
            );
        }
        for (wrong, correct) in [("औज", "ओज"), ("औम्", "ओम्")] {
            assert_eq!(derive(wrong).output, correct);
        }
    }
}

#[test]
fn finite_halanta_endings_do_not_rewrite_lexical_nouns() {
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        let options = CheckOptions {
            orthography_mode: mode,
            ..CheckOptions::default()
        };
        for word in ["छनछन", "प्रजनन", "गजानन", "छन्"] {
            assert!(derive(word).is_correct, "{word}");
            assert!(
                !check_text_with_options(word, options)
                    .iter()
                    .any(|d| d.kind == DiagnosticKind::Error),
                "{word}"
            );
        }
        for (wrong, correct) in [("छन", "छन्"), ("गर्छन", "गर्छन्"), ("गर्दैनन", "गर्दैनन्")]
        {
            assert_eq!(derive(wrong).output, correct);
            assert!(
                check_text_with_options(wrong, options)
                    .iter()
                    .any(|d| d.correction == correct),
                "{wrong}"
            );
        }
    }
}

#[test]
fn gya_corrections_require_a_sanskrit_target_and_cite_its_rule() {
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        let options = CheckOptions {
            orthography_mode: mode,
            ..CheckOptions::default()
        };
        for (wrong, correct) in [("यग्य", "यज्ञ"), ("ग्यान", "ज्ञान"), ("अग्यान", "अज्ञान")]
        {
            let p = derive(wrong);
            assert_eq!(p.output, correct);
            assert!(matches!(
                p.steps[0].rule,
                varnavinyas_prakriya::Rule::VarnaVinyasNiyam("3(ग)(ऊ)-1")
            ));
            assert!(
                check_text_with_options(wrong, options)
                    .iter()
                    .any(|d| d.correction == correct),
                "{wrong}"
            );
        }
        for word in ["ग्यारेज", "ग्यारेन्टी", "ग्यास", "यज्ञ", "ज्ञान"]
        {
            assert!(derive(word).is_correct, "{word}");
            assert!(
                !check_text_with_options(word, options)
                    .iter()
                    .any(|d| d.kind == DiagnosticKind::Error),
                "{word}"
            );
        }
    }
}

#[test]
fn ri_corrections_validate_the_target_origin() {
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        let options = CheckOptions {
            orthography_mode: mode,
            ..CheckOptions::default()
        };
        for (wrong, correct) in [("द्रिष्टि", "दृष्टि"), ("क्रिति", "कृति"), ("रिषि", "ऋषि")]
        {
            assert_eq!(derive(wrong).output, correct, "{wrong}");
            assert!(
                check_text_with_options(wrong, options)
                    .iter()
                    .any(|d| d.correction == correct),
                "{wrong}"
            );
        }
        for word in [
            "क्रिया",
            "गिरि",
            "अरि",
            "रिक्त",
            "रित्तो",
            "रिबन",
            "रिस",
            "दृष्टि",
        ] {
            assert!(derive(word).is_correct, "{word}");
            assert!(
                !check_text_with_options(word, options)
                    .iter()
                    .any(|d| d.kind == DiagnosticKind::Error),
                "{word}"
            );
        }
    }
}

#[test]
fn ya_e_rules_cover_participles_and_pronoun_derivatives() {
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        let options = CheckOptions {
            orthography_mode: mode,
            ..CheckOptions::default()
        };
        for (wrong, correct) in [
            ("लियेको", "लिएको"),
            ("दियेको", "दिएको"),
            ("तेहाँ", "त्यहाँ"),
            ("तेता", "त्यता"),
        ] {
            assert_eq!(derive(wrong).output, correct, "{wrong}");
            assert!(
                check_text_with_options(wrong, options)
                    .iter()
                    .any(|d| d.correction == correct),
                "{wrong}"
            );
        }
        for word in [
            "भयको",
            "आयका",
            "छायाको",
            "लिएको",
            "दिएको",
            "त्यहाँ",
            "एकड",
            "यज्ञ",
        ] {
            assert!(derive(word).is_correct, "{word}");
            assert!(
                !check_text_with_options(word, options)
                    .iter()
                    .any(|d| d.kind == DiagnosticKind::Error),
                "{word}"
            );
        }
    }
}

#[test]
fn reviewed_short_vowel_targets_are_not_hidden_by_alias_headwords() {
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        let options = CheckOptions {
            orthography_mode: mode,
            ..CheckOptions::default()
        };
        for (wrong, correct) in [("ठूलो", "ठुलो"), ("सीप", "सिप")] {
            assert_eq!(derive(wrong).output, correct, "{wrong}");
            let ds = check_text_with_options(wrong, options);
            assert!(
                ds.iter()
                    .any(|d| d.correction == correct && d.kind == DiagnosticKind::Error),
                "{wrong}: {ds:?}"
            );
        }
        for word in ["ठुलो", "सिप", "तीन", "जून", "फूल", "औषधी"]
        {
            assert!(derive(word).is_correct, "{word}");
        }
    }
}

#[test]
fn reviewed_animate_noun_endings_survive_case_reattachment() {
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        let options = CheckOptions {
            orthography_mode: mode,
            ..CheckOptions::default()
        };
        for (wrong, correct) in [
            ("हात्तिको", "हात्तीको"),
            ("हात्तिलाई", "हात्तीलाई"),
            ("खसिको", "खसीको"),
            ("जोगिको", "जोगीको"),
        ] {
            assert_eq!(check_word(wrong).unwrap().correction, correct, "{wrong}");
            let text = format!("🙂 {wrong}।");
            let ds = check_text_with_options(&text, options);
            let d = ds.iter().find(|d| d.incorrect == wrong).expect(wrong);
            assert_eq!(d.correction, correct);
            assert_eq!(d.kind, DiagnosticKind::Error);
            assert_eq!(&text[d.span.0..d.span.1], wrong);
            assert!(
                check_text_with_options(correct, options).is_empty(),
                "{correct}"
            );
        }
        for word in ["पति", "रति", "गति", "कवि"] {
            assert!(derive(word).is_correct, "{word}");
        }
    }
}

#[test]
fn supported_verb_vowels_use_the_numbered_rules() {
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        let options = CheckOptions {
            orthography_mode: mode,
            ..CheckOptions::default()
        };
        for (wrong, correct, code) in [
            ("कीन्ने", "किन्ने", "3(क)(अ)-9"),
            ("सून्ने", "सुन्ने", "3(क)(अ)-9"),
            ("लीने", "लिने", "3(क)(अ)-9"),
            ("खेलीने", "खेलिने", "3(क)(आ)-8"),
            ("लिनूपर्ने", "लिनुपर्ने", "3(क)(इ)-8"),
            ("दिनूपर्ने", "दिनुपर्ने", "3(क)(इ)-8"),
        ] {
            let p = derive(wrong);
            assert_eq!(p.output, correct, "{wrong}");
            assert!(
                matches!(p.steps[0].rule, varnavinyas_prakriya::Rule::VarnaVinyasNiyam(found) if found == code),
                "{wrong}: {:?}",
                p.steps
            );
            let ds = check_text_with_options(wrong, options);
            assert!(
                ds.iter()
                    .any(|d| d.correction == correct && d.kind == DiagnosticKind::Error),
                "{wrong}: {ds:?}"
            );
            let hits = varnavinyas_prakriya::collect_rule_hits(wrong);
            assert_eq!(hits[0].prakriya.output, correct);
            assert!(
                !hits
                    .iter()
                    .any(|h| h.spec_id == Some("hd-tadbhav") && h.prakriya.output == correct)
            );
        }
        for word in [
            "खेलिने",
            "लिनुपर्ने",
            "दिनुपर्ने",
            "पढ्नू",
            "जानू",
            "जननी",
            "रानी",
            "भक्तिनी",
        ] {
            assert!(derive(word).is_correct, "{word}: {:?}", derive(word));
        }
    }
}
