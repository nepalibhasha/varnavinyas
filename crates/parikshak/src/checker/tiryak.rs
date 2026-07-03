use std::collections::HashSet;

use varnavinyas_kosha::kosha;
use varnavinyas_prakriya::{DiagnosticKind, Rule};
use varnavinyas_shabda::has_supported_analysis;

use crate::diagnostic::{Diagnostic, DiagnosticCategory};
use crate::tokenizer::{AnalyzedToken, tokenize_analyzed};

use super::common::overlaps_existing_span;

const RULE_CODE_KA: &str = "PS-Saisanik-7(क)-तिर्यक्";
const RULE_CODE_KHA: &str = "PS-Saisanik-7(ख)-तिर्यक्";
const RULE_CODE_GA: &str = "PS-Saisanik-7(ग)-तिर्यक्";

const TIRYAK_CASE_SUFFIXES: &[&str] = &["ले", "मा", "लाई", "बाट", "देखि", "सँग", "का", "की", "को"];

const SUBRULE_KHA_PRONOUN_STEMS: &[(&str, &str)] = &[
    ("यो", "यस"),
    ("त्यो", "त्यस"),
    ("ऊ", "उस"),
    ("यी", "यिन"),
    ("ती", "तिन"),
];

const SUBRULE_KHA_SURFACE_CORRECTIONS: &[(&str, &str)] =
    &[("मले", "मैले"), ("तँले", "तैँले"), ("तैले", "तैँले")];

const SUBRULE_KHA_SPLIT_SURFACE_CORRECTIONS: &[(&str, &str, &str)] =
    &[("म", "ले", "मैले"), ("तँ", "ले", "तैँले"), ("तै", "ले", "तैँले")];

const SUBRULE_GA_DETERMINERS: &[(&str, &str)] = &[
    ("यो", "यस"),
    ("त्यो", "त्यस"),
    ("मेरो", "मेरा"),
    ("हाम्रो", "हाम्रा"),
    ("तेरो", "तेरा"),
    ("तिम्रो", "तिम्रा"),
    ("उसको", "उसका"),
    ("उनको", "उनका"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TiryakSubrule {
    Ka,
    Kha,
    Ga,
}

impl TiryakSubrule {
    fn rule(self) -> Rule {
        match self {
            Self::Ka => Rule::Vyakaran(RULE_CODE_KA),
            Self::Kha => Rule::Vyakaran(RULE_CODE_KHA),
            Self::Ga => Rule::Vyakaran(RULE_CODE_GA),
        }
    }

    fn explanation(self) -> &'static str {
        match self {
            Self::Ka => {
                "शैक्षणिक व्याकरण ७(क): एको, नु अन्त्यमा आउने कृदन्त शब्दका पछाडि ले, मा विभक्ति लाग्दा तिर्यक् रूपको प्रयोग हुन्छ ।"
            }
            Self::Kha => "शैक्षणिक व्याकरण ७(ख): विभक्तियुक्त सर्वनाम तिर्यक् रूपमा प्रयुक्त हुन्छन् ।",
            Self::Ga => {
                "शैक्षणिक व्याकरण ७(ग): विशेषण (भेदक) का रूपमा सर्वनाम आउँदा विशेष्यमा विभक्ति भए विशेषण (सर्वनाम) पनि तिर्यक् रूपमा लेखिन्छन् ।"
            }
        }
    }

    fn confidence(self) -> f32 {
        match self {
            Self::Ka => 0.94,
            Self::Kha => 0.93,
            Self::Ga => 0.9,
        }
    }
}

#[derive(Debug, Clone)]
struct TiryakCandidate {
    span: (usize, usize),
    incorrect: String,
    correction: String,
    subrule: TiryakSubrule,
}

impl TiryakCandidate {
    fn into_diagnostic(self) -> Diagnostic {
        Diagnostic {
            span: self.span,
            incorrect: self.incorrect,
            correction: self.correction,
            rule: self.subrule.rule(),
            explanation: self.subrule.explanation().to_string(),
            category: DiagnosticCategory::ShuddhaTable,
            kind: DiagnosticKind::Error,
            confidence: self.subrule.confidence(),
            alternate_reasons: Vec::new(),
        }
    }
}

pub(crate) fn check_word_tiryak(word: &str) -> Option<Diagnostic> {
    let token = tokenize_analyzed(word).into_iter().next()?;
    if token.start != 0 || token.end != word.len() {
        return None;
    }

    analyze_joined_token(word, &token).map(TiryakCandidate::into_diagnostic)
}

pub(crate) fn add_tiryak_diagnostics(
    text: &str,
    tokens: &[AnalyzedToken],
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for token in tokens {
        let Some(candidate) = analyze_joined_token(text, token) else {
            continue;
        };
        let span = candidate.span;
        if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
            continue;
        }
        diagnostics.push(candidate.into_diagnostic());
        blocked_spans.insert(span);
    }

    for window in tokens.windows(2) {
        if let Some(candidate) = analyze_split_pair(text, &window[0], &window[1]) {
            let span = candidate.span;
            if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
                continue;
            }
            diagnostics.push(candidate.into_diagnostic());
            blocked_spans.insert(span);
            continue;
        }

        if let Some(candidate) = analyze_determiner_context(text, &window[0], &window[1]) {
            let span = candidate.span;
            if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
                continue;
            }
            diagnostics.push(candidate.into_diagnostic());
            blocked_spans.insert(span);
        }
    }
}

