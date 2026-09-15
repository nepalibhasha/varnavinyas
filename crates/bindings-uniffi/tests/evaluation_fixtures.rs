use serde_json::Value;
use varnavinyas_bindings_uniffi::{OrthographyMode, PunctuationMode, check_text_with_all_options};

#[test]
fn shared_origins_distinguish_unknown_inferred_and_documented() {
    use varnavinyas_bindings_uniffi::{classify, classify_with_provenance};
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../docs/tests/origin_classification.json"
    ))
    .unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        let word = case["word"].as_str().unwrap();
        let actual = classify_with_provenance(word.into());
        let expected = &case["expected"];
        assert_eq!(
            actual.origin.map(|o| format!("{o:?}").to_lowercase()),
            expected["origin"].as_str().map(str::to_owned),
            "{}",
            case["id"]
        );
        assert_eq!(
            format!("{:?}", actual.source).to_lowercase(),
            expected["source"]
        );
        assert!(
            (f64::from(actual.confidence) - expected["confidence"].as_f64().unwrap()).abs() < 1e-6
        );
        assert_eq!(
            format!("{:?}", classify(word.into())).to_lowercase(),
            case["legacy_origin"]
        );
    }
}

#[test]
fn shared_mobile_diagnostics_match_the_public_api() {
    let fixtures: Value =
        serde_json::from_str(include_str!("../../../docs/tests/mobile_diagnostics.json")).unwrap();
    assert_eq!(fixtures["fixture_schema_version"], 1);
    assert_eq!(fixtures["span_unit"], "utf8-bytes");
    for case in fixtures["cases"].as_array().unwrap() {
        let options = &case["options"];
        let mode = match options["orthography_mode"].as_str().unwrap() {
            "academy-strict" => OrthographyMode::AcademyStrict,
            "common-editorial" => OrthographyMode::CommonEditorial,
            other => panic!("unsupported fixture mode: {other}"),
        };
        let actual: Value = serde_json::from_str(&check_text_with_all_options(
            case["text"].as_str().unwrap().into(),
            options["grammar"].as_bool().unwrap(),
            PunctuationMode::Strict,
            mode,
            false,
        ))
        .unwrap();
        assert_eq!(actual, case["expected_diagnostics"], "{}", case["id"]);
    }
}
