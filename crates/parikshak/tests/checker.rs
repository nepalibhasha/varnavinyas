use varnavinyas_parikshak::{
    CheckOptions, DiagnosticCategory, DiagnosticKind, OrthographyMode, PunctuationMode, check_text,
    check_text_with_options, check_word,
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
fn flags_federal_parliament_phrase() {
    let diags = check_text("संघीय संसद नेपाल .");
    let expected = [
        ("संघीय", "सङ्घीय", DiagnosticCategory::Chandrabindu),
        ("संसद", "संसद्", DiagnosticCategory::Halanta),
        (".", "।", DiagnosticCategory::Punctuation),
    ];

    assert_eq!(
        diags.len(),
        expected.len(),
        "Expected only the phrase-level diagnostics, got: {diags:?}"
    );
    for (incorrect, correction, category) in expected {
        assert!(
            diags.iter().any(|d| {
                d.incorrect == incorrect && d.correction == correction && d.category == category
            }),
            "Expected {incorrect} -> {correction} ({category:?}), got: {diags:?}"
        );
    }
}

#[test]
fn flags_nepali_congress_phrase() {
    let diags = check_text("नेपाली कांग्रेस");

    assert_eq!(
        diags.len(),
        1,
        "Expected only कांग्रेस diagnostic, got: {diags:?}"
    );
    let diag = &diags[0];
    assert_eq!(diag.incorrect, "कांग्रेस");
    assert_eq!(diag.correction, "काङ्ग्रेस");
    assert_eq!(diag.category, DiagnosticCategory::Chandrabindu);
}

#[test]
fn flags_common_pancham_varna_spellings_in_strict_mode() {
    let diags = check_text("संघ संचार संकेत");
    let expected = [("संघ", "सङ्घ"), ("संचार", "सञ्चार"), ("संकेत", "सङ्केत")];

    assert_eq!(
        diags.len(),
        expected.len(),
        "Expected only pancham-varna spelling diagnostics, got: {diags:?}"
    );
    for (incorrect, correction) in expected {
        assert!(
            diags.iter().any(|d| {
                d.incorrect == incorrect
                    && d.correction == correction
                    && d.category == DiagnosticCategory::Chandrabindu
            }),
            "Expected {incorrect} -> {correction}, got: {diags:?}"
        );
    }
}

#[test]
fn flags_ps_final_nga_without_ga_spellings() {
    let diags = check_text("गुरुङ्ग बिल्डिङ्ग रङ्ग प्रसङ्ग");
    let expected = [("गुरुङ्ग", "गुरुङ"), ("बिल्डिङ्ग", "बिल्डिङ"), ("रङ्ग", "रङ")];

    assert_eq!(
        diags.len(),
        expected.len(),
        "Expected only PS final-ङ diagnostics, got: {diags:?}"
    );
    for (incorrect, correction) in expected {
        assert!(
            diags.iter().any(|d| {
                d.incorrect == incorrect
                    && d.correction == correction
                    && d.category == DiagnosticCategory::Chandrabindu
            }),
            "Expected {incorrect} -> {correction}, got: {diags:?}"
        );
    }
}

#[test]
fn check_word_applies_ps_final_nga_without_ga_before_suffix() {
    let diag = check_word("गुरुङ्गले").expect("गुरुङ्गले should normalize through गुरुङ + ले");
    assert_eq!(diag.incorrect, "गुरुङ्गले");
    assert_eq!(diag.correction, "गुरुङले");
    assert_eq!(diag.category, DiagnosticCategory::Chandrabindu);
}

#[test]
fn flags_ps_sanskrit_va_words_that_take_ba_even_when_lexicon_attests_va() {
    let diags = check_text("विम्ब विन्दु विना प्रतिविम्ब केन्द्रविन्दु विनाश");
    let expected = [
        ("विम्ब", "बिम्ब"),
        ("विन्दु", "बिन्दु"),
        ("विना", "बिना"),
        ("प्रतिविम्ब", "प्रतिबिम्ब"),
        ("केन्द्रविन्दु", "केन्द्रबिन्दु"),
    ];

    assert_eq!(
        diags.len(),
        expected.len(),
        "Expected only PS Sanskrit-व् to ब diagnostics, got: {diags:?}"
    );
    for (incorrect, correction) in expected {
        assert!(
            diags.iter().any(|d| {
                d.incorrect == incorrect
                    && d.correction == correction
                    && d.category == DiagnosticCategory::BaVa
            }),
            "Expected {incorrect} -> {correction}, got: {diags:?}"
        );
    }
}

#[test]
fn common_editorial_mode_downgrades_reviewed_orthographic_variants() {
    let diags = check_text_with_options(
        "संघीय संघ संचार संकेत संसद नेपाल . नेपाली कांग्रेस",
        CheckOptions {
            orthography_mode: OrthographyMode::CommonEditorial,
            ..Default::default()
        },
    );

    for incorrect in ["संघीय", "संघ", "संचार", "संकेत", "संसद", "कांग्रेस"]
    {
        let diag = diags
            .iter()
            .find(|d| d.incorrect == incorrect)
            .unwrap_or_else(|| panic!("Expected diagnostic for {incorrect}, got: {diags:?}"));
        assert!(
            matches!(diag.kind, DiagnosticKind::Variant),
            "Expected {incorrect} to be a reviewed variant, got: {diag:?}"
        );
    }

    let punctuation = diags
        .iter()
        .find(|d| d.incorrect == ".")
        .unwrap_or_else(|| panic!("Expected punctuation diagnostic, got: {diags:?}"));
    assert!(
        matches!(punctuation.kind, DiagnosticKind::Error),
        "Orthography mode should not relax punctuation, got: {punctuation:?}"
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

#[test]
fn check_word_accepts_unlisted_case_form_backed_by_sibling_entries() {
    let diag = check_word("मच्छिन्द्रनाथको");
    assert!(
        diag.is_none(),
        "Attested sibling inflections should support direct word checks, got: {diag:?}"
    );
}

#[test]
fn check_word_accepts_prefix_plus_case_form() {
    let diag = check_word("निराशाबाट");
    assert!(
        diag.is_none(),
        "Prefix plus case forms should be structurally supported, got: {diag:?}"
    );
}

#[test]
fn check_word_accepts_case_plus_particle_stack() {
    let diag = check_word("रामकोपनि");
    assert!(
        diag.is_none(),
        "Case-marker plus particle stacks should be structurally supported, got: {diag:?}"
    );
}

#[test]
fn check_word_propagates_correction_table_base_through_case_suffix() {
    let diag = check_word("रुपमा").expect("रुपमा should be corrected via रूप + मा");
    assert_eq!(diag.incorrect, "रुपमा");
    assert_eq!(diag.correction, "रूपमा");
}

#[test]
fn check_word_normalizes_chandrabindu_stem_before_reattaching_suffix() {
    let diag = check_word("संगको").expect("संगको should normalize through सँग + को");
    assert_eq!(diag.incorrect, "संगको");
    assert_eq!(diag.correction, "सँगको");
}

#[test]
fn check_word_applies_tiryak_to_joined_eko_plus_case_form() {
    let diag = check_word("भएकोमा").expect("भएकोमा should normalize through तिर्यक्");
    assert_eq!(diag.incorrect, "भएकोमा");
    assert_eq!(diag.correction, "भएकामा");
}

#[test]
fn check_word_applies_tiryak_to_joined_nu_plus_case_form() {
    let diag = check_word("गर्नुले").expect("गर्नुले should normalize through तिर्यक्");
    assert_eq!(diag.incorrect, "गर्नुले");
    assert_eq!(diag.correction, "गर्नाले");
}

#[test]
fn check_word_applies_tiryak_to_direct_pronoun_case_form() {
    let diag = check_word("योले").expect("योले should normalize through तिर्यक्");
    assert_eq!(diag.incorrect, "योले");
    assert_eq!(diag.correction, "यसले");
}

#[test]
fn check_word_applies_tiryak_to_additional_pronoun_surface_forms() {
    let diag = check_word("मले").expect("मले should normalize through तिर्यक्");
    assert_eq!(diag.incorrect, "मले");
    assert_eq!(diag.correction, "मैले");
}

#[test]
fn check_word_accepts_additional_unlisted_outer_affix_forms() {
    for word in ["रामसम्मपनि", "रामसँगै", "रामसँगको", "रामनै", "प्रशासनसम्मपनि"]
    {
        let diag = check_word(word);
        assert!(
            diag.is_none(),
            "Word '{word}' should be structurally supported, got: {diag:?}"
        );
    }
}

#[test]
fn check_word_accepts_contracted_nai_after_n_stem() {
    let diag = check_word("सुधार्नै");
    assert!(
        diag.is_none(),
        "Contracted ...नै forms should be structurally supported, got: {diag:?}"
    );
}

#[test]
fn check_word_accepts_shared_onset_particle_forms() {
    let word = "खलककै";
    let diag = check_word(word);
    assert!(
        diag.is_none(),
        "Shared-onset particle form '{word}' should be structurally supported, got: {diag:?}"
    );
}

#[test]
fn check_word_accepts_shared_onset_case_marker_forms() {
    for word in ["अङ्कको", "अचम्ममा", "अध्यापककी", "अतीततिर", "उससँग"]
    {
        let diag = check_word(word);
        assert!(
            diag.is_none(),
            "Shared-onset case form '{word}' should be structurally supported, got: {diag:?}"
        );
    }
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
fn numeric_tokens_do_not_trigger_spelling_suggestions() {
    for word in ["२०८३", "२००४", "72", "७२%", "२०८३-०८-१४"] {
        let diag = check_word(word);
        assert!(
            diag.is_none(),
            "Numeric token '{word}' should not produce a diagnostic, got: {diag:?}"
        );
    }
}

#[test]
fn abbreviation_number_marker_does_not_trigger_spelling_suggestion() {
    let diag = check_word("नं");
    assert!(
        diag.is_none(),
        "Abbreviation marker नं should not produce a diagnostic, got: {diag:?}"
    );
}

#[test]
fn text_with_devanagari_numbers_remains_unflagged() {
    let diags = check_text("बैठक २०८३-०८-१४ मा ७२% उपस्थितिसहित सम्पन्न भयो।");
    assert!(
        diags.is_empty(),
        "Devanagari numbers should not produce spelling diagnostics, got: {diags:?}"
    );
}

#[test]
fn productive_forms_and_known_compounds_do_not_trigger_nearby_suggestions() {
    for word in ["पुष्पकमल", "बनाइयो", "रहेन", "ढाल्दै", "रामकै"]
    {
        let diag = check_word(word);
        assert!(
            diag.is_none(),
            "Valid or analyzable form '{word}' should not produce edit-distance noise, got: {diag:?}"
        );
    }
}

#[test]
fn punctuation_boundaries_do_not_merge_words_into_false_suggestions() {
    let diags = check_text("नेकपा (माओवादी केन्द्र)को बैठक");
    assert!(
        diags.iter().all(|d| d.incorrect != "केन्द्र)को"),
        "Internal punctuation should split tokens before suggestion fallback, got: {diags:?}"
    );
}

#[test]
fn tiryak_joined_phrase_correction_applies_to_split_kri_danta_plus_case() {
    let diags = check_text("भएको मा निर्णय गरियो।");
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "भएको मा" && d.correction == "भएकामा"),
        "Expected तिर्यक् phrase correction for split भएको मा, got: {diags:?}"
    );
}

#[test]
fn tiryak_direct_pronoun_case_correction_applies_to_split_form() {
    let diags = check_text("यो ले सही सन्देश दियो।");
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "यो ले" && d.correction == "यसले"),
        "Expected तिर्यक् phrase correction for split यो ले, got: {diags:?}"
    );
}

