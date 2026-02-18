use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

/// A diagnostic serialized for JavaScript consumers.
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
struct JsDiagnostic {
    span_start: usize,
    span_end: usize,
    incorrect: String,
    correction: String,
    rule: String,
    rule_code: String,
    explanation: String,
    /// Human-readable category label (Nepali display text).
    category: String,
    /// Stable machine-readable category code (Rust enum variant name).
    category_code: String,
    /// Severity kind: "Error", "Variant", or "Ambiguous".
    kind: String,
    /// Confidence score (0.0–1.0).
    confidence: f32,
}

/// A prakriya step serialized for JavaScript consumers.
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
struct JsStep {
    rule: String,
    description: String,
    before: String,
    after: String,
}

/// A prakriya result serialized for JavaScript consumers.
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
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
    check_text_with_options(text, false)
}

// Check full text with optional grammar-pass diagnostics.
#[wasm_bindgen]
pub fn check_text_with_options(text: &str, grammar: bool) -> String {
    let diags = varnavinyas_parikshak::check_text_with_options(
        text,
        varnavinyas_parikshak::CheckOptions {
            grammar,
            ..Default::default()
        },
    );
    let js_diags: Vec<JsDiagnostic> = diags.into_iter().map(diagnostic_to_js).collect();
    serde_json::to_string(&js_diags).unwrap_or_else(|_| "[]".to_string())
}

/// Check full text with optional grammar-pass diagnostics and return typed JsValue.
#[wasm_bindgen]
pub fn check_text_value(text: &str, grammar: bool) -> Result<JsValue, JsError> {
    let diags = varnavinyas_parikshak::check_text_with_options(
        text,
        varnavinyas_parikshak::CheckOptions {
            grammar,
            ..Default::default()
        },
    );
    let js_diags: Vec<JsDiagnostic> = diags.into_iter().map(diagnostic_to_js).collect();
    serde_wasm_bindgen::to_value(&js_diags)
        .map_err(|e| JsError::new(&format!("failed to serialize diagnostics: {e}")))
}

/// Check a single word. Returns a JSON diagnostic or "null".
#[wasm_bindgen]
pub fn check_word(word: &str) -> String {
    match varnavinyas_parikshak::check_word(word) {
        Some(d) => {
            let js = diagnostic_to_js(d);
            serde_json::to_string(&js).unwrap_or_else(|_| "null".to_string())
        }
        None => "null".to_string(),
    }
}

