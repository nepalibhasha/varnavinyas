use varnavinyas_prakriya::orthographic::{
    rule_ba_va, rule_gya_gyan, rule_ksha_chhya, rule_ri_kri, rule_sibilant, rule_ya_e,
};
use varnavinyas_prakriya::{Rule, derive};

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
fn sibilant_correction() {
    let p = derive("सासन");
    assert_eq!(p.output, "शासन");
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
