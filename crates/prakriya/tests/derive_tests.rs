use varnavinyas_prakriya::orthographic::{
    rule_ba_va, rule_chandrabindu, rule_gya_gyan, rule_ksha_chhya, rule_panchham_varna,
    rule_ri_kri, rule_sibilant, rule_ya_e,
};
use varnavinyas_prakriya::{Rule, collect_rule_hits, derive};

// P1: Corrects अत्याधिक → अत्यधिक
#[test]
fn p1_correct_atyaadhik() {
    let p = derive("अत्याधिक");
    assert_eq!(p.output, "अत्यधिक");
    assert!(!p.is_correct);
    assert!(!p.steps.is_empty());
}

// P2: Corrects मीठो → मिठो
#[test]
fn p2_correct_meetho() {
    let p = derive("मीठो");
    assert_eq!(p.output, "मिठो");
    assert!(!p.is_correct);
    assert!(
        p.steps
            .iter()
            .any(|s| matches!(s.rule, Rule::VarnaVinyasNiyam(_))),
        "Expected VarnaVinyasNiyam citation"
    );
}

// P3: Corrects हामि → हामी
#[test]
fn p3_correct_haami() {
    let p = derive("हामि");
    assert_eq!(p.output, "हामी");
    assert!(!p.is_correct);
}

// P4: Accepts प्रशासन as correct
#[test]
fn p4_prashaasan_correct() {
    let p = derive("प्रशासन");
    assert!(p.is_correct);
    assert_eq!(p.output, "प्रशासन");
}

// P6: Step trace non-empty for corrections
#[test]
fn p6_step_trace_nonempty() {
    let corrections = vec!["मीठो", "हामि", "अत्याधिक", "संसद", "रिषि"];
    for word in corrections {
        let p = derive(word);
        assert!(
            !p.steps.is_empty(),
            "Expected non-empty steps for '{word}', got output='{}'",
            p.output,
        );
    }
}

// P7: Suffix rules work
#[test]
fn p7_suffix_nu_hrasva() {
    let p = derive("स्वीकार्नु");
    assert_eq!(p.output, "स्विकार्नु");
    assert!(!p.is_correct);
}

// P8: Multiple corrections
#[test]
fn p8_suffix_eli_hrasva() {
    let p = derive("पूर्वेली");
    assert_eq!(p.output, "पुर्वेली");
    assert!(!p.is_correct);
}

// Additional tests
#[test]
fn correct_word_passes_through() {
    let correct_words = vec!["नमस्ते", "विज्ञान", "शासन"];
    for word in correct_words {
        let p = derive(word);
        assert!(p.is_correct, "Expected '{word}' to be correct");
        assert_eq!(p.output, word);
    }
}

#[test]
fn aadhi_vriddhi_does_not_overcorrect_attested_non_derivative_word() {
    let p = derive("अधिक");
    assert!(p.is_correct, "अधिक should remain correct, got: {p:?}");
    assert_eq!(p.output, "अधिक");
}

#[test]
fn empty_input() {
    let p = derive("");
    assert!(p.is_correct);
    assert_eq!(p.output, "");
}

#[test]
fn shri_correction() {
    let p = derive("श्रृङ्गार");
    assert_eq!(p.output, "शृङ्गार");
    assert!(!p.is_correct);
}

#[test]
fn redundant_ta_removal() {
    let p = derive("औचित्यता");
    assert_eq!(p.output, "औचित्य");
}

#[test]
fn broad_ta_heuristic_does_not_overcorrect_attested_forms() {
    for word in ["सत्यता", "असत्यता"] {
        let p = derive(word);
        assert!(p.is_correct, "{word} should remain unchanged, got: {p:?}");
        assert_eq!(p.output, word);
    }
}

#[test]
fn ri_to_ri() {
    let p = derive("रिषि");
    assert_eq!(p.output, "ऋषि");
}

#[test]
fn kri_to_kri() {
    let p = derive("क्रिति");
    assert_eq!(p.output, "कृति");
}

// Regression: क्रि→कृ should not over-correct valid tatsam words like "क्रिया".
#[test]
fn kri_not_applied_to_valid_tatsam_kriya() {
    let p = derive("क्रिया");
    assert_eq!(p.output, "क्रिया");
    assert!(p.is_correct);
}

#[test]
fn halanta_required() {
    let p = derive("संसद");
    assert_eq!(p.output, "संसद्");
}

#[test]
fn chandrabindu_correction() {
    let p = derive("सिँह");
    assert_eq!(p.output, "सिंह");
}

#[test]
fn avyaya_chandrabindu_form_beats_tatsam_panchham_fallback() {
    assert_eq!(derive("संग").output, "सँग");
    assert_eq!(derive("संगै").output, "सँगै");
}

#[test]
fn sibilant_correction() {
    let p = derive("सासन");
    assert_eq!(p.output, "शासन");
}

#[test]
fn lexicon_backed_tatsam_sibilant_correction() {
    let p = derive("सपथ");
    assert_eq!(p.output, "शपथ");
}

#[test]
fn exact_headword_sibilant_form_is_not_overcorrected() {
    let p = derive("शाह");
    assert_eq!(p.output, "शाह");
    assert!(p.is_correct);
}

#[test]
fn exact_headword_chandrabindu_form_is_not_overcorrected() {
    let p = derive("भुईं");
    assert_eq!(p.output, "भुईं");
    assert!(p.is_correct);
}

