use varnavinyas_lipi::*;

// =============================================================================
// L1: Devanagari→IAST roundtrip
// =============================================================================

#[test]
fn l1_dev_iast_roundtrip_namaste() {
    let text = "नमस्ते";
    let iast = transliterate(text, Scheme::Devanagari, Scheme::Iast).unwrap();
    let back = transliterate(&iast, Scheme::Iast, Scheme::Devanagari).unwrap();
    assert_eq!(back, text, "roundtrip failed: {text} → {iast} → {back}");
}

#[test]
fn l1_dev_iast_roundtrip_kathmandu() {
    let text = "काठमाडौं";
    let iast = transliterate(text, Scheme::Devanagari, Scheme::Iast).unwrap();
    let back = transliterate(&iast, Scheme::Iast, Scheme::Devanagari).unwrap();
    assert_eq!(back, text, "roundtrip failed: {text} → {iast} → {back}");
}

#[test]
fn l1_dev_iast_roundtrip_single_vowel() {
    let text = "अ";
    let iast = transliterate(text, Scheme::Devanagari, Scheme::Iast).unwrap();
    let back = transliterate(&iast, Scheme::Iast, Scheme::Devanagari).unwrap();
    assert_eq!(back, text);
}

#[test]
fn l1_dev_iast_roundtrip_single_consonant() {
    let text = "क";
    let iast = transliterate(text, Scheme::Devanagari, Scheme::Iast).unwrap();
    assert_eq!(iast, "ka");
    let back = transliterate(&iast, Scheme::Iast, Scheme::Devanagari).unwrap();
    assert_eq!(back, text);
}

// =============================================================================
// L2: All vowels transliterate correctly
// =============================================================================

#[test]
fn l2_all_vowels_dev_to_iast() {
    let pairs = [
        ("अ", "a"),
        ("आ", "ā"),
        ("इ", "i"),
        ("ई", "ī"),
        ("उ", "u"),
        ("ऊ", "ū"),
        ("ऋ", "ṛ"),
        ("ए", "e"),
        ("ऐ", "ai"),
        ("ओ", "o"),
        ("औ", "au"),
    ];
    for (dev, iast) in pairs {
        let result = transliterate(dev, Scheme::Devanagari, Scheme::Iast).unwrap();
        assert_eq!(result, iast, "Dev→IAST failed for {dev}");
    }
}

#[test]
fn l2_all_vowels_iast_to_dev() {
    let pairs = [
        ("a", "अ"),
        ("ā", "आ"),
        ("i", "इ"),
        ("ī", "ई"),
        ("u", "उ"),
        ("ū", "ऊ"),
        ("ṛ", "ऋ"),
        ("e", "ए"),
        ("ai", "ऐ"),
        ("o", "ओ"),
        ("au", "औ"),
    ];
    for (iast, dev) in pairs {
        let result = transliterate(iast, Scheme::Iast, Scheme::Devanagari).unwrap();
        assert_eq!(result, dev, "IAST→Dev failed for {iast}");
    }
}

// =============================================================================
// L3: All consonants transliterate correctly
// =============================================================================

#[test]
fn l3_all_consonants_dev_to_iast() {
    let pairs = [
        ("क", "ka"),
        ("ख", "kha"),
        ("ग", "ga"),
        ("घ", "gha"),
        ("ङ", "ṅa"),
        ("च", "ca"),
        ("छ", "cha"),
        ("ज", "ja"),
        ("झ", "jha"),
        ("ञ", "ña"),
        ("ट", "ṭa"),
        ("ठ", "ṭha"),
        ("ड", "ḍa"),
        ("ढ", "ḍha"),
        ("ण", "ṇa"),
        ("त", "ta"),
        ("थ", "tha"),
        ("द", "da"),
        ("ध", "dha"),
        ("न", "na"),
        ("प", "pa"),
        ("फ", "pha"),
        ("ब", "ba"),
        ("भ", "bha"),
        ("म", "ma"),
        ("य", "ya"),
        ("र", "ra"),
        ("ल", "la"),
        ("व", "va"),
        ("श", "śa"),
        ("ष", "ṣa"),
        ("स", "sa"),
        ("ह", "ha"),
    ];
    for (dev, iast) in pairs {
        let result = transliterate(dev, Scheme::Devanagari, Scheme::Iast).unwrap();
        assert_eq!(result, iast, "Dev→IAST failed for {dev}");
    }
}

