use varnavinyas_akshar::*;

// =============================================================================
// A1: Classifies all Devanagari codepoints in U+0900-U+097F
// =============================================================================

#[test]
fn a1_all_codepoints_classified() {
    for cp in 0x0900u32..=0x097Fu32 {
        if let Some(c) = char::from_u32(cp) {
            assert!(classify(c).is_some(), "U+{cp:04X} ({c}) not classified");
        }
    }
}

#[test]
fn a1_vowels_classified_as_svar() {
    let svars = ['अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ'];
    for c in svars {
        assert_eq!(
            classify(c).unwrap().char_type,
            CharType::Svar,
            "expected Svar for {c}"
        );
    }
}

#[test]
fn a1_consonants_classified_as_vyanjan() {
    let vyanjans = [
        'क', 'ख', 'ग', 'घ', 'ङ', 'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द',
        'ध', 'न', 'प', 'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
    ];
    for c in vyanjans {
        assert_eq!(
            classify(c).unwrap().char_type,
            CharType::Vyanjan,
            "expected Vyanjan for {c}"
        );
    }
}

#[test]
fn a1_matras_classified() {
    let matras = ['ा', 'ि', 'ी', 'ु', 'ू', 'ृ', 'े', 'ै', 'ो', 'ौ'];
    for c in matras {
        assert_eq!(
            classify(c).unwrap().char_type,
            CharType::Matra,
            "expected Matra for {c}"
        );
    }
}

// =============================================================================
// A2: Maps all hrasva↔dirgha vowel pairs
// =============================================================================

#[test]
fn a2_hrasva_dirgha_svar() {
    assert_eq!(hrasva_to_dirgha('इ'), Some('ई'));
    assert_eq!(hrasva_to_dirgha('उ'), Some('ऊ'));
    assert_eq!(dirgha_to_hrasva('ई'), Some('इ'));
    assert_eq!(dirgha_to_hrasva('ऊ'), Some('उ'));
}

#[test]
fn a2_hrasva_dirgha_matra() {
    assert_eq!(hrasva_to_dirgha('ि'), Some('ी'));
    assert_eq!(hrasva_to_dirgha('ु'), Some('ू'));
    assert_eq!(dirgha_to_hrasva('ी'), Some('ि'));
    assert_eq!(dirgha_to_hrasva('ू'), Some('ु'));
}

#[test]
fn a2_bidirectional_roundtrip() {
    let pairs = [('इ', 'ई'), ('उ', 'ऊ'), ('ि', 'ी'), ('ु', 'ू')];
    for (h, d) in pairs {
        assert_eq!(hrasva_to_dirgha(h), Some(d), "hrasva→dirgha failed for {h}");
        assert_eq!(dirgha_to_hrasva(d), Some(h), "dirgha→hrasva failed for {d}");
    }
}

// =============================================================================
// A3: Syllable segmentation: simple word
// =============================================================================

#[test]
fn a3_kathmandu_4_aksharas() {
    let result = split_aksharas("काठमाडौं");
    let texts: Vec<&str> = result.iter().map(|a| a.text.as_str()).collect();
    assert_eq!(texts, vec!["का", "ठ", "मा", "डौं"]);
    assert_eq!(result.len(), 4);
}

// =============================================================================
// A4: Syllable segmentation: conjuncts
// =============================================================================

#[test]
fn a4_namaste_3_aksharas() {
    let result = split_aksharas("नमस्ते");
    let texts: Vec<&str> = result.iter().map(|a| a.text.as_str()).collect();
    assert_eq!(texts, vec!["न", "मस्", "ते"]);
    assert_eq!(result.len(), 3);
}

// =============================================================================
// A5: Conjuncts are one akshara
// =============================================================================

#[test]
fn a5_pra_is_one_akshara() {
    let result = split_aksharas("प्रशासन");
    let texts: Vec<&str> = result.iter().map(|a| a.text.as_str()).collect();
    assert_eq!(texts[0], "प्र", "प्र should be a single akshara");
}

// =============================================================================
// A6: Normalization idempotence (proptest in normalize.rs)
// =============================================================================

#[test]
fn a6_normalize_idempotent_basic() {
    let texts = ["नमस्ते", "काठमाडौं", "प्रशासन", "विज्ञान", ""];
    for text in texts {
        let once = normalize(text);
        let twice = normalize(&once);
        assert_eq!(once, twice, "normalize not idempotent for {text}");
    }
}

// =============================================================================
// A7: Panchham varna identification
// =============================================================================

#[test]
fn a7_panchham_true() {
    assert!(is_panchham('ङ'));
    assert!(is_panchham('ञ'));
    assert!(is_panchham('ण'));
    assert!(is_panchham('न'));
    assert!(is_panchham('म'));
}

#[test]
fn a7_panchham_false() {
    assert!(!is_panchham('क'));
    assert!(!is_panchham('ग'));
    assert!(!is_panchham('ट'));
    assert!(!is_panchham('प'));
    assert!(!is_panchham('य'));
    assert!(!is_panchham('A'));
}

// =============================================================================
// A8: Matra↔svar conversion roundtrip
// =============================================================================

#[test]
fn a8_matra_svar_roundtrip() {
    let matras = ['ा', 'ि', 'ी', 'ु', 'ू', 'ृ', 'े', 'ै', 'ो', 'ौ'];
    for m in matras {
        let s = matra_to_svar(m).unwrap_or_else(|| panic!("matra_to_svar({m}) returned None"));
        let back = svar_to_matra(s).unwrap_or_else(|| panic!("svar_to_matra({s}) returned None"));
        assert_eq!(back, m, "roundtrip failed: {m} → {s} → {back}");
    }
}

// =============================================================================
// A9: Returns None for non-Devanagari
// =============================================================================

#[test]
fn a9_non_devanagari_none() {
    assert!(classify('A').is_none());
    assert!(classify('z').is_none());
    assert!(classify('中').is_none());
    assert!(classify('0').is_none());
    assert!(classify(' ').is_none());
    assert!(classify('\n').is_none());
    assert!(classify('α').is_none());
}

// =============================================================================
// A10: Zero runtime dependencies beyond workspace (verified by Cargo.toml)
// =============================================================================

// A10 is a build-time constraint, not a runtime test. The crate's Cargo.toml
// lists only unicode-segmentation, unicode-normalization, thiserror, and
// rustc-hash. This is verified by `cargo deny check`.

// =============================================================================
// Edge cases
// =============================================================================

#[test]
fn edge_empty_string() {
    let result = split_aksharas("");
    assert!(result.is_empty());
}

#[test]
fn edge_single_consonant() {
    let result = split_aksharas("क");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].text, "क");
}

#[test]
fn edge_byte_offsets_correct() {
    let text = "नमस्ते नेपाल";
    let result = split_aksharas(text);
    for a in &result {
        assert_eq!(&text[a.start..a.end], a.text, "byte offset mismatch");
    }
}

#[test]
fn edge_svar_type_coverage() {
    // Verify svar_type returns correct classification for all known vowels
    assert_eq!(svar_type('अ'), Some(SvarType::Hrasva));
    assert_eq!(svar_type('आ'), Some(SvarType::Dirgha));
    assert_eq!(svar_type('ए'), Some(SvarType::Dirgha));
    assert_eq!(svar_type('ौ'), Some(SvarType::Dirgha));
    assert_eq!(svar_type('क'), None);
}
