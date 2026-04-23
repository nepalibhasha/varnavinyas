use std::collections::BTreeSet;

use varnavinyas_prakriya::derive;

/// Baseline contradictions between Notice examples and current engine behavior.
///
/// This list is an explicit debt register. The test fails if:
/// - a new contradicted example appears (regression), or
/// - an existing contradiction disappears (behavior changed) without updating this file.
const KNOWN_NOTICE_EXAMPLE_CONTRADICTIONS: &[&str] = &[];

fn is_devanagari_token(s: &str) -> bool {
    let mut has_devanagari = false;
    for ch in s.chars() {
        let code = ch as u32;
        if (0x0900..=0x097F).contains(&code) {
            has_devanagari = true;
        } else if ch.is_ascii_whitespace() || matches!(ch, '-' | '+' | '/') {
            continue;
        } else {
            return false;
        }
    }
    has_devanagari
}

fn parse_examples(rhs: &str) -> Vec<String> {
    rhs.split('।')
        .flat_map(|part| part.split(','))
        .filter_map(|raw| {
            let t = raw
                .trim()
                .trim_matches('(')
                .trim_matches(')')
                .trim_matches('"')
                .trim_matches('“')
                .trim_matches('”')
                .trim_matches('‘')
                .trim_matches('’')
                .trim_matches('*')
                .trim()
                .trim_end_matches("आदि")
                .trim()
                .trim_end_matches('।')
                .trim();
            if t.is_empty() || !is_devanagari_token(t) {
                return None;
            }
            Some(t.to_string())
        })
        .collect()
}

fn notice_examples_for_hrasva_dirgha() -> Vec<String> {
    let raw = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/Notices-pages-77-99.md"
    ))
    .expect("read docs/Notices-pages-77-99.md");
    let plain = raw.replace("**", "");

    let mut examples = Vec::new();
    for line in plain.lines().map(str::trim) {
        if !line.contains(':') {
            continue;
        }
        let has_claim = line.contains("ह्रस्व हुन्छ") || line.contains("दीर्घ हुन्छ");
        if !has_claim {
            continue;
        }
        let Some((lhs, rhs)) = line.split_once(':') else {
            continue;
        };
        let lhs = lhs.trim();
        let rhs = rhs.trim();
        let scoped = lhs.contains("सुरुमा") || lhs.contains("बिचमा") || lhs.contains("अन्त्यमा");
        if !scoped {
            continue;
        }
        examples.extend(parse_examples(rhs));
    }
    examples
}

#[test]
fn notices_examples_alignment_guard() {
    let mut actual = BTreeSet::new();

    for example in notice_examples_for_hrasva_dirgha() {
        let p = derive(&example);
        if !p.is_correct && p.output != example {
            actual.insert(example);
        }
    }

    let expected: BTreeSet<String> = KNOWN_NOTICE_EXAMPLE_CONTRADICTIONS
        .iter()
        .map(|s| (*s).to_string())
        .collect();

    assert_eq!(
        actual, expected,
        "Notice/example alignment drift detected.\nUpdate rules/correction-table, then adjust KNOWN_NOTICE_EXAMPLE_CONTRADICTIONS intentionally."
    );
}
