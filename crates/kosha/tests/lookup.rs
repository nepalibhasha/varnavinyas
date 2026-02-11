use varnavinyas_kosha::kosha;

/// K1: The lexicon contains ~109K word forms.
#[test]
fn k1_word_count() {
    let k = kosha();
    assert!(
        k.word_count() > 100_000,
        "Expected >100K words, got {}",
        k.word_count()
    );
}

/// K2: Headword lookup returns POS metadata.
#[test]
fn k2_headword_lookup_returns_pos() {
    let k = kosha();
    // "नेपाले" has POS "वि. [नेपाल+ए]" in the headwords data
    let entry = k.lookup("नेपाले");
    assert!(entry.is_some(), "नेपाले should be a known headword");
    let entry = entry.unwrap();
    assert!(!entry.pos.is_empty(), "POS should not be empty for नेपाले");

    // Headword without POS should still be found (with empty POS)
    let nepal = k.lookup("नेपाल");
    assert!(nepal.is_some(), "नेपाल should be a known headword");
}

/// K3: Lookup is fast. In release mode: < 1μs; in debug: < 10μs.
#[test]
fn k3_lookup_speed() {
    let k = kosha();
    // Warm up
    let _ = k.contains("शासन");

    let start = std::time::Instant::now();
    for _ in 0..10_000 {
        let _ = k.contains("शासन");
    }
    let elapsed = start.elapsed();
    let per_lookup_ns = elapsed.as_nanos() / 10_000;
    // Allow 10μs in debug builds (FST traversal is slower without optimizations)
    assert!(
        per_lookup_ns < 10_000,
        "Expected <10μs per lookup, got {}ns",
        per_lookup_ns
    );
}

/// K4: Unknown words return false / None.
#[test]
fn k4_unknown_words() {
    let k = kosha();
    assert!(!k.contains("xyzxyzxyz"));
    assert!(!k.contains("ज्ञानज्ञानज्ञान"));
    assert!(k.lookup("xyzxyzxyz").is_none());
}

/// K5: Deterministic — multiple calls return same singleton.
#[test]
fn k5_deterministic_singleton() {
    let k1 = kosha();
    let k2 = kosha();
    assert!(std::ptr::eq(k1, k2), "kosha() should return same instance");
}

/// Common Nepali words should be in the lexicon.
#[test]
fn common_words_present() {
    let k = kosha();
    let common = [
        "नेपाल",
        "भाषा",
        "देश",
        "सरकार",
        "शिक्षा",
        "विकास",
        "जनता",
        "शासन",
    ];
    for word in common {
        assert!(k.contains(word), "{word} should be in the lexicon");
    }
}

/// Gold test correct forms should mostly be in the lexicon.
#[test]
fn gold_correct_forms_coverage() {
    let k = kosha();
    let gold_correct = [
        "अत्यधिक",
        "राजनीतिक",
        "उल्लिखित",
        "प्रशासन",
        "विश्लेषण",
        "आकाशवाणी",
        "मन्त्रालय",
        "प्रतिनिधि",
    ];
    for word in gold_correct {
        assert!(
            k.contains(word),
            "gold correct form '{word}' should be in lexicon"
        );
    }
}
