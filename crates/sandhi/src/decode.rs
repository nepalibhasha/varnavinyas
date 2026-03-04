use varnavinyas_akshar::split_aksharas;
use varnavinyas_kosha::kosha;
use varnavinyas_kosha::origin_tag::OriginTag;

use crate::rank::rank_candidates;
use crate::rules::{all_rules, apply_reverse_rule};
use crate::{AuthorityTier, SandhiCandidate};

/// Known one-akshara upasarga forms that are valid left segments in compounds.
const ONE_AKSHARA_UPASARGAS: &[&str] = &["प्र", "वि", "सु", "नि", "आ"];

fn valid_split_parts(left: &str, right: &str) -> bool {
    let left_len = split_aksharas(left).len();
    let right_len = split_aksharas(right).len();
    let valid_left = left_len >= 2 || ONE_AKSHARA_UPASARGAS.contains(&left);
    valid_left && right_len >= 2
}

pub(crate) fn is_known_one_akshara_upasarga(left: &str) -> bool {
    ONE_AKSHARA_UPASARGAS.contains(&left)
}

/// Split a word at potential sandhi boundaries and return ranked candidates.
pub fn split(word: &str) -> Vec<SandhiCandidate> {
    if split_aksharas(word).len() < 3 {
        return Vec::new();
    }

    let mut out = Vec::new();
    let lex = kosha();

    for (i, _) in word.char_indices().skip(1) {
        let (raw_left, raw_right) = word.split_at(i);
        for rule in all_rules() {
            if rule.reverse {
                apply_reverse_rule(rule, word, raw_left, raw_right, lex, &mut out);
            }
        }
    }

    out.retain(|candidate| {
        if !valid_split_parts(&candidate.left, &candidate.right) {
            return false;
        }
        if split_aksharas(&candidate.left).len() < 2 {
            return lex.origin_of(word) == Some(OriginTag::Tatsam)
                && lex.origin_of(&candidate.right) == Some(OriginTag::Tatsam);
        }
        true
    });

    let mut ranked = rank_candidates(out, word, lex);
    ranked.retain(|candidate| !matches!(candidate.authority, AuthorityTier::Exploratory));
    ranked
}

/// Return the safest single split candidate for public-facing consumers.
pub fn split_best(word: &str) -> Option<SandhiCandidate> {
    split(word).into_iter().find(|candidate| {
        matches!(
            candidate.authority,
            AuthorityTier::Authoritative | AuthorityTier::Likely
        )
    })
}
