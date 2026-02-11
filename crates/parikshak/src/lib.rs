mod checker;
mod diagnostic;
mod tokenizer;

pub use checker::{check_text, check_word};
pub use diagnostic::{Diagnostic, DiagnosticCategory};
pub use tokenizer::{Token, tokenize};

/// Error type for parikshak operations.
#[derive(Debug, thiserror::Error)]
pub enum ParikshakError {
    #[error("empty input")]
    EmptyInput,
}
