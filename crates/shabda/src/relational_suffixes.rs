use crate::AffixKind;
use std::sync::LazyLock;

const DATA: &str = include_str!("../../../data/rule_inventories/relational_suffixes.tsv");
static FORMS: LazyLock<Vec<(&str, AffixKind)>> = LazyLock::new(|| parse(DATA));

pub(crate) fn suffix_kind(form: &str) -> Option<AffixKind> {
    FORMS
        .iter()
        .find_map(|&(text, kind)| (text == form).then_some(kind))
}

fn parse(data: &str) -> Vec<(&str, AffixKind)> {
    let mut lines = data.lines();
    assert_eq!(lines.next(), Some("form\tkind\tsource\treview_status"));
    let mut forms = Vec::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 4, "invalid relational suffix: {line}");
        assert!(
            !fields[0].is_empty() && !fields[2].is_empty(),
            "missing relational suffix evidence: {line}"
        );
        assert_eq!(
            fields[3], "reviewed",
            "unreviewed relational suffix: {line}"
        );
        assert!(
            !forms.iter().any(|(form, _)| *form == fields[0]),
            "duplicate relational suffix: {line}"
        );
        let kind = match fields[1] {
            "postposition" => AffixKind::Postposition,
            "comparison_marker" => AffixKind::ComparisonMarker,
            _ => panic!("unsupported relational suffix kind: {line}"),
        };
        forms.push((fields[0], kind));
    }
    forms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_classifies_only_reviewed_relational_suffixes() {
        for (form, kind) in parse(DATA) {
            assert!(crate::tables::CASE_MARKERS.contains(&form), "{form}");
            assert_eq!(suffix_kind(form), Some(kind));
        }
        for form in ["को", "की", "का", "ले", "लाई", "मा", "रहेको"]
        {
            assert_eq!(suffix_kind(form), None);
        }
    }

    #[test]
    fn inventory_rejects_missing_source_status_and_duplicates() {
        for row in [
            "सम्म\tpostposition\t\treviewed",
            "सम्म\tpostposition\tsource\tdraft",
            "सम्म\tunknown\tsource\treviewed",
            "सम्म\tpostposition\tsource\treviewed\nसम्म\tpostposition\tsource\treviewed",
        ] {
            let data = format!("form\tkind\tsource\treview_status\n{row}");
            assert!(std::panic::catch_unwind(|| parse(&data)).is_err());
        }
    }
}
