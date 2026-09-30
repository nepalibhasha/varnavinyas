use varnavinyas_parikshak::{CheckOptions, OrthographyMode, check_text_with_options, check_word};

#[test]
fn source_approved_dotted_initials_are_preserved_in_both_modes() {
    for mode in [
        OrthographyMode::AcademyStrict,
        OrthographyMode::CommonEditorial,
    ] {
        for text in [
            "बी.बी.सी.",
            "सी.डी.ओ.\nनेपाल",
            "बी.बी.सी. समाचार प्रसारण गर्छ।",
            "सी.डी.ओ. कार्यालय खुला छ।",
            "एस.एल.सी. परीक्षा भयो।",
            "🙂 ‘बी.बी.सी.’ समाचार",
            "त्रि.वि.ले परीक्षा लियो।",
        ] {
            let diagnostics = check_text_with_options(
                text,
                CheckOptions {
                    orthography_mode: mode,
                    ..CheckOptions::default()
                },
            );
            assert!(diagnostics.is_empty(), "{text}: {diagnostics:?}");
        }
    }
}

#[test]
fn abbreviation_guard_does_not_hide_surrounding_errors() {
    let text = "हामि बी.बी.सी. समाचार पढ्छौँ.\n";
    let diagnostics = check_text_with_options(text, CheckOptions::default());
    assert!(
        diagnostics
            .iter()
            .any(|d| d.incorrect == "हामि" && d.correction == "हामी")
    );
    assert!(
        diagnostics
            .iter()
            .any(|d| d.incorrect == "." && d.correction == "।")
    );
    for diagnostic in diagnostics {
        assert_eq!(
            &text[diagnostic.span.0..diagnostic.span.1],
            diagnostic.incorrect
        );
        assert_ne!(diagnostic.incorrect, "सी");
    }
    // Standalone ordinary words still use the word rules; protection is contextual.
    assert_eq!(check_word("सी").unwrap().correction, "सि");
}
