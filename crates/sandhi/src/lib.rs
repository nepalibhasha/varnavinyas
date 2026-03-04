mod apply;
mod consonant_sandhi;
mod decode;
mod rank;
mod rules;
mod types;
mod visarga_sandhi;
mod vowel_sandhi;

pub use apply::{apply, apply_all};
pub use consonant_sandhi::apply_consonant_sandhi;
pub use decode::{split, split_best};
pub use types::{
    AuthorityTier, LexicalStatus, RuleFamily, SandhiCandidate, SandhiError, SandhiResult,
    SandhiType,
};
pub use visarga_sandhi::apply_visarga_sandhi;
pub use vowel_sandhi::apply_vowel_sandhi;

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
