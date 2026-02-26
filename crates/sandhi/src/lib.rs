mod consonant_sandhi;
mod split;
mod visarga_sandhi;
mod vowel_sandhi;

pub use consonant_sandhi::apply_consonant_sandhi;
pub use split::split;
pub use visarga_sandhi::apply_visarga_sandhi;
pub use vowel_sandhi::apply_vowel_sandhi;

/// Categories of sandhi rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandhiType {
    /// Vowel sandhi (अच् सन्धि): vowels combining at morpheme boundaries.
    VowelSandhi,
    /// Visarga sandhi (विसर्ग सन्धि): visarga transformations.
    VisargaSandhi,
    /// Consonant sandhi (हल् सन्धि): consonant assimilations.
    ConsonantSandhi,
}

impl SandhiType {
    /// Canonical Devanagari label for display surfaces.
    pub fn display_label(self) -> &'static str {
        match self {
            Self::VowelSandhi => "स्वर सन्धि",
            Self::VisargaSandhi => "विसर्ग सन्धि",
            Self::ConsonantSandhi => "व्यञ्जन सन्धि",
        }
    }
}

/// Result of a sandhi operation.
#[derive(Debug, Clone)]
pub struct SandhiResult {
    pub output: String,
    pub sandhi_type: SandhiType,
    pub rule_citation: &'static str,
}

/// Error type for sandhi operations.
#[derive(Debug, thiserror::Error)]
pub enum SandhiError {
    #[error("empty input")]
    EmptyInput,

    #[error("no sandhi rule applies for '{first}' + '{second}'")]
    NoRuleApplies { first: String, second: String },
}

/// Apply sandhi: combine two morphemes.
/// Tries vowel, visarga, and consonant sandhi in that order.
pub fn apply(first: &str, second: &str) -> Result<SandhiResult, SandhiError> {
    if first.is_empty() || second.is_empty() {
        return Err(SandhiError::EmptyInput);
    }

    // Try visarga sandhi first (specific prefix patterns)
    if let Some(result) = apply_visarga_sandhi(first, second) {
        return Ok(result);
    }

    // Try consonant sandhi (prefix assimilation)
    if let Some(result) = apply_consonant_sandhi(first, second) {
        return Ok(result);
    }

    // Try vowel sandhi
    if let Some(result) = apply_vowel_sandhi(first, second) {
        return Ok(result);
    }

    // No sandhi applies — simple concatenation
    Err(SandhiError::NoRuleApplies {
        first: first.to_string(),
        second: second.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::SandhiType;

    #[test]
    fn sandhi_type_display_labels_are_devanagari() {
        assert_eq!(SandhiType::VowelSandhi.display_label(), "स्वर सन्धि");
        assert_eq!(SandhiType::VisargaSandhi.display_label(), "विसर्ग सन्धि");
        assert_eq!(SandhiType::ConsonantSandhi.display_label(), "व्यञ्जन सन्धि");
    }
}
