use varnavinyas_parikshak::{
    CheckOptions, DiagnosticKind, PunctuationMode, check_text, check_text_with_options, check_word,
};

/// C1: Paragraph with known incorrect words produces diagnostics.
#[test]
fn c1_paragraph_with_errors() {
    let text = "अत्याधिक राजनैतिक प्रशाशन भयो।";
    let diags = check_text(text);
    assert!(
        !diags.is_empty(),
        "Should detect errors in paragraph with known incorrect words"
    );
    // At least some of these should be caught
    let corrections: Vec<&str> = diags.iter().map(|d| d.correction.as_str()).collect();
    // अत्याधिक → अत्यधिक is in the correction table
    assert!(
        corrections.contains(&"अत्यधिक"),
        "Should correct अत्याधिक → अत्यधिक, got: {corrections:?}"
    );
}

/// C2: Paragraph with all correct words produces no word diagnostics.
#[test]
fn c2_correct_paragraph() {
    let text = "नेपाल राम्रो देश हो। यहाँ हिमाल छ।";
    let diags = check_text(text);
    assert!(
        diags.is_empty(),
        "Correct text should have no diagnostics, got: {diags:?}"
    );
}

#[test]
fn attested_sibling_inflection_allows_unlisted_case_form() {
    let diags = check_text("मच्छिन्द्रनाथको मन्दिर");
    assert!(
        diags.is_empty(),
        "Attested sibling inflections should allow supported case forms, got: {diags:?}"
    );
}

/// C3: Diagnostics have span, correction, rule, explanation.
#[test]
fn c3_diagnostic_fields() {
    let diag = check_word("अत्याधिक");
    assert!(diag.is_some(), "अत्याधिक should produce a diagnostic");
    let diag = diag.unwrap();
    assert_eq!(diag.incorrect, "अत्याधिक");
    assert_eq!(diag.correction, "अत्यधिक");
    assert!(!diag.explanation.is_empty());
    // Span should cover the word
    assert_eq!(diag.span.0, 0);
    assert_eq!(diag.span.1, "अत्याधिक".len());
}

/// C4: Multi-paragraph handling.
#[test]
fn c4_multi_paragraph() {
    // Use two known correction-table entries across paragraphs
    let text = "अत्याधिक काम भयो।\n\nउल्लेखित कुरा छ।";
    let diags = check_text(text);
    assert!(
        diags.len() >= 2,
        "Should find errors in both paragraphs, got {} diagnostics: {:?}",
        diags.len(),
        diags
            .iter()
            .map(|d| format!("{} → {}", d.incorrect, d.correction))
            .collect::<Vec<_>>()
    );
}

/// C5: Performance — 100 words should process quickly.
#[test]
fn c5_performance() {
    let word = "नेपाल";
    let text = std::iter::repeat(word)
        .take(100)
        .collect::<Vec<_>>()
        .join(" ");
    let start = std::time::Instant::now();
    let _ = check_text(&text);
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 5000,
        "100 words should process in <5s, took {}ms",
        elapsed.as_millis()
    );
}

/// C6: No false positives on correct gold.toml forms.
#[test]
fn c6_no_false_positives_on_correct() {
    let correct_words = [
        "अत्यधिक",
        "राजनीतिक",
        "उल्लिखित",
        "प्रशासन",
        "नेपाल",
        "भाषा",
        "शिक्षा",
        "विकास",
    ];
    for word in correct_words {
        let diag = check_word(word);
        assert!(
            diag.is_none(),
            "Correct word '{word}' should not produce a diagnostic, got: {diag:?}"
        );
    }
}

#[test]
fn adhyan_misspelling_is_flagged_as_error() {
    let diag = check_word("अध्यन");
    assert!(
        diag.is_some(),
        "Known misspelling अध्यन should produce a diagnostic"
    );
    let diag = diag.unwrap();
    assert!(
        matches!(diag.kind, DiagnosticKind::Error),
        "अध्यन should be an Error (correction table), got: {:?}",
        diag.kind
    );
    assert_eq!(diag.incorrect, "अध्यन");
    assert_eq!(diag.correction, "अध्ययन");
}

