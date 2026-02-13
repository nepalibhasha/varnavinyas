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

#[cfg(feature = "grammar-pass")]
#[test]
fn grammar_pass_flags_plural_after_quantifier() {
    let text = "धेरै मानिसहरु आए।";
    let diags = check_text_with_options(text, CheckOptions { grammar: true });

    assert!(
        diags.iter().any(|d| {
            d.rule == varnavinyas_prakriya::Rule::Vyakaran("quantifier-plural-redundancy")
                && matches!(d.kind, DiagnosticKind::Variant)
        }),
        "Expected quantifier-plural heuristic diagnostic, got: {diags:?}"
    );
}

#[cfg(feature = "grammar-pass")]
#[test]
fn grammar_pass_flags_ergative_with_intransitive_predicate() {
    let text = "रामले गयो।";
    let diags = check_text_with_options(text, CheckOptions { grammar: true });

    assert!(
        diags.iter().any(|d| {
            d.rule == varnavinyas_prakriya::Rule::Vyakaran("ergative-le-intransitive")
                && matches!(d.kind, DiagnosticKind::Variant)
        }),
        "Expected ergative/intransitive heuristic diagnostic, got: {diags:?}"
    );
}

#[cfg(feature = "grammar-pass")]
#[test]
fn grammar_pass_flags_genitive_mismatch_before_plural() {
    let text = "रामको किताबहरु हराए।";
    let diags = check_text_with_options(text, CheckOptions { grammar: true });

    assert!(
        diags.iter().any(|d| {
            d.rule == varnavinyas_prakriya::Rule::Vyakaran("genitive-mismatch-plural")
                && matches!(d.kind, DiagnosticKind::Variant)
        }),
        "Expected genitive/plural mismatch heuristic diagnostic, got: {diags:?}"
    );
}