/// Check a single word and return typed JsValue (object or null).
#[wasm_bindgen]
pub fn check_word_value(word: &str) -> Result<JsValue, JsError> {
    match varnavinyas_parikshak::check_word(word) {
        Some(d) => serde_wasm_bindgen::to_value(&diagnostic_to_js(d))
            .map_err(|e| JsError::new(&format!("failed to serialize diagnostic: {e}"))),
        None => Ok(JsValue::NULL),
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
    let js = prakriya_to_js(varnavinyas_prakriya::derive(word));
    serde_json::to_string(&js).unwrap_or_else(|_| "{}".to_string())
}

/// Derive the correct form and return typed JsValue.
#[wasm_bindgen]
pub fn derive_value(word: &str) -> Result<JsValue, JsError> {
    let js = prakriya_to_js(varnavinyas_prakriya::derive(word));
    serde_wasm_bindgen::to_value(&js)
        .map_err(|e| JsError::new(&format!("failed to serialize prakriya: {e}")))
}

/// A word analysis result serialized for JavaScript consumers.
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
struct JsWordAnalysis {
    word: String,
    origin: String,
    origin_source: String,
    origin_confidence: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_language: Option<String>,
    is_correct: bool,
    correction: Option<String>,
    rule_notes: Vec<JsRuleNote>,
}

/// A rule note serialized for JavaScript consumers.
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
struct JsRuleNote {
    rule: String,
    rule_code: String,
    explanation: String,
}

/// Analyze a word: get origin classification, correction (if any), and explanatory rule notes.
/// Returns a JSON object with word, origin, is_correct, correction, and rule_notes.
#[wasm_bindgen]
pub fn analyze_word(word: &str) -> String {
    let js = word_analysis_to_js(varnavinyas_prakriya::analyze(word));
    serde_json::to_string(&js).unwrap_or_else(|_| "{}".to_string())
}

/// Analyze a word and return typed JsValue.
#[wasm_bindgen]
pub fn analyze_word_value(word: &str) -> Result<JsValue, JsError> {
    let js = word_analysis_to_js(varnavinyas_prakriya::analyze(word));
    serde_wasm_bindgen::to_value(&js)
        .map_err(|e| JsError::new(&format!("failed to serialize analysis: {e}")))
}

/// A morpheme decomposition result serialized for JavaScript consumers.
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
struct JsMorpheme {
    root: String,
    prefixes: Vec<String>,
    suffixes: Vec<String>,
    origin: String,
}

/// Decompose a word into root, prefixes, suffixes, and origin.
/// Returns a JSON object with root, prefixes, suffixes, and origin.
#[wasm_bindgen]
pub fn decompose_word(word: &str) -> String {
    let js = morpheme_to_js(varnavinyas_shabda::decompose(word));
    serde_json::to_string(&js).unwrap_or_else(|_| "{}".to_string())
}

/// Decompose a word and return typed JsValue.
#[wasm_bindgen]
pub fn decompose_word_value(word: &str) -> Result<JsValue, JsError> {
    let js = morpheme_to_js(varnavinyas_shabda::decompose(word));
    serde_wasm_bindgen::to_value(&js)
        .map_err(|e| JsError::new(&format!("failed to serialize morpheme: {e}")))
}

/// A sandhi apply result serialized for JavaScript consumers.
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
struct JsSandhiResult {
    output: String,
    sandhi_type: String,
    rule_citation: String,
}

/// A sandhi split entry serialized for JavaScript consumers.
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
struct JsSandhiSplit {
    left: String,
    right: String,
    output: String,
    sandhi_type: String,
    rule_citation: String,
}

/// Apply sandhi: join two morphemes.
/// Returns JSON: `{ output, sandhi_type, rule_citation }` or `{ "error": "..." }`.
#[wasm_bindgen]
pub fn sandhi_apply(first: &str, second: &str) -> String {
    match varnavinyas_sandhi::apply(first, second) {
        Ok(res) => {
            let js = sandhi_result_to_js(res);
            serde_json::to_string(&js).unwrap_or_else(|_| "{}".to_string())
        }
        Err(e) => serde_json::json!({ "error": e.to_string() }).to_string(),
    }
}

/// Apply sandhi and return typed JsValue.
#[wasm_bindgen]
pub fn sandhi_apply_value(first: &str, second: &str) -> Result<JsValue, JsError> {
    match varnavinyas_sandhi::apply(first, second) {
        Ok(res) => serde_wasm_bindgen::to_value(&sandhi_result_to_js(res))
            .map_err(|e| JsError::new(&format!("failed to serialize sandhi apply result: {e}"))),
        Err(e) => Err(JsError::new(&e.to_string())),
    }
}

/// Split a word at sandhi boundaries.
/// Returns JSON array: `[{ left, right, output, sandhi_type, rule_citation }, ...]`.
#[wasm_bindgen]
pub fn sandhi_split(word: &str) -> String {
    let results = varnavinyas_sandhi::split(word);
    let js_results: Vec<JsSandhiSplit> = results
        .into_iter()
        .map(|(left, right, res)| sandhi_split_to_js(left, right, res))
        .collect();
    serde_json::to_string(&js_results).unwrap_or_else(|_| "[]".to_string())
}

/// Split sandhi and return typed JsValue.
#[wasm_bindgen]
pub fn sandhi_split_value(word: &str) -> Result<JsValue, JsError> {
    let results = varnavinyas_sandhi::split(word);
    let js_results: Vec<JsSandhiSplit> = results
        .into_iter()
        .map(|(left, right, res)| sandhi_split_to_js(left, right, res))
        .collect();
    serde_wasm_bindgen::to_value(&js_results)
        .map_err(|e| JsError::new(&format!("failed to serialize sandhi split result: {e}")))
}

fn sandhi_result_to_js(res: varnavinyas_sandhi::SandhiResult) -> JsSandhiResult {
    JsSandhiResult {
        output: res.output,
        sandhi_type: sandhi_type_to_string(res.sandhi_type),
        rule_citation: res.rule_citation.to_string(),
    }
}

fn sandhi_split_to_js(
    left: String,
    right: String,
    res: varnavinyas_sandhi::SandhiResult,
) -> JsSandhiSplit {
    JsSandhiSplit {
        left,
        right,
        output: res.output,
        sandhi_type: sandhi_type_to_string(res.sandhi_type),
        rule_citation: res.rule_citation.to_string(),
    }
}

fn sandhi_type_to_string(st: varnavinyas_sandhi::SandhiType) -> String {
    match st {
        varnavinyas_sandhi::SandhiType::VowelSandhi => "VowelSandhi".into(),
        varnavinyas_sandhi::SandhiType::VisargaSandhi => "VisargaSandhi".into(),
        varnavinyas_sandhi::SandhiType::ConsonantSandhi => "ConsonantSandhi".into(),
    }
}

fn origin_to_string(origin: varnavinyas_shabda::Origin) -> String {
    match origin {
        varnavinyas_shabda::Origin::Tatsam => "tatsam".into(),
        varnavinyas_shabda::Origin::Tadbhav => "tadbhav".into(),
        varnavinyas_shabda::Origin::Deshaj => "deshaj".into(),
        varnavinyas_shabda::Origin::Aagantuk => "aagantuk".into(),
    }
}

fn origin_source_to_string(source: varnavinyas_shabda::OriginSource) -> String {
    match source {
        varnavinyas_shabda::OriginSource::Override => "override".into(),
        varnavinyas_shabda::OriginSource::Kosha => "kosha".into(),
        varnavinyas_shabda::OriginSource::Heuristic => "heuristic".into(),
    }
}

fn diagnostic_to_js(d: varnavinyas_parikshak::Diagnostic) -> JsDiagnostic {
    JsDiagnostic {
        span_start: d.span.0,
        span_end: d.span.1,
        incorrect: d.incorrect,
        correction: d.correction,
        rule: d.rule.to_string(),
        rule_code: d.rule.code().to_string(),
        explanation: d.explanation,
        category: d.category.to_string(),
        category_code: d.category.as_code().to_string(),
        kind: d.kind.as_code().to_string(),
        confidence: d.confidence,
    }
}

fn prakriya_to_js(p: varnavinyas_prakriya::Prakriya) -> JsPrakriya {
    JsPrakriya {
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
    }
}

fn word_analysis_to_js(analysis: varnavinyas_prakriya::WordAnalysis) -> JsWordAnalysis {
    JsWordAnalysis {
        word: analysis.word,
        origin: origin_to_string(analysis.origin),
        origin_source: origin_source_to_string(analysis.origin_source),
        origin_confidence: analysis.origin_confidence,
        source_language: analysis.source_language,
        is_correct: analysis.is_correct,
        correction: analysis.correction,
        rule_notes: analysis
            .rule_notes
            .into_iter()
            .map(|n| JsRuleNote {
                rule: n.rule.to_string(),
                rule_code: n.rule.code().to_string(),
                explanation: n.explanation,
            })
            .collect(),
    }
}

fn morpheme_to_js(m: varnavinyas_shabda::Morpheme) -> JsMorpheme {
    JsMorpheme {
        root: m.root,
        prefixes: m.prefixes,
        suffixes: m.suffixes,
        origin: origin_to_string(m.origin),
    }
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