#[test]
fn tiryak_irregular_split_pronoun_forms_keep_kha_rule_attribution() {
    let diags = check_text("म ले भनें। तँ ले सुनिस्।");
    let m_le = diags
        .iter()
        .find(|d| d.incorrect == "म ले" && d.correction == "मैले")
        .expect("Expected तिर्यक् correction for म ले");
    assert!(
        m_le.explanation.contains("७(ख)"),
        "Expected ७(ख) explanation for म ले, got: {m_le:?}"
    );

    let tan_le = diags
        .iter()
        .find(|d| d.incorrect == "तँ ले" && d.correction == "तैँले")
        .expect("Expected तिर्यक् correction for तँ ले");
    assert!(
        tan_le.explanation.contains("७(ख)"),
        "Expected ७(ख) explanation for तँ ले, got: {tan_le:?}"
    );
}

#[test]
fn tiryak_determiner_correction_applies_before_inflected_head_noun() {
    let diags = check_text("अब यो प्रसारणका प्रमुख समाचारहरू सुन्नुहोस्।");
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "यो" && d.correction == "यस"),
        "Expected determiner तिर्यक् correction before inflected head noun, got: {diags:?}"
    );
}

#[test]
fn tiryak_does_not_overcorrect_non_trigger_suffixes() {
    let diag = check_word("गर्नुको");
    assert!(
        diag.is_none(),
        "Non-trigger suffixes should not force तिर्यक्, got: {diag:?}"
    );
}

