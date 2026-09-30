use varnavinyas_akshar::{is_halanta, is_matra, is_svar, is_vyanjan, split_aksharas};

fn is_initial_char(ch: char) -> bool {
    is_svar(ch) || is_vyanjan(ch) || is_matra(ch) || is_halanta(ch)
}

fn is_initial_unit(unit: &str) -> bool {
    split_aksharas(unit).len() == 1
        // English letter names with a final consonant have two written aksharas.
        || matches!(unit, "एस" | "एल" | "एम" | "एन" | "एफ" | "एच" | "आर" | "एक्स")
}

/// UTF-8 byte spans of compact dotted Devanagari abbreviations.
///
/// PS-Saisanik hrasva/dirgha (छ) explicitly preserves long vowels in dotted
/// initials such as बी.बी.सी. and सी.डी.ओ. Require at least two single-akshara
/// initials or English letter names, each with a dot: an ordinary word followed by a period is not an
/// abbreviation. The span includes the final abbreviation dot, but not an
/// attached case marker or following punctuation.
pub fn dotted_abbreviation_spans(text: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut covered_until = 0;
    let mut previous = None;
    for (start, ch) in text.char_indices() {
        let starts_unit = is_svar(ch) || is_vyanjan(ch);
        let starts_at_boundary = previous.is_none_or(|ch| !is_initial_char(ch));
        previous = Some(ch);
        if start < covered_until || !starts_unit || !starts_at_boundary {
            continue;
        }

        let mut end = start;
        let mut units = 0;
        loop {
            let rest = &text[end..];
            let unit_len = rest
                .char_indices()
                .find(|(_, ch)| !is_initial_char(*ch))
                .map_or(rest.len(), |(idx, _)| idx);
            let unit = &rest[..unit_len];
            if unit.is_empty()
                || !unit
                    .chars()
                    .next()
                    .is_some_and(|ch| is_svar(ch) || is_vyanjan(ch))
                || !is_initial_unit(unit)
                || !rest[unit_len..].starts_with('.')
            {
                break;
            }
            end += unit_len + 1;
            units += 1;
        }
        if units >= 2 {
            spans.push((start, end));
            covered_until = end;
        }
    }
    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_source_examples_and_attached_case_markers() {
        for text in ["बी.बी.सी.", "सी.डी.ओ.", "एस.एल.सी.", "त्रि.वि.ले"]
        {
            let spans = dotted_abbreviation_spans(text);
            assert_eq!(spans.len(), 1, "{text}");
            assert_eq!(&text[spans[0].0..spans[0].1], text.trim_end_matches("ले"));
        }
    }

    #[test]
    fn preserves_byte_spans_with_emoji_quotes_and_newlines() {
        let text = "🙂 ‘बी.बी.सी.’\nसी.डी.ओ. कार्यालय";
        let found: Vec<_> = dotted_abbreviation_spans(text)
            .into_iter()
            .map(|(start, end)| &text[start..end])
            .collect();
        assert_eq!(found, ["बी.बी.सी.", "सी.डी.ओ."]);
    }

    #[test]
    fn ordinary_words_numbers_and_ellipsis_are_not_initial_chains() {
        for text in [
            "राम. काम.",
            "म यहाँ हुँ.",
            "बी.",
            "सी.नेपाल",
            "१.२.",
            "बी...सी.",
        ] {
            assert!(dotted_abbreviation_spans(text).is_empty(), "{text}");
        }
    }
}
