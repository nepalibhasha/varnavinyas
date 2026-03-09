mod ajanta;
mod halanta;

// Academy source:
// docs/Notices-pages-77-99.md
// 3(ङ) हलन्त र अजन्त प्रयोगसम्बन्धी नियम
//
// Organization note:
// - keep halanta-required subrules and ajanta-required subrules in separate files
// - keep the exported orchestrator (`rule_halanta`) here

pub use halanta::SPEC_HALANTA;

pub fn rule_halanta(input: &str) -> Option<crate::prakriya::Prakriya> {
    if input.is_empty() {
        return None;
    }
    if let Some(p) = halanta::rule_halanta_required(input) {
        return Some(p);
    }
    if let Some(p) = ajanta::rule_ajanta_required(input) {
        return Some(p);
    }
    None
}
