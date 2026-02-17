#[test]
fn diagnostics_avoid_non_standard_terms() {
    // Guard user-facing diagnostic wording against non-standard coined labels.
    let correction_table = include_str!("../src/correction_table.rs");
    let gold = include_str!("../../../docs/tests/gold.toml");

    let banned = [
        "पूर्वकालिक",
        "absolutive",
        "Absolutive",
        "single-meaning",
        "demonym",
        "adjectival/demonym",
        "kinship",
    ];

    for line in correction_table.lines() {
        if line.contains("description:") {
            for term in banned {
                assert!(
                    !line.contains(term),
                    "Non-standard terminology found in correction description: {term}"
                );
            }
        }
    }

    for line in gold.lines() {
        if line.trim_start().starts_with("rule = ") {
            for term in banned {
                assert!(
                    !line.contains(term),
                    "Non-standard terminology found in gold rule text: {term}"
                );
            }
        }
    }
}