#[test]
fn exact_headword_final_hrasva_variant_is_not_overcorrected() {
    let p = derive("औषधी");
    assert_eq!(p.output, "औषधी");
    assert!(p.is_correct);
}

#[test]
fn documented_hrasva_tail_compound_is_not_overcorrected() {
    let p = derive("पाथीघरमुनि");
    assert_eq!(p.output, "पाथीघरमुनि");
    assert!(p.is_correct);
}

#[test]
fn panchham_correction() {
    let p = derive("संघीय");
    assert_eq!(p.output, "सङ्घीय");
}

#[test]
fn aagantuk_sa_not_sha() {
    let p = derive("रजिष्टर");
    assert_eq!(p.output, "रजिस्टर");
}

#[test]
fn sufi_does_not_take_prefix_hrasva_path() {
    let hits = collect_rule_hits("सूफी");
    assert!(
        hits.iter().all(|hit| {
            hit.prakriya
                .steps
                .first()
                .is_none_or(|step| step.rule != Rule::VarnaVinyasNiyam("3(क)(अ)-1"))
        }),
        "Loanword सूफी should not trigger the सु-उपसर्ग rule, got: {hits:?}"
    );
}

#[test]
fn multi_answer_accepts_any_alternative() {
    // धैर्यता → "धीरता/धैर्य" — either alternative is acceptable
    let p = derive("धैर्यता");
    let alternatives = ["धीरता", "धैर्य"];
    assert!(
        alternatives.contains(&p.output.as_str()),
        "Expected one of {alternatives:?}, got '{}'",
        p.output,
    );
}

// Regression: क्रि→कृ must NOT fire on non-tatsam words (e.g. loanwords)
#[test]
fn kri_not_applied_to_loanword() {
    let p = derive("क्रिकेट");
    // क्रिकेट is Aagantuk (loanword) — must not become कृकेट
    assert_eq!(p.output, "क्रिकेट");
    assert!(p.is_correct);
}

// Regression: -नु hrasva must only change the last dirgha before suffix
#[test]
fn nu_hrasva_scoped_to_last_dirgha() {
    // Word with TWO ी — only the last one before -नु should change
    // "खरीदीनु" (hypothetical: खरीद + ी + नु) → "खरीदिनु" (second ी→ि, first ई stays)
    let p = derive("खरीदीनु");
    assert_eq!(
        p.output, "खरीदिनु",
        "Only the last ई before -नु should become hrasva"
    );
    assert!(!p.is_correct);
}

// Regression: -नु must not fire on words where नु is internal (not a suffix)
#[test]
fn nu_not_applied_to_internal() {
    let p = derive("अनुभव");
    // अनुभव contains "नु" but it's not a suffix — word should pass through unchanged
    assert_eq!(p.output, "अनुभव");
    assert!(p.is_correct);
}

// =================================================================
// O7: Missing Orthography Rules — acceptance criteria
// =================================================================

// O7.1c: Tatsam ष preserved (sibilant rule does not overwrite)
#[test]
fn o7_tatsam_retroflex_sibilant_preserved() {
    let p = derive("भाषा");
    assert!(p.is_correct, "Tatsam भाषा must not be changed");
}

// O7.3: halanta required on tatsam -मान्/-वान् suffix words
#[test]
fn o7_halanta_mahaan() {
    let p = derive("महान");
    assert!(!p.is_correct);
    assert_eq!(p.output, "महान्");
    assert!(matches!(p.steps[0].rule, Rule::VarnaVinyasNiyam(_)));
}

#[test]
fn o7_halanta_buddhimaan() {
    let p = derive("बुद्धिमान");
    assert_eq!(p.output, "बुद्धिमान्");
}

#[test]
fn o7_halanta_bhagavaan() {
    let p = derive("भगवान");
    assert_eq!(p.output, "भगवान्");
}

#[test]
fn o7_halanta_vidvaan() {
    let p = derive("विद्वान");
    assert_eq!(p.output, "विद्वान्");
}

#[test]
fn o7_halanta_shrimaan() {
    let p = derive("श्रीमान");
    assert_eq!(p.output, "श्रीमान्");
}

#[test]
fn o7_halanta_verb_plural() {
    let p = derive("जान्छन");
    assert_eq!(p.output, "जान्छन्");
}

#[test]
fn o7_halanta_verb_second_person() {
    let p = derive("गर्छस");
    assert_eq!(p.output, "गर्छस्");
}

#[test]
fn o7_ajanta_terminal_chha_without_halanta() {
    let p = derive("जान्छ्");
    assert_eq!(p.output, "जान्छ");
}

// O7.4: क्ष/छ corrections via correction table
#[test]
fn o7_ksha_chhya_lakshya() {
    let p = derive("लछ्य");
    assert_eq!(p.output, "लक्ष्य");
}

#[test]
fn o7_ksha_chhya_ichchha() {
    let p = derive("इक्षा");
    assert_eq!(p.output, "इच्छा");
}

#[test]
fn o7_ksha_chhya_kshetra() {
    let p = derive("छेत्र");
    assert_eq!(p.output, "क्षेत्र");
}

#[test]
fn o7_gya_gyan_agyan() {
    let p = derive("अग्यान");
    assert_eq!(p.output, "अज्ञान");
}

#[test]
fn o7_gya_gyan_keeps_loanword() {
    let p = derive("ग्यारेज");
    assert!(p.is_correct);
    assert_eq!(p.output, "ग्यारेज");
}

