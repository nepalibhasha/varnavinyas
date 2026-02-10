mod consonant;
mod devanagari;
mod normalize;
mod syllable;
mod vowel;

pub use consonant::{Varga, is_panchham, varga};
pub use devanagari::{
    CharType, DevanagariChar, classify, is_halanta, is_matra, is_svar, is_vyanjan,
};
pub use normalize::normalize;
pub use syllable::{Akshara, split_aksharas};
pub use vowel::{
    SvarType, dirgha_to_hrasva, hrasva_to_dirgha, matra_to_svar, svar_to_matra, svar_type,
};

/// Error type for akshar operations.
#[derive(Debug, thiserror::Error)]
pub enum AksharError {
    #[error("invalid Devanagari codepoint: U+{0:04X}")]
    InvalidCodepoint(u32),

    #[error("empty input")]
    EmptyInput,

    #[error("normalization failed: {0}")]
    NormalizationError(String),
}
