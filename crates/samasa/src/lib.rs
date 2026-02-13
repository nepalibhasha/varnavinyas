use varnavinyas_kosha::WordEntry;
use varnavinyas_kosha::kosha;

/// Initial samasa taxonomy for MVP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamasaType {
    Tatpurusha,
    Karmadharaya,
    Dvigu,
    Bahuvrihi,
    Dvandva,
    Avyayibhava,
    Unknown,
}

/// Ranked compound interpretation candidate.
#[derive(Debug, Clone, PartialEq)]
pub struct SamasaCandidate {
    pub left: String,
    pub right: String,
    pub samasa_type: SamasaType,
    pub score: f32,
    pub vigraha: String,
}

/// Analyze a word as potential samasa.
///
/// MVP behavior:
/// - generate candidates from sandhi split
/// - add direct lexical boundary candidates (for compounds with no explicit sandhi mutation)
/// - assign heuristic samasa type + score
/// - return ranked candidates
pub fn analyze_compound(word: &str) -> Vec<SamasaCandidate> {
    if word.is_empty() {
        return Vec::new();
    }

    let lex = kosha();
    let mut out = Vec::new();

    // Strategy 1: sandhi-backed candidates.
    for (left, right, _res) in varnavinyas_sandhi::split(word) {
        push_candidate(&mut out, &left, &right, 0.0);
    }

    // Strategy 2: direct lexical boundary scan.
    // Useful for compounds like एकचक्र where no sandhi mutation is needed.
    for (i, _) in word.char_indices().skip(1) {
        let (left, right) = word.split_at(i);
        if !lex.contains(left) || !lex.contains(right) {
            continue;
        }
        // Avoid noisy tiny fragments except numerals used in dvigu.
        if left.chars().count() < 2 && !is_numeral(left) {
            continue;
        }
        if right.chars().count() < 2 {
            continue;
        }
        push_candidate(&mut out, left, right, -0.05);
    }

    out.sort_by(|a, b| {
        b.score
            .total_cmp(&a.score)
            .then_with(|| a.left.cmp(&b.left))
            .then_with(|| a.right.cmp(&b.right))
    });
    out.dedup_by(|a, b| a.left == b.left && a.right == b.right && a.samasa_type == b.samasa_type);
    out
}

fn push_candidate(out: &mut Vec<SamasaCandidate>, left: &str, right: &str, score_adjust: f32) {
    let lex = kosha();
    if !lex.contains(left) || !lex.contains(right) {
        return;
    }

    let left_entry = lex.lookup(left);
    let right_entry = lex.lookup(right);
    let (samasa_type, base_score) = classify_candidate(left, right, left_entry, right_entry);
    let score = (base_score + score_adjust).clamp(0.0, 1.0);
    let vigraha = make_vigraha(left, right, samasa_type);

    out.push(SamasaCandidate {
        left: left.to_string(),
        right: right.to_string(),
        samasa_type,
        score,
        vigraha,
    });
}

fn classify_candidate(
    left: &str,
    right: &str,
    left_entry: Option<&WordEntry>,
    right_entry: Option<&WordEntry>,
) -> (SamasaType, f32) {
    let left_pos = left_entry.map(|e| e.pos).unwrap_or("");
    let right_pos = right_entry.map(|e| e.pos).unwrap_or("");

    // Dvigu: numeral-led compounds.
    if is_numeral(left) {
        return (SamasaType::Dvigu, 0.92);
    }

    // Avyayibhava: indeclinable (अव्यय) leading component.
    if left_pos.contains("अव्य") {
        return (SamasaType::Avyayibhava, 0.86);
    }

    // Karmadharaya: adjective + noun.
    if (left_pos.contains("वि.") || is_adjectival_prefix(left)) && right_pos.contains("ना.")
    {
        return (SamasaType::Karmadharaya, 0.84);
    }

    // Bahuvrihi (weak MVP signal): adjective + adjective.
    if left_pos.contains("वि.") && right_pos.contains("वि.") {
        return (SamasaType::Bahuvrihi, 0.74);
    }

    // Dvandva only for known coordinative pairs in MVP.
    if is_known_dvandva_pair(left, right) {
        return (SamasaType::Dvandva, 0.8);
    }

    // Noun + noun defaults to tatpurusha in MVP.
    if left_pos.contains("ना.") && right_pos.contains("ना.") {
        return (SamasaType::Tatpurusha, 0.82);
    }

    // Default for lexically plausible split is Tatpurusha in MVP.
    if left_entry.is_some() && right_entry.is_some() {
        return (SamasaType::Tatpurusha, 0.78);
    }

    (SamasaType::Unknown, 0.5)
}