#[test]
fn tiryak_subrule_ga_applies_to_possessive_determiners_with_inflected_heads() {
    let diags = check_text("मेरो भाइहरू आए।");
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "मेरो" && d.correction == "मेरा"),
        "Expected ७(ग) determiner correction before noun with plural suffix, got: {diags:?}"
    );
}

#[test]
fn tiryak_subrule_ga_matches_multiple_saishanik_examples() {
    let diags =
        check_text("हाम्रो घरमा बसौँ। तिम्रो साथीबाट खबर आयो। उसको किताबहरू हराए। उनको खेलौनाहरू फुटे।");
    for (incorrect, correction) in [
        ("हाम्रो", "हाम्रा"),
        ("तिम्रो", "तिम्रा"),
        ("उसको", "उसका"),
        ("उनको", "उनका"),
    ] {
        let diag = diags
            .iter()
            .find(|d| d.incorrect == incorrect && d.correction == correction)
            .unwrap_or_else(|| {
                panic!("Expected ७(ग) example {incorrect} -> {correction}, got: {diags:?}")
            });
        assert!(
            diag.explanation.contains("७(ग)"),
            "Expected ७(ग) explanation for {incorrect}, got: {diag:?}"
        );
    }
}

#[test]
fn tiryak_subrule_ga_skips_uninflected_head_nouns() {
    let diags = check_text("मेरो भाइ आए।");
    assert!(
        diags.iter().all(|d| d.incorrect != "मेरो"),
        "Bare head nouns should not trigger ७(ग) oblique determiner correction, got: {diags:?}"
    );
}

#[test]
fn unicode_dash_after_quote_verb_does_not_trigger_false_suggestion() {
    let diags = check_text("राम भने– ‘घर जाऔँ।’");
    assert!(
        diags.iter().all(|d| d.incorrect != "भने–"),
        "Unicode dash punctuation should not be attached to भने, got: {diags:?}"
    );
}