fn analyze_joined_token(text: &str, token: &AnalyzedToken) -> Option<TiryakCandidate> {
    let surface = token.surface();
    let (subrule, correction) = classify_joined_surface(surface.as_ref())?;
    Some(TiryakCandidate {
        span: (token.start, token.end),
        incorrect: text[token.start..token.end].to_string(),
        correction,
        subrule,
    })
}

fn analyze_split_pair(
    text: &str,
    left: &AnalyzedToken,
    right: &AnalyzedToken,
) -> Option<TiryakCandidate> {
    let left_surface = left.surface();
    let right_surface = right.surface();
    let (subrule, correction) = classify_split_pair(left_surface.as_ref(), right_surface.as_ref())?;
    Some(TiryakCandidate {
        span: (left.start, right.end),
        incorrect: text[left.start..right.end].to_string(),
        correction,
        subrule,
    })
}

fn analyze_determiner_context(
    text: &str,
    left: &AnalyzedToken,
    right: &AnalyzedToken,
) -> Option<TiryakCandidate> {
    if !is_inflected_nounish_token(right) {
        return None;
    }

    let left_surface = left.surface();
    let correction = SUBRULE_GA_DETERMINERS
        .iter()
        .find_map(|(bare, oblique)| (*bare == left_surface.as_ref()).then_some(*oblique))?;

    if correction == left_surface.as_ref() {
        return None;
    }

    Some(TiryakCandidate {
        span: (left.start, left.end),
        incorrect: text[left.start..left.end].to_string(),
        correction: correction.to_string(),
        subrule: TiryakSubrule::Ga,
    })
}

fn classify_joined_surface(surface: &str) -> Option<(TiryakSubrule, String)> {
    if let Some(correction) = subrule_kha_surface_correction(surface) {
        return Some((TiryakSubrule::Kha, correction));
    }

    if let Some(correction) = subrule_ka_joined_correction(surface) {
        return Some((TiryakSubrule::Ka, correction));
    }

    if let Some(correction) = subrule_kha_joined_correction(surface) {
        return Some((TiryakSubrule::Kha, correction));
    }

    None
}

fn classify_split_pair(left_surface: &str, right_surface: &str) -> Option<(TiryakSubrule, String)> {
    if let Some(correction) = subrule_kha_split_surface_correction(left_surface, right_surface) {
        return Some((TiryakSubrule::Kha, correction));
    }

    if let Some(correction) = subrule_ka_pair_correction(left_surface, right_surface) {
        return Some((TiryakSubrule::Ka, correction));
    }

    if let Some(correction) = subrule_kha_pair_correction(left_surface, right_surface) {
        return Some((TiryakSubrule::Kha, correction));
    }

    None
}

fn subrule_kha_split_surface_correction(left_surface: &str, right_surface: &str) -> Option<String> {
    SUBRULE_KHA_SPLIT_SURFACE_CORRECTIONS.iter().find_map(
        |(incorrect_left, incorrect_right, correct)| {
            (*incorrect_left == left_surface && *incorrect_right == right_surface)
                .then_some((*correct).to_string())
        },
    )
}

fn subrule_ka_pair_correction(left_surface: &str, right_surface: &str) -> Option<String> {
    subrule_ka_stem_and_suffix(left_surface, right_surface)
}

fn subrule_ka_joined_correction(surface: &str) -> Option<String> {
    for suffix in ["ले", "मा"] {
        if let Some(stem) = surface.strip_suffix(suffix) {
            if let Some(correction) = subrule_ka_stem_and_suffix(stem, suffix) {
                return Some(correction);
            }
        }
    }
    None
}

fn subrule_ka_stem_and_suffix(stem: &str, suffix: &str) -> Option<String> {
    if !matches!(suffix, "ले" | "मा") {
        return None;
    }

    if let Some(base) = stem.strip_suffix("ो") {
        if stem.ends_with("एको") {
            return Some(format!("{base}ा{suffix}"));
        }
    }

    if let Some(base) = stem.strip_suffix("ु") {
        if stem.ends_with("नु") {
            return Some(format!("{base}ा{suffix}"));
        }
    }

    None
}

fn subrule_kha_pair_correction(left_surface: &str, right_surface: &str) -> Option<String> {
    subrule_kha_stem_and_suffix(left_surface, right_surface)
}

