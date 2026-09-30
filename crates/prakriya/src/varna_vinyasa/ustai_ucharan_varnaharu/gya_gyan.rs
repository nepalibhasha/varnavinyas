use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::step::Step;
use varnavinyas_kosha::kosha;
use varnavinyas_shabda::{Origin, classify};

// Academy 3(ग)(ऊ): prefer ज्ञ-series only when candidate lemma is attested.
// -----------------------------------------------------------------------------
// 3(ग)(ऊ) 'ज्ञ', 'ग्या', 'ग्याँ' को प्रयोग
// -----------------------------------------------------------------------------
pub fn rule_gya_gyan(input: &str) -> Option<Prakriya> {
    let kosha = kosha();
    if kosha.is_rule_protected(input) {
        return None;
    }
    if !input.contains("ग्य") {
        return None;
    }
    const SUBS: &[(&str, &str)] = &[("ग्याँ", "ज्ञा"), ("ग्या", "ज्ञा"), ("ग्य", "ज्ञ")];
    for &(from, to) in SUBS {
        if input.contains(from) {
            let candidate = input.replace(from, to);
            if candidate != input
                && kosha.is_correction_target(&candidate)
                && kosha.is_rule_protected(&candidate)
                && matches!(classify(&candidate), Origin::Tatsam)
            {
                // Subrule 1 describes Sanskrit ज्ञ. Subrules 2/3 preserve
                // native ग्याँ and loanword ग्या; they cannot cite this rewrite.
                let citation = "3(ग)(ऊ)-1";
                return Some(Prakriya::corrected(
                    input,
                    &candidate,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam(citation),
                        format!("ज्ञ/ग्याँ/ग्या भेद: {} → {}", from, to),
                        input,
                        &candidate,
                    )],
                ));
            }
        }
    }
    None
}
