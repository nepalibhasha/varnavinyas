use varnavinyas_kosha::Kosha;
use varnavinyas_kosha::origin_tag::OriginTag;

use crate::{AuthorityTier, LexicalStatus, RuleFamily, SandhiCandidate};

pub fn rank_candidates(
    mut candidates: Vec<SandhiCandidate>,
    surface: &str,
    lex: &Kosha,
) -> Vec<SandhiCandidate> {
    let surface_origin = lex.origin_of(surface);
    let surface_is_lexicalized = lex.lookup(surface).is_some();

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

        // Favor origin-consistent tatsam compounds, where most classical sandhi is expected.
        if surface_origin == Some(OriginTag::Tatsam) {
            let left_tatsam = lex.origin_of(&candidate.left) == Some(OriginTag::Tatsam)
                || candidate.lexical_left == LexicalStatus::KnownBoundForm;
            let right_tatsam = lex.origin_of(&candidate.right) == Some(OriginTag::Tatsam);
            if left_tatsam && right_tatsam {
                score += 0.10;
            }
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
