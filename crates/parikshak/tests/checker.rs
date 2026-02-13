use varnavinyas_parikshak::{check_text, check_word};

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
