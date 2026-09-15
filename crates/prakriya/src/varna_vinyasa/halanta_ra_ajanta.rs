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

// Academy 3(ङ)-1 gives root notation, not a context-free spelling correction.
const DHATU_FORMS: &[&str] = &["पढ", "भन", "उठ", "डुल", "हिँड", "हेर", "देख"];

pub(crate) fn context_dependent_verb_note(word: &str) -> Option<crate::Explanation> {
    let stem = word.strip_suffix('्').unwrap_or(word);
    if DHATU_FORMS.contains(&stem) || ajanta::is_imperative_form(word) {
        Some(crate::Explanation::new(
            crate::Rule::VarnaVinyasNiyam("3(ङ)-1, 3(ङ)-अजन्त-3"),
            "धातुरूपमा हलन्त र सामान्य आदरार्थी आज्ञार्थ क्रियामा अजन्त लेखिन्छ; व्याकरणिक प्रयोग नखुल्दा यी रूपलाई स्वतः सच्याइँदैन",
        ))
    } else {
        None
    }
}

pub fn rule_halanta(input: &str) -> Option<crate::model::prakriya::Prakriya> {
    if input.is_empty() || context_dependent_verb_note(input).is_some() {
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