#[test]
fn unknown_simple_word_remains_unflagged() {
    let diag = check_word("झ्क्ष्ट्र्व्ङ");
    assert!(
        diag.is_none(),
        "Simple unknown forms should remain unflagged to avoid noisy false positives"
    );
}

#[test]
fn common_mula_avyaya_are_not_overcorrected() {
    for word in ["पनि", "अनि"] {
        let diag = check_word(word);
        assert!(
            diag.is_none(),
            "Common मूल अव्यय '{word}' should not be overcorrected, got: {diag:?}"
        );
    }
}

#[test]
fn chandrabindu_word_gets_chandrabindu_category() {
    let diag = check_word("आउछ").expect("Expected diagnostic for आउछ");
    assert_eq!(diag.correction, "आउँछ");
    assert_eq!(
        diag.category,
        varnavinyas_parikshak::DiagnosticCategory::Chandrabindu
    );
}

#[test]
fn halanta_word_gets_halanta_category() {
    let diag = check_word("महान").expect("Expected diagnostic for महान");
    assert_eq!(diag.correction, "महान्");
    assert_eq!(
        diag.category,
        varnavinyas_parikshak::DiagnosticCategory::Halanta
    );
}

#[test]
fn ba_va_word_gets_ba_va_category() {
    let diag = check_word("बिकास").expect("Expected diagnostic for बिकास");
    assert_eq!(diag.correction, "विकास");
    assert_eq!(
        diag.category,
        varnavinyas_parikshak::DiagnosticCategory::BaVa
    );
}

#[test]
fn tatsam_padanta_halanta_beats_edit_distance() {
    let diag = check_word("जगत").expect("Expected diagnostic for जगत");
    assert_eq!(diag.correction, "जगत्");
    assert_eq!(
        diag.category,
        varnavinyas_parikshak::DiagnosticCategory::Halanta
    );
    assert!(
        !matches!(diag.kind, DiagnosticKind::Ambiguous),
        "जगत should be explained by halanta restoration, not edit-distance fallback: {diag:?}"
    );
}

#[test]
fn ya_e_correction_gets_ya_e_category() {
    let diag = check_word("यकता").expect("Expected diagnostic for यकता");
    assert_eq!(diag.correction, "एकता");
    assert_eq!(
        diag.category,
        varnavinyas_parikshak::DiagnosticCategory::YaE
    );
}

#[test]
fn ri_kri_correction_gets_ri_kri_category() {
    let diag = check_word("रिषि").expect("Expected diagnostic for रिषि");
    assert_eq!(diag.correction, "ऋषि");
    assert_eq!(
        diag.category,
        varnavinyas_parikshak::DiagnosticCategory::RiKri
    );
}

#[test]
fn aadhi_vriddhi_correction_gets_aadhi_vriddhi_category() {
    let diag = check_word("व्यवहारिक").expect("Expected diagnostic for व्यवहारिक");
    assert_eq!(diag.correction, "व्यावहारिक");
    assert_eq!(
        diag.category,
        varnavinyas_parikshak::DiagnosticCategory::AadhiVriddhi
    );
}

#[test]
fn attested_non_derivative_word_is_not_flagged_by_aadhi_vriddhi() {
    let diag = check_word("अधिक");
    assert!(
        diag.is_none(),
        "अधिक should not be flagged by aadhi-vriddhi, got: {diag:?}"
    );
}

/// Punctuation diagnostics integrated into check_text.
#[test]
fn punctuation_in_check_text() {
    let text = "नेपाल राम्रो देश हो.";
    let diags = check_text(text);
    let punct_diags: Vec<_> = diags
        .iter()
        .filter(|d| d.category == varnavinyas_parikshak::DiagnosticCategory::Punctuation)
        .collect();
    assert_eq!(
        punct_diags.len(),
        1,
        "Should detect period misuse, got: {punct_diags:?}"
    );
    assert!(
        punct_diags
            .iter()
            .all(|d| matches!(d.kind, DiagnosticKind::Error)),
        "Default punctuation mode should emit errors, got: {punct_diags:?}"
    );
}

