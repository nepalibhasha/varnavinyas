mod morphology;
mod origin;
pub mod tables;

pub use morphology::{Morpheme, decompose};
pub use origin::{
    Origin, OriginDecision, OriginSource, classify, classify_with_provenance, source_language,
};

/// Error type for shabda operations.
#[derive(Debug, thiserror::Error)]
pub enum ShabdaError {
    #[error("empty input")]
    EmptyInput,

    #[error("unknown word: {0}")]
    UnknownWord(String),
}
