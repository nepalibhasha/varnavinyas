#[cfg(feature = "legacy")]
mod legacy;
mod mapping;
mod scheme;

pub use scheme::{LipiError, Scheme};

/// Transliterate text from one scheme to another.
pub fn transliterate(input: &str, from: Scheme, to: Scheme) -> Result<String, LipiError> {
    if input.is_empty() {
        return Ok(String::new());
    }
    if from == to {
        return Ok(input.to_string());
    }
    mapping::transliterate_impl(input, from, to)
}

/// Attempt to detect the scheme of the input text.
pub fn detect_scheme(input: &str) -> Option<Scheme> {
    scheme::detect_scheme_impl(input)
}
