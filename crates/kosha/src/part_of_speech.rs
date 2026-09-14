//! Stable part-of-speech predicates for dictionary metadata.
//!
//! Generated assets use expanded grammatical labels, but these predicates also
//! accept legacy abbreviations so test seams and older callers do not need to
//! inspect raw strings. Bracketed source/etymology metadata is ignored.

fn is_label_char(ch: char) -> bool {
    ch.is_alphanumeric() || ('\u{0900}'..='\u{097f}').contains(&ch) || ch == '.'
}

fn segment_has_label(segment: &str, label: &str) -> bool {
    segment.match_indices(label).any(|(start, matched)| {
        let before_is_label = segment[..start]
            .chars()
            .next_back()
            .is_some_and(is_label_char);
        let end = start + matched.len();
        let after_is_label = segment[end..].chars().next().is_some_and(is_label_char);
        !before_is_label && !after_is_label
    })
}

fn outside_brackets_has_any(pos: &str, labels: &[&str]) -> bool {
    let mut rest = pos;
    loop {
        let Some(open) = rest.find('[') else {
            return labels.iter().any(|label| segment_has_label(rest, label));
        };
        if labels
            .iter()
            .any(|label| segment_has_label(&rest[..open], label))
        {
            return true;
        }
        let Some(close) = rest[open + 1..].find(']') else {
            return false;
        };
        rest = &rest[open + close + 2..];
    }
}

pub fn is_noun(pos: &str) -> bool {
    outside_brackets_has_any(pos, &["नाम", "ना.", "ना"])
}

pub fn is_adjective(pos: &str) -> bool {
    outside_brackets_has_any(pos, &["विशेषण", "वि.", "वि"])
}

pub fn is_adverb(pos: &str) -> bool {
    outside_brackets_has_any(
        pos,
        &[
            "क्रियाविशेषण",
            "क्रियायोगी",
            "क्रि.वि.",
            "क्रिवि.",
            "क्रि. यो.",
            "क्रि.यो.",
        ],
    )
}

pub fn is_namayogi(pos: &str) -> bool {
    outside_brackets_has_any(
        pos,
        &["नामयोगी", "ना.यो.", "ना. यो.", "ना. यो", "नायो.", "नाम यो"],
    )
}

pub fn is_conjunction(pos: &str) -> bool {
    outside_brackets_has_any(pos, &["संयोजक", "संयो.", "संयो"])
}

pub fn is_avyaya(pos: &str) -> bool {
    outside_brackets_has_any(pos, &["अव्यय", "अव्य."])
}

pub fn is_onomatopoeic(pos: &str) -> bool {
    // Unlike grammatical labels, अनुकरण मूल is commonly recorded as
    // etymological evidence inside brackets (for example `[अ.मू. टिल्]`).
    ["अनुकरण मूल", "अ.मू.", "अ. मू.", "अमू."]
        .iter()
        .any(|label| segment_has_label(pos, label))
}

pub fn is_pronoun(pos: &str) -> bool {
    outside_brackets_has_any(pos, &["सर्वनाम", "सर्व.", "सर्व"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expanded_and_abbreviated_labels_are_equivalent() {
        assert!(is_noun("नाम [सं.]"));
        assert!(is_noun("ना. [सं.]"));
        assert!(is_noun("ना [सं.]"));
        assert!(is_adjective("विशेषण / नाम"));
        assert!(is_adjective("वि. / ना."));
        assert!(is_adjective("वि / ना"));
        assert!(is_adverb("क्रियाविशेषण [हि.]"));
        assert!(is_adverb("क्रियायोगी [सं.]"));
        assert!(is_adverb("क्रि.वि. [हि.]"));
        assert!(is_onomatopoeic("क्रियाविशेषण [अ.मू. टिल्]"));
    }

    #[test]
    fn labels_do_not_match_inside_other_labels_or_metadata() {
        assert!(!is_noun("नामयोगी"));
        assert!(!is_noun("सर्वनाम"));
        assert!(!is_adjective("क्रियाविशेषण"));
        assert!(!is_adjective("ना.वि."));
        assert!(!is_noun("विशेषण [सं. नामन्]"));
    }
}
