use varnavinyas_shabda::{Origin, classify, decompose, tables};

// S1: Classifies विज्ञान as Tatsam
#[test]
fn s1_classify_tatsam_vigyaan() {
    assert_eq!(classify("विज्ञान"), Origin::Tatsam);
}

// S2: Classifies आगो as Tadbhav
#[test]
fn s2_classify_tadbhav_aago() {
    assert_eq!(classify("आगो"), Origin::Tadbhav);
}

// S3: Classifies टोपी as Deshaj
#[test]
fn s3_classify_deshaj_topi() {
    assert_eq!(classify("टोपी"), Origin::Deshaj);
}

// S4: Classifies कम्प्युटर as Aagantuk
#[test]
fn s4_classify_aagantuk_computer() {
    assert_eq!(classify("कम्प्युटर"), Origin::Aagantuk);
}

// S5: Decomposes प्रशासन
#[test]
fn s5_decompose_prashaasan() {
    let m = decompose("प्रशासन");
    assert_eq!(m.prefixes, vec!["प्र"]);
    assert_eq!(m.root, "शासन");
}

// S6: Decomposes उल्लिखित
#[test]
fn s6_decompose_ullikhit() {
    let m = decompose("उल्लिखित");
    assert_eq!(m.prefixes, vec!["उत्"]);
    assert_eq!(m.root, "लिखित");
}

// Additional classification tests
#[test]
fn classify_tatsam_words() {
    assert_eq!(classify("ऋषि"), Origin::Tatsam);
    assert_eq!(classify("शेष"), Origin::Tatsam);
    assert_eq!(classify("लक्ष्य"), Origin::Tatsam);
    assert_eq!(classify("कृति"), Origin::Tatsam);
    assert_eq!(classify("महत्त्व"), Origin::Tatsam);
}

#[test]
fn classify_tadbhav_words() {
    assert_eq!(classify("हात"), Origin::Tadbhav);
    assert_eq!(classify("मिठो"), Origin::Tadbhav);
    assert_eq!(classify("हामी"), Origin::Tadbhav);
    assert_eq!(classify("दिदी"), Origin::Tadbhav);
}

#[test]
fn classify_aagantuk_words() {
    assert_eq!(classify("रजिस्टर"), Origin::Aagantuk);
    assert_eq!(classify("इन्डिया"), Origin::Aagantuk);
    assert_eq!(classify("मुद्दा"), Origin::Aagantuk);
}

#[test]
fn classify_deshaj_words() {
    assert_eq!(classify("टोपी"), Origin::Deshaj);
    assert_eq!(classify("चुला"), Origin::Deshaj);
    assert_eq!(classify("भाका"), Origin::Deshaj);
}

#[test]
fn decompose_empty() {
    let m = decompose("");
    assert_eq!(m.root, "");
    assert!(m.prefixes.is_empty());
    assert!(m.suffixes.is_empty());
}

#[test]
fn decompose_simple_word() {
    let m = decompose("शासन");
    assert!(m.prefixes.is_empty());
    // Should not strip any prefix from शासन itself
    assert!(!m.root.is_empty());
}

// Regression: prefix + suffix should co-occur when root remains large enough
#[test]
fn decompose_prefix_and_suffix_together() {
    // सुन्दरता with prefix = no prefix, suffix = ता, root = सुन्दर
    let m = decompose("सुन्दरता");
    assert!(m.prefixes.is_empty());
    assert_eq!(m.suffixes, vec!["ता"]);
    assert_eq!(m.root, "सुन्दर");
}

// Regression: उल्लिखित must NOT over-decompose (root must stay "लिखित", not "लिख")
#[test]
fn decompose_ullikhit_no_over_decompose() {
    let m = decompose("उल्लिखित");
    assert_eq!(m.prefixes, vec!["उत्"]);
    // "लिखित" should NOT be further decomposed to "लिख" + suffix "ित"
    // because the root would be too short (3 chars)
    assert_eq!(m.root, "लिखित");
    assert!(m.suffixes.is_empty());
}

/// PREFIX_FORMS must be sorted by descending sandhi_form byte length.
#[test]
fn prefix_forms_sorted_descending_by_byte_length() {
    let forms = tables::PREFIX_FORMS;
    for window in forms.windows(2) {
        let a_len = window[0].1.len();
        let b_len = window[1].1.len();
        assert!(
            a_len >= b_len,
            "PREFIX_FORMS not sorted: {:?} ({}B) before {:?} ({}B)",
            window[0].1,
            a_len,
            window[1].1,
            b_len
        );
    }
}

/// SUFFIXES must be sorted by descending byte length.
#[test]
fn suffixes_sorted_descending_by_byte_length() {
    let suffixes = tables::SUFFIXES;
    for window in suffixes.windows(2) {
        let a_len = window[0].len();
        let b_len = window[1].len();
        assert!(
            a_len >= b_len,
            "SUFFIXES not sorted: {:?} ({}B) before {:?} ({}B)",
            window[0],
            a_len,
            window[1],
            b_len
        );
    }
}
