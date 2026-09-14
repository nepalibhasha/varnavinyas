use varnavinyas_kosha::Kosha;
use varnavinyas_kosha::origin_tag::OriginTag;
use varnavinyas_kosha::part_of_speech::is_noun;

use crate::{AuthorityTier, LexicalStatus, RuleFamily, SandhiCandidate};

/// Lexicalized proper names whose broad dictionary POS label cannot distinguish
/// them from ordinary nouns.
const REVIEWED_LEXICALIZED_PROPER_NAMES: &[&str] = &["नेपाल"];

pub fn rank_candidates(
    mut candidates: Vec<SandhiCandidate>,
    surface: &str,
    lex: &Kosha,
) -> Vec<SandhiCandidate> {
    let surface_origin = lex.origin_of(surface);
    let surface_entry = lex.lookup(surface);
    let surface_is_lexicalized = surface_entry.is_some();
    let surface_is_noun = surface_entry.is_some_and(|entry| is_noun(entry.pos));
    let surface_is_proper_name = REVIEWED_LEXICALIZED_PROPER_NAMES.contains(&surface);

    for candidate in &mut candidates {
        let mut score: f32 = 0.0;

        if candidate.forward_verified {
            score += 0.30;
        }

        match candidate.lexical_left {
            LexicalStatus::KnownHeadword => score += 0.15,
            LexicalStatus::KnownBoundForm => score += 0.10,
            LexicalStatus::KnownSurface => score += 0.08,
            LexicalStatus::Unknown => {}
        }

        match candidate.lexical_right {
            LexicalStatus::KnownHeadword => score += 0.20,
            LexicalStatus::KnownBoundForm => score += 0.10,
            LexicalStatus::KnownSurface => score += 0.10,
            LexicalStatus::Unknown => {}
        }

        score += match candidate.family {
            RuleFamily::DirectJoin => 0.15,
            RuleFamily::VisargaR => 0.12,
            RuleFamily::VisargaSibilant => 0.12,
            RuleFamily::ConsonantAssimilation => 0.10,
            RuleFamily::VowelGuna => 0.10,
            RuleFamily::VowelVriddhi => 0.10,
            RuleFamily::Yan => 0.08,
            RuleFamily::Ayadi => 0.08,
        };

        // Favor tatsam compounds: both parts being classical Sanskrit words is strong
        // evidence of a genuine sandhi junction, regardless of whether the surface
        // word itself appears in the lexicon as a tagged entry.
        let left_tatsam = lex.origin_of(&candidate.left) == Some(OriginTag::Tatsam)
            || candidate.lexical_left == LexicalStatus::KnownBoundForm;
        let right_tatsam = lex.origin_of(&candidate.right) == Some(OriginTag::Tatsam);
        if left_tatsam && right_tatsam {
            score += 0.10;
        }

        // If the surface is already a lexicalized word and not explicitly tatsam,
        // require stronger evidence before promoting a classical sandhi split.
        if surface_is_lexicalized
            && surface_origin != Some(OriginTag::Tatsam)
            && !matches!(candidate.family, RuleFamily::DirectJoin)
        {
            score -= 0.10;
        }

        // Yan/Ayadi reconstructions overgenerate more often on everyday lexicalized forms.
        if surface_is_lexicalized
            && surface_origin != Some(OriginTag::Tatsam)
            && matches!(candidate.family, RuleFamily::Yan | RuleFamily::Ayadi)
        {
            score -= 0.08;
        }

        // Lexicalized nouns need stronger evidence than a mechanical reconstruction,
        // but retain genuine classical compounds when both members are tatsam. Proper
        // names remain atomic for orthography UX even if their candidate parts happen
        // to look classical.
        let weak_lexicalized_noun = surface_is_noun
            && surface_origin != Some(OriginTag::Tatsam)
            && !(left_tatsam && right_tatsam);
        if (surface_is_proper_name || weak_lexicalized_noun)
            && !matches!(candidate.family, RuleFamily::DirectJoin)
        {
            score -= 0.40;
        }

        // Penalize minimal-boundary one-akshara left segments outside direct-join style cases.
        if candidate.left.chars().count() <= 2
            && !matches!(candidate.family, RuleFamily::DirectJoin)
        {
            score -= 0.05;
        }

        candidate.confidence = score.clamp(0.0_f32, 1.0_f32);
        candidate.authority = if candidate.confidence >= 0.85 {
            AuthorityTier::Authoritative
        } else if candidate.confidence >= 0.65 {
            AuthorityTier::Likely
        } else if candidate.confidence >= 0.40 {
            AuthorityTier::Plausible
        } else {
            AuthorityTier::Exploratory
        };
    }

    candidates.sort_by(|a, b| {
        b.authority
            .cmp(&a.authority)
            .then_with(|| b.confidence.total_cmp(&a.confidence))
            .then_with(|| a.left.cmp(&b.left))
            .then_with(|| a.right.cmp(&b.right))
    });
    candidates.dedup_by(|a, b| a.left == b.left && a.right == b.right);
    candidates
}
