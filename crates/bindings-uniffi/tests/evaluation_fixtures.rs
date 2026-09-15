use serde_json::Value;
use varnavinyas_bindings_uniffi::{OrthographyMode, PunctuationMode, check_text_with_all_options};

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