fn subrule_kha_joined_correction(surface: &str) -> Option<String> {
    for &suffix in TIRYAK_CASE_SUFFIXES {
        if let Some(stem) = surface.strip_suffix(suffix) {
            if let Some(correction) = subrule_kha_stem_and_suffix(stem, suffix) {
                return Some(correction);
            }
        }
    }
    None
}

fn subrule_kha_stem_and_suffix(stem: &str, suffix: &str) -> Option<String> {
    if !TIRYAK_CASE_SUFFIXES.contains(&suffix) {
        return None;
    }

    let mapped = SUBRULE_KHA_PRONOUN_STEMS
        .iter()
        .find_map(|(bare, oblique)| (*bare == stem).then_some(*oblique))?;
    let candidate = format!("{mapped}{suffix}");
    if candidate_is_supported(&candidate) {
        Some(candidate)
    } else {
        None
    }
}

fn subrule_kha_surface_correction(surface: &str) -> Option<String> {
    SUBRULE_KHA_SURFACE_CORRECTIONS
        .iter()
        .find_map(|(incorrect, correct)| (*incorrect == surface).then_some((*correct).to_string()))
}

fn candidate_is_supported(candidate: &str) -> bool {
    let lex = kosha();
    lex.contains(candidate) || lex.lookup(candidate).is_some() || has_supported_analysis(candidate)
}

fn is_inflected_nounish_token(token: &AnalyzedToken) -> bool {
    if token.suffix.is_none() {
        return false;
    }

    let surface = token.surface();
    let lex = kosha();

    let stem_entry = lex.lookup(&token.stem);
    let full_entry = lex.lookup(surface.as_ref());
    let stem_pos = stem_entry.map(|entry| entry.pos);
    let full_pos = full_entry.map(|entry| entry.pos);

    is_nounish_pos(stem_pos)
        || is_nounish_pos(full_pos)
        || lex.contains(&token.stem)
        || lex.contains(&surface)
}

fn is_nounish_pos(pos: Option<&str>) -> bool {
    let Some(pos) = pos else {
        return false;
    };

    pos.contains("नाम") || pos.contains("ना.") || pos.contains("सर्व")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::tokenize_analyzed;

    #[test]
    fn joined_participle_tiryak_correction_applies() {
        let token = tokenize_analyzed("भएकोमा").into_iter().next().unwrap();
        let analysis = analyze_joined_token("भएकोमा", &token).unwrap();
        assert_eq!(analysis.subrule, TiryakSubrule::Ka);
        assert_eq!(analysis.correction, "भएकामा");
    }

    #[test]
    fn infinitive_tiryak_correction_applies() {
        let token = tokenize_analyzed("गर्नुले").into_iter().next().unwrap();
        let analysis = analyze_joined_token("गर्नुले", &token).unwrap();
        assert_eq!(analysis.subrule, TiryakSubrule::Ka);
        assert_eq!(analysis.correction, "गर्नाले");
    }

    #[test]
    fn direct_pronoun_case_tiryak_correction_applies() {
        let token = tokenize_analyzed("योले").into_iter().next().unwrap();
        let analysis = analyze_joined_token("योले", &token).unwrap();
        assert_eq!(analysis.subrule, TiryakSubrule::Kha);
        assert_eq!(analysis.correction, "यसले");
    }

    #[test]
    fn expanded_pronoun_surface_correction_applies() {
        let token = tokenize_analyzed("मले").into_iter().next().unwrap();
        let analysis = analyze_joined_token("मले", &token).unwrap();
        assert_eq!(analysis.subrule, TiryakSubrule::Kha);
        assert_eq!(analysis.correction, "मैले");
    }

    #[test]
    fn split_pronoun_surface_correction_applies() {
        let text = "म ले";
        let tokens = tokenize_analyzed(text);
        let analysis = analyze_split_pair(text, &tokens[0], &tokens[1]).unwrap();
        assert_eq!(analysis.subrule, TiryakSubrule::Kha);
        assert_eq!(analysis.correction, "मैले");
    }

    #[test]
    fn determiner_oblique_correction_requires_nounish_inflected_head() {
        let text = "यो प्रसारणका";
        let tokens = tokenize_analyzed(text);
        let analysis = analyze_determiner_context(text, &tokens[0], &tokens[1]).unwrap();
        assert_eq!(analysis.subrule, TiryakSubrule::Ga);
        assert_eq!(analysis.incorrect, "यो");
        assert_eq!(analysis.correction, "यस");
    }

    #[test]
    fn bare_next_word_does_not_trigger_subrule_ga() {
        let text = "यो प्रसारण";
        let tokens = tokenize_analyzed(text);
        assert!(analyze_determiner_context(text, &tokens[0], &tokens[1]).is_none());
    }
}
