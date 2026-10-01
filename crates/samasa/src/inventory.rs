//! Source-reviewed word formations; lexical coincidence cannot supply vigraha.
use std::sync::LazyLock;

use crate::SamasaType;

const DATA: &str = include_str!("../../../data/rule_inventories/samasa.tsv");
static ENTRIES: LazyLock<Vec<Entry<'static>>> = LazyLock::new(|| parse_inventory(DATA));

pub(super) struct Entry<'a> {
    pub word: &'a str,
    pub left: &'a str,
    pub right: &'a str,
    pub samasa_type: SamasaType,
    pub vigraha: &'a str,
}

pub(super) fn lookup(word: &str) -> Option<&'static Entry<'static>> {
    ENTRIES.iter().find(|entry| entry.word == word)
}

fn parse_inventory(data: &str) -> Vec<Entry<'_>> {
    let mut lines = data.lines();
    assert_eq!(
        lines.next(),
        Some("word\tleft\tright\tsamasa_type\tvigraha\tsource\treview_status")
    );
    let mut entries: Vec<Entry<'_>> = Vec::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 7, "expected seven fields: {line}");
        assert!(
            fields[..6].iter().all(|field| !field.trim().is_empty()),
            "missing formation or provenance: {line}"
        );
        assert_eq!(fields[6], "reviewed", "unreviewed compound: {line}");
        assert!(
            fields[0] != fields[1] && fields[0] != fields[2],
            "self-repeating compound: {line}"
        );
        assert!(
            !entries.iter().any(|entry| entry.word == fields[0]),
            "duplicate compound: {line}"
        );
        let samasa_type = match fields[3] {
            "Tatpurusha" => SamasaType::Tatpurusha,
            "Karmadharaya" => SamasaType::Karmadharaya,
            "Dvigu" => SamasaType::Dvigu,
            "Bahuvrihi" => SamasaType::Bahuvrihi,
            "Dvandva" => SamasaType::Dvandva,
            "Avyayibhava" => SamasaType::Avyayibhava,
            _ => panic!("unsupported reviewed samasa type: {line}"),
        };
        entries.push(Entry {
            word: fields[0],
            left: fields[1],
            right: fields[2],
            samasa_type,
            vigraha: fields[4],
        });
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "word\tleft\tright\tsamasa_type\tvigraha\tsource\treview_status\n";

    #[test]
    fn reviewed_inventory_has_lexical_members_and_exact_compositions() {
        let lex = varnavinyas_kosha::kosha();
        for entry in parse_inventory(DATA) {
            assert!(lex.lookup(entry.left).is_some(), "{}", entry.left);
            assert!(lex.lookup(entry.right).is_some(), "{}", entry.right);
            assert!(
                format!("{}{}", entry.left, entry.right) == entry.word
                    || varnavinyas_sandhi::apply_all(entry.left, entry.right)
                        .iter()
                        .any(|result| result.output == entry.word),
                "{}: inventory pair must reproduce the word",
                entry.word
            );
        }
    }

    #[test]
    #[should_panic(expected = "missing formation or provenance")]
    fn missing_source_is_rejected() {
        parse_inventory(&format!(
            "{HEADER}एकचक्र\tएक\tचक्र\tDvigu\tएक चक्र\t\treviewed"
        ));
    }

    #[test]
    #[should_panic(expected = "unreviewed compound")]
    fn unreviewed_row_is_rejected() {
        parse_inventory(&format!(
            "{HEADER}एकचक्र\tएक\tचक्र\tDvigu\tएक चक्र\tPragya\tdraft"
        ));
    }

    #[test]
    #[should_panic(expected = "duplicate compound")]
    fn duplicate_row_is_rejected() {
        let row = "एकचक्र\tएक\tचक्र\tDvigu\tएक चक्र\tPragya\treviewed\n";
        parse_inventory(&format!("{HEADER}{row}{row}"));
    }

    #[test]
    #[should_panic(expected = "self-repeating compound")]
    fn self_repeating_row_is_rejected() {
        parse_inventory(&format!(
            "{HEADER}आयात\tआ\tआयात\tTatpurusha\tआ को आयात\tPragya\treviewed"
        ));
    }
}
