use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

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
    check_text_with_all_options(text, grammar, "academy-strict")
}

/// Check full text with grammar and orthography policy options.
#[wasm_bindgen]
pub fn check_text_with_all_options(text: &str, grammar: bool, orthography_mode: &str) -> String {
    let orthography_mode = parse_orthography_mode(orthography_mode)
        .unwrap_or(varnavinyas_parikshak::OrthographyMode::AcademyStrict);
    let diags = varnavinyas_parikshak::check_text_with_options(
        text,
        varnavinyas_parikshak::CheckOptions {
            grammar,
            orthography_mode,
            ..Default::default()
        },
    );
    let api_diags: Vec<varnavinyas_parikshak::ApiDiagnostic> =
        diags.into_iter().map(Into::into).collect();
    serde_json::to_string(&api_diags).unwrap_or_else(|_| "[]".to_string())
}

/// Check full text with optional grammar-pass diagnostics and return typed JsValue.
#[wasm_bindgen]
pub fn check_text_value(text: &str, grammar: bool) -> Result<JsValue, JsError> {
    check_text_value_with_options(text, grammar, "academy-strict")
}

/// Check full text with grammar and orthography policy options and return typed JsValue.
#[wasm_bindgen]
pub fn check_text_value_with_options(
    text: &str,
    grammar: bool,
    orthography_mode: &str,
) -> Result<JsValue, JsError> {
    let orthography_mode = parse_orthography_mode(orthography_mode)?;
    let diags = varnavinyas_parikshak::check_text_with_options(
        text,
        varnavinyas_parikshak::CheckOptions {
            grammar,
            orthography_mode,
            ..Default::default()
        },
    );
    let api_diags: Vec<varnavinyas_parikshak::ApiDiagnostic> =
        diags.into_iter().map(Into::into).collect();
    serde_wasm_bindgen::to_value(&api_diags)
        .map_err(|e| JsError::new(&format!("failed to serialize diagnostics: {e}")))
}

fn parse_orthography_mode(mode: &str) -> Result<varnavinyas_parikshak::OrthographyMode, JsError> {
    match mode {
        "academy-strict" | "academy_strict" => {
            Ok(varnavinyas_parikshak::OrthographyMode::AcademyStrict)
        }
        "common-editorial" | "common_editorial" => {
            Ok(varnavinyas_parikshak::OrthographyMode::CommonEditorial)
        }
        other => Err(JsError::new(&format!("unknown orthography mode: {other}"))),
    }
}

/// Check a single word. Returns a JSON diagnostic or "null".
#[wasm_bindgen]
pub fn check_word(word: &str) -> String {
    match varnavinyas_parikshak::check_word(word) {
        Some(d) => {
            let api = varnavinyas_parikshak::ApiDiagnostic::from(d);
            serde_json::to_string(&api).unwrap_or_else(|_| "null".to_string())
        }
        None => "null".to_string(),
    }
}

