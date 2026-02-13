use varnavinyas_sandhi::{SandhiType, apply, split};

// D1: Vowel sandhi: apply
#[test]
fn d1_vowel_sandhi_yan() {
    let result = apply("अति", "अधिक").unwrap();
    assert_eq!(result.output, "अत्यधिक");
    assert_eq!(result.sandhi_type, SandhiType::VowelSandhi);
}

// D2: Visarga sandhi: apply (visarga → र before vowel)
#[test]
fn d2_visarga_sandhi_to_ra() {
    let result = apply("पुनः", "अवलोकन").unwrap();
    assert_eq!(result.output, "पुनरवलोकन");
    assert_eq!(result.sandhi_type, SandhiType::VisargaSandhi);
}

// D3: Visarga retained before sibilant
#[test]
fn d3_visarga_retained() {
    let result = apply("पुनः", "स्थापना").unwrap();
    assert_eq!(result.output, "पुनःस्थापना");
    assert_eq!(result.sandhi_type, SandhiType::VisargaSandhi);
}

// D4: Consonant assimilation
#[test]
fn d4_consonant_assimilation() {
    let result = apply("उत्", "लिखित").unwrap();
    assert_eq!(result.output, "उल्लिखित");
    assert_eq!(result.sandhi_type, SandhiType::ConsonantSandhi);
}

// D5: Sandhi split — vowel
#[test]
fn d5_split_vowel_sandhi() {
    let results = split("अत्यधिक");
    assert!(
        results
            .iter()
            .any(|(first, second, _)| first == "अति" && second == "अधिक"),
        "Expected to find split (अति, अधिक) in results: {results:?}"
    );
}

// D6: Sandhi split — visarga
#[test]
fn d6_split_visarga_sandhi() {
    let results = split("पुनरवलोकन");
    assert!(
        results
            .iter()
            .any(|(first, second, _)| first == "पुनः" && second == "अवलोकन"),
        "Expected to find split (पुनः, अवलोकन) in results: {results:?}"
    );
}

// Additional sandhi tests
#[test]
fn visarga_before_sa() {
    let result = apply("पुनः", "संरचना").unwrap();
    assert_eq!(result.output, "पुनःसंरचना");
}

#[test]
fn consonant_ut_cha() {
    let result = apply("उत्", "चारण").unwrap();
    assert_eq!(result.output, "उच्चारण");
}

#[test]
fn consonant_ut_nati() {
    let result = apply("उत्", "नति").unwrap();
    assert_eq!(result.output, "उन्नति");
}

#[test]
fn empty_input_error() {
    assert!(apply("", "test").is_err());
    assert!(apply("test", "").is_err());
}

// Gemination (महत् + त्व = महत्त्व)
#[test]
fn gemination_mahat_tva() {
    let result = apply("महत्", "त्व").unwrap();
    assert_eq!(result.output, "महत्त्व");
    assert_eq!(result.sandhi_type, SandhiType::ConsonantSandhi);
}

// Visarga → sibilant (satva sandhi)
#[test]
fn visarga_to_palatal_sibilant() {
    // निः + चय → निश्चय
    let result = apply("निः", "चय").unwrap();
    assert_eq!(result.output, "निश्चय");
    assert_eq!(result.sandhi_type, SandhiType::VisargaSandhi);
}

#[test]
fn visarga_to_dental_sibilant() {
    // नमः + ते → नमस्ते
    let result = apply("नमः", "ते").unwrap();
    assert_eq!(result.output, "नमस्ते");
    assert_eq!(result.sandhi_type, SandhiType::VisargaSandhi);
}

#[test]
fn visarga_to_retroflex_sibilant() {
    // निः + ठुर → निष्ठुर
    let result = apply("निः", "ठुर").unwrap();
    assert_eq!(result.output, "निष्ठुर");
    assert_eq!(result.sandhi_type, SandhiType::VisargaSandhi);
}

// Consonant satva: निस् + चल → निश्चल
#[test]
fn consonant_nis_chal() {
    let result = apply("निस्", "चल").unwrap();
    assert_eq!(result.output, "निश्चल");
    assert_eq!(result.sandhi_type, SandhiType::ConsonantSandhi);
}

// Consonant satva: दुस् + चरित्र → दुश्चरित्र
#[test]
fn consonant_dus_charitr() {
    let result = apply("दुस्", "चरित्र").unwrap();
    assert_eq!(result.output, "दुश्चरित्र");
    assert_eq!(result.sandhi_type, SandhiType::ConsonantSandhi);
}