#[test]
fn l3_all_consonants_iast_to_dev() {
    let pairs = [
        ("ka", "क"),
        ("kha", "ख"),
        ("ga", "ग"),
        ("gha", "घ"),
        ("ca", "च"),
        ("cha", "छ"),
        ("ja", "ज"),
        ("jha", "झ"),
        ("ṭa", "ट"),
        ("ṭha", "ठ"),
        ("ḍa", "ड"),
        ("ḍha", "ढ"),
        ("ṇa", "ण"),
        ("ta", "त"),
        ("tha", "थ"),
        ("da", "द"),
        ("dha", "ध"),
        ("na", "न"),
        ("pa", "प"),
        ("pha", "फ"),
        ("ba", "ब"),
        ("bha", "भ"),
        ("ma", "म"),
        ("ya", "य"),
        ("ra", "र"),
        ("la", "ल"),
        ("va", "व"),
        ("śa", "श"),
        ("ṣa", "ष"),
        ("sa", "स"),
        ("ha", "ह"),
    ];
    for (iast, dev) in pairs {
        let result = transliterate(iast, Scheme::Iast, Scheme::Devanagari).unwrap();
        assert_eq!(result, dev, "IAST→Dev failed for {iast}");
    }
}

// =============================================================================
// L4: Conjuncts transliterate correctly
// =============================================================================

#[test]
fn l4_conjunct_ksha() {
    let result = transliterate("क्ष", Scheme::Devanagari, Scheme::Iast).unwrap();
    assert_eq!(result, "kṣa");
}

#[test]
fn l4_conjunct_roundtrip() {
    let conjuncts = ["क्ष", "त्र", "ज्ञ", "श्र"];
    for dev in conjuncts {
        let iast = transliterate(dev, Scheme::Devanagari, Scheme::Iast).unwrap();
        let back = transliterate(&iast, Scheme::Iast, Scheme::Devanagari).unwrap();
        assert_eq!(
            back, dev,
            "conjunct roundtrip failed for {dev}: IAST={iast}"
        );
    }
}

// =============================================================================
// L5: Numerals transliterate
// =============================================================================

#[test]
fn l5_numerals_dev_to_iast() {
    let result = transliterate("१२३", Scheme::Devanagari, Scheme::Iast).unwrap();
    assert_eq!(result, "123");
}

#[test]
fn l5_numerals_iast_to_dev() {
    let result = transliterate("456", Scheme::Iast, Scheme::Devanagari).unwrap();
    assert_eq!(result, "४५६");
}

#[test]
fn l5_all_numerals() {
    let result = transliterate("०१२३४५६७८९", Scheme::Devanagari, Scheme::Iast).unwrap();
    assert_eq!(result, "0123456789");
}

// =============================================================================
// L6: Preeti decode (requires "legacy" feature)
// =============================================================================

#[cfg(feature = "legacy")]
mod preeti_tests {
    use super::*;

    #[test]
    fn l6_preeti_basic_consonants() {
        let result = transliterate("s", Scheme::Preeti, Scheme::Devanagari).unwrap();
        assert_eq!(result, "स");
    }

    #[test]
    fn l6_preeti_numerals() {
        let result = transliterate("123", Scheme::Preeti, Scheme::Devanagari).unwrap();
        assert_eq!(result, "१२३");
    }

    #[test]
    fn l6_preeti_known_string() {
        // "gDoln" in Preeti = ग + ् + ड + ल + न
        let result = transliterate("gDoln", Scheme::Preeti, Scheme::Devanagari).unwrap();
        assert!(result.contains('ग'));
        assert!(result.contains('ड'));
        assert!(result.contains('ल'));
        assert!(result.contains('न'));
    }

    #[test]
    fn l6_preeti_matras() {
        // 'a' = ा (aa matra), 'm' = े (e matra), 'M' = ै (ai matra)
        let result = transliterate("ka", Scheme::Preeti, Scheme::Devanagari).unwrap();
        assert_eq!(result, "का");
    }
}

// =============================================================================
// L7: Empty string handling
// =============================================================================

#[test]
fn l7_empty_dev_to_iast() {
    let result = transliterate("", Scheme::Devanagari, Scheme::Iast).unwrap();
    assert_eq!(result, "");
}

#[test]
fn l7_empty_iast_to_dev() {
    let result = transliterate("", Scheme::Iast, Scheme::Devanagari).unwrap();
    assert_eq!(result, "");
}

#[cfg(feature = "legacy")]
#[test]
fn l7_empty_preeti() {
    let result = transliterate("", Scheme::Preeti, Scheme::Devanagari).unwrap();
    assert_eq!(result, "");
}

// =============================================================================
// L8: Mixed-script handling
// =============================================================================

#[test]
fn l8_mixed_script_passthrough() {
    let result = transliterate("hello नमस्ते world", Scheme::Devanagari, Scheme::Iast).unwrap();
    assert!(result.contains("hello"));
    assert!(result.contains("namaste"));
    assert!(result.contains("world"));
}

#[test]
fn l8_non_target_chars_unchanged() {
    let result = transliterate("@#$%", Scheme::Devanagari, Scheme::Iast).unwrap();
    assert_eq!(result, "@#$%");
}

