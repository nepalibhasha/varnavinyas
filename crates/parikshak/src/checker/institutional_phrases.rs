//! Reviewed two-word spacing prescriptions, not morphological decomposition.
use std::sync::LazyLock;

const DATA: &str = include_str!("../../../../data/rule_inventories/institutional_spacing.tsv");
const SOURCE: &str = "PS-Saisanik:5(आ)(ख)";
static PHRASES: LazyLock<Vec<Phrase<'static>>> = LazyLock::new(|| parse(DATA));

pub(super) struct Phrase<'a> {
    pub left: &'a str,
    pub right: &'a str,
}

pub(super) fn phrases() -> &'static [Phrase<'static>] {
    &PHRASES
}

fn parse(data: &str) -> Vec<Phrase<'_>> {
    let mut lines = data.lines();
    assert_eq!(lines.next(), Some("left\tright\tsource\treview_status"));
    let mut phrases: Vec<Phrase<'_>> = Vec::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 4, "invalid spacing pair: {line}");
        assert!(
            fields.iter().all(|f| !f.is_empty()),
            "missing field: {line}"
        );
        assert!(
            fields[..2]
                .iter()
                .all(|f| !f.chars().any(char::is_whitespace)),
            "spacing pair members must be single words: {line}"
        );
        assert!(
            fields[2].split(';').any(|s| s == SOURCE),
            "missing rule source: {line}"
        );
        assert_eq!(fields[3], "reviewed", "unreviewed spacing pair: {line}");
        assert!(
            !phrases
                .iter()
                .any(|p| p.left == fields[0] && p.right == fields[1]),
            "duplicate spacing pair: {line}"
        );
        phrases.push(Phrase {
            left: fields[0],
            right: fields[1],
        });
    }
    phrases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_contains_source_pairs_without_generalizing_member_words() {
        assert_eq!(phrases().len(), 22);
        assert!(
            phrases()
                .iter()
                .any(|p| p.left == "समाज" && p.right == "सेवा")
        );
        assert!(!phrases().iter().any(|p| matches!(p.left, "वायु" | "जन")));
    }

    #[test]
    fn inventory_rejects_missing_source_unreviewed_and_duplicate_pairs() {
        for row in [
            "समाज\tसेवा\t\treviewed",
            "समाज\tसेवा\tOtherSource\treviewed",
            "समाज\tसेवा\tPS-Saisanik:5(आ)(ख)\tdraft",
            "समाज\tसेवा\tPS-Saisanik:5(आ)(ख)\treviewed\nसमाज\tसेवा\tPS-Saisanik:5(आ)(ख)\treviewed",
            "समाज सेवा\tसेवा\tPS-Saisanik:5(आ)(ख)\treviewed",
        ] {
            let data = format!("left\tright\tsource\treview_status\n{row}");
            assert!(std::panic::catch_unwind(|| parse(&data)).is_err());
        }
    }
}
