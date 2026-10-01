use serde_json::{Value, json};
use varnavinyas_bindings_uniffi::{OrthographyMode, PunctuationMode, check_text_with_all_options};

fn main() {
    let mut cases = Vec::new();
    for (id, text, grammar) in [
        ("clean", "हामी", false),
        ("pronoun", "हामि", false),
        ("ambiguous-verb", "हेर हेर्", false),
        ("reviewed-variant", "संघीय", false),
        ("utf8-multiline", "नेपाल\nहामि", false),
        ("alternate-reasons", "भौतीक", false),
        ("grammar", "सूर्योदय भयो।", true),
        ("punctuation", "नेपाल , राम्रो", false),
        ("converb-inflected-predicate", "🙂 चिठी लेखि पठाइयो।", false),
        ("converb-inflected-object", "पत्रहरूलाई लेखि पठाइन्।", true),
        ("converb-adverb-reading", "तिम्रा लेखि उनी मरेबराबरै भए।", false),
        ("converb-genitive-guard", "पत्रको लेखि पठाइन्।", false),
        (
            "grammar-no-spurious-compounds",
            "सवारीमा आयात विकास यातायात आर्थिक दशकमा व्यवस्थापन रिसाइकल",
            true,
        ),
        ("grammar-inflected-compound", "🙂 सूर्योदयमा", true),
        (
            "progressive-participles",
            "🙂 खोजिरहेको खोजिरहेकी खोजिरहेका",
            false,
        ),
        ("progressive-context", "म काम खोजिरहेको छु।", true),
        (
            "progressive-distinct-verbs",
            "🙂 उर्लिरहेको ओर्लिरहेको उडिरहेको",
            true,
        ),
        ("relational-suffix-join", "🙂 मानिस सरह घर सम्म", false),
        (
            "relational-suffix-guard",
            "मानिसको सरह घरको सम्म मानिस जस्तै",
            true,
        ),
        (
            "lexical-compound-spacing",
            "🙂 वायुसेवा वायुसेवाको वायुसेवाले जनसेवा राज्यव्यवस्था",
            true,
        ),
        ("reviewed-institutional-spacing", "नेपालसरकार समाजसेवा", false),
    ] {
        for (mode, orthography) in [
            ("academy-strict", OrthographyMode::AcademyStrict),
            ("common-editorial", OrthographyMode::CommonEditorial),
        ] {
            let expected: Value = serde_json::from_str(&check_text_with_all_options(
                text.into(),
                grammar,
                PunctuationMode::Strict,
                orthography,
                false,
            ))
            .unwrap();
            cases.push(json!({"id": format!("{id}-{mode}"), "text": text,
                "options": {"grammar": grammar, "punctuation_mode": "strict",
                    "orthography_mode": mode, "include_noop_heuristics": false},
                "expected_diagnostics": expected}));
        }
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "fixture_schema_version": 1, "diagnostic_schema_version": 1,
            "span_unit": "utf8-bytes", "cases": cases
        }))
        .unwrap()
    );
}
