use serde::Serialize;
use wasm_bindgen::prelude::*;

/// A diagnostic serialized for JavaScript consumers.
#[derive(Serialize)]
struct JsDiagnostic {
    span_start: usize,
    span_end: usize,
    incorrect: String,
    correction: String,
    rule: String,
    explanation: String,
    /// Human-readable category label (Nepali display text).
    category: String,
    /// Stable machine-readable category code (Rust enum variant name).
    category_code: String,
}

/// A prakriya step serialized for JavaScript consumers.
#[derive(Serialize)]
struct JsStep {
    rule: String,
    description: String,
    before: String,
    after: String,
}

/// A prakriya result serialized for JavaScript consumers.
#[derive(Serialize)]
struct JsPrakriya {
    input: String,
    output: String,
    is_correct: bool,
    steps: Vec<JsStep>,
}

/// Check a full text for spelling and punctuation issues.
/// Returns a JSON string array of diagnostics.
#[wasm_bindgen]
pub fn check_text(text: &str) -> String {
    let diags = varnavinyas_parikshak::check_text(text);
    let js_diags: Vec<JsDiagnostic> = diags
        .into_iter()
        .map(|d| JsDiagnostic {
            span_start: d.span.0,
            span_end: d.span.1,
            incorrect: d.incorrect,
            correction: d.correction,
            rule: d.rule.to_string(),
            explanation: d.explanation,
            category: d.category.to_string(),
            category_code: d.category.as_code().to_string(),
        })
        .collect();
    serde_json::to_string(&js_diags).unwrap_or_else(|_| "[]".to_string())
}

/// Check a single word. Returns a JSON diagnostic or "null".
#[wasm_bindgen]
pub fn check_word(word: &str) -> String {
    match varnavinyas_parikshak::check_word(word) {
        Some(d) => {
            let js = JsDiagnostic {
                span_start: d.span.0,
                span_end: d.span.1,
                incorrect: d.incorrect,
                correction: d.correction,
                rule: d.rule.to_string(),
                explanation: d.explanation,
                category: d.category.to_string(),
                category_code: d.category.as_code().to_string(),
            };
            serde_json::to_string(&js).unwrap_or_else(|_| "null".to_string())
        }
        None => "null".to_string(),
    }
}

/// Transliterate text between scripts.
/// `from` and `to` must be "Devanagari" or "Iast".
#[wasm_bindgen]
pub fn transliterate(input: &str, from: &str, to: &str) -> Result<String, JsError> {
    let from_scheme = parse_scheme(from)?;
    let to_scheme = parse_scheme(to)?;
    varnavinyas_lipi::transliterate(input, from_scheme, to_scheme)
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Derive the correct form of a word with step tracing.
/// Returns a JSON object with input, output, is_correct, and steps.
#[wasm_bindgen]
pub fn derive(word: &str) -> String {
    let p = varnavinyas_prakriya::derive(word);
    let js = JsPrakriya {
        input: p.input,
        output: p.output,
        is_correct: p.is_correct,
        steps: p
            .steps
            .into_iter()
            .map(|s| JsStep {
                rule: s.rule.to_string(),
                description: s.description,
                before: s.before,
                after: s.after,
            })
            .collect(),
    };
    serde_json::to_string(&js).unwrap_or_else(|_| "{}".to_string())
}

fn parse_scheme(s: &str) -> Result<varnavinyas_lipi::Scheme, JsError> {
    match s {
        "Devanagari" | "devanagari" => Ok(varnavinyas_lipi::Scheme::Devanagari),
        "Iast" | "iast" | "IAST" => Ok(varnavinyas_lipi::Scheme::Iast),
        _ => Err(JsError::new(&format!(
            "Unknown scheme '{s}'. Use 'Devanagari' or 'Iast'."
        ))),
    }
}
