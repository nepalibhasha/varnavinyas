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
fn ramailo_dirgha_corrected() {
    let p = derive("रमाईलो");
    assert_eq!(p.output, "रमाइलो");
    assert!(!p.is_correct);
}
