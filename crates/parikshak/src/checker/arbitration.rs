use varnavinyas_prakriya::{DiagnosticKind, Rule};

use crate::diagnostic::{Diagnostic, DiagnosticReason};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiagnosticPass {
    Word,
    Tiryak,
    Padayog,
    Context,
    Style,
    Grammar,
    Punctuation,
}

impl DiagnosticPass {
    fn rank(self) -> u8 {
        match self {
            Self::Word => 6,
            Self::Tiryak => 5,
            Self::Padayog => 4,
            Self::Context => 3,
            Self::Style => 2,
            Self::Grammar => 1,
            Self::Punctuation => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Specificity {
    Exact,
    CuratedInventory,
    Generalized,
    Heuristic,
}

impl Specificity {
    fn rank(self) -> u8 {
        match self {
            Self::Exact => 4,
            Self::CuratedInventory => 3,
            Self::Generalized => 2,
            Self::Heuristic => 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Candidate<'a> {
    diagnostic: &'a Diagnostic,
    pass: DiagnosticPass,
    specificity: Specificity,
}

impl<'a> Candidate<'a> {
    fn new(diagnostic: &'a Diagnostic) -> Self {
        Self {
            diagnostic,
            pass: infer_pass(diagnostic),
            specificity: infer_specificity(diagnostic),
        }
    }

    fn is_padayog(self) -> bool {
        self.pass == DiagnosticPass::Padayog
    }

    fn precedence_tuple(self) -> (u8, u8, u8, u16) {
        (
            kind_rank(self.diagnostic.kind),
            self.specificity.rank(),
            self.pass.rank(),
            confidence_rank(self.diagnostic.confidence),
        )
    }
}

pub(super) fn resolve_diagnostic_overlaps(diagnostics: &mut Vec<Diagnostic>) {
    resolve_padayog_overlaps(diagnostics);
    merge_same_replacement_diagnostics(diagnostics);
}

fn resolve_padayog_overlaps(diagnostics: &mut Vec<Diagnostic>) {
    let mut remove = vec![false; diagnostics.len()];

    for i in 0..diagnostics.len() {
        if remove[i] {
            continue;
        }
        for j in (i + 1)..diagnostics.len() {
            if remove[j] {
                continue;
            }

            let left = Candidate::new(&diagnostics[i]);
            let right = Candidate::new(&diagnostics[j]);
            let Some(winner) = same_span_padayog_winner(left, right) else {
                continue;
            };

            if winner == 0 {
                remove[j] = true;
            } else {
                remove[i] = true;
                break;
            }
        }
    }

    retain_by_remove_mask(diagnostics, &remove);

    let padayog_spans: Vec<(usize, usize)> = diagnostics
        .iter()
        .filter(|diagnostic| {
            let candidate = Candidate::new(diagnostic);
            candidate.is_padayog() && !is_ambiguous(candidate)
        })
        .map(|diagnostic| diagnostic.span)
        .collect();

    diagnostics.retain(|diagnostic| {
        !padayog_spans
            .iter()
            .any(|&span| diagnostic.span != span && contains_span(span, diagnostic.span))
    });
}

fn retain_by_remove_mask(diagnostics: &mut Vec<Diagnostic>, remove: &[bool]) {
    let mut idx = 0;
    diagnostics.retain(|_| {
        let keep = !remove[idx];
        idx += 1;
        keep
    });
}

fn same_span_padayog_winner(left: Candidate<'_>, right: Candidate<'_>) -> Option<usize> {
    if !(left.is_padayog() || right.is_padayog()) {
        return None;
    }

    if left.diagnostic.span == right.diagnostic.span {
        return higher_precedence_index(left, right);
    }

    None
}

fn higher_precedence_index(left: Candidate<'_>, right: Candidate<'_>) -> Option<usize> {
    match left.precedence_tuple().cmp(&right.precedence_tuple()) {
        std::cmp::Ordering::Greater => Some(0),
        std::cmp::Ordering::Less => Some(1),
        std::cmp::Ordering::Equal => None,
    }
}

fn is_ambiguous(candidate: Candidate<'_>) -> bool {
    candidate.precedence_tuple().0 <= kind_rank(DiagnosticKind::Ambiguous)
}

fn contains_span(outer: (usize, usize), inner: (usize, usize)) -> bool {
    outer != inner && outer.0 <= inner.0 && inner.1 <= outer.1
}

pub(super) fn overlaps_existing_span(
    diagnostics: &[Diagnostic],
    candidate: (usize, usize),
) -> bool {
    diagnostics
        .iter()
        .filter(|diagnostic| blocks_overlapping_candidate(Candidate::new(diagnostic)))
        .any(|diagnostic| diagnostic.span.0 < candidate.1 && candidate.0 < diagnostic.span.1)
}

fn blocks_overlapping_candidate(candidate: Candidate<'_>) -> bool {
    candidate.precedence_tuple().0 > kind_rank(DiagnosticKind::Ambiguous)
}

fn infer_pass(diagnostic: &Diagnostic) -> DiagnosticPass {
    match diagnostic.rule {
        Rule::ChihnaNiyam(_) => DiagnosticPass::Punctuation,
        Rule::VarnaVinyasNiyam("3(घ)") => DiagnosticPass::Padayog,
        Rule::VarnaVinyasNiyam(code) if code.contains("-context-") => DiagnosticPass::Context,
        Rule::Vyakaran(code) if is_tiryak_rule(code) => DiagnosticPass::Tiryak,
        Rule::Vyakaran("section4-phrase-style" | "section4-phrase-style-inferred-ko-ka") => {
            DiagnosticPass::Style
        }
        Rule::Vyakaran(_) => DiagnosticPass::Grammar,
        Rule::VarnaVinyasNiyam(_) | Rule::ShuddhaAshuddha(_) => DiagnosticPass::Word,
    }
}

fn infer_specificity(diagnostic: &Diagnostic) -> Specificity {
    match diagnostic.rule {
        Rule::ShuddhaAshuddha(_) | Rule::ChihnaNiyam(_) => Specificity::Exact,
        Rule::Vyakaran("section4-phrase-style") => Specificity::Exact,
        Rule::Vyakaran("section4-phrase-style-inferred-ko-ka") => Specificity::Generalized,
        Rule::Vyakaran(code) if is_tiryak_rule(code) => Specificity::Exact,
        Rule::Vyakaran(_) => Specificity::Heuristic,
        Rule::VarnaVinyasNiyam("3(घ)") => infer_padayog_specificity(diagnostic),
        Rule::VarnaVinyasNiyam(code) if code.contains("-context-") => Specificity::CuratedInventory,
        Rule::VarnaVinyasNiyam(code) => infer_varna_vinyas_specificity(code),
    }
}

fn infer_padayog_specificity(diagnostic: &Diagnostic) -> Specificity {
    if diagnostic.explanation.contains("पदवियोग (च)") {
        return Specificity::Exact;
    }
    if diagnostic.explanation.starts_with("शैक्षणिक व्याकरण") {
        return Specificity::CuratedInventory;
    }
    Specificity::Generalized
}

fn infer_varna_vinyas_specificity(code: &str) -> Specificity {
    if code.contains("-पदान्त") {
        return Specificity::Exact;
    }
    if code.contains("-lex") || code.contains("-PS-Saisanik") || code == "3(ई)" {
        return Specificity::CuratedInventory;
    }
    Specificity::Generalized
}

fn is_tiryak_rule(code: &str) -> bool {
    code.starts_with("PS-Saisanik-7(") && code.ends_with("-तिर्यक्")
}

fn kind_rank(kind: DiagnosticKind) -> u8 {
    match kind {
        DiagnosticKind::Error => 3,
        DiagnosticKind::Variant => 2,
        DiagnosticKind::Ambiguous => 1,
    }
}

fn confidence_rank(confidence: f32) -> u16 {
    (confidence.clamp(0.0, 1.0) * 1000.0).round() as u16
}

fn merge_same_replacement_diagnostics(diagnostics: &mut Vec<Diagnostic>) {
    let mut merged = Vec::with_capacity(diagnostics.len());

    for diagnostic in diagnostics.drain(..) {
        if let Some(existing) = merged
            .iter_mut()
            .find(|existing| has_same_replacement(existing, &diagnostic))
        {
            merge_duplicate_into(existing, diagnostic);
        } else {
            merged.push(diagnostic);
        }
    }

    *diagnostics = merged;
}

fn has_same_replacement(left: &Diagnostic, right: &Diagnostic) -> bool {
    left.span == right.span && left.correction == right.correction
}

fn merge_duplicate_into(existing: &mut Diagnostic, duplicate: Diagnostic) {
    if Candidate::new(&duplicate).precedence_tuple() > Candidate::new(existing).precedence_tuple() {
        let old_primary = std::mem::replace(existing, duplicate);
        push_primary_as_alternate(existing, &old_primary);
        for reason in old_primary.alternate_reasons {
            push_unique_reason(existing, reason);
        }
    } else {
        push_primary_as_alternate(existing, &duplicate);
        for reason in duplicate.alternate_reasons {
            push_unique_reason(existing, reason);
        }
    }
}

fn push_primary_as_alternate(diagnostic: &mut Diagnostic, alternate: &Diagnostic) {
    push_unique_reason(
        diagnostic,
        DiagnosticReason {
            rule: alternate.rule,
            explanation: alternate.explanation.clone(),
            correction: Some(alternate.correction.clone()),
            category: None,
        },
    );
}

fn push_unique_reason(diagnostic: &mut Diagnostic, reason: DiagnosticReason) {
    if reason_matches_primary(diagnostic, &reason) {
        return;
    }
    if diagnostic
        .alternate_reasons
        .iter()
        .any(|existing| same_reason(existing, &reason))
    {
        return;
    }
    diagnostic.alternate_reasons.push(reason);
}

fn reason_matches_primary(diagnostic: &Diagnostic, reason: &DiagnosticReason) -> bool {
    diagnostic.rule == reason.rule
        && diagnostic.explanation == reason.explanation
        && reason.correction.as_deref() == Some(diagnostic.correction.as_str())
}

fn same_reason(left: &DiagnosticReason, right: &DiagnosticReason) -> bool {
    left.rule == right.rule
        && left.explanation == right.explanation
        && left.correction == right.correction
        && left.category == right.category
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
        diagnostic_with(span, rule, kind, incorrect, "y", "x", Vec::new())
    }

    fn diagnostic_with(
        span: (usize, usize),
        rule: Rule,
        kind: DiagnosticKind,
        incorrect: &str,
        correction: &str,
        explanation: &str,
        alternate_reasons: Vec<DiagnosticReason>,
    ) -> Diagnostic {
        Diagnostic {
            span,
            incorrect: incorrect.to_string(),
            correction: correction.to_string(),
            rule,
            explanation: explanation.to_string(),
            category: DiagnosticCategory::ShuddhaTable,
            kind,
            confidence: 1.0,
            alternate_reasons,
        }
    }

    #[test]
    fn padayog_error_suppresses_weaker_nested_variant() {
        let mut diagnostics = vec![
            diagnostic(
                (0, 12),
                Rule::VarnaVinyasNiyam("3(घ)"),
                DiagnosticKind::Error,
                "padayog",
            ),
            diagnostic(
                (3, 9),
                Rule::Vyakaran("section4-phrase-style"),
                DiagnosticKind::Variant,
                "nested",
            ),
        ];

        resolve_diagnostic_overlaps(&mut diagnostics);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].incorrect, "padayog");
    }

    #[test]
    fn same_span_word_error_beats_generalized_padayog_candidate() {
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
        assert_eq!(diagnostics[0].incorrect, "same-span");
    }

    #[test]
    fn exact_padayog_particle_split_beats_generalized_word_candidate() {
        let mut diagnostics = vec![
            diagnostic(
                (20, 24),
                Rule::ShuddhaAshuddha("Section 4"),
                DiagnosticKind::Error,
                "unrelated",
            ),
            diagnostic_with(
                (0, 6),
                Rule::VarnaVinyasNiyam("3(घ)"),
                DiagnosticKind::Error,
                "padayog",
                "padayog-correct",
                "शैक्षणिक व्याकरण पदवियोग (च): शब्दाश्रित निपातहरू पदवियोग गरी लेखिन्छन् ।",
                Vec::new(),
            ),
            diagnostic_with(
                (0, 6),
                Rule::VarnaVinyasNiyam("3(क)(अ)-5"),
                DiagnosticKind::Error,
                "word",
                "word-correct",
                "x",
                Vec::new(),
            ),
        ];

        resolve_diagnostic_overlaps(&mut diagnostics);

        assert_eq!(diagnostics.len(), 2);
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.incorrect == "padayog"),
            "exact padayog candidate should survive same-span generalized word rule at a nonzero index: {diagnostics:?}"
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.incorrect == "unrelated"),
            "unrelated diagnostic should survive arbitration: {diagnostics:?}"
        );
    }

    #[test]
    fn ambiguous_padayog_candidate_loses_to_same_span_error() {
        let mut diagnostics = vec![
            diagnostic_with(
                (0, 6),
                Rule::VarnaVinyasNiyam("3(घ)"),
                DiagnosticKind::Ambiguous,
                "padayog",
                "padayog-correct",
                "x",
                Vec::new(),
            ),
            diagnostic_with(
                (0, 6),
                Rule::ShuddhaAshuddha("Section 4"),
                DiagnosticKind::Error,
                "same-span",
                "word-correct",
                "x",
                Vec::new(),
            ),
        ];

        resolve_diagnostic_overlaps(&mut diagnostics);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].incorrect, "same-span");
    }

    #[test]
    fn ambiguous_padayog_candidate_does_not_suppress_nested_error() {
        let mut diagnostics = vec![
            diagnostic_with(
                (0, 12),
                Rule::VarnaVinyasNiyam("3(घ)"),
                DiagnosticKind::Ambiguous,
                "padayog",
                "padayog-correct",
                "x",
                Vec::new(),
            ),
            diagnostic_with(
                (3, 9),
                Rule::ShuddhaAshuddha("Section 4"),
                DiagnosticKind::Error,
                "nested",
                "word-correct",
                "x",
                Vec::new(),
            ),
        ];

        resolve_diagnostic_overlaps(&mut diagnostics);

        assert_eq!(diagnostics.len(), 2);
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.incorrect == "nested"),
            "nested error should survive ambiguous padayog candidate: {diagnostics:?}"
        );
    }

    #[test]
    fn same_span_same_correction_merges_duplicate_as_alternate_reason() {
        let mut diagnostics = vec![
            diagnostic_with(
                (0, 6),
                Rule::ShuddhaAshuddha("Section 4"),
                DiagnosticKind::Error,
                "primary",
                "correct",
                "primary explanation",
                Vec::new(),
            ),
            diagnostic_with(
                (0, 6),
                Rule::Vyakaran("samasa-heuristic"),
                DiagnosticKind::Variant,
                "duplicate",
                "correct",
                "secondary explanation",
                Vec::new(),
            ),
        ];

        resolve_diagnostic_overlaps(&mut diagnostics);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule, Rule::ShuddhaAshuddha("Section 4"));
        assert_eq!(diagnostics[0].alternate_reasons.len(), 1);
        assert_eq!(
            diagnostics[0].alternate_reasons[0].rule,
            Rule::Vyakaran("samasa-heuristic")
        );
        assert_eq!(
            diagnostics[0].alternate_reasons[0].explanation,
            "secondary explanation"
        );
    }

    #[test]
    fn same_span_same_correction_prefers_higher_precedence_primary() {
        let mut diagnostics = vec![
            Diagnostic {
                confidence: 0.55,
                ..diagnostic_with(
                    (0, 6),
                    Rule::Vyakaran("samasa-heuristic"),
                    DiagnosticKind::Variant,
                    "weaker",
                    "correct",
                    "weaker explanation",
                    Vec::new(),
                )
            },
            diagnostic_with(
                (0, 6),
                Rule::ShuddhaAshuddha("Section 4"),
                DiagnosticKind::Error,
                "stronger",
                "correct",
                "stronger explanation",
                Vec::new(),
            ),
        ];

        resolve_diagnostic_overlaps(&mut diagnostics);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule, Rule::ShuddhaAshuddha("Section 4"));
        assert_eq!(diagnostics[0].incorrect, "stronger");
        assert_eq!(diagnostics[0].alternate_reasons.len(), 1);
        assert_eq!(
            diagnostics[0].alternate_reasons[0].rule,
            Rule::Vyakaran("samasa-heuristic")
        );
    }

    #[test]
    fn classifies_pass_from_rule_metadata() {
        let cases = [
            (
                Rule::ShuddhaAshuddha("Section 4"),
                DiagnosticPass::Word,
                "word",
            ),
            (
                Rule::Vyakaran("PS-Saisanik-7(क)-तिर्यक्"),
                DiagnosticPass::Tiryak,
                "tiryak",
            ),
            (
                Rule::VarnaVinyasNiyam("3(घ)"),
                DiagnosticPass::Padayog,
                "padayog",
            ),
            (
                Rule::VarnaVinyasNiyam("3(ङ)-context-होस्"),
                DiagnosticPass::Context,
                "context",
            ),
            (
                Rule::Vyakaran("section4-phrase-style"),
                DiagnosticPass::Style,
                "style",
            ),
            (
                Rule::Vyakaran("samasa-heuristic"),
                DiagnosticPass::Grammar,
                "grammar",
            ),
            (
                Rule::ChihnaNiyam("Section 5"),
                DiagnosticPass::Punctuation,
                "punctuation",
            ),
        ];

        for (rule, expected, label) in cases {
            let diag = diagnostic((0, 1), rule, DiagnosticKind::Error, label);
            assert_eq!(Candidate::new(&diag).pass, expected, "{label}");
        }
    }

    #[test]
    fn classifies_specificity_from_rule_metadata() {
        let cases = [
            (
                Rule::ShuddhaAshuddha("Section 4"),
                Specificity::Exact,
                "table",
            ),
            (
                Rule::Vyakaran("section4-phrase-style-inferred-ko-ka"),
                Specificity::Generalized,
                "inferred-style",
            ),
            (
                Rule::VarnaVinyasNiyam("3(ङ)-context-होस्"),
                Specificity::CuratedInventory,
                "context",
            ),
            (
                Rule::Vyakaran("samasa-heuristic"),
                Specificity::Heuristic,
                "grammar",
            ),
            (
                Rule::VarnaVinyasNiyam("3(ङ)-पदान्त"),
                Specificity::Exact,
                "padanta-halanta",
            ),
            (
                Rule::VarnaVinyasNiyam("3(क)(अ)-5"),
                Specificity::Generalized,
                "broad-word-rule",
            ),
        ];

        for (rule, expected, label) in cases {
            let diag = diagnostic((0, 1), rule, DiagnosticKind::Error, label);
            assert_eq!(Candidate::new(&diag).specificity, expected, "{label}");
        }

        let padayog = diagnostic_with(
            (0, 1),
            Rule::VarnaVinyasNiyam("3(घ)"),
            DiagnosticKind::Error,
            "padayog",
            "y",
            "शैक्षणिक व्याकरण पदवियोग (च): शब्दाश्रित निपातहरू पदवियोग गरी लेखिन्छन् ।",
            Vec::new(),
        );
        assert_eq!(Candidate::new(&padayog).specificity, Specificity::Exact);
    }

    #[test]
    fn precedence_tuple_encodes_kind_specificity_pass_confidence() {
        let word = diagnostic(
            (0, 1),
            Rule::ShuddhaAshuddha("Section 4"),
            DiagnosticKind::Error,
            "word",
        );
        let style = Diagnostic {
            confidence: 0.72,
            ..diagnostic(
                (0, 1),
                Rule::Vyakaran("section4-phrase-style-inferred-ko-ka"),
                DiagnosticKind::Variant,
                "style",
            )
        };
        let grammar = Diagnostic {
            confidence: 0.55,
            ..diagnostic(
                (0, 1),
                Rule::Vyakaran("samasa-heuristic"),
                DiagnosticKind::Ambiguous,
                "grammar",
            )
        };

        assert!(
            Candidate::new(&word).precedence_tuple() > Candidate::new(&style).precedence_tuple()
        );
        assert!(
            Candidate::new(&style).precedence_tuple() > Candidate::new(&grammar).precedence_tuple()
        );
    }
}
