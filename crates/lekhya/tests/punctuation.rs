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
    assert_eq!(diags[0].expected, "।");
}
