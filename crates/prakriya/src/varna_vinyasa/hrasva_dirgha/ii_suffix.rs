//! Reviewed derivations: a mechanically removable ी is not proof of an ई suffix.
use std::sync::LazyLock;

const DATA: &str = include_str!("../../../../../data/rule_inventories/ii_suffix_derivatives.tsv");
static DERIVATIVES: LazyLock<Vec<&'static str>> = LazyLock::new(|| parse_inventory(DATA));
pub(super) const EXPLANATION: &str = "'ई' प्रत्यय अन्त्यमा आउने शब्दहरू दीर्घ हुन्छन्";

pub(super) fn is_reviewed_derivative(word: &str) -> bool {
    DERIVATIVES.contains(&word)
}

fn parse_inventory(data: &str) -> Vec<&str> {
    let mut lines = data.lines();
    assert_eq!(lines.next(), Some("root\toutput\tsource\treview_status"));
    let mut outputs = Vec::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 4, "expected four fields: {line}");
        let (root, output, source, status) = (fields[0], fields[1], fields[2], fields[3]);
        assert!(
            !root.is_empty() && !source.is_empty(),
            "missing provenance: {line}"
        );
        assert_eq!(status, "reviewed", "unreviewed derivative: {line}");
        assert_eq!(output, format!("{root}ी"), "invalid ई derivative: {line}");
        assert!(!outputs.contains(&output), "duplicate derivative: {line}");
        outputs.push(output);
    }
    outputs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_examples_are_reviewed_and_unique() {
        assert_eq!(parse_inventory(DATA).len(), 6);
        assert!(is_reviewed_derivative("योगी"));
        assert!(!is_reviewed_derivative("हामी"));
    }

    #[test]
    #[should_panic(expected = "missing provenance")]
    fn missing_source_is_rejected() {
        parse_inventory("root\toutput\tsource\treview_status\nयोग\tयोगी\t\treviewed");
    }

    #[test]
    #[should_panic(expected = "unreviewed derivative")]
    fn unreviewed_row_is_rejected() {
        parse_inventory("root\toutput\tsource\treview_status\nयोग\tयोगी\tNotices-3(क)(ऊ)-1\tdraft");
    }
}
