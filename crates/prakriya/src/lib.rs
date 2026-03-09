pub mod analysis;
mod correction_table;
mod engine;
pub mod explanation;
pub mod model;
mod niyama_registry;
pub mod orthographic;
pub mod presentation;
mod runtime;
mod usage_fixes;
pub mod varna_vinyasa;

pub use analysis::{RuleNote, WordAnalysis, analyze};
pub use correction_table::contains as is_in_correction_table;
pub use engine::{collect_rule_hits, derive};
pub use explanation::Explanation;
pub use model::prakriya::{Prakriya, RuleHit};
pub use model::rule::Rule;
pub use model::rule_spec::{DiagnosticKind, PatternRule, RuleCategory, RuleSpec};
pub use model::step::Step;
pub use presentation::{ApiRuleNote, ApiWordAnalysis};

/// Error type for prakriya operations.
#[derive(Debug, thiserror::Error)]
pub enum PrakriyaError {
    #[error("empty input")]
    EmptyInput,
}
