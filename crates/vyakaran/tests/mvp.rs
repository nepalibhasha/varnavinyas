#[cfg(feature = "vyakaran-mvp")]
use varnavinyas_vyakaran::{Case, MorphAnalyzer, Number, RuleBasedAnalyzer, Tense};

#[cfg(feature = "vyakaran-mvp")]
#[test]
fn detects_plural_genitive_stack() {
    let analyzer = RuleBasedAnalyzer;
    let analyses = analyzer
        .analyze("केटाहरूको")
        .expect("analysis should succeed");
    let nominal = analyses
        .iter()
        .find(|a| a.features.case == Some(Case::Genitive))
        .expect("expected genitive analysis");

    assert_eq!(nominal.features.number, Some(Number::Plural));
    assert_eq!(nominal.suffix.as_deref(), Some("हरूको"));
}

#[cfg(feature = "vyakaran-mvp")]
#[test]
fn detects_progressive_present() {
    let analyzer = RuleBasedAnalyzer;
    let analyses = analyzer.analyze("गर्दैछ").expect("analysis should succeed");
    assert!(
        analyses
            .iter()
            .any(|a| a.features.tense == Some(Tense::Present) && a.suffix.as_deref() == Some("छ"))
    );
}
