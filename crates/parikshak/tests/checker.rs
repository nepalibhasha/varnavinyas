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
