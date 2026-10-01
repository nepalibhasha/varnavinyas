use varnavinyas_parikshak::{
    CheckOptions, DiagnosticCategory, OrthographyMode, check_text_with_options, check_word,
};
use varnavinyas_prakriya::DiagnosticKind;

fn options() -> impl Iterator<Item = CheckOptions> {
    [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ]
    .into_iter()
    .flat_map(|orthography_mode| {
        [false, true].into_iter().map(move |grammar| CheckOptions {
            orthography_mode,
            grammar,
            ..CheckOptions::default()
        })
    })
}

#[test]
fn result_complements_identify_pari_converbs_without_changing_word_checks() {
    assert!(check_word("पारि").is_none());
    for text in [
        "🙂 काम पूरा पारि घर फर्कियो।",
        "वरिपरि घुमाएर वृत्ताकार पारि बनाइएको चौतारो",
        "काम पूरा पारि अगाडि बढ्यो।",
        "काम पूरा पारि राखेर जानुभयो।",
        "काम पूरा\tपारि\u{a0}घर फर्कियो।",
    ] {
        for options in options() {
            let diagnostics = check_text_with_options(text, options);
            let hits: Vec<_> = diagnostics
                .iter()
                .filter(|d| d.incorrect == "पारि")
                .collect();
            assert_eq!(hits.len(), 1, "{text}: {diagnostics:?}");
            let d = hits[0];
            assert_eq!(d.correction, "पारी");
            assert_eq!(d.kind, DiagnosticKind::Error);
            assert_eq!(d.category, DiagnosticCategory::HrasvaDirgha);
            assert_eq!(d.rule.code(), "PS-Saisanik-ह्रस्वदीर्घ-(भ)-context-कृदन्त");
            assert_eq!(&text[d.span.0..d.span.1], "पारि");
            let correct = text.replace("पारि", "पारी");
            assert!(
                !check_text_with_options(&correct, options)
                    .iter()
                    .any(|d| d.incorrect == "पारी")
            );
        }
    }
}

#[test]
fn spatial_and_unbounded_pari_contexts_are_preserved() {
    for text in [
        "राम खोला पारि बस्छ।",
        "नदी पारि बनाइएको चौतारो",
        "कामको पारि राम्रो छ।",
        "पूरा। पारि घर फर्कियो।",
        "पूरा, पारि घर फर्कियो।",
        "पूरा पारि; घर फर्कियो।",
        "पूरा\nपारि घर फर्कियो।",
        "पूरा पारि\nघर फर्कियो।",
        "पूरा\rपारि घर फर्कियो।",
        "पूरा\u{85}पारि घर फर्कियो।",
        "पूरा पारि\u{2028}घर फर्कियो।",
        "पूरा पारि\u{000c}घर फर्कियो।",
        "पूरा ‘पारि’ घर फर्कियो।",
        "पूरा पारि ३. घर फर्कियो।",
        "पूरा पारि",
        "पारि घर फर्कियो।",
    ] {
        for options in options() {
            assert!(
                !check_text_with_options(text, options)
                    .iter()
                    .any(|d| d.incorrect == "पारि"),
                "{text}"
            );
        }
    }
}

