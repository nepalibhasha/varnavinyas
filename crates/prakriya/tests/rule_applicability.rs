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
