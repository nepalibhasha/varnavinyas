#[cfg(feature = "vyakaran-mvp")]
use varnavinyas_vyakaran::{Case, MorphAnalyzer, Number, Person, RuleBasedAnalyzer, Tense};

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

#[cfg(feature = "vyakaran-mvp")]
#[test]
fn transforms_multiple_present_person_endings_to_negative() {
    assert_eq!(
        varnavinyas_vyakaran::transform_negative("गर्छु").as_deref(),
        Some("गर्दिन")
    );
    assert_eq!(
        varnavinyas_vyakaran::transform_negative("गर्छौ").as_deref(),
        Some("गर्दैनौ")
    );
    assert_eq!(
        varnavinyas_vyakaran::transform_negative("गर्छन्").as_deref(),
        Some("गर्दैनन्")
    );
}

#[cfg(feature = "vyakaran-mvp")]
#[test]
fn detects_person_in_present_negative_endings() {
    let analyzer = RuleBasedAnalyzer;

    let first = analyzer.analyze("गर्दिन").expect("analysis should succeed");
    assert!(first.iter().any(|a| {
        a.suffix.as_deref() == Some("दिन")
            && a.features.tense == Some(Tense::Present)
            && a.features.person == Some(Person::First)
    }));

    let third = analyzer.analyze("गर्दैनन्").expect("analysis should succeed");
    assert!(third.iter().any(|a| {
        a.suffix.as_deref() == Some("दैनन्")
            && a.features.tense == Some(Tense::Present)
            && a.features.person == Some(Person::Third)
    }));
}

#[cfg(feature = "vyakaran-mvp")]
#[test]
fn detects_na_prefix_in_finite_present_forms() {
    let analyzer = RuleBasedAnalyzer;

    let analyses = analyzer.analyze("नगर्छु").expect("analysis should succeed");
    assert!(analyses.iter().any(|a| {
        a.prefix.as_deref() == Some("न")
            && a.lemma == "गर्छु"
            && a.suffix.as_deref() == Some("छु")
            && a.features.tense == Some(Tense::Present)
            && a.features.person == Some(Person::First)
    }));

    let analyses = analyzer.analyze("नजान्छ").expect("analysis should succeed");
    assert!(analyses.iter().any(|a| {
        a.prefix.as_deref() == Some("न")
            && a.lemma == "जान्छ"
            && a.suffix.as_deref() == Some("छ")
            && a.features.tense == Some(Tense::Present)
            && a.features.person == Some(Person::Third)
    }));
}
