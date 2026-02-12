use varnavinyas_kosha::{kosha, origin_tag};

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

/// Bracket invariant: in headwords.tsv, the **first** `[…]` bracket in each
/// entry is either:
///   (a) a recognized origin tag that `parse_origin_tag` handles, or
///   (b) a word-formation note (e.g. `[word+word]`, `[(द्वि.) X]`) that does
///       NOT start with any origin abbreviation.
///
/// This means `parse_origin_tag`'s first-bracket strategy is safe: it will
/// never skip an origin tag in favour of a word-formation note when the origin
/// tag comes first. If this test fails, the parser or abbreviation lists in
/// `origin_tag.rs` need updating.
#[test]
fn bracket_invariant_all_brackets_are_origin_or_known() {
    let _k = kosha(); // ensure singleton is initialized

    // Origin abbreviation prefixes that must always parse successfully.
    // If a first-bracket starts with one of these, parse_origin_tag MUST
    // return Some(_). (Matches the prefixes in origin_tag::classify_tag.)
    let origin_prefixes: &[&str] = &[
        "सं",
        "अ.",
        "अ ",
        "अङ्",
        "अङ.",
        "अड्",
        "फा",
        "तु",
        "था",
        "फ्रा",
        "फ्रे",
        "पोर्त",
        "ग्री",
        "स्पे",
        "जापा",
        "भा.",
        "प्रा",
        "हि",
        "भो.",
        "मरा",
        "मै",
        "नेवा",
        "लि",
        "मो.",
        "मगा",
        "डो",
        "बा.",
    ];

    let headwords_data = include_str!("../../../data/headwords.tsv");
    let mut missed_origins: Vec<String> = Vec::new();

    for line in headwords_data.lines() {
        let mut parts = line.splitn(2, '\t');
        let word = parts.next().unwrap_or("").trim();
        let pos = parts.next().unwrap_or("").trim();
        if pos.is_empty() || !pos.contains('[') {
            continue;
        }

        // Extract the FIRST bracket only (mirrors parse_origin_tag behavior)
        let Some(start) = pos.find('[') else {
            continue;
        };
        let Some(end_rel) = pos[start..].find(']') else {
            continue;
        };
        let tag_content = pos[start + 1..start + end_rel].trim();

        // Check: does this bracket start with an origin abbreviation?
        let looks_like_origin = origin_prefixes.iter().any(|p| tag_content.starts_with(p));

        if looks_like_origin {
            // It SHOULD parse to Some. If it doesn't, the parser has a gap.
            let parsed = origin_tag::parse_origin_tag(pos);
            if parsed.is_none() {
                missed_origins.push(format!("{word}\t[{tag_content}]"));
            }
        }
        // If it doesn't look like an origin, it's a word-formation note —
        // parse_origin_tag returning None is correct.
    }

    assert!(
        missed_origins.is_empty(),
        "Found {} first-brackets that look like origin tags but failed to parse. \
         Update origin_tag::classify_tag. First 20:\n{}",
        missed_origins.len(),
        missed_origins
            .iter()
            .take(20)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
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
