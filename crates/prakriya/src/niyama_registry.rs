use crate::model::rule_spec::PatternRule;
use crate::usage_fixes;
use crate::varna_vinyasa;

/// Rules organized by orthography domain.
///
/// This is a registry-level organization layer over the current descriptive
/// module layout:
/// - `varna_vinyasa::*` for Academy orthography families
/// - `usage_fixes::*` for later cleanup-style rules
pub fn varna_vinyasa_rules() -> Vec<PatternRule> {
    varna_vinyasa::varna_vinyasa_rules()
}

pub fn usage_fix_rules() -> Vec<PatternRule> {
    usage_fixes::usage_fix_rules()
}
