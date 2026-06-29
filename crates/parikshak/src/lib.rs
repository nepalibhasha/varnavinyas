mod checker;
mod diagnostic;
pub mod presentation;
mod tokenizer;

pub use checker::{
    CheckOptions, OrthographyMode, PunctuationMode, check_text, check_text_with_options,
    check_word, check_word_with_options,
};
pub use diagnostic::{
    Diagnostic, DiagnosticCategory, DiagnosticReason, diagnostic_reason_category,
};
pub use presentation::{ApiDiagnostic, ApiDiagnosticReason};
pub use tokenizer::{AnalyzedToken, Token, tokenize, tokenize_analyzed};
pub use varnavinyas_prakriya::DiagnosticKind;

/// Error type for parikshak operations.
#[derive(Debug, thiserror::Error)]
pub enum ParikshakError {
    #[error("empty input")]
    EmptyInput,
}