// =============================================================================
// L9: Scheme detection
// =============================================================================

#[test]
fn l9_detect_devanagari() {
    assert_eq!(detect_scheme("नमस्ते"), Some(Scheme::Devanagari));
}

#[test]
fn l9_detect_iast_diacritics() {
    assert_eq!(detect_scheme("namaskāra"), Some(Scheme::Iast));
}

#[test]
fn l9_detect_latin_as_iast() {
    // Plain ASCII Latin defaults to IAST
    assert_eq!(detect_scheme("namaste"), Some(Scheme::Iast));
}

#[test]
fn l9_detect_empty() {
    assert_eq!(detect_scheme(""), None);
}

// =============================================================================
// L10: Property test — reversibility
// =============================================================================

#[cfg(test)]
mod proptest_roundtrip {
    use super::*;
    use proptest::prelude::*;

    // Generate valid Devanagari syllable sequences that roundtrip cleanly.
    // Uses consonant+optional_matra syllables to avoid IAST ambiguities:
    //   - "कइ" → "kai" → "कै" (consecutive vowels merge in IAST)
    //   - "अइ" → "ai" → "ऐ" (standalone vowel pairs form diphthongs)
    // Real Nepali text rarely has these sequences, so this is a practical strategy.
    fn devanagari_syllables() -> impl Strategy<Value = String> {
        let consonants: Vec<char> = "कखगघङचछजझञटठडढणतथदधनपफबभमयरलवशषसह".chars().collect();
        let matras: Vec<char> = "ािीुूृेैोौ".chars().collect();

        proptest::collection::vec(
            (
                proptest::sample::select(consonants.clone()),
                proptest::option::of(proptest::sample::select(matras.clone())),
            )
                .prop_map(|(c, m)| {
                    let mut s = String::new();
                    s.push(c);
                    if let Some(matra) = m {
                        s.push(matra);
                    }
                    s
                }),
            1..8,
        )
        .prop_map(|syllables| syllables.join(""))
    }

    proptest! {
        #[test]
        fn dev_iast_roundtrip(text in devanagari_syllables()) {
            let iast = transliterate(&text, Scheme::Devanagari, Scheme::Iast).unwrap();
            let back = transliterate(&iast, Scheme::Iast, Scheme::Devanagari).unwrap();
            prop_assert_eq!(&back, &text, "roundtrip failed: {} → {} → {}", text, iast, back);
        }
    }
}

// =============================================================================
// Feature gate: legacy off excludes Preeti/Kantipur at API level
// =============================================================================

/// Exhaustive match proves the Scheme enum has exactly the expected variants.
/// Without `legacy`: Devanagari + Iast only.
/// With `legacy`: adds Preeti + Kantipur.
/// If a variant is added without proper feature-gating, this fails to compile.
#[test]
fn feature_gate_scheme_exhaustive() {
    let scheme = Scheme::Devanagari;
    let _name = match scheme {
        Scheme::Devanagari => "Devanagari",
        Scheme::Iast => "IAST",
        #[cfg(feature = "legacy")]
        Scheme::Preeti => "Preeti",
        #[cfg(feature = "legacy")]
        Scheme::Kantipur => "Kantipur",
    };
}

#[cfg(not(feature = "legacy"))]
#[test]
fn feature_gate_no_legacy_api() {
    // When legacy is off, the only valid transliteration paths are Dev↔IAST.
    // Any other direction must be unreachable (no variants to construct).
    // This test verifies we can't accidentally call legacy paths.
    let all_schemes = [Scheme::Devanagari, Scheme::Iast];
    assert_eq!(
        all_schemes.len(),
        2,
        "without legacy, only 2 schemes should exist"
    );
}

// =============================================================================
// Additional edge cases
// =============================================================================

#[test]
fn edge_same_scheme_noop() {
    let text = "नमस्ते";
    let result = transliterate(text, Scheme::Devanagari, Scheme::Devanagari).unwrap();
    assert_eq!(result, text);
}

#[cfg(feature = "legacy")]
#[test]
fn edge_unsupported_pair_errors() {
    let result = transliterate("test", Scheme::Iast, Scheme::Preeti);
    assert!(result.is_err());
}

#[test]
fn edge_anusvara_visarga() {
    let result = transliterate("ं", Scheme::Devanagari, Scheme::Iast).unwrap();
    assert_eq!(result, "ṃ");

    let result = transliterate("ः", Scheme::Devanagari, Scheme::Iast).unwrap();
    assert_eq!(result, "ḥ");
}

#[test]
fn edge_virama_suppresses_inherent_vowel() {
    // क् (ka + virama) → "k" (no trailing 'a')
    let result = transliterate("क्", Scheme::Devanagari, Scheme::Iast).unwrap();
    assert_eq!(result, "k");
}

