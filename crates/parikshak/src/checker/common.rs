use crate::diagnostic::Diagnostic;
use varnavinyas_prakriya::DiagnosticKind;

pub(super) fn whitespace_segments(text: &str) -> Vec<(&str, usize, usize)> {
    let mut out = Vec::new();
    let mut pos = 0;
    for seg in text.split_whitespace() {
        let start = text[pos..].find(seg).map(|i| pos + i).unwrap_or(pos);
        let end = start + seg.len();
        if let Some((core, core_start, core_end)) = trim_outer_punctuation(text, start, end) {
            out.push((core, core_start, core_end));
        }
        pos = end;
    }
    out
}

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

fn trim_outer_punctuation(text: &str, start: usize, end: usize) -> Option<(&str, usize, usize)> {
    let slice = &text[start..end];

    let mut left = 0;
    for (idx, ch) in slice.char_indices() {
        if is_outer_punctuation_char(ch) {
            left = idx + ch.len_utf8();
        } else {
            break;
        }
    }

    if left >= slice.len() {
        return None;
    }

    let trimmed = &slice[left..];
    let mut right = trimmed.len();
    for (idx, ch) in trimmed.char_indices().rev() {
        if is_outer_punctuation_char(ch) {
            right = idx;
        } else {
            break;
        }
    }

    if right == 0 {
        return None;
    }

    let core_start = start + left;
    let core_end = core_start + right;
    Some((&text[core_start..core_end], core_start, core_end))
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

fn is_outer_punctuation_char(c: char) -> bool {
    !c.is_whitespace() && is_boundary_char(c)
}
