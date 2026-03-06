mod aadhi_vriddhi;
pub mod analysis;
mod chandrabindu_shirbindu;
mod correction_table;
mod engine;
mod halanta_ra_ajanta;
mod hrasva_dirgha;
mod niyama_registry;
pub mod orthographic;
pub mod prakriya;
pub mod rule;
pub mod rule_spec;
pub mod step;
mod structural;
mod ustai_ucharan_varnaharu;

pub use analysis::{RuleNote, WordAnalysis, analyze};
pub use correction_table::contains as is_in_correction_table;
pub use engine::derive;
pub use prakriya::Prakriya;
pub use rule::Rule;
pub use rule_spec::{DiagnosticKind, PatternRule, RuleCategory, RuleSpec};
pub use step::Step;

/// Error type for prakriya operations.
#[derive(Debug, thiserror::Error)]
pub enum PrakriyaError {
    #[error("empty input")]
    EmptyInput,
}
