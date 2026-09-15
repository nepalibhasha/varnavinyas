use varnavinyas_prakriya::{collect_rule_hits, derive};

#[test]
fn lexical_categories_are_not_inferred_from_another_words_ending() {
    for word in ["कि", "पछि", "गजानन", "प्रजनन", "अधिप्रजनन", "कालीन"]
    {
        assert!(
            collect_rule_hits(word).is_empty(),
            "{word}: {:?}",
            collect_rule_hits(word)
        );
    }
}

#[test]
fn supported_verb_and_suffix_rules_keep_their_specific_reasons() {
    for (input, output, citation) in [
        ("गरि", "गरी", "3(क)(ऊ)-14"),
        ("जान्छन", "जान्छन्", "3(ङ)-3"),
        ("गर्दैनन", "गर्दैनन्", "3(ङ)-3"),
        ("योगि", "योगी", "3(क)(ऊ)-1"),
        ("कमीटी", "कमिटी", "3(क)(आ)-5"),
    ] {
        let p = derive(input);
        assert_eq!(p.output, output, "{input}");
        assert_eq!(p.steps[0].rule.code(), citation, "{input}");
    }
}

#[test]
fn generic_ba_va_does_not_reverse_reviewed_forms_or_damage_inflections() {
    for word in [
        "बनेको",
        "आवाजमा",
        "जिम्मेवारीको",
        "आइतबारमा",
        "बुधबार",
        "बिनाविभागीय",
    ] {
        assert_eq!(derive(word).output, word);
    }
    for (input, expected) in [
        ("बिकास", "विकास"),
        ("विन्दु", "बिन्दु"),
        ("बुधवार", "बुधबार"),
        ("विनाविभागीय", "बिनाविभागीय"),
    ] {
        let output = derive(input).output;
        assert_eq!(output, expected);
        assert_eq!(
            derive(&output).output,
            output,
            "correction must remain stable: {input}"
        );
    }
}

#[test]
fn accepted_forms_explain_their_specific_spelling_without_a_correction() {
    for (word, code) in [("भाउजू", "3(क)(ऊ)-3"), ("जाऊ", "3(क)(ऊ)-10")] {
        let analysis = varnavinyas_prakriya::analyze(word);
        assert!(analysis.is_correct, "{analysis:?}");
        assert!(analysis.correction.is_none());
        assert_eq!(analysis.rule_notes[0].rule.code(), code);
        assert!(
            !analysis
                .rule_notes
                .iter()
                .any(|note| note.explanation.contains("ह्रस्व नियम लागू"))
        );
    }
    let unrelated = varnavinyas_prakriya::analyze("आवाज");
    assert!(
        !unrelated
            .rule_notes
            .iter()
            .any(|note| matches!(note.rule.code(), "3(क)(ऊ)-3" | "3(क)(ऊ)-10"))
    );
}

#[test]
fn root_and_imperative_spellings_do_not_correct_each_other() {
    for stem in [
        "हेर",
        "पढ",
        "भन",
        "उठ",
        "डुल",
        "हिँड",
        "देख",
        "गर",
        "बुझ",
        "लुक",
        "लेख",
    ] {
        for word in [stem.to_string(), format!("{stem}्")] {
            assert!(
                collect_rule_hits(&word).is_empty(),
                "{word}: {:?}",
                collect_rule_hits(&word)
            );
            assert_eq!(derive(&derive(&word).output).output, word);
            let a = varnavinyas_prakriya::analyze(&word);
            assert!(
                a.rule_notes
                    .iter()
                    .any(|n| n.rule.code() == "3(ङ)-1, 3(ङ)-अजन्त-3")
            );
        }
    }
    assert_eq!(derive("जान्छन").output, "जान्छन्");
    assert_eq!(derive("बाहिर्").output, "बाहिर");
}

#[test]
fn unknown_or_inferred_origin_does_not_generate_etymological_facts() {
    for word in ["कखगघङ", "नेपाले"] {
        let a = varnavinyas_prakriya::analyze(word);
        assert!(a.rule_notes.is_empty(), "{word}: {:?}", a.rule_notes);
        let api = varnavinyas_prakriya::ApiWordAnalysis::from(a);
        assert_eq!(api.origin, "unknown");
        assert_eq!(api.origin_source, "unknown");
        assert_eq!(api.origin_confidence, 0.0);
    }
    let documented = varnavinyas_prakriya::analyze("अध्ययन");
    assert!(!documented.rule_notes.is_empty());
}

#[test]
fn pronoun_alternates_require_independent_grammatical_evidence() {
    for input in ["तीमी", "तीनी", "यीनी", "ऊनी"] {
        assert_eq!(derive(input).steps[0].rule.code(), "3(क)(अ)-5", "{input}");
        assert!(
            collect_rule_hits(input)
                .iter()
                .all(|hit| hit.prakriya.steps[0].rule.code() != "3(क)(अ)-3")
        );
    }
    for (input, output, codes) in [
        ("हामि", "हामी", &["3(क)(ऊ)-7"][..]),
        ("तिमि", "तिमी", &["3(क)(ऊ)-7"][..]),
        ("यिनि", "यिनी", &["3(क)(ऊ)-7"][..]),
        // The headword explicitly documents तिन+ई, so keep this alternate.
        ("तिनि", "तिनी", &["3(क)(ऊ)-7", "3(क)(ऊ)-1"][..]),
        // उनी also has an adjective sense in the lexicon.
        ("उनि", "उनी", &["3(क)(ऊ)-7", "3(क)(ऊ)-8"][..]),
    ] {
        let hits = collect_rule_hits(input);
        assert_eq!(derive(input).output, output);
        let actual: Vec<_> = hits
            .iter()
            .map(|h| h.prakriya.steps[0].rule.code())
            .collect();
        assert_eq!(actual, codes, "{input}: {hits:?}");
    }
    for input in ["योगि", "त्यागि", "ज्ञानि", "घाति", "द्रोहि"]
    {
        assert!(
            collect_rule_hits(input).iter().any(|hit| hit
                .prakriya
                .steps
                .iter()
                .any(|step| step.rule.code() == "3(क)(ऊ)-1")),
            "{input}"
        );
    }
    assert_eq!(
        varnavinyas_prakriya::analyze("हामी").rule_notes[0]
            .rule
            .code(),
        "3(क)(ऊ)-7"
    );
    let adjective = collect_rule_hits("भौतीक");
    assert!(
        adjective.len() > 1,
        "distinct supported reasons must remain"
    );
}