fn make_vigraha(left: &str, right: &str, t: SamasaType) -> String {
    match t {
        SamasaType::Tatpurusha => format!("{left} को {right}"),
        SamasaType::Karmadharaya => format!("{left} {right}"),
        SamasaType::Dvigu => format!("{left} वटा {right}"),
        SamasaType::Bahuvrihi => format!("{left} {right} भएको"),
        SamasaType::Dvandva => format!("{left} र {right}"),
        SamasaType::Avyayibhava => format!("{left} {right}"),
        SamasaType::Unknown => format!("{left}+{right}"),
    }
}

fn is_numeral(s: &str) -> bool {
    matches!(
        s,
        "एक" | "द्वि"
            | "त्रि"
            | "चतुर"
            | "चतु"
            | "पञ्च"
            | "षड"
            | "षट"
            | "सप्त"
            | "अष्ट"
            | "नव"
            | "दश"
    )
}

fn is_adjectival_prefix(s: &str) -> bool {
    matches!(s, "मह" | "सु")
}

fn is_known_dvandva_pair(left: &str, right: &str) -> bool {
    matches!(
        (left, right),
        ("राम", "लक्ष्मण") | ("माता", "पिता") | ("दिन", "रात") | ("सुख", "दुःख")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_dvigu_from_numeral() {
        let (t, score) = classify_candidate("एक", "चक्र", None, None);
        assert_eq!(t, SamasaType::Dvigu);
        assert!(score > 0.9);
    }

    #[test]
    fn classify_avyayibhava_from_pos() {
        let left = WordEntry {
            word: "उपरि",
            pos: "अव्य.",
        };
        let right = WordEntry {
            word: "भाग",
            pos: "ना.",
        };
        let (t, _) = classify_candidate("उपरि", "भाग", Some(&left), Some(&right));
        assert_eq!(t, SamasaType::Avyayibhava);
    }

    #[test]
    fn classify_karmadharaya_from_adj_noun() {
        let left = WordEntry {
            word: "मह",
            pos: "वि.",
        };
        let right = WordEntry {
            word: "उत्सव",
            pos: "ना.",
        };
        let (t, _) = classify_candidate("मह", "उत्सव", Some(&left), Some(&right));
        assert_eq!(t, SamasaType::Karmadharaya);
    }

    #[test]
    fn classify_bahuvrihi_from_adj_adj() {
        let left = WordEntry {
            word: "नील",
            pos: "वि.",
        };
        let right = WordEntry {
            word: "कण्ठ",
            pos: "वि.",
        };
        let (t, _) = classify_candidate("नील", "कण्ठ", Some(&left), Some(&right));
        assert_eq!(t, SamasaType::Bahuvrihi);
    }

    #[test]
    fn classify_dvandva_for_known_pair() {
        let left = WordEntry {
            word: "राम",
            pos: "ना.",
        };
        let right = WordEntry {
            word: "लक्ष्मण",
            pos: "ना.",
        };
        let (t, _) = classify_candidate("राम", "लक्ष्मण", Some(&left), Some(&right));
        assert_eq!(t, SamasaType::Dvandva);
    }

    #[test]
    fn analyze_compound_returns_ranked_candidates() {
        let candidates = analyze_compound("सूर्योदय");
        assert!(!candidates.is_empty());
        assert!(candidates.windows(2).all(|w| w[0].score >= w[1].score));
    }

    #[test]
    fn analyze_compound_direct_split_fallback() {
        let candidates = analyze_compound("एकचक्र");
        assert!(
            candidates
                .iter()
                .any(|c| c.left == "एक" && c.right == "चक्र")
        );
    }
}