#[test]
fn tatsam_sibilant_word_gets_rule_backed_correction() {
    let diag = check_word("सपथ").expect("Expected diagnostic for सपथ");
    assert_eq!(diag.correction, "शपथ");
    assert_eq!(
        diag.category,
        varnavinyas_parikshak::DiagnosticCategory::ShaShaS
    );
    assert!(
        !matches!(diag.kind, DiagnosticKind::Ambiguous),
        "सपथ should be a rule-backed sibilant correction, not edit-distance fallback: {diag:?}"
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
fn ba_va_o_class_does_not_rewrite_supported_u_initial_verb_form() {
    let diag = check_word("उठे");
    assert!(
        diag.is_none(),
        "उठे should remain a valid verb form, got: {diag:?}"
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
fn text_level_tatsam_halanta_beats_nipat_split() {
    let diags = check_text("जगत ठुलो छ। अकस्मात भयो। साक्षात देखियो। पश्चात आयो।");

    let jagat = diags
        .iter()
        .find(|d| d.incorrect == "जगत")
        .unwrap_or_else(|| panic!("Expected halanta diagnostic for जगत, got: {diags:?}"));
    assert_eq!(jagat.correction, "जगत्");
    assert_eq!(jagat.category, DiagnosticCategory::Halanta);

    for wrong_split in ["जग त", "अकस्मा त", "साक्षा त", "पश्चा त"]
    {
        assert!(
            diags.iter().all(|d| d.correction != wrong_split),
            "Tatsam padanta halanta forms must not be intercepted by निपात split {wrong_split}, got: {diags:?}"
        );
    }
}

#[test]
fn ps_halanta_inventory_covered_forms_restore_padanta_halanta() {
    for (incorrect, correct) in [
        ("पृथक", "पृथक्"),
        ("सम्राट", "सम्राट्"),
        ("जगत", "जगत्"),
        ("अर्थात", "अर्थात्"),
        ("अकस्मात", "अकस्मात्"),
        ("साक्षात", "साक्षात्"),
        ("पश्चात", "पश्चात्"),
        ("विद्युत", "विद्युत्"),
        ("विपत", "विपत्"),
        ("आपत", "आपत्"),
        ("संसद", "संसद्"),
        ("बृहत", "बृहत्"),
        ("महान", "महान्"),
        ("स्वयम", "स्वयम्"),
        ("अनुष्टुप", "अनुष्टुप्"),
        ("शुभम", "शुभम्"),
    ] {
        let diags = check_text(incorrect);
        assert!(
            diags.iter().any(|d| {
                d.incorrect == incorrect
                    && d.correction == correct
                    && d.category == DiagnosticCategory::Halanta
            }),
            "Expected {incorrect} -> {correct} via halanta restoration, got: {diags:?}"
        );
    }
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

#[test]
fn punctuation_allows_dictionary_numbered_list_markers() {
    let text = "शब्द\nनाम [संस्कृत]\n१. अनुभूत विषयवस्तुलाई व्यक्त गरिने वर्णात्मक वा ध्वन्यात्मक ध्वनि; आवाज।\n२. वर्ण वा वर्णसमूहबाट बनेको कुनै अर्थ बुझाउने ध्वनि; सार्थक पद; लबज।\n१. नाम अनुभूत विषयलाई व्यक्त गरिने वर्णात्मक वा ध्वन्यात्मक आबाज।\n२. नाम वर्ण वा वर्णसमूहबाट बनेको कुनै अर्थ बुझाउने ध्वनि; सार्थक पद; लबज।";
    let diags = check_text(text);
    let punct_diags: Vec<_> = diags
        .iter()
        .filter(|d| d.category == varnavinyas_parikshak::DiagnosticCategory::Punctuation)
        .collect();
    assert!(
        punct_diags.is_empty(),
        "Numbered definition markers should not emit punctuation diagnostics, got: {punct_diags:?}"
    );
}

#[test]
fn punctuation_allows_contact_metadata_dots() {
    let text = "धुलिखेल, काभ्रे, नेपाल | +९७७ ११ ४९५००१, ४९५१००, ४९५२०० | फ्याक्स +९७७ ११ ४९०४९७, ०१ ५१८६४१४\nvc@ku.edu.np | info@ku.edu.np | www.ku.edu.np | पो.ब.नं. ६२५०, काठमाडौँ.";
    let diags = check_text(text);
    assert!(
        diags.is_empty(),
        "Contact metadata should not emit diagnostics, got: {diags:?}"
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
fn generalized_vibhakti_pachhi_namayogi_split_requires_actual_vibhakti_on_left() {
    let diags = check_text("वास्तविकताभन्दा आफूभन्दा तिमीभन्दा।");
    for incorrect in ["वास्तविकताभन्दा", "आफूभन्दा", "तिमीभन्दा"]
    {
        assert!(
            diags.iter().all(|d| d.incorrect != incorrect),
            "Joined nameyogi form {incorrect} should not be split without a real vibhakti host, got: {diags:?}"
        );
    }
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
fn generalized_padayog_vibhakti_join_skips_numeric_date_segments() {
    let text = "फागुन २१ मा कार्यक्रम छ।";
    let diags = check_text(text);
    assert!(
        diags.iter().all(|d| d.incorrect != "२१ मा"),
        "Numeric date segments should not be joined under 3(घ)-पदयोग-३, got: {diags:?}"
    );
}

#[test]
fn generalized_padayog_namayogi_join_handles_following_vibhakti() {
    let text = "सरकार संग को निर्णय आयो।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "सरकार संग को" && d.correction == "सरकारसँगको"),
        "Expected generalized 3(घ)-पदयोग-४ join, got: {diags:?}"
    );
}

#[test]
fn generalized_padayog_namayogi_join_handles_panchham_variant_before_vibhakti() {
    let text = "सरकार सङ को निर्णय आयो।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "सरकार सङ को" && d.correction == "सरकारसँगको"),
        "Expected generalized layered join for सङ को, got: {diags:?}"
    );
}

#[test]
fn generalized_padayog_namayogi_join_handles_prati() {
    let text = "अवसर प्रति आस्था देखियो।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "अवसर प्रति" && d.correction == "अवसरप्रति"),
        "Expected generalized 3(घ)-पदयोग-४ join for प्रति, got: {diags:?}"
    );
}

#[test]
fn generalized_padayog_namayogi_join_skips_non_headword_adverb_hosts() {
    let diags = check_text("छिटै भित्र पस्ने किसिमले।");
    assert!(
        diags.iter().all(|d| d.incorrect != "छिटै भित्र"),
        "Non-headword adverb hosts should not be force-joined with नामयोगी, got: {diags:?}"
    );
}

#[test]
fn generalized_padayog_namayogi_plus_vibhakti_join_handles_partially_joined_prati() {
    let text = "म प्रतिको धारणा प्रष्ट छ।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "म प्रतिको" && d.correction == "मप्रतिको"),
        "Expected generalized layered 3(घ) join for प्रतिको, got: {diags:?}"
    );
}

#[test]
fn generalized_padayog_pratyaya_join_handles_juu_after_known_name() {
    let text = "राम शाह ज्यू आए।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "शाह ज्यू" && d.correction == "शाहज्यू"),
        "Expected generalized 3(घ)-पदयोग-२ join for ज्यू, got: {diags:?}"
    );
    assert!(
        diags.iter().all(|d| d.incorrect != "शाह"),
        "Exact headword शाह should not trigger a sibilant correction, got: {diags:?}"
    );
}

#[test]
fn generalized_padayog_pratyaya_join_normalizes_jyu_to_juu_in_honorific_context() {
    let text = "शाह ज्यु आए। मन्त्री ज्युहरु पनि आए।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "शाह ज्यु" && d.correction == "शाहज्यू"),
        "Expected honorific-context normalization शाह ज्यु -> शाहज्यू, got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "मन्त्री ज्युहरु" && d.correction == "मन्त्रीज्यूहरु"),
        "Expected honorific-context normalization मन्त्री ज्युहरु -> मन्त्रीज्यूहरु, got: {diags:?}"
    );
}

#[test]
fn saishanik_comparison_particles_are_split_not_joined() {
    let text = "उनीजस्तै आए, डोल्माजस्तो देखियो, बताएजसरी गरियो।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "उनीजस्तै" && d.correction == "उनी जस्तै"),
        "Expected शैक्षणिक comparison split for उनीजस्तै, got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "डोल्माजस्तो" && d.correction == "डोल्मा जस्तो"),
        "Expected शैक्षणिक comparison split for डोल्माजस्तो, got: {diags:?}"
    );
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "बताएजसरी" && d.correction == "बताए जसरी"),
        "Expected शैक्षणिक comparison split for बताएजसरी, got: {diags:?}"
    );
}

#[test]
fn sarah_join_rule_still_applies() {
    let text = "बुद्धि सरह सोच राख।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "बुद्धि सरह" && d.correction == "बुद्धिसरह"),
        "Expected सरह join to remain active, got: {diags:?}"
    );
}

#[test]
fn documented_false_positive_pins_remain_unflagged() {
    let text = "सामान्यीकरण केन्द्रीकरण एकीकरण राष्ट्रिय क्षत्रिय इन्द्रिय श्रोत्रिय श्रेणी युवती सूची औषधी अञ्जली शेर्पा जोशी शाह कुशवाहले शुभ समाचार।";
    let diags = check_text(text);

    for token in [
        "सामान्यीकरण",
        "केन्द्रीकरण",
        "एकीकरण",
        "राष्ट्रिय",
        "क्षत्रिय",
        "इन्द्रिय",
        "श्रोत्रिय",
        "श्रेणी",
        "युवती",
        "सूची",
        "औषधी",
        "अञ्जली",
        "शेर्पा",
        "जोशी",
        "शाह",
        "कुशवाहले",
    ] {
        assert!(
            diags.iter().all(|d| d.incorrect != token),
            "Expected {token} to remain unflagged, got: {diags:?}"
        );
    }

    assert!(
        diags.iter().all(|d| {
            !(d.incorrect == "शुभ समाचार" || d.correction == "शुभसमाचार")
        }),
        "Split शुभ समाचार should not be joined, got: {diags:?}"
    );
}

