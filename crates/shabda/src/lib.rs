mod morphology;
mod origin;
mod progressive;
pub mod tables;

pub use morphology::{
    AffixAnalysis, AffixKind, AffixSegment, Morpheme, RootCandidate, analyze_affixes,
    best_analysis, best_root, decompose, has_known_root, has_supported_analysis,
    lookup_root_candidates,
};
pub use origin::{
    Origin, OriginDecision, OriginSource, classify, classify_with_provenance, source_language,
};
pub use progressive::{ProgressiveAnalysis, analyze_progressive, has_supported_progressive_form};

/// Error type for shabda operations.
#[derive(Debug, thiserror::Error)]
pub enum ShabdaError {
    #[error("empty input")]
    EmptyInput,

    #[error("unknown word: {0}")]
    UnknownWord(String),
}
