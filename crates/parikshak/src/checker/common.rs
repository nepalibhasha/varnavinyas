use crate::diagnostic::Diagnostic;
use varnavinyas_prakriya::DiagnosticKind;

pub(super) fn is_devanagari_word(s: &str) -> bool {
    !s.is_empty()
        && s.chars().all(|c| {
            ('\u{0900}'..='\u{097F}').contains(&c)
                && !matches!(
                    c,
                    '।' | ',' | '.' | '!' | '?' | ';' | ':' | '“' | '”' | '‘' | '’'
                )
        })
}

pub(super) fn is_numeric_segment(s: &str) -> bool {
    let mut saw_digit = false;
    for ch in s.chars() {
        if ch.is_numeric() {
            saw_digit = true;
            continue;
        }
        return false;
    }
    saw_digit
}

pub(super) fn overlaps_existing_span(
    diagnostics: &[Diagnostic],
    candidate: (usize, usize),
) -> bool {
    diagnostics
        .iter()
        .filter(|d| !is_non_blocking_diagnostic(d))
        .any(|d| d.span.0 < candidate.1 && candidate.0 < d.span.1)
}

fn is_non_blocking_diagnostic(d: &Diagnostic) -> bool {
    matches!(d.kind, DiagnosticKind::Ambiguous)
}

pub(super) fn is_word_boundary(text: &str, start: usize, end: usize) -> bool {
    let prev_ok = if start == 0 {
        true
    } else {
        text[..start]
            .chars()
            .next_back()
            .is_none_or(is_boundary_char)
    };

    let next_ok = if end >= text.len() {
        true
    } else {
        text[end..].chars().next().is_none_or(is_boundary_char)
    };

    prev_ok && next_ok
}

fn is_boundary_char(c: char) -> bool {
    c.is_whitespace()
        || matches!(
            c,
            '.' | ','
                | '!'
                | '?'
                | ';'
                | ':'
                | '-'
                | '('
                | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | '"'
                | '\''
                | '/'
                | '।'
                | '…'
        )
}