#[test]
fn flags_ps_iya_exception_dirgha_forms() {
    let diags = check_text("राष्ट्रीय क्षत्रीय इन्द्रीय श्रोत्रीय");
    let expected = [
        ("राष्ट्रीय", "राष्ट्रिय"),
        ("क्षत्रीय", "क्षत्रिय"),
        ("इन्द्रीय", "इन्द्रिय"),
        ("श्रोत्रीय", "श्रोत्रिय"),
    ];

    assert_eq!(
        diags.len(),
        expected.len(),
        "Expected only PS ईय exception diagnostics, got: {diags:?}"
    );
    for (incorrect, correction) in expected {
        assert!(
            diags.iter().any(|d| {
                d.incorrect == incorrect
                    && d.correction == correction
                    && d.category == DiagnosticCategory::HrasvaDirgha
            }),
            "Expected {incorrect} -> {correction}, got: {diags:?}"
        );
    }
}

#[test]
fn ps_final_dirgha_exception_diagnostics_apply() {
    let diags = check_text("श्रेणि युवति सूचि अञ्जलि श्रद्धाञ्जलि आवलि शब्दावलि औषधि।");
    for (incorrect, correction) in [
        ("श्रेणि", "श्रेणी"),
        ("युवति", "युवती"),
        ("सूचि", "सूची"),
        ("अञ्जलि", "अञ्जली"),
        ("श्रद्धाञ्जलि", "श्रद्धाञ्जली"),
        ("आवलि", "आवली"),
        ("शब्दावलि", "शब्दावली"),
        ("औषधि", "औषधी"),
    ] {
        assert!(
            diags.iter().any(|d| {
                d.incorrect == incorrect
                    && d.correction == correction
                    && d.category == DiagnosticCategory::HrasvaDirgha
            }),
            "Expected PS final-dirgha exception {incorrect} -> {correction}, got: {diags:?}"
        );
    }
}

#[test]
fn saishanik_swarup_join_applies() {
    let diags = check_text("फल स्वरूप यो नतिजा आयो।");
    let diag = diags
        .iter()
        .find(|d| d.incorrect == "फल स्वरूप" && d.correction == "फलस्वरूप")
        .expect("Expected स्वरूप join correction");
    assert!(
        diag.explanation.contains("पदयोग (ङ)"),
        "Expected शैक्षणिक पदयोग (ङ) explanation, got: {diag:?}"
    );
}

#[test]
fn saishanik_namik_kriya_splits_apply() {
    let diags = check_text("थाहापाउनु राम्रो हो। मनपर्नु सजिलो छैन। मायागर्नु आवश्यक छ।");
    for (incorrect, correction) in [
        ("थाहापाउनु", "थाहा पाउनु"),
        ("मनपर्नु", "मन पर्नु"),
        ("मायागर्नु", "माया गर्नु"),
    ] {
        let diag = diags
            .iter()
            .find(|d| d.incorrect == incorrect && d.correction == correction)
            .unwrap_or_else(|| {
                panic!("Expected नामिक क्रिया split {incorrect} -> {correction}, got: {diags:?}")
            });
        assert!(
            diag.explanation.contains("पदवियोग (घ)"),
            "Expected शैक्षणिक पदवियोग (घ) explanation for {incorrect}, got: {diag:?}"
        );
    }
}

#[test]
fn saishanik_gari_splits_apply() {
    let diags = check_text("बुझिनेगरी सम्झाऊ। ढिलोगरी नआऊ।");
    for (incorrect, correction) in [("बुझिनेगरी", "बुझिने गरी"), ("ढिलोगरी", "ढिलो गरी")]
    {
        let diag = diags
            .iter()
            .find(|d| d.incorrect == incorrect && d.correction == correction)
            .unwrap_or_else(|| {
                panic!("Expected गरी split {incorrect} -> {correction}, got: {diags:?}")
            });
        assert!(
            diag.explanation.contains("पदवियोग (ङ)"),
            "Expected शैक्षणिक पदवियोग (ङ) explanation for {incorrect}, got: {diag:?}"
        );
    }
}

#[test]
fn saishanik_middle_name_join_applies_when_joined_name_is_attested() {
    let diags = check_text("लक्ष्मी प्रसाद देवकोटा नेपाली साहित्यका महत्त्वपूर्ण व्यक्तित्व हुन्।");
    let diag = diags
        .iter()
        .find(|d| d.incorrect == "लक्ष्मी प्रसाद देवकोटा" && d.correction == "लक्ष्मीप्रसाद देवकोटा")
        .expect("Expected middle-name join correction");
    assert!(
        diag.explanation.contains("पदयोग (ञ)"),
        "Expected शैक्षणिक पदयोग (ञ) explanation, got: {diag:?}"
    );
}

#[test]
fn saishanik_middle_name_join_does_not_overfire_on_generic_lexical_triples() {
    let diags = check_text("शुभ यात्रा\nशिक्षा");
    assert!(
        diags.iter().all(|d| d.correction != "शुभयात्रा शिक्षा"),
        "Middle-name join should not cross line breaks or generic lexical triples, got: {diags:?}"
    );
}

#[test]
fn saishanik_ekarthi_joins_apply_conservatively() {
    let diags = check_text("प्रधान मन्त्री, शिक्षा मन्त्री, शुभ कामना, शुभ यात्रा, कीर्ति पुर, ललित पुर।");
    for (incorrect, correction) in [
        ("प्रधान मन्त्री", "प्रधानमन्त्री"),
        ("शिक्षा मन्त्री", "शिक्षामन्त्री"),
        ("शुभ कामना", "शुभकामना"),
        ("शुभ यात्रा", "शुभयात्रा"),
        ("कीर्ति पुर", "कीर्तिपुर"),
        ("ललित पुर", "ललितपुर"),
    ] {
        let diag = diags
            .iter()
            .find(|d| d.incorrect == incorrect && d.correction == correction)
            .unwrap_or_else(|| {
                panic!("Expected एकार्थी join {incorrect} -> {correction}, got: {diags:?}")
            });
        assert!(
            diag.explanation.contains("पदयोग (ट)"),
            "Expected शैक्षणिक पदयोग (ट) explanation for {incorrect}, got: {diag:?}"
        );
    }
}

