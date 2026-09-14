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
