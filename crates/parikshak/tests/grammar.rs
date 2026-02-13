#[cfg(feature = "grammar-pass")]
use varnavinyas_parikshak::{CheckOptions, DiagnosticKind, check_text_with_options};

#[cfg(feature = "grammar-pass")]
#[test]
fn grammar_pass_emits_variant_or_ambiguous_hints() {
    let text = "सूर्योदय भयो";
    let diags = check_text_with_options(text, CheckOptions { grammar: true });
    assert!(
        diags
            .iter()
            .any(|d| matches!(d.kind, DiagnosticKind::Variant | DiagnosticKind::Ambiguous)),
        "Expected grammar-pass heuristic diagnostics, got: {diags:?}"
    );
}