#[test]
fn ramailo_dirgha_corrected() {
    let p = derive("रमाईलो");
    assert_eq!(p.output, "रमाइलो");
    assert!(!p.is_correct);
}

#[test]
fn notice_examples_are_not_overcorrected() {
    for word in ["क्षेत्रीय", "संज्ञा", "एसिया", "त्यता", "तापनि", "हरूवा"]
    {
        let p = derive(word);
        assert!(
            p.is_correct,
            "Expected '{word}' to remain correct, got '{}'",
            p.output
        );
    }
}

#[test]
fn notice_example_wrong_eshiya_gets_corrected() {
    let p = derive("एशिया");
    assert_eq!(p.output, "एसिया");
    assert!(!p.is_correct);
}

#[test]
fn notice_section4_uparokta_gets_corrected() {
    let p = derive("उपरोक्त");
    assert_eq!(p.output, "उपर्युक्त");
    assert!(!p.is_correct);
}

// =================================================================
// O8: Section 3(ग) numbered-subrule citation checks
// =================================================================

fn has_varna_niyam_code(p: &varnavinyas_prakriya::Prakriya, code: &str) -> bool {
    p.steps
        .iter()
        .any(|s| matches!(s.rule, Rule::VarnaVinyasNiyam(c) if c == code))
}

#[test]
fn o8_ga_a_9_aagantuk_s_normalization_citation() {
    let p = rule_sibilant("शहिद").expect("expected sibilant correction");
    assert_eq!(p.output, "सहिद");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(अ)-9") || has_varna_niyam_code(&p, "3(ग)(अ)-8"),
        "Expected 3(ग)(अ)-8 or 3(ग)(अ)-9 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_v_1_bi_prefix_citation() {
    let p = rule_ba_va("बिदेश").expect("expected ba/va correction");
    assert_eq!(p.output, "विदेश");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-व-1"),
        "Expected 3(ग)(आ)-व-1 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_v_3_suffix_group_citation() {
    let p = rule_ba_va("मान्यबर").expect("expected ba/va correction");
    assert_eq!(p.output, "मान्यवर");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-व-3"),
        "Expected 3(ग)(आ)-व-3 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_v_10_trailing_group_citation() {
    let p = rule_ba_va("जिम्मेबारी").expect("expected ba/va correction");
    assert_eq!(p.output, "जिम्मेवारी");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-व-10"),
        "Expected 3(ग)(आ)-व-10 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_b_1_contextual_citation() {
    let p = rule_ba_va("वुद्धि").expect("expected ba/va correction");
    assert_eq!(p.output, "बुद्धि");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-ब-1"),
        "Expected 3(ग)(आ)-ब-1 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_b_4_adjective_bucket_citation() {
    let p = rule_ba_va("वुढो").expect("expected ba/va correction");
    assert_eq!(p.output, "बुढो");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-ब-4"),
        "Expected 3(ग)(आ)-ब-4 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_b_5_avyaya_bucket_citation() {
    let p = rule_ba_va("वरु").expect("expected ba/va correction");
    assert_eq!(p.output, "बरु");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-ब-5"),
        "Expected 3(ग)(आ)-ब-5 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_b_6_verb_bucket_citation() {
    let p = rule_ba_va("वग्नु").expect("expected ba/va correction");
    assert_eq!(p.output, "बग्नु");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-ब-6"),
        "Expected 3(ग)(आ)-ब-6 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_i_y_4_tatsam_ya_class_citation() {
    let p = rule_ya_e("एथार्थ").expect("expected ya/e correction");
    assert_eq!(p.output, "यथार्थ");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(इ)-य-4"),
        "Expected 3(ग)(इ)-य-4 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_ii_ri_1_tatsam_ri_kri_citation() {
    let p = rule_ri_kri("रिषि").expect("expected ri/kri correction");
    assert_eq!(p.output, "ऋषि");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(ई)-ऋ-1"),
        "Expected 3(ग)(ई)-ऋ-1 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_u_ksha_1_citation() {
    let p = rule_ksha_chhya("लछ्य").expect("expected ksha/chhya correction");
    assert_eq!(p.output, "लक्ष्य");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(उ)-क्ष-1"),
        "Expected 3(ग)(उ)-क्ष-1 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_uu_3_gya_to_gyaana_citation() {
    let p = rule_gya_gyan("अग्यान").expect("expected gya/gyan correction");
    assert_eq!(p.output, "अज्ञान");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(ऊ)-3"),
        "Expected 3(ग)(ऊ)-3 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_v_2_vai_ref_ri_class_citation() {
    let p = rule_ba_va("बर्ष").expect("expected ba/va correction");
    assert_eq!(p.output, "वर्ष");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-व-2"),
        "Expected 3(ग)(आ)-व-2 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_v_4_sam_prefix_citation() {
    let p = rule_ba_va("संबाद").expect("expected ba/va correction");
    assert_eq!(p.output, "संवाद");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-व-4"),
        "Expected 3(ग)(आ)-व-4 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_v_5_final_v_citation() {
    let p = rule_ba_va("मानब").expect("expected ba/va correction");
    assert_eq!(p.output, "मानव");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-व-5"),
        "Expected 3(ग)(आ)-व-5 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_v_7_adjective_class_citation() {
    let p = rule_ba_va("जुबाडे").expect("expected ba/va correction");
    assert_eq!(p.output, "जुवाडे");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-व-7"),
        "Expected 3(ग)(आ)-व-7 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_v_8_verb_class_citation() {
    let p = rule_ba_va("खुबाउनु").expect("expected ba/va correction");
    assert_eq!(p.output, "खुवाउनु");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-व-8"),
        "Expected 3(ग)(आ)-व-8 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_v_9_avyaya_class_citation() {
    let p = rule_ba_va("बरिपरि").expect("expected ba/va correction");
    assert_eq!(p.output, "वरिपरि");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-व-9"),
        "Expected 3(ग)(आ)-व-9 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_o_1_class_citation() {
    let p = rule_ba_va("उडार").expect("expected o-class correction");
    assert_eq!(p.output, "ओडार");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-ओ-1"),
        "Expected 3(ग)(आ)-ओ-1 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_o_2_verb_citation() {
    let p = rule_ba_va("देऊस्").expect("expected o-verb correction");
    assert_eq!(p.output, "देओस्");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-ओ-2"),
        "Expected 3(ग)(आ)-ओ-2 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o8_ga_aa_o_3_tatsam_citation() {
    let p = rule_ba_va("उजस्वी").expect("expected tatsam o-correction");
    assert_eq!(p.output, "ओजस्वी");
    assert!(
        has_varna_niyam_code(&p, "3(ग)(आ)-ओ-3"),
        "Expected 3(ग)(आ)-ओ-3 citation, got: {:?}",
        p.steps
    );
}