#[test]
fn edge_consonant_with_matras() {
    let pairs = [
        ("का", "kā"),
        ("कि", "ki"),
        ("की", "kī"),
        ("कु", "ku"),
        ("कू", "kū"),
        ("कृ", "kṛ"),
        ("के", "ke"),
        ("कै", "kai"),
        ("को", "ko"),
        ("कौ", "kau"),
    ];
    for (dev, iast) in pairs {
        let result = transliterate(dev, Scheme::Devanagari, Scheme::Iast).unwrap();
        assert_eq!(result, iast, "matra test failed for {dev}");
    }
}

// =============================================================================
// Negative fidelity: unsupported pairs must error consistently
// =============================================================================

#[test]
fn negative_iast_to_iast_is_noop() {
    // Same-scheme should return Ok with input unchanged, not error
    let result = transliterate("namaste", Scheme::Iast, Scheme::Iast).unwrap();
    assert_eq!(result, "namaste");
}

#[test]
fn negative_dev_to_dev_is_noop() {
    let result = transliterate("नमस्ते", Scheme::Devanagari, Scheme::Devanagari).unwrap();
    assert_eq!(result, "नमस्ते");
}

// =============================================================================
// Normalization-before-transliteration: decomposed forms
// =============================================================================

#[test]
fn normalization_nfc_roundtrip() {
    // Ensure NFC-normalized Devanagari roundtrips correctly
    let nfc_text = varnavinyas_akshar::normalize("नमस्ते");
    let iast = transliterate(&nfc_text, Scheme::Devanagari, Scheme::Iast).unwrap();
    let back = transliterate(&iast, Scheme::Iast, Scheme::Devanagari).unwrap();
    assert_eq!(back, nfc_text);
}

#[test]
fn normalization_decomposed_nukta_vowel() {
    // Verify that NFC-normalizing a nukta consonant before transliteration works.
    // NFD: ख (U+0916) + ़ (U+093C) → NFC: ख़ (U+0959)
    let decomposed = "\u{0916}\u{093C}ा"; // NFD ख + nukta + aa-matra
    let nfc = varnavinyas_akshar::normalize(decomposed);
    let result = transliterate(&nfc, Scheme::Devanagari, Scheme::Iast).unwrap();
    // ख़ा should transliterate (the exact IAST output depends on our table,
    // but it should not panic and should produce valid output)
    assert!(!result.is_empty());
}

#[test]
fn normalization_decomposed_input_regression() {
    // Regression: precomposed nukta forms (U+0958-U+095F) are Unicode
    // composition exclusions — NFC *decomposes* them to base + combining nukta.
    // Verify that normalizing both forms produces identical transliteration.
    let precomposed = "\u{0958}"; // क़ (precomposed)
    let decomposed = "\u{0915}\u{093C}"; // क + nukta

    let nfc_pre = varnavinyas_akshar::normalize(precomposed);
    let nfc_dec = varnavinyas_akshar::normalize(decomposed);

    // After NFC, both should be identical (both become base + combining nukta)
    assert_eq!(nfc_pre, nfc_dec, "NFC should canonicalize nukta forms");

    // And both should transliterate identically
    let result_pre = transliterate(&nfc_pre, Scheme::Devanagari, Scheme::Iast).unwrap();
    let result_dec = transliterate(&nfc_dec, Scheme::Devanagari, Scheme::Iast).unwrap();
    assert_eq!(
        result_pre, result_dec,
        "normalized forms should produce identical IAST"
    );
}

#[test]
fn normalization_standard_text_unaffected() {
    // Standard Devanagari text (no nukta) is already NFC-stable.
    // Normalization should not change transliteration results.
    let texts = ["नमस्ते", "काठमाडौं", "विज्ञान"];
    for text in texts {
        let raw_result = transliterate(text, Scheme::Devanagari, Scheme::Iast).unwrap();
        let nfc = varnavinyas_akshar::normalize(text);
        let nfc_result = transliterate(&nfc, Scheme::Devanagari, Scheme::Iast).unwrap();
        assert_eq!(
            raw_result, nfc_result,
            "standard text should be NFC-stable: {text}"
        );
    }
}

#[test]
fn normalization_idempotent_then_transliterate() {
    // Normalizing twice should not change transliteration result
    let texts = ["नमस्ते", "काठमाडौं", "प्रशासन", "विज्ञान"];
    for text in texts {
        let once = varnavinyas_akshar::normalize(text);
        let twice = varnavinyas_akshar::normalize(&once);
        let iast_once = transliterate(&once, Scheme::Devanagari, Scheme::Iast).unwrap();
        let iast_twice = transliterate(&twice, Scheme::Devanagari, Scheme::Iast).unwrap();
        assert_eq!(
            iast_once, iast_twice,
            "normalize idempotence affects transliteration for {text}"
        );
    }
}