/// Check a single word and return typed JsValue (object or null).
#[wasm_bindgen]
pub fn check_word_value(word: &str) -> Result<JsValue, JsError> {
    match varnavinyas_parikshak::check_word(word) {
        Some(d) => serde_wasm_bindgen::to_value(&varnavinyas_parikshak::ApiDiagnostic::from(d))
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

/// Analyze a word: get origin classification, correction (if any), and explanatory rule notes.
/// Returns a JSON object with word, origin, is_correct, correction, and rule_notes.
#[wasm_bindgen]
pub fn analyze_word(word: &str) -> String {
    let api = varnavinyas_prakriya::ApiWordAnalysis::from(varnavinyas_prakriya::analyze(word));
    serde_json::to_string(&api).unwrap_or_else(|_| "{}".to_string())
}

/// Analyze a word and return typed JsValue.
#[wasm_bindgen]
pub fn analyze_word_value(word: &str) -> Result<JsValue, JsError> {
    let api = varnavinyas_prakriya::ApiWordAnalysis::from(varnavinyas_prakriya::analyze(word));
    serde_wasm_bindgen::to_value(&api)
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

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
struct JsAffixSegment {
    text: String,
    kind: String,
}

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
struct JsAffixAnalysis {
    surface: String,
    stem: String,
    root: String,
    prefixes: Vec<String>,
    prefix_segments: Vec<JsAffixSegment>,
    suffixes: Vec<String>,
    suffix_segments: Vec<JsAffixSegment>,
    score: u16,
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

/// Collect conservative affix analyses for a word.
#[wasm_bindgen]
pub fn analyze_affixes(word: &str) -> String {
    let js: Vec<JsAffixAnalysis> = varnavinyas_shabda::analyze_affixes(word)
        .into_iter()
        .map(affix_analysis_to_js)
        .collect();
    serde_json::to_string(&js).unwrap_or_else(|_| "[]".to_string())
}

/// Collect affix analyses and return typed JsValue.
#[wasm_bindgen]
pub fn analyze_affixes_value(word: &str) -> Result<JsValue, JsError> {
    let js: Vec<JsAffixAnalysis> = varnavinyas_shabda::analyze_affixes(word)
        .into_iter()
        .map(affix_analysis_to_js)
        .collect();
    serde_wasm_bindgen::to_value(&js)
        .map_err(|e| JsError::new(&format!("failed to serialize affix analyses: {e}")))
}

/// Return the highest-ranked affix analysis as JSON or "null".
#[wasm_bindgen]
pub fn best_affix_analysis(word: &str) -> String {
    match varnavinyas_shabda::best_analysis(word) {
        Some(analysis) => serde_json::to_string(&affix_analysis_to_js(analysis))
            .unwrap_or_else(|_| "null".to_string()),
        None => "null".to_string(),
    }
}

/// Return the highest-ranked affix analysis or null.
#[wasm_bindgen]
pub fn best_affix_analysis_value(word: &str) -> Result<JsValue, JsError> {
    match varnavinyas_shabda::best_analysis(word) {
        Some(analysis) => serde_wasm_bindgen::to_value(&affix_analysis_to_js(analysis))
            .map_err(|e| JsError::new(&format!("failed to serialize affix analysis: {e}"))),
        None => Ok(JsValue::NULL),
    }
}

/// Return whether a word has any supported affix analysis.
#[wasm_bindgen]
pub fn has_supported_affix_analysis(word: &str) -> bool {
    varnavinyas_shabda::has_supported_analysis(word)
}

/// A samasa (compound) candidate serialized for JavaScript consumers.
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
struct JsSamasaCandidate {
    left: String,
    right: String,
    samasa_type: String,
    score: f32,
    vigraha: String,
}

/// Analyze a word as a potential compound (samasa).
/// Returns a JSON array of ranked candidates.
#[wasm_bindgen]
pub fn analyze_compound(word: &str) -> String {
    let js: Vec<JsSamasaCandidate> = varnavinyas_samasa::analyze_compound(word)
        .into_iter()
        .map(samasa_to_js)
        .collect();
    serde_json::to_string(&js).unwrap_or_else(|_| "[]".to_string())
}

/// Analyze compound and return typed JsValue.
#[wasm_bindgen]
pub fn analyze_compound_value(word: &str) -> Result<JsValue, JsError> {
    let js: Vec<JsSamasaCandidate> = varnavinyas_samasa::analyze_compound(word)
        .into_iter()
        .map(samasa_to_js)
        .collect();
    serde_wasm_bindgen::to_value(&js)
        .map_err(|e| JsError::new(&format!("failed to serialize compound analysis: {e}")))
}

fn samasa_to_js(c: varnavinyas_samasa::SamasaCandidate) -> JsSamasaCandidate {
    JsSamasaCandidate {
        left: c.left,
        right: c.right,
        samasa_type: samasa_type_to_string(c.samasa_type),
        score: c.score,
        vigraha: c.vigraha,
    }
}

fn samasa_type_to_string(t: varnavinyas_samasa::SamasaType) -> String {
    match t {
        varnavinyas_samasa::SamasaType::Tatpurusha => "तत्पुरुष".into(),
        varnavinyas_samasa::SamasaType::Karmadharaya => "कर्मधारय".into(),
        varnavinyas_samasa::SamasaType::Dvigu => "द्विगु".into(),
        varnavinyas_samasa::SamasaType::Bahuvrihi => "बहुव्रीहि".into(),
        varnavinyas_samasa::SamasaType::Dvandva => "द्वन्द्व".into(),
        varnavinyas_samasa::SamasaType::Avyayibhava => "अव्ययीभाव".into(),
        varnavinyas_samasa::SamasaType::Unknown => "अज्ञात".into(),
    }
}

/// A sandhi apply result serialized for JavaScript consumers.
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
struct JsSandhiResult {
    output: String,
    sandhi_type: String,
    family: String,
    rule_id: String,
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
    family: String,
    rule_id: String,
    rule_citation: String,
    authority: String,
    confidence: f32,
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
    let js_results: Vec<JsSandhiSplit> = results.into_iter().map(sandhi_split_to_js).collect();
    serde_json::to_string(&js_results).unwrap_or_else(|_| "[]".to_string())
}

/// Split sandhi and return typed JsValue.
#[wasm_bindgen]
pub fn sandhi_split_value(word: &str) -> Result<JsValue, JsError> {
    let results = varnavinyas_sandhi::split(word);
    let js_results: Vec<JsSandhiSplit> = results.into_iter().map(sandhi_split_to_js).collect();
    serde_wasm_bindgen::to_value(&js_results)
        .map_err(|e| JsError::new(&format!("failed to serialize sandhi split result: {e}")))
}

/// Return the best compound-oriented sandhi split candidate, if any.
#[wasm_bindgen]
pub fn sandhi_split_best_for_compound(word: &str) -> String {
    match varnavinyas_sandhi::split_best_for_compound(word) {
        Some(candidate) => serde_json::to_string(&sandhi_split_to_js(candidate))
            .unwrap_or_else(|_| "null".to_string()),
        None => "null".to_string(),
    }
}

/// Return the best compound-oriented sandhi split candidate as typed JsValue.
#[wasm_bindgen]
pub fn sandhi_split_best_for_compound_value(word: &str) -> Result<JsValue, JsError> {
    match varnavinyas_sandhi::split_best_for_compound(word) {
        Some(candidate) => {
            serde_wasm_bindgen::to_value(&sandhi_split_to_js(candidate)).map_err(|e| {
                JsError::new(&format!(
                    "failed to serialize best sandhi split result: {e}"
                ))
            })
        }
        None => Ok(JsValue::NULL),
    }
}

fn sandhi_result_to_js(res: varnavinyas_sandhi::SandhiResult) -> JsSandhiResult {
    JsSandhiResult {
        output: res.output,
        sandhi_type: res.sandhi_type.display_label().to_string(),
        family: rule_family_to_string(res.family).to_string(),
        rule_id: res.rule_id.to_string(),
        rule_citation: res.rule_citation.to_string(),
    }
}

fn sandhi_split_to_js(candidate: varnavinyas_sandhi::SandhiCandidate) -> JsSandhiSplit {
    JsSandhiSplit {
        left: candidate.left,
        right: candidate.right,
        output: candidate.surface,
        sandhi_type: candidate.sandhi_type.display_label().to_string(),
        family: rule_family_to_string(candidate.family).to_string(),
        rule_id: candidate.rule_id.to_string(),
        rule_citation: candidate.rule_citation.to_string(),
        authority: authority_tier_to_string(candidate.authority).to_string(),
        confidence: candidate.confidence,
    }
}

fn authority_tier_to_string(tier: varnavinyas_sandhi::AuthorityTier) -> &'static str {
    match tier {
        varnavinyas_sandhi::AuthorityTier::Authoritative => "Authoritative",
        varnavinyas_sandhi::AuthorityTier::Likely => "Likely",
        varnavinyas_sandhi::AuthorityTier::Plausible => "Plausible",
        varnavinyas_sandhi::AuthorityTier::Exploratory => "Exploratory",
    }
}

fn rule_family_to_string(family: varnavinyas_sandhi::RuleFamily) -> &'static str {
    match family {
        varnavinyas_sandhi::RuleFamily::DirectJoin => "DirectJoin",
        varnavinyas_sandhi::RuleFamily::VowelGuna => "VowelGuna",
        varnavinyas_sandhi::RuleFamily::VowelVriddhi => "VowelVriddhi",
        varnavinyas_sandhi::RuleFamily::Yan => "Yan",
        varnavinyas_sandhi::RuleFamily::Ayadi => "Ayadi",
        varnavinyas_sandhi::RuleFamily::VisargaR => "VisargaR",
        varnavinyas_sandhi::RuleFamily::VisargaSibilant => "VisargaSibilant",
        varnavinyas_sandhi::RuleFamily::ConsonantAssimilation => "ConsonantAssimilation",
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

fn origin_to_string(origin: varnavinyas_shabda::Origin) -> String {
    match origin {
        varnavinyas_shabda::Origin::Tatsam => "tatsam".into(),
        varnavinyas_shabda::Origin::Tadbhav => "tadbhav".into(),
        varnavinyas_shabda::Origin::Deshaj => "deshaj".into(),
        varnavinyas_shabda::Origin::Aagantuk => "aagantuk".into(),
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

fn affix_kind_to_string(kind: varnavinyas_shabda::AffixKind) -> String {
    match kind {
        varnavinyas_shabda::AffixKind::Prefix => "prefix".into(),
        varnavinyas_shabda::AffixKind::PluralMarker => "plural_marker".into(),
        varnavinyas_shabda::AffixKind::CaseMarker => "case_marker".into(),
        varnavinyas_shabda::AffixKind::Particle => "particle".into(),
    }
}

fn affix_segment_to_js(segment: varnavinyas_shabda::AffixSegment) -> JsAffixSegment {
    JsAffixSegment {
        text: segment.text,
        kind: affix_kind_to_string(segment.kind),
    }
}

fn affix_analysis_to_js(analysis: varnavinyas_shabda::AffixAnalysis) -> JsAffixAnalysis {
    JsAffixAnalysis {
        surface: analysis.surface,
        stem: analysis.stem,
        root: analysis.root,
        prefixes: analysis.prefixes,
        prefix_segments: analysis
            .prefix_segments
            .into_iter()
            .map(affix_segment_to_js)
            .collect(),
        suffixes: analysis.suffixes,
        suffix_segments: analysis
            .suffix_segments
            .into_iter()
            .map(affix_segment_to_js)
            .collect(),
        score: analysis.score,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandhi_type_labels_are_devanagari() {
        assert_eq!(
            varnavinyas_sandhi::SandhiType::VowelSandhi
                .display_label()
                .to_string(),
            "स्वर सन्धि"
        );
        assert_eq!(
            varnavinyas_sandhi::SandhiType::VisargaSandhi
                .display_label()
                .to_string(),
            "विसर्ग सन्धि"
        );
        assert_eq!(
            varnavinyas_sandhi::SandhiType::ConsonantSandhi
                .display_label()
                .to_string(),
            "व्यञ्जन सन्धि"
        );
    }

    #[test]
    fn samasa_type_labels_are_stable() {
        assert_eq!(
            samasa_type_to_string(varnavinyas_samasa::SamasaType::Tatpurusha),
            "तत्पुरुष"
        );
        assert_eq!(
            samasa_type_to_string(varnavinyas_samasa::SamasaType::Karmadharaya),
            "कर्मधारय"
        );
        assert_eq!(
            samasa_type_to_string(varnavinyas_samasa::SamasaType::Dvigu),
            "द्विगु"
        );
        assert_eq!(
            samasa_type_to_string(varnavinyas_samasa::SamasaType::Bahuvrihi),
            "बहुव्रीहि"
        );
        assert_eq!(
            samasa_type_to_string(varnavinyas_samasa::SamasaType::Dvandva),
            "द्वन्द्व"
        );
        assert_eq!(
            samasa_type_to_string(varnavinyas_samasa::SamasaType::Avyayibhava),
            "अव्ययीभाव"
        );
        assert_eq!(
            samasa_type_to_string(varnavinyas_samasa::SamasaType::Unknown),
            "अज्ञात"
        );
    }

    #[test]
    fn analyze_compound_returns_expected_json_fields() {
        let json = analyze_compound("सूर्योदय");
        let parsed: serde_json::Value =
            serde_json::from_str(&json).expect("compound analysis must return valid JSON");
        let arr = parsed
            .as_array()
            .expect("compound analysis payload must be an array");

        assert!(!arr.is_empty(), "expected at least one compound candidate");

        let first = &arr[0];
        assert!(
            first
                .get("left")
                .and_then(serde_json::Value::as_str)
                .is_some(),
            "candidate must include string field 'left'"
        );
        assert!(
            first
                .get("right")
                .and_then(serde_json::Value::as_str)
                .is_some(),
            "candidate must include string field 'right'"
        );
        assert!(
            first
                .get("samasa_type")
                .and_then(serde_json::Value::as_str)
                .is_some(),
            "candidate must include string field 'samasa_type'"
        );
        assert!(
            first
                .get("score")
                .and_then(serde_json::Value::as_f64)
                .is_some(),
            "candidate must include numeric field 'score'"
        );
        assert!(
            first
                .get("vigraha")
                .and_then(serde_json::Value::as_str)
                .is_some(),
            "candidate must include string field 'vigraha'"
        );
    }

    #[test]
    fn check_word_exposes_alternate_reasons_for_multi_hit_word() {
        let json = check_word("भौतीक");
        let parsed: serde_json::Value =
            serde_json::from_str(&json).expect("check_word must return valid JSON");
        let alternates = parsed
            .get("alternate_reasons")
            .and_then(serde_json::Value::as_array)
            .expect("multi-hit word should include alternate_reasons");
        assert!(!alternates.is_empty());
    }

    #[test]
    fn best_affix_analysis_returns_typed_segments() {
        let json = best_affix_analysis("रामसम्मपनि");
        let parsed: serde_json::Value =
            serde_json::from_str(&json).expect("affix analysis must return valid JSON");
        assert_eq!(parsed.get("stem").and_then(|v| v.as_str()), Some("राम"));
        assert_eq!(
            parsed
                .get("suffixes")
                .and_then(|v| v.as_array())
                .map(|arr| arr.len()),
            Some(2)
        );
        assert_eq!(
            parsed["suffix_segments"][0]["kind"].as_str(),
            Some("case_marker")
        );
        assert_eq!(
            parsed["suffix_segments"][1]["kind"].as_str(),
            Some("particle")
        );
    }
}
