use crate::rules::all_rules;
use crate::{SandhiError, SandhiResult};

/// Apply sandhi: combine two morphemes.
/// Tries forward rules in registry order.
pub fn apply(first: &str, second: &str) -> Result<SandhiResult, SandhiError> {
    if first.is_empty() || second.is_empty() {
        return Err(SandhiError::EmptyInput);
    }

    for rule in all_rules() {
        if rule.forward {
            if let Some(result) = (rule.apply)(rule, first, second) {
                return Ok(result);
            }
        }
    }

    Err(SandhiError::NoRuleApplies {
        first: first.to_string(),
        second: second.to_string(),
    })
}

/// Enumerate all distinct forward sandhi results that apply to this boundary.
pub fn apply_all(first: &str, second: &str) -> Vec<SandhiResult> {
    if first.is_empty() || second.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();

    for rule in all_rules() {
        if rule.forward {
            if let Some(result) = (rule.apply)(rule, first, second) {
                out.push(result);
            }
        }
    }

    out.dedup_by(|a, b| {
        a.output == b.output
            && a.sandhi_type == b.sandhi_type
            && a.rule_id == b.rule_id
            && a.rule_citation == b.rule_citation
    });
    out
}
