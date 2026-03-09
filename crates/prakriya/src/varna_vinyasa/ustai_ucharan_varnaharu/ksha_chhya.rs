use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::step::Step;
use varnavinyas_kosha::kosha;

// Academy 3(ग)(उ): canonicalize क्ष/छ/च्छ variants using known lexical targets.
// -----------------------------------------------------------------------------
// 3(ग)(उ) 'क्ष/क्षे/क्ष्य' र 'छ/छे/छ्य' को प्रयोग
// -----------------------------------------------------------------------------
pub fn rule_ksha_chhya(input: &str) -> Option<Prakriya> {
    let kosha = kosha();
    if kosha.contains(input) {
        return None;
    }
    if !input.contains("क्ष") && !input.contains('छ') && !input.contains("च्छ") {
        return None;
    }
    const SUBS: &[(&str, &str)] = &[
        ("छ्य", "क्ष्य"),
        ("क्ष्य", "छ्य"),
        ("छे", "क्षे"),
        ("क्षे", "छे"),
        ("क्ष", "च्छ"),
        ("च्छ", "क्ष"),
        ("छ", "क्ष"),
        ("क्ष", "छ"),
    ];
    for &(from, to) in SUBS {
        if input.contains(from) {
            let candidate = input.replace(from, to);
            if kosha.contains(&candidate) {
                let citation = if to.contains("क्ष") {
                    "3(ग)(उ)-क्ष-1"
                } else if to == "छे" || from == "क्षे" {
                    "3(ग)(उ)-छे-2"
                } else {
                    "3(ग)(उ)-छ्य-3"
                };
                return Some(Prakriya::corrected(
                    input,
                    &candidate,
                    vec![Step::new(
                        Rule::VarnaVinyasNiyam(citation),
                        format!("क्ष/छ भेद: {} → {}", from, to),
                        input,
                        &candidate,
                    )],
                ));
            }
        }
    }
    None
}
