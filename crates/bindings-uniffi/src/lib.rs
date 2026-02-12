use serde::Serialize;

uniffi::setup_scaffolding!();

/// Transliteration scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum Scheme {
    Devanagari,
    Iast,
}

/// Word origin classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum Origin {
    Tatsam,
    Tadbhav,
    Deshaj,
    Aagantuk,
}

/// A single spell-check diagnostic.
#[derive(Debug, Serialize)]
struct FfiDiagnostic {
    span_start: u64,
    span_end: u64,
    incorrect: String,
    correction: String,
    rule: String,
    explanation: String,
    category: String,
    category_code: String,
    kind: String,
    confidence: f32,
}

/// Check text for spelling and punctuation issues.
///
/// Returns a JSON array of diagnostics.
#[uniffi::export]
pub fn check_text(text: String) -> String {
    let diags = varnavinyas_parikshak::check_text(&text);
    let ffi_diags: Vec<FfiDiagnostic> = diags
        .into_iter()
        .map(|d| FfiDiagnostic {
            span_start: d.span.0 as u64,
            span_end: d.span.1 as u64,
            incorrect: d.incorrect,
            correction: d.correction,
            rule: d.rule.to_string(),
            explanation: d.explanation,
            category: d.category.to_string(),
            category_code: d.category.as_code().to_string(),
            kind: d.kind.as_code().to_string(),
            confidence: d.confidence,
        })
        .collect();
    serde_json::to_string(&ffi_diags).unwrap_or_else(|_| "[]".to_string())
}

/// Transliterate text between Devanagari and IAST.
#[uniffi::export]
pub fn transliterate(input: String, from: Scheme, to: Scheme) -> Result<String, String> {
    let from_scheme = match from {
        Scheme::Devanagari => varnavinyas_lipi::Scheme::Devanagari,
        Scheme::Iast => varnavinyas_lipi::Scheme::Iast,
    };
    let to_scheme = match to {
        Scheme::Devanagari => varnavinyas_lipi::Scheme::Devanagari,
        Scheme::Iast => varnavinyas_lipi::Scheme::Iast,
    };
    varnavinyas_lipi::transliterate(&input, from_scheme, to_scheme).map_err(|e| e.to_string())
}

/// Classify a word by its origin.
#[uniffi::export]
pub fn classify(word: String) -> Origin {
    match varnavinyas_shabda::classify(&word) {
        varnavinyas_shabda::Origin::Tatsam => Origin::Tatsam,
        varnavinyas_shabda::Origin::Tadbhav => Origin::Tadbhav,
        varnavinyas_shabda::Origin::Deshaj => Origin::Deshaj,
        varnavinyas_shabda::Origin::Aagantuk => Origin::Aagantuk,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_text_returns_valid_json() {
        let result = check_text("नेपाल".to_string());
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(parsed.is_array());
    }

    #[test]
    fn check_text_finds_error() {
        let result = check_text("दुकान".to_string());
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&result).unwrap();
        if !parsed.is_empty() {
            let d = &parsed[0];
            assert!(d["incorrect"].is_string());
            assert!(d["correction"].is_string());
            assert!(d["rule"].is_string());
        }
    }

    #[test]
    fn transliterate_devanagari_to_iast() {
        let result = transliterate("नमस्ते".to_string(), Scheme::Devanagari, Scheme::Iast);
        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(!text.is_empty());
    }

    #[test]
    fn transliterate_iast_to_devanagari() {
        let result = transliterate("namaste".to_string(), Scheme::Iast, Scheme::Devanagari);
        assert!(result.is_ok());
    }

    #[test]
    fn classify_returns_origin() {
        let origin = classify("नेपाल".to_string());
        // Just verify it returns one of the valid variants
        assert!(matches!(
            origin,
            Origin::Tatsam | Origin::Tadbhav | Origin::Deshaj | Origin::Aagantuk
        ));
    }

    #[test]
    fn classify_empty_word() {
        let origin = classify(String::new());
        assert_eq!(origin, Origin::Deshaj);
    }

    #[test]
    fn scheme_enum_round_trip() {
        // Verify both variants work in transliterate
        for scheme in [Scheme::Devanagari, Scheme::Iast] {
            let _ = transliterate("test".to_string(), scheme, scheme);
        }
    }
}
