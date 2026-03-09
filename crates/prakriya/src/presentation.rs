use serde::Serialize;
use varnavinyas_shabda::{Origin, OriginSource};

use crate::analysis::{RuleNote, WordAnalysis};

/// Stable serializable analysis shape for JSON/binding adapters.
#[derive(Debug, Clone, Serialize)]
pub struct ApiWordAnalysis {
    pub word: String,
    pub origin: String,
    pub origin_source: String,
    pub origin_confidence: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language: Option<String>,
    pub is_correct: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction: Option<String>,
    pub rule_notes: Vec<ApiRuleNote>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternate_rule_notes: Vec<ApiRuleNote>,
}

/// Stable serializable rule-note shape.
#[derive(Debug, Clone, Serialize)]
pub struct ApiRuleNote {
    pub rule: String,
    pub rule_code: String,
    pub explanation: String,
}

impl From<&RuleNote> for ApiRuleNote {
    fn from(note: &RuleNote) -> Self {
        Self {
            rule: note.rule.to_string(),
            rule_code: note.rule.code().to_string(),
            explanation: note.explanation.clone(),
        }
    }
}

impl From<RuleNote> for ApiRuleNote {
    fn from(note: RuleNote) -> Self {
        Self::from(&note)
    }
}

impl From<&WordAnalysis> for ApiWordAnalysis {
    fn from(analysis: &WordAnalysis) -> Self {
        Self {
            word: analysis.word.clone(),
            origin: origin_to_string(analysis.origin).to_string(),
            origin_source: origin_source_to_string(analysis.origin_source).to_string(),
            origin_confidence: analysis.origin_confidence,
            source_language: analysis.source_language.clone(),
            is_correct: analysis.is_correct,
            correction: analysis.correction.clone(),
            rule_notes: analysis.rule_notes.iter().map(ApiRuleNote::from).collect(),
            alternate_rule_notes: analysis
                .alternate_rule_notes
                .iter()
                .map(ApiRuleNote::from)
                .collect(),
        }
    }
}

impl From<WordAnalysis> for ApiWordAnalysis {
    fn from(analysis: WordAnalysis) -> Self {
        Self::from(&analysis)
    }
}

fn origin_to_string(origin: Origin) -> &'static str {
    match origin {
        Origin::Tatsam => "tatsam",
        Origin::Tadbhav => "tadbhav",
        Origin::Deshaj => "deshaj",
        Origin::Aagantuk => "aagantuk",
    }
}

fn origin_source_to_string(source: OriginSource) -> &'static str {
    match source {
        OriginSource::Override => "override",
        OriginSource::Kosha => "kosha",
        OriginSource::Heuristic => "heuristic",
    }
}
