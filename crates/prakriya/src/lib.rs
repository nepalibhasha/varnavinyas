pub mod analysis;
mod correction_table;
mod engine;
mod niyama_registry;
pub mod orthographic;
pub mod prakriya;
pub mod presentation;
pub mod rule;
pub mod rule_spec;
pub mod step;
mod usage_fixes;
pub mod varna_vinyasa;

pub use analysis::{RuleNote, WordAnalysis, analyze};
pub use correction_table::contains as is_in_correction_table;
pub use engine::{collect_rule_hits, derive};
pub use prakriya::{Prakriya, RuleHit};
pub use presentation::{ApiRuleNote, ApiWordAnalysis};
pub use rule::Rule;
pub use rule_spec::{DiagnosticKind, PatternRule, RuleCategory, RuleSpec};
pub use step::Step;

/// Error type for prakriya operations.
#[derive(Debug, thiserror::Error)]
pub enum PrakriyaError {
    #[error("empty input")]
    EmptyInput,
}
