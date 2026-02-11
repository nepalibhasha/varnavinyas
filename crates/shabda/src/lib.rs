mod morphology;
mod origin;
mod tables;

pub use morphology::{Morpheme, decompose};
pub use origin::{Origin, classify};

/// Error type for shabda operations.
#[derive(Debug, thiserror::Error)]
pub enum ShabdaError {
    #[error("empty input")]
    EmptyInput,

    #[error("unknown word: {0}")]
    UnknownWord(String),
}
