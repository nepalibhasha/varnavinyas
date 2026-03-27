use crate::model::rule_spec::PatternRule;

mod shuddha_ashuddha;

// Shuddha/ashuddha-style cleanup rules outside the Academy varna-vinyasa families.
pub use shuddha_ashuddha::{SPEC_SHRI, rule_shri_correction};

pub fn usage_fix_rules() -> Vec<PatternRule> {
    vec![
        // Section 4-style structural rules
        PatternRule {
            spec: SPEC_SHRI,
            apply: rule_shri_correction,
        },
    ]
}