#[test]
fn spelling_inside_smart_quotes_is_detected() {
    let text = "“अत्याधिक”";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "अत्याधिक" && d.correction == "अत्यधिक"),
        "Expected spelling diagnostic inside smart quotes, got: {diags:?}"
    );
}

#[test]
fn punctuation_normalized_editorial_emits_variant() {
    let text = "नेपाल राम्रो देश हो.";
    let diags = check_text_with_options(
        text,
        CheckOptions {
            punctuation_mode: PunctuationMode::NormalizedEditorial,
            ..Default::default()
        },
    );
    let punct_diags: Vec<_> = diags
        .iter()
        .filter(|d| d.category == varnavinyas_parikshak::DiagnosticCategory::Punctuation)
        .collect();
    assert_eq!(
        punct_diags.len(),
        1,
        "Should detect punctuation even in normalized-editorial mode, got: {punct_diags:?}"
    );
    assert!(
        punct_diags
            .iter()
            .all(|d| matches!(d.kind, DiagnosticKind::Variant)),
        "Normalized-editorial punctuation should be style variants, got: {punct_diags:?}"
    );
}

/// Regression test: ensure suffix is preserved in correction string.
/// "बिज्ञानमा" -> stem "बिज्ञान" (wrong) + suffix "मा".
/// Correction should be "विज्ञान" + "मा" = "विज्ञानमा".
#[test]
fn suffix_preservation_in_correction() {
    let text = "बिज्ञानमा";
    let diags = check_text(text);
    assert_eq!(diags.len(), 1);
    let diag = &diags[0];

    // The critical check:
    assert_eq!(diag.incorrect, "बिज्ञानमा");
    assert_eq!(diag.correction, "विज्ञानमा");
    assert_eq!(diag.span.1 - diag.span.0, text.len());
}

#[test]
fn check_text_with_default_options_matches_check_text() {
    let text = "अत्याधिक राजनैतिक प्रशाशन भयो।";
    let a = check_text(text);
    let b = varnavinyas_parikshak::check_text_with_options(
        text,
        varnavinyas_parikshak::CheckOptions::default(),
    );
    assert_eq!(a.len(), b.len());
}

#[test]
fn padayog_phrase_join_detected() {
    let text = "म सँग पुस्तक छ।";
    let diags = check_text(text);
    let hit = diags
        .iter()
        .find(|d| d.incorrect == "म सँग")
        .expect("Expected padayog diagnostic for 'म सँग'");
    assert_eq!(hit.correction, "मसँग");
}

#[test]
fn padayog_phrase_multiple_detected() {
    let text = "आज्ञा अनुसार काम गर। तिमी भन्दा ऊ छिटो आयो।";
    let diags = check_text(text);

    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "आज्ञा अनुसार" && d.correction == "आज्ञाअनुसार"),
        "Expected 'आज्ञा अनुसार' -> 'आज्ञाअनुसार', got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "तिमी भन्दा" && d.correction == "तिमीभन्दा"),
        "Expected 'तिमी भन्दा' -> 'तिमीभन्दा', got: {diags:?}"
    );
}

#[test]
fn padayog_ota_varga_sambandhi_detected() {
    let text = "तीन ओटा शिक्षक वर्ग र ज्ञान सम्बन्धी छलफल भयो।";
    let diags = check_text(text);

    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "तीन ओटा" && d.correction == "तीनओटा"),
        "Expected 'तीन ओटा' -> 'तीनओटा', got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "शिक्षक वर्ग" && d.correction == "शिक्षकवर्ग"),
        "Expected 'शिक्षक वर्ग' -> 'शिक्षकवर्ग', got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "ज्ञान सम्बन्धी" && d.correction == "ज्ञानसम्बन्धी"),
        "Expected 'ज्ञान सम्बन्धी' -> 'ज्ञानसम्बन्धी', got: {diags:?}"
    );
}

