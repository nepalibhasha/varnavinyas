use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::step::Step;
use varnavinyas_kosha::kosha;
use varnavinyas_shabda::{Origin, classify};

// Academy 3(ग)(ई): enforce तत्सम ऋ/कृ forms only when lexically plausible.
// -----------------------------------------------------------------------------
// 3(ग)(ई) 'ऋ' र 'रि' को प्रयोग
// -----------------------------------------------------------------------------
pub fn rule_ri_kri(input: &str) -> Option<Prakriya> {
    let origin = classify(input);
    if !matches!(origin, Origin::Tatsam) {
        return None;
    }
    let lex = kosha();
    if lex.contains(input) {
        return None;
    }

    if let Some(rest) = input.strip_prefix("रि") {
        if rest.starts_with('ष') || rest.starts_with('त') {
            let output = format!("ऋ{rest}");
            if lex.contains(&output) {
                return Some(Prakriya::corrected(
                    input,
                    &output,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam("3(ग)(ई)-ऋ-1"),
                        "तत्सम शब्दमा ऋ हुन्छ (रि होइन)",
                        input,
                        &output,
                    )],
                ));
            }
        }
    }
    if input.contains("क्रि") {
        let output = input.replace("क्रि", "कृ");
        if output != input && lex.contains(&output) {
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ग)(ई)-ऋ-1"),
                    "तत्सम शब्दमा कृ हुन्छ (क्रि होइन)",
                    input,
                    &output,
                )],
            ));
        }
    }
    None
}
