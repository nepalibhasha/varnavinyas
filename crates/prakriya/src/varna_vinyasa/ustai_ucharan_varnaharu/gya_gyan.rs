use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::step::Step;
use varnavinyas_kosha::kosha;

// Academy 3(ग)(ऊ): prefer ज्ञ-series only when candidate lemma is attested.
// -----------------------------------------------------------------------------
// 3(ग)(ऊ) 'ज्ञ', 'ग्या', 'ग्याँ' को प्रयोग
// -----------------------------------------------------------------------------
pub fn rule_gya_gyan(input: &str) -> Option<Prakriya> {
    let kosha = kosha();
    if kosha.contains(input) {
        return None;
    }
    if !input.contains("ज्ञ") && !input.contains("ग्या") && !input.contains("ग्याँ")
    {
        return None;
    }
    const SUBS: &[(&str, &str)] = &[("ग्याँ", "ज्ञा"), ("ग्या", "ज्ञा")];
    for &(from, to) in SUBS {
        if input.contains(from) {
            let candidate = input.replace(from, to);
            if candidate != input && kosha.contains(&candidate) {
                let citation = if from == "ग्याँ" {
                    "3(ग)(ऊ)-2"
                } else {
                    "3(ग)(ऊ)-3"
                };
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
