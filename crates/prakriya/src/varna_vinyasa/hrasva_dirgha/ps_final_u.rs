//! Exact source examples, not a blanket rule for every word ending in ऊ.
use std::sync::LazyLock;

const DATA: &str = include_str!("../../../../../data/rule_inventories/ps_final_u_hrasva.tsv");
static WORDS: LazyLock<Vec<&'static str>> = LazyLock::new(|| parse_inventory(DATA));
pub(super) const RULE_CODE: &str = "3(क)(इ)-PS-Saisanik-(छ)";
pub(super) const EXPLANATION: &str =
    "शैक्षणिक व्याकरण (छ): नु, भु, धु, रु, लु अन्त्य भएका सूचीकृत तत्सम शब्दमा उकार ह्रस्व हुन्छ";

pub(super) fn is_reviewed_example(word: &str) -> bool {
    WORDS.contains(&word)
}

fn parse_inventory(data: &str) -> Vec<&str> {
    let mut lines = data.lines();
    assert_eq!(lines.next(), Some("word\tsource\treview_status"));
    let mut words = Vec::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 3, "expected three fields: {line}");
        let (word, source, status) = (fields[0], fields[1], fields[2]);
        assert!(
            !word.is_empty() && !source.is_empty(),
            "missing provenance: {line}"
        );
        assert_eq!(status, "reviewed", "unreviewed example: {line}");
        assert!(
            ["नु", "भु", "धु", "रु", "लु"]
                .iter()
                .any(|ending| word.ends_with(ending)),
            "not a source final-उ class: {line}"
        );
        assert!(!words.contains(&word), "duplicate example: {line}");
        words.push(word);
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_examples_are_unique_and_do_not_include_long_vowel_neighbors() {
        assert_eq!(parse_inventory(DATA).len(), 15);
        assert!(is_reviewed_example("प्रभु"));
        assert!(is_reviewed_example("श्रद्धालु"));
        assert!(!is_reviewed_example("वधू"));
        assert!(!is_reviewed_example("वधु"));
    }

    #[test]
    #[should_panic(expected = "missing provenance")]
    fn missing_source_is_rejected() {
        parse_inventory("word\tsource\treview_status\nप्रभु\t\treviewed");
    }

    #[test]
    #[should_panic(expected = "unreviewed example")]
    fn unreviewed_row_is_rejected() {
        parse_inventory("word\tsource\treview_status\nप्रभु\tPS-Saisanik-(छ)\tdraft");
    }

    #[test]
    #[should_panic(expected = "duplicate example")]
    fn duplicate_example_is_rejected() {
        parse_inventory(
            "word\tsource\treview_status\nप्रभु\tPS-(छ)\treviewed\nप्रभु\tPS-(छ)\treviewed",
        );
    }
}
