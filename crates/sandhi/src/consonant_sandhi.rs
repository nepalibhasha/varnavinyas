use crate::{SandhiResult, SandhiType};

/// Apply the first matching consonant sandhi rule from the registry.
pub fn apply_consonant_sandhi(first: &str, second: &str) -> Option<SandhiResult> {
    crate::apply_all(first, second)
        .into_iter()
        .find(|result| result.sandhi_type == SandhiType::ConsonantSandhi)
}
