mod builder;
mod kosha;

pub use kosha::{Kosha, WordEntry, kosha};

/// Error type for kosha operations.
#[derive(Debug, thiserror::Error)]
pub enum KoshaError {
    #[error("FST build error: {0}")]
    FstBuild(String),

    #[error("empty lexicon")]
    EmptyLexicon,
}
