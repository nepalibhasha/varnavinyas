mod abbreviation;
mod punctuation;

pub use abbreviation::dotted_abbreviation_spans;
pub use punctuation::{LekhyaDiagnostic, PunctuationMark, check_punctuation};

/// Error type for lekhya operations.
#[derive(Debug, thiserror::Error)]
pub enum LekhyaError {
    #[error("empty input")]
    EmptyInput,
}