#[test]
fn saishanik_institutional_phrase_splits_apply() {
    let diags = check_text("नेपालसरकार र परराष्ट्रमन्त्रालयले विज्ञप्ति जारी गरे।");
    for (incorrect, correction_prefix) in [
        ("नेपालसरकार", "नेपाल सरकार"),
        ("परराष्ट्रमन्त्रालयले", "परराष्ट्र मन्त्रालय"),
    ] {
        let diag = diags
            .iter()
            .find(|d| d.incorrect == incorrect && d.correction.starts_with(correction_prefix))
            .unwrap_or_else(|| {
                panic!(
                    "Expected institutional split {incorrect} -> {correction_prefix}..., got: {diags:?}"
                )
            });
        assert!(
            diag.explanation.contains("पदवियोग (ख)"),
            "Expected शैक्षणिक पदवियोग (ख) explanation for {incorrect}, got: {diag:?}"
        );
    }
}

#[test]
fn saishanik_title_name_splits_apply() {
    let diags = check_text("मुगुजिल्ला र राराताल नेपालको परिचित नाम हुन्।");
    for (incorrect, correction) in [("मुगुजिल्ला", "मुगु जिल्ला"), ("राराताल", "रारा ताल")]
    {
        let diag = diags
            .iter()
            .find(|d| d.incorrect == incorrect && d.correction == correction)
            .unwrap_or_else(|| {
                panic!("Expected title-name split {incorrect} -> {correction}, got: {diags:?}")
            });
        assert!(
            diag.explanation.contains("पदवियोग (ञ)"),
            "Expected शैक्षणिक पदवियोग (ञ) explanation for {incorrect}, got: {diag:?}"
        );
    }
}

#[test]
fn saishanik_title_name_split_requires_terminal_suffix_boundary() {
    let diags = check_text("स्वर्गका राजा; देवताका राजा।");
    assert!(
        diags.iter().all(|d| d.incorrect != "स्वर्गका"),
        "Internal substring matches like स्वर्गका -> ष् वर्गका should not trigger title-name splitting, got: {diags:?}"
    );
}

#[test]
fn saishanik_multiword_samasa_splits_apply() {
    let diags = check_text(
        "नेपालपत्रकारमहासङ्घ, नेपालप्रज्ञाप्रतिष्ठान, मानवअधिकारआयोग, लोकसेवाआयोग, अख्तियारदुरुपयोगअनुसन्धानआयोग, नेपालविद्युत्प्राधिकरण।",
    );
    for (incorrect, correction) in [
        ("नेपालपत्रकारमहासङ्घ", "नेपाल पत्रकार महासङ्घ"),
        ("नेपालप्रज्ञाप्रतिष्ठान", "नेपाल प्रज्ञा-प्रतिष्ठान"),
        ("मानवअधिकारआयोग", "मानव अधिकार आयोग"),
        ("लोकसेवाआयोग", "लोक सेवा आयोग"),
        ("अख्तियारदुरुपयोगअनुसन्धानआयोग", "अख्तियार दुरुपयोग अनुसन्धान आयोग"),
        ("नेपालविद्युत्प्राधिकरण", "नेपाल विद्युत् प्राधिकरण"),
    ] {
        let diag = diags
            .iter()
            .find(|d| d.incorrect == incorrect && d.correction == correction)
            .unwrap_or_else(|| {
                panic!(
                    "Expected multiword samasa split {incorrect} -> {correction}, got: {diags:?}"
                )
            });
        assert!(
            diag.explanation.contains("पदवियोग (ट)"),
            "Expected शैक्षणिक पदवियोग (ट) explanation for {incorrect}, got: {diag:?}"
        );
    }
}

#[test]
fn saishanik_vibhakti_pachhi_namayogi_splits_apply() {
    let diags = check_text("दीपेशकानिम्ति, सोनमकालागि, बाटोदेखिमाथि, मामाकोसमेत, उसकोभन्दा।");
    for (incorrect, correction, explanation_fragments) in [
        (
            "दीपेशकानिम्ति",
            "दीपेशका निम्ति",
            &["पदवियोग-३", "पदवियोग (क)"][..],
        ),
        (
            "सोनमकालागि",
            "सोनमका लागि",
            &["पदवियोग-३", "पदवियोग (क)"][..],
        ),
        ("बाटोदेखिमाथि", "बाटोदेखि माथि", &["पदवियोग (क)"][..]),
        ("मामाकोसमेत", "मामाको समेत", &["पदवियोग (क)"][..]),
        ("उसकोभन्दा", "उसको भन्दा", &["पदवियोग (क)"][..]),
    ] {
        let diag = diags
            .iter()
            .find(|d| d.incorrect == incorrect && d.correction == correction)
            .unwrap_or_else(|| {
                panic!(
                    "Expected विभक्तिपछि नामयोगी split {incorrect} -> {correction}, got: {diags:?}"
                )
            });
        assert!(
            explanation_fragments
                .iter()
                .any(|fragment| diag.explanation.contains(fragment)),
            "Expected one of {explanation_fragments:?} for {incorrect}, got: {diag:?}"
        );
    }
}

#[test]
fn saishanik_sarthak_dwitva_splits_apply_generally() {
    let diags =
        check_text("जहाँजहाँ, जोजो, केके, वनवन, सडकसडक, राम्रोराम्रो, हुन्छहुन्छ, बस्योबस्यो, हिँड्योहिँड्यो।");
    for (incorrect, correction) in [
        ("जहाँजहाँ", "जहाँ जहाँ"),
        ("जोजो", "जो जो"),
        ("केके", "के के"),
        ("वनवन", "वन वन"),
        ("सडकसडक", "सडक सडक"),
        ("राम्रोराम्रो", "राम्रो राम्रो"),
        ("हुन्छहुन्छ", "हुन्छ हुन्छ"),
        ("बस्योबस्यो", "बस्यो बस्यो"),
        ("हिँड्योहिँड्यो", "हिँड्यो हिँड्यो"),
    ] {
        let diag = diags
            .iter()
            .find(|d| d.incorrect == incorrect && d.correction == correction)
            .unwrap_or_else(|| {
                panic!("Expected सार्थक द्वित्व split {incorrect} -> {correction}, got: {diags:?}")
            });
        assert!(
            diag.explanation.contains("पदवियोग (ग)"),
            "Expected शैक्षणिक पदवियोग (ग) explanation for {incorrect}, got: {diag:?}"
        );
    }
}

