use std::collections::HashSet;

use varnavinyas_prakriya::{DiagnosticKind, Rule};

use crate::diagnostic::Diagnostic;

pub(super) fn resolve_diagnostic_overlaps(diagnostics: &mut Vec<Diagnostic>) {
    let padayog_spans: Vec<(usize, usize)> = diagnostics
        .iter()
        .filter(|d| matches!(d.rule, Rule::VarnaVinyasNiyam("3(घ)")))
        .map(|d| d.span)
        .collect();
    let same_span_non_ambiguous_padayog: HashSet<(usize, usize)> = diagnostics
        .iter()
        .filter(|d| {
            matches!(d.rule, Rule::VarnaVinyasNiyam("3(घ)"))
                && !matches!(d.kind, DiagnosticKind::Ambiguous)
        })
        .map(|d| d.span)
        .collect();

    diagnostics.retain(|diag| {
        let nested = padayog_spans.iter().any(|&(start, end)| {
            diag.span != (start, end) && start <= diag.span.0 && diag.span.1 <= end
        });
        if nested {
            return false;
        }

        if same_span_non_ambiguous_padayog.contains(&diag.span)
            && !matches!(diag.rule, Rule::VarnaVinyasNiyam("3(घ)"))
        {
            return false;
        }

        if matches!(diag.kind, DiagnosticKind::Ambiguous)
            && same_span_non_ambiguous_padayog.contains(&diag.span)
        {
            return false;
        }

        true
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DiagnosticCategory;

    fn diagnostic(
        span: (usize, usize),
        rule: Rule,
        kind: DiagnosticKind,
        incorrect: &str,
    ) -> Diagnostic {
        Diagnostic {
            span,
            incorrect: incorrect.to_string(),
            correction: "y".to_string(),
            rule,
            explanation: "x".to_string(),
            category: DiagnosticCategory::ShuddhaTable,
            kind,
            confidence: 1.0,
            alternate_reasons: Vec::new(),
        }
    }

    #[test]
    fn removes_diagnostics_nested_inside_padayog_span() {
        let mut diagnostics = vec![
            diagnostic(
                (0, 12),
                Rule::VarnaVinyasNiyam("3(घ)"),
                DiagnosticKind::Error,
                "padayog",
            ),
            diagnostic(
                (3, 9),
                Rule::ShuddhaAshuddha("Section 4"),
                DiagnosticKind::Error,
                "nested",
            ),
        ];

        resolve_diagnostic_overlaps(&mut diagnostics);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].incorrect, "padayog");
    }

    #[test]
    fn same_span_padayog_error_suppresses_non_padayog_candidate() {
        let mut diagnostics = vec![
            diagnostic(
                (0, 6),
                Rule::VarnaVinyasNiyam("3(घ)"),
                DiagnosticKind::Error,
                "padayog",
            ),
            diagnostic(
                (0, 6),
                Rule::ShuddhaAshuddha("Section 4"),
                DiagnosticKind::Error,
                "same-span",
            ),
        ];

        resolve_diagnostic_overlaps(&mut diagnostics);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].incorrect, "padayog");
    }

    #[test]
    fn ambiguous_padayog_candidate_does_not_suppress_same_span_error() {
        let mut diagnostics = vec![
            diagnostic(
                (0, 6),
                Rule::VarnaVinyasNiyam("3(घ)"),
                DiagnosticKind::Ambiguous,
                "padayog",
            ),
            diagnostic(
                (0, 6),
                Rule::ShuddhaAshuddha("Section 4"),
                DiagnosticKind::Error,
                "same-span",
            ),
        ];

        resolve_diagnostic_overlaps(&mut diagnostics);

        assert_eq!(diagnostics.len(), 2);
    }
}
