mod checker;
mod diagnostic;
mod tokenizer;

pub use checker::{CheckOptions, PunctuationMode, check_text, check_text_with_options, check_word};
pub use diagnostic::{Diagnostic, DiagnosticCategory};
pub use tokenizer::{AnalyzedToken, Token, tokenize, tokenize_analyzed};
pub use varnavinyas_prakriya::DiagnosticKind;

/// Error type for parikshak operations.
#[derive(Debug, thiserror::Error)]
pub enum ParikshakError {
    #[error("empty input")]
    EmptyInput,
}
