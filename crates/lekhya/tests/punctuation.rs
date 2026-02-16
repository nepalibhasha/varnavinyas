use varnavinyas_lekhya::check_punctuation;

/// Y1: Detect period used as sentence-end instead of purna viram.
#[test]
fn y1_period_as_sentence_end() {
    let diags = check_punctuation("नेपाली भाषा राम्रो छ.");
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].found, ".");
    assert_eq!(diags[0].expected, "।");
    assert!(diags[0].rule.contains("पूर्णविराम"));
}

/// Y1: Multiple sentences with periods.
#[test]
fn y1_multiple_periods() {
    let diags = check_punctuation("नेपाल राम्रो देश हो. यहाँ हिमाल छ.");
    assert_eq!(diags.len(), 2);
}

/// Y2: Correct Nepali text should produce no diagnostics.
#[test]
fn y2_correct_punctuation() {
    let diags = check_punctuation("नेपाल राम्रो देश हो। यहाँ हिमाल छ।");
    assert!(diags.is_empty());
}

/// Y3: Ellipsis (ASCII dots) should be flagged.
#[test]
fn y3_ellipsis_detection() {
    let diags = check_punctuation("उसले भन्यो... म जान्छु।");
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].found, "...");
    assert_eq!(diags[0].expected, "…");
}

/// Y4: Empty and whitespace-only inputs should produce no diagnostics.
#[test]
fn y4_empty_input() {
    assert!(check_punctuation("").is_empty());
    assert!(check_punctuation("   ").is_empty());
}

/// Mixed Nepali and English text — only Nepali periods flagged.
#[test]
fn mixed_text() {
    let diags = check_punctuation("He said hello. उसले भन्यो.");
    // Only the second period (after Devanagari) should be flagged
    assert_eq!(diags.len(), 1);
    assert!(diags[0].span.0 > 10, "Should flag the second period");
    assert_eq!(diags[0].expected, "।");
}

/// Regression: Smart quotes should work after parentheses too, not just whitespace.
/// ("नेपाल") -> ("“नेपाल”")
#[test]
fn regression_smart_quotes_after_parens() {
    let text = "(\"नेपाल\")";
    let diags = check_punctuation(text);

    // We expect 2 diagnostics: opening and closing quote
    let opening = diags
        .iter()
        .find(|d| d.span.0 == 1)
        .expect("Opening quote not flagged");
    assert_eq!(
        opening.expected, "\u{201C}",
        "Expected opening quote “ after ("
    );

    let closing_pos = text.rfind('"').unwrap();
    let closing = diags
        .iter()
        .find(|d| d.span.0 == closing_pos)
        .expect("Closing quote not flagged");
    assert_eq!(
        closing.expected, "\u{201D}",
        "Expected closing quote ” before )"
    );
}

/// Regression: Sentence ending in a number should still correct '.' to '।'.
/// Digits (०-९) should NOT be treated as abbreviations.
#[test]
fn regression_number_sentence_end() {
    // "जम्मा ५. " -> Trailing space triggers abbreviation check logic
    let text = "जम्मा ५. ";
    let diags = check_punctuation(text);

    // We expect 1 diagnostic: "." -> "।"
    assert!(
        !diags.is_empty(),
        "Missed correction for sentence ending in number"
    );
    assert_eq!(diags[0].expected, "।");
}

/// Section 5: निर्देशक supports :, -, and :-; bare ':' must not be forced to ':-'.
#[test]
fn regression_nirdeshak_colon_allowed() {
    let diags = check_punctuation("नेपालका ठुला नदीहरू हुन्: कोसी, गण्डकी र कर्णाली।");
    assert!(diags.is_empty(), "Colon form should be accepted for निर्देशक");
}

/// Section 5: सङ्क्षेप examples like "अ. दु. अ. आ." must not be flagged.
#[test]
fn regression_devanagari_abbreviation_chain_allowed() {
    let text = "अ. दु. अ. आ.ले सबैलाई सचेत गराएको छ।";
    let diags = check_punctuation(text);
    assert!(
        diags.is_empty(),
        "Chained Devanagari abbreviations should be allowed, got: {diags:?}"
    );
}

/// Section 5: compact dotted forms like "त्रि.वि." must not be flagged.
#[test]
fn regression_compact_devanagari_abbreviation_allowed() {
    let text = "त्रि.वि.ले परीक्षाफल प्रकाशित गर्‍यो।";
    let diags = check_punctuation(text);
    assert!(
        diags.is_empty(),
        "Compact Devanagari abbreviations should be allowed, got: {diags:?}"
    );
}

#[test]
fn regression_tiryak_viram_spacing() {
    let diags = check_punctuation("तिमी / उहाँ आउनुहुन्छ।");
    assert!(
        diags
            .iter()
            .any(|d| d.expected == "/" && d.rule.contains("तिर्यक् विराम")),
        "Expected slash spacing diagnostic, got: {diags:?}"
    );
}

#[test]
fn regression_aijan_spacing_pair() {
    let diags = check_punctuation("राम , , श्याम");
    assert!(
        diags
            .iter()
            .any(|d| d.expected == ",," && d.rule.contains("ऐजन")),
        "Expected ऐजन spacing diagnostic, got: {diags:?}"
    );
}

#[test]
fn regression_unmatched_parentheses() {
    let diags = check_punctuation("अहिले हरितगृह प्रभाव (तातोपन सञ्चित भइरहनु ले समस्या बढायो।");
    assert!(
        diags
            .iter()
            .any(|d| d.found == "(" && d.rule.contains("कोष्ठक")),
        "Expected unmatched '(' diagnostic, got: {diags:?}"
    );
}