#[test]
fn competing_noun_and_verb_readings_offer_reviewable_suggestions() {
    for (short, long, text) in [
        ("मिलाइ", "मिलाई", "विषय मिति मिलाइ लेखिएको सूची"),
        ("मिलाइ", "मिलाई", "कुरा मिलाइ नाफा खाने व्यक्ति"),
        ("पकाइ", "पकाई", "रस पकाइ बाक्लो बनाइएको खाद्य पदार्थ"),
        ("पकाइ", "पकाई", "घिउमा पकाइ पागमा डुबाएर बनाइने दाना"),
        ("लगाइ", "लगाई", "बोटबिरुवा लगाइ गरिने उत्सव"),
        ("लगाइ", "लगाई", "टिको लगाइ दिएर घर गयो।"),
        ("बनाइ", "बनाई", "वातावरण बनाइ स्वास्थ्योपचार गर्ने कार्यक्रम"),
        ("बनाइ", "बनाई", "घुमर्किएको बनाइ घिउमा पकाएर राख्यो।"),
        ("भइ", "भई", "निश्चिन्त भइ समय बिताई बस्नु"),
        ("भइ", "भई", "मिलित भइ दबिएको कुरा"),
        ("लेखि", "लेखी", "रोहबरमा लेखि हस्ताक्षर गरिएको कागज"),
        ("लेखि", "लेखी", "निकासा लिन लेखि पठाइने कागज"),
        // New constructions outside the dictionary probes.
        ("लेखि", "लेखी", "🙂 चिठी लेखि पठायो।"),
        ("पकाइ", "पकाई", "खाना पकाइ खायो।"),
        ("बनाइ", "बनाई", "खाना बनाइ राख्यो।"),
    ] {
        assert!(check_word(short).is_none(), "{short}");
        for options in options() {
            let diagnostics = check_text_with_options(text, options);
            let hits: Vec<_> = diagnostics
                .iter()
                .filter(|d| d.incorrect == short)
                .collect();
            assert_eq!(hits.len(), 1, "{text}: {diagnostics:?}");
            let d = hits[0];
            assert_eq!(d.correction, long);
            assert_eq!(d.kind, DiagnosticKind::Ambiguous);
            assert_eq!(
                d.evidence,
                varnavinyas_parikshak::DiagnosticEvidence::Heuristic
            );
            assert!(d.confidence < 0.8);
            assert!(d.explanation.contains("अभिप्राय हेरेर मात्र"));
            assert_eq!(&text[d.span.0..d.span.1], short);
            assert!(
                !check_text_with_options(&text.replace(short, long), options)
                    .iter()
                    .any(|d| d.incorrect == long)
            );
        }
    }
}

#[test]
fn nominal_adverbial_and_disconnected_contexts_do_not_trigger_suggestions() {
    for text in [
        "यसको मिलाइ राम्रो छ।",
        "खानाको पकाइ राम्रो भयो।",
        "यो लगाइ कठिन छ।",
        "यसको बनाइ राम्रो छ।",
        "उसको भइ राम्रो छ।",
        "तिम्रा लेखि उनी मरेबराबरै भए।",
        "मिलाइ गरिएको छ।",
        "खाना पकाइ सिक्नु पर्छ।",
        "टिको लगाइ राम्रो बनाइएको छ।",
        "कुरा मिलाइको बारेमा लेखिएको छ।",
        "चिठी लेखि",
        "लेखि पठायो।",
        "चिठी, लेखि पठायो।",
        "चिठी ‘लेखि’ पठायो।",
        "चिठी\nलेखि पठायो।",
        "चिठी लेखि; पठायो।",
        "चिठी लेखि\nपठायो।",
        "चिठी लेखि २. पठायो।",
        "कुरा मिलाइ नाफा, खाने व्यक्ति",
        "कुरा मिलाइ नाफा\nखाने व्यक्ति",
    ] {
        for options in options() {
            assert!(
                !check_text_with_options(text, options)
                    .iter()
                    .any(|d| d.rule.code() == "PS-Saisanik-ह्रस्वदीर्घ-(भ)-context-कृदन्त"),
                "{text}"
            );
        }
    }
}

#[test]
fn converb_candidates_coexist_with_final_predicate_and_word_errors() {
    let text = "हामि काम पूरा पारि राखेर सफल होस। चिठी लेखि पठायो।";
    for options in options() {
        let diagnostics = check_text_with_options(text, options);
        for (short, long) in [
            ("हामि", "हामी"),
            ("पारि", "पारी"),
            ("होस", "होस्"),
            ("लेखि", "लेखी"),
        ] {
            assert_eq!(
                diagnostics
                    .iter()
                    .filter(|d| d.incorrect == short && d.correction == long)
                    .count(),
                1,
                "{diagnostics:?}"
            );
        }
    }
}