#[test]
fn padabiyog_split_cases_detected() {
    let text = "देशकालागि काम गरियो। मेरानिम्ति खबर छ। रामकामा लेखिएको छ।";
    let diags = check_text(text);

    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "देशकालागि" && d.correction == "देशका लागि"),
        "Expected 'देशकालागि' -> 'देशका लागि', got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "मेरानिम्ति" && d.correction == "मेरा निम्ति"),
        "Expected 'मेरानिम्ति' -> 'मेरा निम्ति', got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "रामकामा" && d.correction == "रामका मा"),
        "Expected 'रामकामा' -> 'रामका मा', got: {diags:?}"
    );
}

#[test]
fn padayog_padabiyog_notice_subrules_sampled() {
    let text = "राम ले देश को कुरा गर्‍यो, किन भने देशकालागि चारजना थिए र पढ्नुनैपर्छ भन्यो।";
    let diags = check_text(text);

    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "राम ले" && d.correction == "रामले"),
        "Expected 3(घ)-पदयोग-३ sample correction, got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "देश को" && d.correction == "देशको"),
        "Expected 3(घ)-पदयोग-३ sample correction, got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "किन भने" && d.correction == "किनभने"),
        "Expected 3(घ)-पदयोग-९ sample correction, got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "देशकालागि" && d.correction == "देशका लागि"),
        "Expected 3(घ)-पदवियोग-३ sample correction, got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "चारजना" && d.correction == "चार जना"),
        "Expected 3(घ)-पदवियोग-११ sample correction, got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "पढ्नुनैपर्छ" && d.correction == "पढ्नु नै पर्छ"),
        "Expected 3(घ)-पदवियोग-८ sample correction, got: {diags:?}"
    );
}

#[test]
fn generalized_padayog_vibhakti_join_applies_beyond_fixed_pairs() {
    let text = "विद्यालय मा कार्यक्रम छ।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "विद्यालय मा" && d.correction == "विद्यालयमा"),
        "Expected generalized 3(घ)-पदयोग-३ join, got: {diags:?}"
    );
}

#[test]
fn generalized_padabiyog_vibhakti_split_applies_beyond_fixed_pairs() {
    let text = "हाम्रालागि यो समाजकानिम्ति राम्रो हो।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "हाम्रालागि" && d.correction == "हाम्रा लागि"),
        "Expected generalized 3(घ)-पदवियोग-३ split for लागि, got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "समाजकानिम्ति" && d.correction == "समाजका निम्ति"),
        "Expected generalized 3(घ)-पदवियोग-३ split for निम्ति, got: {diags:?}"
    );
}

#[test]
fn generalized_padayog_conjunction_join_handles_variable_spacing() {
    let text = "म जान्नँ, किन   भने समय थिएन।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "किन   भने" && d.correction == "किनभने"),
        "Expected generalized 3(घ)-पदयोग-९ join with variable spacing, got: {diags:?}"
    );
}

#[test]
fn generalized_padabiyog_verb_complex_split_applies_beyond_fixed_pairs() {
    let text = "उनी खेल्दैछन् र भोलि हिँड्नेछन् ।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "खेल्दैछन्" && d.correction == "खेल्दै छन्"),
        "Expected generalized 3(घ)-पदवियोग-६ split, got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "हिँड्नेछन्" && d.correction == "हिँड्ने छन्"),
        "Expected generalized 3(घ)-पदवियोग-७ split, got: {diags:?}"
    );
}

#[test]
fn section4_style_variants_are_opt_in() {
    let text = "कार्यक्रमको सम्बन्धमा छलफल भयो।";

    let off = check_text(text);
    assert!(
        off.iter()
            .all(|d| d.rule != varnavinyas_prakriya::Rule::Vyakaran("section4-phrase-style")),
        "Style variants should not appear in default mode, got: {off:?}"
    );

    let on = check_text_with_options(
        text,
        CheckOptions {
            grammar: true,
            ..Default::default()
        },
    );
    assert!(
        on.iter().any(|d| {
            d.rule == varnavinyas_prakriya::Rule::Vyakaran("section4-phrase-style")
                && d.correction == "कार्यक्रमका सम्बन्धमा"
                && matches!(d.kind, DiagnosticKind::Variant)
        }),
        "Expected style variant when grammar mode is enabled, got: {on:?}"
    );
}