// =================================================================
// O9: Section 3(ङ) halanta/ajanta numbered-subrule regressions
// =================================================================

#[test]
fn o9_nga_halanta_1_root_forms() {
    let p = derive("पढ");
    assert_eq!(p.output, "पढ्");
    let p2 = derive("भन");
    assert_eq!(p2.output, "भन्");
}

#[test]
fn o9_nga_halanta_2_second_person_forms() {
    assert_eq!(derive("गइस").output, "गइस्");
    assert_eq!(derive("भन्छस").output, "भन्छस्");
    assert_eq!(derive("लेख्छस").output, "लेख्छस्");
}

#[test]
fn o9_nga_halanta_3_plural_honorific_forms() {
    assert_eq!(derive("सुन्छन").output, "सुन्छन्");
    assert_eq!(derive("गर्दैनन").output, "गर्दैनन्");
}

#[test]
fn o9_nga_halanta_4_suffix_forms() {
    assert_eq!(derive("गुणवान").output, "गुणवान्");
    assert_eq!(derive("गुरुवत").output, "गुरुवत्");
}

#[test]
fn o9_nga_halanta_tatsam_padanta_restoration() {
    let p = derive("जगत");
    assert_eq!(p.output, "जगत्");
    assert!(
        has_varna_niyam_code(&p, "3(ङ)-पदान्त"),
        "Expected 3(ङ)-पदान्त citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o9_nga_ajanta_8_terminal_halanta_removed() {
    assert_eq!(derive("कस्").output, "कस");
    assert_eq!(derive("जवान्").output, "जवान");
    assert_eq!(derive("कठोर्").output, "कठोर");
}

#[test]
fn o9_nga_ajanta_1_singletons() {
    assert_eq!(derive("र्").output, "र");
    assert_eq!(derive("न्").output, "न");
}

#[test]
fn o9_nga_ajanta_2_vowel_avyaya() {
    assert_eq!(derive("बाहिर्").output, "बाहिर");
    assert_eq!(derive("आज्").output, "आज");
}

#[test]
fn o9_nga_ajanta_3_ajnartha() {
    assert_eq!(derive("भन्").output, "भन");
    assert_eq!(derive("लेख्").output, "लेख");
}

#[test]
fn o9_nga_ajanta_4_negative_n() {
    assert_eq!(derive("गर्दैन्").output, "गर्दैन");
    assert_eq!(derive("भन्दैन्").output, "भन्दैन");
}

#[test]
fn o9_nga_ajanta_6_asamapak() {
    assert_eq!(derive("गर्न्").output, "गर्न");
    assert_eq!(derive("हेर्न्").output, "हेर्न");
}

#[test]
fn o9_nga_ajanta_7_onomatopoeic() {
    assert_eq!(derive("टिलिक्क्").output, "टिलिक्क");
    assert_eq!(derive("स्वाट्ट्").output, "स्वाट्ट");
}

// =================================================================
// O10: Section 3(ख) numbered-subrule citation checks
// =================================================================

#[test]
fn o10_kha_aa_1_tatsam_no_chandrabindu() {
    let p = rule_chandrabindu("सँवाद").expect("expected chandrabindu correction");
    assert_eq!(p.output, "संवाद");
    assert!(
        has_varna_niyam_code(&p, "3(ख)(आ)-1"),
        "Expected 3(ख)(आ)-1 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o10_kha_aa_2_first_person_nasal() {
    let p = rule_chandrabindu("जान्छौं").expect("expected first-person nasal correction");
    assert_eq!(p.output, "जान्छौँ");
    assert!(
        has_varna_niyam_code(&p, "3(ख)(आ)-2"),
        "Expected 3(ख)(आ)-2 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o10_kha_aa_3_da_dai_forms() {
    let p = rule_chandrabindu("आउंदा").expect("expected da/dai nasal correction");
    assert_eq!(p.output, "आउँदा");
    assert!(
        has_varna_niyam_code(&p, "3(ख)(आ)-3"),
        "Expected 3(ख)(आ)-3 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o10_kha_aa_4_dvisvaranta_with_chha_tha() {
    let p = rule_chandrabindu("आउंछ").expect("expected dvisvaranta chha/tha correction");
    assert_eq!(p.output, "आउँछ");
    assert!(
        has_varna_niyam_code(&p, "3(ख)(आ)-4"),
        "Expected 3(ख)(आ)-4 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o10_kha_a_2_ng_group_panchham() {
    let p = rule_panchham_varna("संकेत").expect("expected panchham correction");
    assert_eq!(p.output, "सङ्केत");
    assert!(
        has_varna_niyam_code(&p, "3(ख)(अ)-2-ङ्"),
        "Expected 3(ख)(अ)-2-ङ् citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o10_kha_a_2_nya_group_panchham() {
    let p = rule_panchham_varna("संचार").expect("expected panchham correction");
    assert_eq!(p.output, "सञ्चार");
    assert!(
        has_varna_niyam_code(&p, "3(ख)(अ)-2-ञ्"),
        "Expected 3(ख)(अ)-2-ञ् citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o10_kha_a_2_nna_group_panchham() {
    let p = rule_panchham_varna("कंटक").expect("expected panchham correction");
    assert_eq!(p.output, "कण्टक");
    assert!(
        has_varna_niyam_code(&p, "3(ख)(अ)-2-ण्"),
        "Expected 3(ख)(अ)-2-ण् citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o10_kha_a_2_na_group_panchham() {
    let p = rule_panchham_varna("संतोष").expect("expected panchham correction");
    assert_eq!(p.output, "सन्तोष");
    assert!(
        has_varna_niyam_code(&p, "3(ख)(अ)-2-न्"),
        "Expected 3(ख)(अ)-2-न् citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o10_kha_a_2_ma_group_panchham() {
    let p = rule_panchham_varna("संपन्न").expect("expected panchham correction");
    assert_eq!(p.output, "सम्पन्न");
    assert!(
        has_varna_niyam_code(&p, "3(ख)(अ)-2-म्"),
        "Expected 3(ख)(अ)-2-म् citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o10_kha_a_3_non_tatsam_retroflex_cluster_normalization() {
    let p = derive("झण्डा");
    assert_eq!(p.output, "झन्डा");
    assert!(
        has_varna_niyam_code(&p, "3(ख)(अ)-3"),
        "Expected 3(ख)(अ)-3 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o10_kha_a_3_non_tatsam_foreign_nd_cluster_normalization() {
    let p = derive("फाउण्डेसन");
    assert_eq!(p.output, "फाउन्डेसन");
    assert!(
        has_varna_niyam_code(&p, "3(ख)(अ)-3"),
        "Expected 3(ख)(अ)-3 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o10_kha_a_3_non_tatsam_palatal_cluster_normalization() {
    let p = rule_panchham_varna("इञ्जिन").expect("expected non-tatsam cluster normalization");
    assert_eq!(p.output, "इन्जिन");
    assert!(
        has_varna_niyam_code(&p, "3(ख)(अ)-3"),
        "Expected 3(ख)(अ)-3 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o11_ka_pronoun_final_dirgha() {
    let p = derive("हामि");
    assert_eq!(p.output, "हामी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-7"),
        "Expected 3(क)(ऊ)-7 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o11_ka_pronoun_initial_hrasva() {
    let p = derive("तीमी");
    assert_eq!(p.output, "तिमी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(अ)-5"),
        "Expected 3(क)(अ)-5 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o11_ka_kinship_initial_mid_hrasva() {
    let p = derive("बहीनी");
    assert_eq!(p.output, "बहिनी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(अ)-12"),
        "Expected 3(क)(अ)-12 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o11_ka_kinship_with_suffix_still_normalizes() {
    let p = derive("मीतिनिले");
    assert_eq!(p.output, "मितिनीले");
    assert!(
        has_varna_niyam_code(&p, "3(क)(अ)-12"),
        "Expected 3(क)(अ)-12 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o11_ka_feminine_kinship_final_dirgha() {
    let p = derive("भाउजु");
    assert_eq!(p.output, "भाउजू");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-3"),
        "Expected 3(क)(ऊ)-3 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o11_ka_bhavavachi_final_dirgha() {
    let p = derive("गरिबि");
    assert_eq!(p.output, "गरिबी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-15"),
        "Expected 3(क)(ऊ)-15 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o11_ka_honorific_final_dirgha() {
    let p = derive("रामजि");
    assert_eq!(p.output, "रामजी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-16"),
        "Expected 3(क)(ऊ)-16 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o11_ka_language_final_dirgha() {
    let p = derive("नेपालि");
    assert_eq!(p.output, "नेपाली");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-11"),
        "Expected 3(क)(ऊ)-11 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o11_ka_river_final_dirgha() {
    let p = derive("कोसि");
    assert_eq!(p.output, "कोसी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-11"),
        "Expected 3(क)(ऊ)-11 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o11_ka_hi_final_dirgha() {
    let p = derive("कोहि");
    assert_eq!(p.output, "कोही");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-12"),
        "Expected 3(क)(ऊ)-12 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o11_ka_ari_tari_adverb_dirgha() {
    let p = derive("यसरि");
    assert_eq!(p.output, "यसरी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-13"),
        "Expected 3(क)(ऊ)-13 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o11_ka_profession_jati_thar_i_dirgha() {
    let p = derive("व्यापारि");
    assert_eq!(p.output, "व्यापारी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-5"),
        "Expected 3(क)(ऊ)-5 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o11_ka_profession_jati_thar_u_dirgha() {
    let p = derive("थारु");
    assert_eq!(p.output, "थारू");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-5"),
        "Expected 3(क)(ऊ)-5 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o12_ka_a_1_prefix_hrasva() {
    let p = derive("नीबन्ध");
    assert_eq!(p.output, "निबन्ध");
    assert!(
        has_varna_niyam_code(&p, "3(क)(अ)-1"),
        "Expected 3(क)(अ)-1 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o12_ka_a_2_dvi_tri_hrasva() {
    let p = derive("द्वीतीय");
    assert_eq!(p.output, "द्वितीय");
    assert!(
        has_varna_niyam_code(&p, "3(क)(अ)-2"),
        "Expected 3(क)(अ)-2 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o12_ka_a_3_avyutpanna_name_hrasva() {
    let p = derive("कीसान");
    assert_eq!(p.output, "किसान");
    assert!(
        has_varna_niyam_code(&p, "3(क)(अ)-3"),
        "Expected 3(क)(अ)-3 citation, got: {:?}",
        p.steps
    );
    assert_eq!(p.steps[0].description, "सबै अव्युत्पन्न नामहरू सुरुमा ह्रस्व हुन्छन्");
}

#[test]
fn o12_ka_a_4_aagantuk_name_hrasva() {
    let p = derive("ईन्साफ");
    assert_eq!(p.output, "इन्साफ");
    assert!(
        has_varna_niyam_code(&p, "3(क)(अ)-4"),
        "Expected 3(क)(अ)-4 citation, got: {:?}",
        p.steps
    );
    assert_eq!(p.steps[0].description, "सबै आगन्तुक नामहरू सुरुमा ह्रस्व हुन्छन्");
}

#[test]
fn o12_ka_a_1_upasarga_correct_form_passes() {
    let p = derive("सुमार्ग");
    assert!(
        p.is_correct,
        "Expected उपसर्ग form 'सुमार्ग' to remain correct"
    );
    assert_eq!(p.output, "सुमार्ग");
}

#[test]
fn o12_ka_a_1_upasarga_dirgha_is_corrected() {
    let p = derive("सूमार्ग");
    assert_eq!(p.output, "सुमार्ग");
    assert!(
        has_varna_niyam_code(&p, "3(क)(अ)-1"),
        "Expected 3(क)(अ)-1 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o12_ka_a_6_adjective_hrasva() {
    let p = derive("ईमानदार");
    assert_eq!(p.output, "इमानदार");
    assert!(
        has_varna_niyam_code(&p, "3(क)(अ)-6"),
        "Expected 3(क)(अ)-6 citation, got: {:?}",
        p.steps
    );
    assert_eq!(p.steps[0].description, "विशेषणको सुरुका इकार उकार ह्रस्व हुन्छन्");
}

#[test]
fn o12_ka_a_7_number_hrasva() {
    let p = derive("ऊन्नाइस");
    assert_eq!(p.output, "उन्नाइस");
    assert!(
        has_varna_niyam_code(&p, "3(क)(अ)-7"),
        "Expected 3(क)(अ)-7 citation, got: {:?}",
        p.steps
    );
    assert_eq!(
        p.steps[0].description,
        "सङ्ख्यावाचक शब्दहरू 'तीन'बाहेक सबै सुरुमा ह्रस्व हुन्छन्"
    );
}

#[test]
fn o12_ka_a_7_three_exception_passes() {
    let p = derive("तीन");
    assert!(
        p.is_correct,
        "Expected Academy exception 'तीन' to remain correct"
    );
    assert_eq!(p.output, "तीन");
}

#[test]
fn o12_ka_a_10_avyaya_hrasva() {
    let p = derive("भीत्र");
    assert_eq!(p.output, "भित्र");
    assert!(
        has_varna_niyam_code(&p, "3(क)(अ)-10"),
        "Expected 3(क)(अ)-10 citation, got: {:?}",
        p.steps
    );
    assert_eq!(p.steps[0].description, "अव्ययहरू सबै सुरुमा ह्रस्व हुन्छन्");
}

#[test]
fn o12_ka_a_11_onomatopoeic_hrasva() {
    let p = derive("कीटिक्क");
    assert_eq!(p.output, "किटिक्क");
    assert!(
        has_varna_niyam_code(&p, "3(क)(अ)-11"),
        "Expected 3(क)(अ)-11 citation, got: {:?}",
        p.steps
    );
    assert_eq!(p.steps[0].description, "अनुकरणात्मक शब्दहरू सबै सुरुमा ह्रस्व हुन्छन्");
}

#[test]
fn o13_ka_aa_1_medial_prefix_hrasva() {
    let p = derive("अभीमान");
    assert_eq!(p.output, "अभिमान");
    assert!(
        has_varna_niyam_code(&p, "3(क)(आ)-1"),
        "Expected 3(क)(आ)-1 citation, got: {:?}",
        p.steps
    );
    assert_eq!(
        p.steps[0].description,
        "अनु, अभि, अति, अधि, प्रति, परि उपसर्ग लागेका शब्दमा बिचमा ह्रस्व हुन्छ"
    );
}

#[test]
fn o13_ka_aa_2_medial_suffix_hrasva() {
    let p = derive("भौतीक");
    assert_eq!(p.output, "भौतिक");
    assert!(
        has_varna_niyam_code(&p, "3(क)(आ)-2"),
        "Expected 3(क)(आ)-2 citation, got: {:?}",
        p.steps
    );
    assert_eq!(
        p.steps[0].description,
        "इक, इका, इत, इम, इमा, इष्ठ, उक प्रत्यय लागेका शब्दमा बिचमा ह्रस्व हुन्छ"
    );
}

#[test]
fn o13_ka_aa_3_medial_derived_name_hrasva() {
    let p = derive("बिसाऊनी");
    assert_eq!(p.output, "बिसाउनी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(आ)-3"),
        "Expected 3(क)(आ)-3 citation, got: {:?}",
        p.steps
    );
    assert_eq!(p.steps[0].description, "व्युत्पन्न नामहरू सबै बिचमा ह्रस्व हुन्छन्");
}

#[test]
fn o13_ka_aa_4_medial_underived_name_hrasva() {
    let p = derive("कुकूर");
    assert_eq!(p.output, "कुकुर");
    assert!(
        has_varna_niyam_code(&p, "3(क)(आ)-4"),
        "Expected 3(क)(आ)-4 citation, got: {:?}",
        p.steps
    );
    assert_eq!(p.steps[0].description, "अव्युत्पन्न नामहरू सबै बिचमा ह्रस्व हुन्छन्");
}

#[test]
fn o13_ka_aa_5_medial_aagantuk_name_hrasva() {
    let p = derive("कानून");
    assert_eq!(p.output, "कानुन");
    assert!(
        has_varna_niyam_code(&p, "3(क)(आ)-5"),
        "Expected 3(क)(आ)-5 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o13_ka_aa_6_medial_adjective_hrasva() {
    let p = derive("पोसीलो");
    assert_eq!(p.output, "पोसिलो");
    assert!(
        has_varna_niyam_code(&p, "3(क)(आ)-6"),
        "Expected 3(क)(आ)-6 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o13_ka_aa_9_medial_avyaya_hrasva() {
    let p = derive("अहीले");
    assert_eq!(p.output, "अहिले");
    assert!(
        has_varna_niyam_code(&p, "3(क)(आ)-9"),
        "Expected 3(क)(आ)-9 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o13_ka_aa_10_medial_onomatopoeic_hrasva() {
    let p = derive("टिलीक्क");
    assert_eq!(p.output, "टिलिक्क");
    assert!(
        has_varna_niyam_code(&p, "3(क)(आ)-10"),
        "Expected 3(क)(आ)-10 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o15_ka_u_2_suffix_family_preserves_dirgha_karana() {
    let p = derive("एकिकरण");
    assert_eq!(p.output, "एकीकरण");
    assert!(
        has_varna_niyam_code(&p, "3(क)(उ)-2"),
        "Expected 3(क)(उ)-2 citation, got: {:?}",
        p.steps
    );
    assert_eq!(
        p.steps[0].description,
        "करण, कृत, कार, भवन, भूत, भावसँग जोडिएका शब्दमा बिचमा दीर्घ हुन्छ"
    );
}

#[test]
fn o15_ka_u_2_suffix_family_preserves_dirgha_krita() {
    let p = derive("एकिकृत");
    assert_eq!(p.output, "एकीकृत");
    assert!(
        has_varna_niyam_code(&p, "3(क)(उ)-2"),
        "Expected 3(क)(उ)-2 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o15_ka_ii_2_su_prefix_preserves_dirgha_sukti() {
    let p = derive("सुक्ति");
    assert_eq!(p.output, "सूक्ति");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ई)-2"),
        "Expected 3(क)(ई)-2 citation, got: {:?}",
        p.steps
    );
    assert_eq!(
        p.steps[0].description,
        "उकारादि शब्दमा 'सु' उपसर्ग लागेर बनेका शब्दमा सुरुमा दीर्घ हुन्छ"
    );
}

#[test]
fn o15_ka_ii_1_initial_tatsam_dirgha_ishwar() {
    let p = derive("इश्वर");
    assert_eq!(p.output, "ईश्वर");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ई)-1"),
        "Expected 3(क)(ई)-1 citation, got: {:?}",
        p.steps
    );
    assert_eq!(
        p.steps[0].description,
        "संस्कृतबाट नेपालीमा जस्ताको तस्तै आएका शब्दका सुरुमा दीर्घ हुन्छ"
    );
}

#[test]
fn o15_ka_ii_1_initial_tatsam_dirgha_irshya() {
    let p = derive("इर्ष्या");
    assert_eq!(p.output, "ईर्ष्या");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ई)-1"),
        "Expected 3(क)(ई)-1 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o15_ka_ii_1_initial_tatsam_dirgha_bhumi() {
    let p = derive("भुमि");
    assert_eq!(p.output, "भूमि");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ई)-1"),
        "Expected 3(क)(ई)-1 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o15_ka_ii_1_initial_tatsam_dirgha_suchana() {
    let p = derive("सुचना");
    assert_eq!(p.output, "सूचना");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ई)-1"),
        "Expected 3(क)(ई)-1 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o15_ka_ii_2_su_prefix_preserves_dirgha_sukta() {
    let p = derive("सुक्त");
    assert_eq!(p.output, "सूक्त");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ई)-2"),
        "Expected 3(क)(ई)-2 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o16_ka_uu_1_final_ii_suffix_dirgha_yogi() {
    let p = derive("योगि");
    assert_eq!(p.output, "योगी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-1"),
        "Expected 3(क)(ऊ)-1 citation, got: {:?}",
        p.steps
    );
    assert_eq!(
        p.steps[0].description,
        "'ई' प्रत्यय अन्त्यमा आउने शब्दहरू दीर्घ हुन्छन्"
    );
}

#[test]
fn o16_ka_uu_1_final_ii_suffix_dirgha_tyagi() {
    let p = derive("त्यागि");
    assert_eq!(p.output, "त्यागी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-1"),
        "Expected 3(क)(ऊ)-1 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o16_ka_uu_1_does_not_override_mula_avyaya_pani() {
    let p = derive("पनि");
    assert!(p.is_correct, "Expected मूल अव्यय 'पनि' to remain correct");
    assert_eq!(p.output, "पनि");
}

#[test]
fn o16_ka_uu_1_does_not_override_mula_avyaya_ani() {
    let p = derive("अनि");
    assert!(p.is_correct, "Expected मूल अव्यय 'अनि' to remain correct");
    assert_eq!(p.output, "अनि");
}

#[test]
fn o16_ka_uu_2_final_vati_vi_dirgha_rupavati() {
    let p = derive("रूपवति");
    assert_eq!(p.output, "रूपवती");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-2"),
        "Expected 3(क)(ऊ)-2 citation, got: {:?}",
        p.steps
    );
    assert_eq!(
        p.steps[0].description,
        "वती, वी प्रत्यय लागेर बनेका शब्दहरू दीर्घ हुन्छन्"
    );
}

#[test]
fn o16_ka_uu_2_final_vati_vi_dirgha_gunavati() {
    let p = derive("गुणवति");
    assert_eq!(p.output, "गुणवती");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-2"),
        "Expected 3(क)(ऊ)-2 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o16_ka_uu_2_final_vati_vi_dirgha_medhavi() {
    let p = derive("मेधावि");
    assert_eq!(p.output, "मेधावी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-2"),
        "Expected 3(क)(ऊ)-2 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o16_ka_uu_2_final_vati_vi_dirgha_tapasvi() {
    let p = derive("तपस्वि");
    assert_eq!(p.output, "तपस्वी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-2"),
        "Expected 3(क)(ऊ)-2 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o16_ka_uu_8_adjective_final_dirgha_dhani() {
    let p = derive("धनि");
    assert_eq!(p.output, "धनी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-8"),
        "Expected 3(क)(ऊ)-8 citation, got: {:?}",
        p.steps
    );
    assert_eq!(p.steps[0].description, "सबै ईकारान्त विशेषणहरू दीर्घ हुन्छन्");
}

#[test]
fn o16_ka_uu_8_adjective_final_dirgha_rogi() {
    let p = derive("रोगि");
    assert_eq!(p.output, "रोगी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-8"),
        "Expected 3(क)(ऊ)-8 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o16_ka_uu_9_final_number_dirgha_dui() {
    let p = derive("दुइ");
    assert_eq!(p.output, "दुई");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-9"),
        "Expected 3(क)(ऊ)-9 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o16_ka_uu_9_final_number_dirgha_sathi() {
    let p = derive("साठि");
    assert_eq!(p.output, "साठी");
    assert!(
        has_varna_niyam_code(&p, "3(क)(ऊ)-9"),
        "Expected 3(क)(ऊ)-9 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o14_ka_i_2_location_inanimate_final_hrasva() {
    let p = derive("आलू");
    assert_eq!(p.output, "आलु");
    assert!(
        has_varna_niyam_code(&p, "3(क)(इ)-2"),
        "Expected 3(क)(इ)-2 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o14_ka_i_3_suffix_family_final_hrasva() {
    let p = derive("गराई");
    assert_eq!(p.output, "गराइ");
    assert!(
        has_varna_niyam_code(&p, "3(क)(इ)-3"),
        "Expected 3(क)(इ)-3 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o14_ka_i_4_adjective_suffix_final_hrasva() {
    let p = derive("सिकारू");
    assert_eq!(p.output, "सिकारु");
    assert!(
        has_varna_niyam_code(&p, "3(क)(इ)-4"),
        "Expected 3(क)(इ)-4 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o14_ka_i_5_mula_avyaya_final_hrasva() {
    let p = derive("अगाडी");
    assert_eq!(p.output, "अगाडि");
    assert!(
        has_varna_niyam_code(&p, "3(क)(इ)-5"),
        "Expected 3(क)(इ)-5 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o14_ka_i_6_vibhakti_final_hrasva() {
    let p = derive("निम्ती");
    assert_eq!(p.output, "निम्ति");
    assert!(
        has_varna_niyam_code(&p, "3(क)(इ)-6"),
        "Expected 3(क)(इ)-6 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o14_ka_i_7_ti_avyaya_final_hrasva() {
    let p = derive("त्यती");
    assert_eq!(p.output, "त्यति");
    assert!(
        has_varna_niyam_code(&p, "3(क)(इ)-7"),
        "Expected 3(क)(इ)-7 citation, got: {:?}",
        p.steps
    );
}

#[test]
fn o14_ka_i_9_ti_dhi_ni_ti_pi_final_hrasva() {
    let p = derive("नीती");
    assert_eq!(p.output, "नीति");
    assert!(
        has_varna_niyam_code(&p, "3(क)(इ)-9"),
        "Expected 3(क)(इ)-9 citation, got: {:?}",
        p.steps
    );
}
