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

#[cfg(feature = "vyakaran-mvp")]
#[test]
fn detects_na_prefix_in_nonfinite_forms() {
    let analyzer = RuleBasedAnalyzer;
    let analyses = analyzer.analyze("नगर्दा").expect("analysis should succeed");

    assert!(
        analyses
            .iter()
            .any(|a| { a.prefix.as_deref() == Some("न") && a.lemma == "गर्दा" })
    );

    let analyses = analyzer.analyze("नखाई").expect("analysis should succeed");
    assert!(
        analyses
            .iter()
            .any(|a| { a.prefix.as_deref() == Some("न") && a.lemma == "खाई" })
    );
}

#[cfg(feature = "vyakaran-mvp")]
#[test]
fn transforms_present_positive_to_negative() {
    let out = varnavinyas_vyakaran::transform_negative("गर्छ");
    assert_eq!(out.as_deref(), Some("गर्दैन"));
}