#[test]
fn lexicalized_reduplication_headword_is_not_forced_apart() {
    let diags = check_text("वातविकार आदिका कारणले घाँटीबाट हिक्कहिक्क निस्कने आवाज।");
    assert!(
        diags.iter().all(|d| d.incorrect != "हिक्कहिक्क"),
        "Exact headword हिक्कहिक्क should not trigger सार्थक द्वित्व splitting, got: {diags:?}"
    );
}

#[test]
fn exact_headword_chandrabindu_variant_is_not_flagged_in_dictionary_prose() {
    let diags = check_text("बस्दा दुई खुट्टा र चाकले मात्र भुईं छुने गरी।");
    assert!(
        diags.iter().all(|d| d.incorrect != "भुईं"),
        "Exact headword भुईं should not be overcorrected in dictionary prose, got: {diags:?}"
    );
}

#[test]
fn exact_headword_final_hrasva_variant_is_not_flagged_in_dictionary_prose() {
    let diags = check_text("औषधीको प्रयोगले रोग निको हुन सक्छ।");
    assert!(
        diags.iter().all(|d| d.incorrect != "औषधी"),
        "Exact headword औषधी should not be overcorrected in dictionary prose, got: {diags:?}"
    );
}

#[test]
fn documented_hrasva_tail_compound_is_not_flagged_in_dictionary_prose() {
    let diags = check_text("सामान पाथीघरमुनि राखिएको थियो।");
    assert!(
        diags.iter().all(|d| d.incorrect != "पाथीघरमुनि"),
        "Joined compounds with documented hrasva tails like मुनि should stay clean, got: {diags:?}"
    );
}

#[test]
fn productive_yo_verb_forms_do_not_trigger_nearby_fallback() {
    let diags = check_text("हिँड्यो हिँड्यो।");
    assert!(
        diags.is_empty(),
        "Expected productive यो-verb forms to be accepted without fallback noise, got: {diags:?}"
    );
}

#[test]
fn saishanik_jana_split_applies_beyond_fixed_examples() {
    let diags = check_text("आठजना विद्यार्थी, दशजना मान्छे, पाँचजना केटा आए।");
    for (incorrect, correction) in [
        ("आठजना", "आठ जना"),
        ("दशजना", "दश जना"),
        ("पाँचजना", "पाँच जना"),
    ] {
        let diag = diags
            .iter()
            .find(|d| d.incorrect == incorrect && d.correction == correction)
            .unwrap_or_else(|| {
                panic!("Expected जना split {incorrect} -> {correction}, got: {diags:?}")
            });
        assert!(
            diag.explanation.contains("पदवियोग (छ)"),
            "Expected शैक्षणिक पदवियोग (छ) explanation for {incorrect}, got: {diag:?}"
        );
    }
}

#[test]
fn saishanik_jana_split_does_not_split_lexicalized_yojana() {
    let diags = check_text("अब योजना, सामग्री");
    assert!(
        diags.iter().all(|d| d.incorrect != "योजना"),
        "Expected lexicalized योजना to avoid जना split false positive, got: {diags:?}"
    );
}

#[test]
fn saishanik_divisive_na_is_not_generalized_without_coordination_context() {
    let diags = check_text("नराम नश्याम आए। नरामले गीत गाई। केहीँ नभाको जात्रा हाडी गाउँमा !");
    assert!(
        diags.iter().all(|d| {
            !matches!(
                (d.incorrect.as_str(), d.correction.as_str()),
                ("नराम", "न राम")
                    | ("नश्याम", "न श्याम")
                    | ("नरामले", "न रामले")
                    | ("नभाको", "न भाको")
            )
        }),
        "Expected no generalized विभाजक-न rewrite without coordination cues, got: {diags:?}"
    );
}

#[test]
fn saishanik_nipat_split_applies_to_school_grammar_examples() {
    let diags = check_text(
        "त्योत गयो। ऊनि जान्छ। रामनै चिकित्सक हो। बहिनीपो आई। हरिझैँ देखियो। दीपकचाहिँ मसँग रिसायो। केटोमात्र चौरमा दौड्यो। ऊ कहिले आउँछखै? यति पाठ पढल। तिमी घर जाऊत। आयोअरे। गयोक्या?",
    );
    for (incorrect, correction, explanation_fragment) in [
        ("त्योत", "त्यो त", "शब्दाश्रित"),
        ("ऊनि", "ऊ नि", "शब्दाश्रित"),
        ("रामनै", "राम नै", "शब्दाश्रित"),
        ("बहिनीपो", "बहिनी पो", "शब्दाश्रित"),
        ("हरिझैँ", "हरि झैँ", "शब्दाश्रित"),
        ("दीपकचाहिँ", "दीपक चाहिँ", "शब्दाश्रित"),
        ("आउँछखै", "आउँछ खै", "वाक्याश्रित"),
        ("पढल", "पढ ल", "शब्दाश्रित"),
        ("जाऊत", "जाऊ त", "शब्दाश्रित"),
        ("आयोअरे", "आयो अरे", "वाक्याश्रित"),
        ("गयोक्या", "गयो क्या", "वाक्याश्रित"),
    ] {
        let diag = diags
            .iter()
            .find(|d| d.incorrect == incorrect && d.correction == correction)
            .unwrap_or_else(|| {
                panic!("Expected निपात split {incorrect} -> {correction}, got: {diags:?}")
            });
        assert!(
            diag.explanation.contains("पदवियोग (च)"),
            "Expected शैक्षणिक पदवियोग (च) explanation for {incorrect}, got: {diag:?}"
        );
        assert!(
            diag.explanation.contains(explanation_fragment),
            "Expected {explanation_fragment} निपात explanation for {incorrect}, got: {diag:?}"
        );
    }
    assert!(
        diags
            .iter()
            .all(|d| d.incorrect != "ऊनि" || d.explanation.contains("पदवियोग (च)")),
        "Specific निपात rule should suppress weaker same-span diagnostics on ऊनि, got: {diags:?}"
    );
}

