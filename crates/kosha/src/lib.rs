mod builder;
mod kosha;
pub mod origin_tag;

pub use kosha::{Kosha, WordEntry, kosha};
pub use origin_tag::{OriginTag, parse_source_language};

/// Error type for kosha operations.
#[derive(Debug, thiserror::Error)]
pub enum KoshaError {
    #[error("FST build error: {0}")]
    FstBuild(String),

    #[error("empty lexicon")]
    EmptyLexicon,
}
