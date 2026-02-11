mod correction_table;
mod engine;
mod hrasva_dirgha;
mod orthographic;
pub mod prakriya;
pub mod rule;
pub mod step;
mod structural;

pub use engine::derive;
pub use prakriya::Prakriya;
pub use rule::Rule;
pub use step::Step;

/// Error type for prakriya operations.
#[derive(Debug, thiserror::Error)]
pub enum PrakriyaError {
    #[error("empty input")]
    EmptyInput,
}