#[test]
fn saishanik_nipat_split_handles_contracted_nai_after_n_stem() {
    let diags = check_text("समस्या सुधार्नै पर्छ।");
    let diag = diags
        .iter()
        .find(|d| d.incorrect == "सुधार्नै" && d.correction == "सुधार्न नै")
        .unwrap_or_else(|| panic!("Expected contracted नै split, got: {diags:?}"));
    assert!(
        diag.explanation.contains("पदवियोग (च)"),
        "Expected निपात split explanation, got: {diag:?}"
    );
}

#[test]
fn saishanik_nipat_split_does_not_split_lexicalized_kunai() {
    let diags = check_text("कुनै व्यक्ति आएन।");
    assert!(
        diags.iter().all(|d| d.incorrect != "कुनै"),
        "Expected lexicalized word कुनै to avoid nipat split false positive, got: {diags:?}"
    );
}

#[test]
fn saishanik_nipat_split_prefers_nearby_whole_word_for_short_suffixes() {
    let diags = check_text("मोडल");
    assert!(
        diags
            .iter()
            .all(|d| !(d.incorrect == "मोडल" && d.correction == "मोड ल")),
        "Expected nearby whole-word analysis to beat short-nipat split on मोडल, got: {diags:?}"
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
fn generalized_padabiyog_ne_cha_split_handles_longer_future_form() {
    let text = "उठाइरहनेछु";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "उठाइरहनेछु" && d.correction == "उठाइरहने छु"),
        "Expected generalized 3(घ)-पदवियोग-७ split, got: {diags:?}"
    );
}

#[test]
fn generalized_padabiyog_ne_cha_split_handles_trailing_comma() {
    let text = "लागिरहनेछु,";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "लागिरहनेछु" && d.correction == "लागिरहने छु"),
        "Expected generalized 3(घ)-पदवियोग-७ split with trailing comma, got: {diags:?}"
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
fn section4_inferred_ko_ka_style_variants_generalize_in_grammar_mode() {
    let text = "घरको लागि समाजको निम्ति विषयको सम्बन्धमा तथ्यको आधारमा रोगको बारेमा।";

    let off = check_text(text);
    assert!(
        off.iter().all(|d| {
            d.rule != varnavinyas_prakriya::Rule::Vyakaran("section4-phrase-style-inferred-ko-ka")
        }),
        "Inferred style variants should not appear in default mode, got: {off:?}"
    );

    let on = check_text_with_options(
        text,
        CheckOptions {
            grammar: true,
            ..Default::default()
        },
    );
    let expected = [
        ("घरको लागि", "घरका लागि"),
        ("समाजको निम्ति", "समाजका निम्ति"),
        ("विषयको सम्बन्धमा", "विषयका सम्बन्धमा"),
        ("तथ्यको आधारमा", "तथ्यका आधारमा"),
        ("रोगको बारेमा", "रोगका बारेमा"),
    ];

    for (incorrect, correction) in expected {
        assert!(
            on.iter().any(|d| {
                d.rule
                    == varnavinyas_prakriya::Rule::Vyakaran("section4-phrase-style-inferred-ko-ka")
                    && d.incorrect == incorrect
                    && d.correction == correction
                    && matches!(d.kind, DiagnosticKind::Variant)
                    && d.explanation.contains("अनुमित")
            }),
            "Expected inferred style variant {incorrect} -> {correction}, got: {on:?}"
        );
    }
}

#[test]
fn section4_inferred_ko_ka_style_variants_require_exact_follower_word() {
    let text = "घरको लागिरहने कुरा र विषयको सम्बन्धमात्र फरक छन्।";
    let diags = check_text_with_options(
        text,
        CheckOptions {
            grammar: true,
            ..Default::default()
        },
    );

    assert!(
        diags.iter().all(|d| {
            d.rule != varnavinyas_prakriya::Rule::Vyakaran("section4-phrase-style-inferred-ko-ka")
        }),
        "Longer follower tokens should not trigger inferred style variants, got: {diags:?}"
    );
}

#[test]
fn section4_inferred_ko_ka_style_variants_use_shared_token_spans() {
    let text = "(घरको लागि,) तर घरको,लागि होइन।";
    let diags = check_text_with_options(
        text,
        CheckOptions {
            grammar: true,
            ..Default::default()
        },
    );

    assert!(
        diags.iter().any(|d| {
            d.rule == varnavinyas_prakriya::Rule::Vyakaran("section4-phrase-style-inferred-ko-ka")
                && d.incorrect == "घरको लागि"
                && d.correction == "घरका लागि"
                && matches!(d.kind, DiagnosticKind::Variant)
        }),
        "Expected inferred style variant across shared token span, got: {diags:?}"
    );
    assert!(
        diags.iter().all(|d| d.incorrect != "घरको,लागि"),
        "Comma without a whitespace gap should preserve prior conservative behavior, got: {diags:?}"
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

#[test]
fn sentence_context_phrase_backed_hos_correction_applies_in_blessing_sentence() {
    let text = "नेपाल आमाको जय होस ।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "होस" && d.correction == "होस्"),
        "Expected sentence-level phrase-backed final-token होस -> होस् correction, got: {diags:?}"
    );
}

#[test]
fn sentence_context_phrase_backed_hos_correction_applies_at_end_of_input() {
    let text = "नेपाल आमाको जय होस";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "होस" && d.correction == "होस्"),
        "Expected sentence-final phrase-backed final-token होस -> होस् correction without punctuation, got: {diags:?}"
    );
}

#[test]
fn sentence_context_structural_hos_correction_applies_for_final_benedictive_predicate() {
    let text = "सबैको भलो होस ।";
    let diags = check_text(text);
    assert!(
        diags
            .iter()
            .any(|d| d.incorrect == "होस" && d.correction == "होस्"),
        "Expected sentence-level final predicate होस -> होस् correction, got: {diags:?}"
    );
}

#[test]
fn sentence_context_does_not_overcorrect_nominal_hos_usage() {
    let text = "उसको होस हरायो ।";
    let diags = check_text(text);
    assert!(
        diags.iter().all(|d| d.incorrect != "होस"),
        "Nominal होस usage should remain untouched, got: {diags:?}"
    );
}
