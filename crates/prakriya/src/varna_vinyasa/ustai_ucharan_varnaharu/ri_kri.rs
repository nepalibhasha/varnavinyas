use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::step::Step;
use varnavinyas_kosha::kosha;
use varnavinyas_shabda::{Origin, classify};

// Academy 3(ग)(ई)-ऋ-1: Sanskrit ऋ and consonant+ृ spellings.
// Validate the corrected lemma's origin; a misspelling may lack origin metadata.
pub fn rule_ri_kri(input: &str) -> Option<Prakriya> {
    let lex = kosha();
    if lex.is_rule_protected(input) {
        return None;
    }
    let mut candidates = Vec::new();
    if let Some(rest) = input.strip_prefix("रि") {
        candidates.push(format!("ऋ{rest}"));
    }
    for (start, matched) in input.match_indices("्रि") {
        if !input[..start]
            .chars()
            .next_back()
            .is_some_and(varnavinyas_akshar::is_vyanjan)
        {
            continue;
        }
        candidates.push(format!(
            "{}ृ{}",
            &input[..start],
            &input[start + matched.len()..]
        ));
    }
    for output in candidates {
        if lex.is_rule_protected(&output)
            && lex.is_correction_target(&output)
            && matches!(classify(&output), Origin::Tatsam)
        {
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ग)(ई)-ऋ-1"),
                    "तत्सम शब्दको मानक रूपमा ऋ वा व्यञ्जनसँग जोडिएको ृ प्रयोग हुन्छ (रि/्रि होइन)",
                    input,
                    &output,
                )],
            ));
        }
    }
    None
}
