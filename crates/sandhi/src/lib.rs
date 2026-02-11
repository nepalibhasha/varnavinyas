mod consonant_sandhi;
mod visarga_sandhi;
mod vowel_sandhi;

pub use consonant_sandhi::apply_consonant_sandhi;
pub use visarga_sandhi::apply_visarga_sandhi;
pub use vowel_sandhi::apply_vowel_sandhi;

/// Categories of sandhi rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandhiType {
    /// Vowel sandhi (अच् सन्धि): vowels combining at morpheme boundaries.
    VowelSandhi,
    /// Visarga sandhi (विसर्ग सन्धि): visarga transformations.
    VisargaSandhi,
    /// Consonant sandhi (हल् सन्धि): consonant assimilations.
    ConsonantSandhi,
}

/// Result of a sandhi operation.
#[derive(Debug, Clone)]
pub struct SandhiResult {
    pub output: String,
    pub sandhi_type: SandhiType,
    pub rule_citation: &'static str,
}

/// Error type for sandhi operations.
#[derive(Debug, thiserror::Error)]
pub enum SandhiError {
    #[error("empty input")]
    EmptyInput,

    #[error("no sandhi rule applies for '{first}' + '{second}'")]
    NoRuleApplies { first: String, second: String },
}

/// Apply sandhi: combine two morphemes.
/// Tries vowel, visarga, and consonant sandhi in that order.
pub fn apply(first: &str, second: &str) -> Result<SandhiResult, SandhiError> {
    if first.is_empty() || second.is_empty() {
        return Err(SandhiError::EmptyInput);
    }

    // Try visarga sandhi first (specific prefix patterns)
    if let Some(result) = apply_visarga_sandhi(first, second) {
        return Ok(result);
    }

    // Try consonant sandhi (prefix assimilation)
    if let Some(result) = apply_consonant_sandhi(first, second) {
        return Ok(result);
    }

    // Try vowel sandhi
    if let Some(result) = apply_vowel_sandhi(first, second) {
        return Ok(result);
    }

    // No sandhi applies — simple concatenation
    Err(SandhiError::NoRuleApplies {
        first: first.to_string(),
        second: second.to_string(),
    })
}

/// Split a word at potential sandhi boundaries.
/// Returns the first valid decomposition found for each prefix pattern,
/// as (first, second, sandhi_result).
pub fn split(word: &str) -> Vec<(String, String, SandhiResult)> {
    let mut results = Vec::new();

    // Try known prefix splits
    let prefixes_to_try = [
        ("अति", "अत्य"),
        ("पुनः", "पुनर"),
        ("पुनः", "पुनः"),
        ("उत्", "उल्ल"),
        ("उत्", "उल्"),
        ("उत्", "उच्च"),
        ("उत्", "उच्"),
        ("सम्", "सं"),
        ("सम्", "सङ्"),
        ("स", "सा"), // दीर्घ sandhi: स + अ → सा
        ("प्र", "प्र"),
    ];

    for &(canonical, form) in &prefixes_to_try {
        if let Some(rest) = word.strip_prefix(form) {
            if !rest.is_empty() {
                results.extend(try_vowel_reconstructions(canonical, form, rest, word));
            }
        }
    }

    results
}

/// Reconstruct the second morpheme from a prefix split.
/// For sandhi where the initial vowel of the second morpheme was consumed
/// (e.g., यण्: इ+अ→य, विसर्ग: ः+अ→र), we try restoring different vowel starts.
fn reconstruct_second(canonical: &str, form: &str, rest: &str) -> String {
    match (canonical, form) {
        ("अति", "अत्य") => {
            // यण् sandhi consumed the initial vowel of second morpheme.
            // Most commonly अ (inherent vowel of य), but could be others.
            format!("अ{rest}")
        }
        ("पुनः", "पुनर") => {
            // Visarga → र before vowel (consumed अ)
            format!("अ{rest}")
        }
        ("स", "सा") => {
            // दीर्घ sandhi: अ + अ → आ
            format!("अ{rest}")
        }
        _ => rest.to_string(),
    }
}

/// Try vowel reconstructions for a sandhi split.
/// Returns the first valid (first, second, result) triple found.
fn try_vowel_reconstructions(
    canonical: &str,
    form: &str,
    rest: &str,
    word: &str,
) -> Vec<(String, String, SandhiResult)> {
    let mut results = Vec::new();

    // Primary reconstruction
    let second = reconstruct_second(canonical, form, rest);
    if let Ok(result) = apply(canonical, &second) {
        if result.output == word {
            results.push((canonical.to_string(), second, result));
            return results; // exact match, no need to try others
        }
    }

    // For patterns that consumed a vowel, try other vowel starts
    let needs_vowel_try = matches!(
        (canonical, form),
        ("अति", "अत्य") | ("पुनः", "पुनर") | ("स", "सा")
    );
    if needs_vowel_try {
        for vowel in ["आ", "इ", "ई", "उ", "ऊ", "ए", "ओ"] {
            let candidate = format!("{vowel}{rest}");
            if let Ok(result) = apply(canonical, &candidate) {
                if result.output == word {
                    results.push((canonical.to_string(), candidate, result));
                    break;
                }
            }
        }
    }

    results
}
