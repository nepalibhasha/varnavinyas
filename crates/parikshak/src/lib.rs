mod checker;
mod diagnostic;
mod tokenizer;

pub use checker::{check_text, check_word};
pub use diagnostic::{Diagnostic, DiagnosticCategory};
pub use varnavinyas_prakriya::DiagnosticKind;
pub use tokenizer::{AnalyzedToken, Token, tokenize, tokenize_analyzed};

/// Error type for parikshak operations.
#[derive(Debug, thiserror::Error)]
pub enum ParikshakError {
    #[error("empty input")]
    EmptyInput,
}
