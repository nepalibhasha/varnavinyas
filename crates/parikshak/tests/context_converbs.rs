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
