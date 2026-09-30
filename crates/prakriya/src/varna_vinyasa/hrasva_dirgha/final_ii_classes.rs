//! Source-reviewed semantic evidence; a final vowel or guessed POS is not enough.
use std::sync::LazyLock;

const DATA: &str =
    include_str!("../../../../../data/rule_inventories/final_ii_semantic_classes.tsv");
static ENTRIES: LazyLock<Vec<Entry<'static>>> = LazyLock::new(|| parse_inventory(DATA));

pub(super) struct Entry<'a> {
    pub word: &'a str,
    pub rule_code: &'static str,
    pub explanation: &'static str,
}

pub(super) fn lookup(word: &str) -> Option<&'static Entry<'static>> {
    ENTRIES.iter().find(|entry| entry.word == word)
}

fn parse_inventory(data: &str) -> Vec<Entry<'_>> {
    let mut lines = data.lines();
    assert_eq!(lines.next(), Some("word\tclass\tsource\treview_status"));
    let mut entries: Vec<Entry<'_>> = Vec::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let fields: Vec<_> = line.split('\t').collect();
        assert_eq!(fields.len(), 4, "expected four fields: {line}");
        let (word, class, source, status) = (fields[0], fields[1], fields[2], fields[3]);
        assert!(
            !word.is_empty() && !source.is_empty(),
            "missing provenance: {line}"
        );
        assert_eq!(status, "reviewed", "unreviewed semantic class: {line}");
        assert!(word.ends_with('ी'), "not an ई-final source example: {line}");
        let (source_code, rule_code, explanation) = match class {
            "feminine_adjective" => (
                "Notices-3(क)(ऊ)-4",
                "3(क)(ऊ)-4",
                "सूचीकृत स्त्रीलिङ्गी विशेषणको अन्त्यमा ईकार दीर्घ हुन्छ",
            ),
            "inanimate_noun" => (
                "Notices-3(क)(ऊ)-6",
                "3(क)(ऊ)-6",
                "सूचीकृत ईकारान्त निर्जीव नामको अन्त्यमा ईकार दीर्घ हुन्छ",
            ),
            "ps_inanimate_noun" => (
                "PS-Saisanik-ह्रस्वदीर्घ-(थ)",
                "PS-Saisanik-ह्रस्वदीर्घ-(थ)",
                "शैक्षणिक व्याकरण (थ): सूचीकृत निर्जीव वस्तुबोधक नाममा अन्त्यको ईकार दीर्घ हुन्छ",
            ),
            _ => panic!("unknown semantic class: {line}"),
        };
        assert!(
            source.split(';').any(|code| code == source_code),
            "class/source mismatch: {line}"
        );
        assert!(
            !entries.iter().any(|entry| entry.word == word),
            "duplicate example: {line}"
        );
        entries.push(Entry {
            word,
            rule_code,
            explanation,
        });
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_source_examples_have_reviewed_semantic_evidence() {
        assert_eq!(parse_inventory(DATA).len(), 29);
        assert_eq!(lookup("गोरी").unwrap().rule_code, "3(क)(ऊ)-4");
        assert_eq!(lookup("फर्सी").unwrap().rule_code, "3(क)(ऊ)-6");
        assert_eq!(
            lookup("कोदाली").unwrap().rule_code,
            "PS-Saisanik-ह्रस्वदीर्घ-(थ)"
        );
        for word in ["माथि", "प्रभु", "सम्धी", "ज्ञानी", "अपरिचिती"]
        {
            assert!(lookup(word).is_none(), "{word}");
        }
    }

    #[test]
    fn invalid_rows_are_rejected() {
        let header = "word\tclass\tsource\treview_status\n";
        for row in [
            "गोरी\tfeminine_adjective\t\treviewed",
            "गोरी\tfeminine_adjective\tNotices-3(क)(ऊ)-4\tdraft",
            "गोरी\tnoun\tNotices-3(क)(ऊ)-4\treviewed",
            "गोरी\tfeminine_adjective\tNotices-3(क)(ऊ)-6\treviewed",
            "गोरि\tfeminine_adjective\tNotices-3(क)(ऊ)-4\treviewed",
            "गोरी\tfeminine_adjective\tNotices-3(क)(ऊ)-4\treviewed\nगोरी\tfeminine_adjective\tNotices-3(क)(ऊ)-4\treviewed",
        ] {
            assert!(
                std::panic::catch_unwind(|| {
                    let _ = parse_inventory(&format!("{header}{row}"));
                })
                .is_err()
            );
        }
    }
}