#[test]
fn section4_sentence_style_variant_detected() {
    let text = "यहाँको सहयोगप्रति म कृतघ्न छु।";
    let diags = check_text_with_options(
        text,
        CheckOptions {
            grammar: true,
            ..Default::default()
        },
    );

    assert!(
        diags.iter().any(|d| {
            d.rule == varnavinyas_prakriya::Rule::Vyakaran("section4-phrase-style")
                && d.correction == "यहाँको सहयोगप्रति म कृतज्ञ छु"
                && matches!(d.kind, DiagnosticKind::Variant)
        }),
        "Expected कृतघ्न/कृतज्ञ style suggestion, got: {diags:?}"
    );
}

#[test]
fn section4_phrase_variant_marmahat() {
    let text = "उनी मर्माहित भएको देखिन्थ्यो।";
    let diags = check_text_with_options(
        text,
        CheckOptions {
            grammar: true,
            ..Default::default()
        },
    );

    assert!(
        diags.iter().any(|d| {
            d.rule == varnavinyas_prakriya::Rule::Vyakaran("section4-phrase-style")
                && d.correction == "मर्माहत भएको"
                && matches!(d.kind, DiagnosticKind::Variant)
        }),
        "Expected मर्माहित/मर्माहत style suggestion, got: {diags:?}"
    );
}

#[test]
fn section4_sentence_word_order_variant_detected() {
    let text = "म अब कार्यक्रम सञ्चालन गर्न गइरहेको छु वा जाँदै छु।";
    let diags = check_text_with_options(
        text,
        CheckOptions {
            grammar: true,
            ..Default::default()
        },
    );

    assert!(
        diags.iter().any(|d| {
            d.rule == varnavinyas_prakriya::Rule::Vyakaran("section4-phrase-style")
                && d.correction == "म अब कार्यक्रम सञ्चालन गर्दै छु"
                && matches!(d.kind, DiagnosticKind::Variant)
        }),
        "Expected sentence-style suggestion, got: {diags:?}"
    );
}

#[test]
fn section4_complex_sentence_variant_detected() {
    let text = "स्थानीय जनशक्तिको श्रमदानबाट दश किलोमिटर लामो गाडी गुड्न सक्ने सडक निर्माण गरियो।";
    let diags = check_text_with_options(
        text,
        CheckOptions {
            grammar: true,
            ..Default::default()
        },
    );

    assert!(
        diags.iter().any(|d| {
            d.rule == varnavinyas_prakriya::Rule::Vyakaran("section4-phrase-style")
                && d.correction
                    == "स्थानीय जनशक्तिको श्रमदानबाट गाडी गुड्न सक्ने दश किलोमिटर लामो सडक निर्माण गरियो"
                && matches!(d.kind, DiagnosticKind::Variant)
        }),
        "Expected complex sentence style suggestion, got: {diags:?}"
    );
}

#[test]
fn nga_halanta_lemma_rule_is_suppressed_in_imperative_sentence_context() {
    let text = "कृपया भन।";
    let diags = check_text(text);
    assert!(
        diags.iter().any(|d| {
            d.incorrect == "भन"
                && d.correction == "भन्"
                && matches!(d.kind, DiagnosticKind::Ambiguous)
        }),
        "Imperative sentence context should surface ambiguous भन -> भन् guidance, got: {diags:?}"
    );
}

#[test]
fn nga_halanta_lemma_rule_still_applies_for_standalone_token() {
    let text = "भन";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "भन" && d.correction == "भन्"),
        "Standalone token should still allow भन -> भन् lemma suggestion, got: {diags:?}"
    );
}
