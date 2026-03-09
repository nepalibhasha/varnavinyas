use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::model::step::Step;
use varnavinyas_akshar::{is_matra, is_svar, is_vyanjan};

// Academy source context:
// docs/Notices-pages-77-99.md, Section 3.
//
// This is a compact derivational rule used by the checker for forms like
// "अर्थिक" -> "आर्थिक". It is not arranged as a numbered subsection family
// like 3(क)/(ख)/(ग)/(ङ), so the file stays intentionally small.

pub const SPEC_AADHI_VRIDDHI: RuleSpec = RuleSpec {
    id: "ortho-aadhi-vriddhi",
    category: RuleCategory::AadhiVriddhi,
    kind: DiagnosticKind::Error,
    priority: 340,
    citation: Rule::VarnaVinyasNiyam("3(क)"),
    examples: &[("अर्थिक", "आर्थिक"), ("इतिहासिक", "ऐतिहासिक")],
};

fn apply_vriddhi(chars: &[char]) -> Option<Vec<char>> {
    for (i, &c) in chars.iter().enumerate() {
        if is_svar(c) {
            let vriddhi = match c {
                'अ' => 'आ',
                'इ' | 'ई' => 'ऐ',
                'उ' | 'ऊ' => 'औ',
                'आ' | 'ऐ' | 'औ' => return None,
                _ => return None,
            };
            let mut result = chars.to_vec();
            result[i] = vriddhi;
            return Some(result);
        }
        if is_matra(c) {
            let vriddhi = match c {
                'ि' | 'ी' => 'ै',
                'ु' | 'ू' => 'ौ',
                'ा' | 'ै' | 'ौ' => return None,
                _ => return None,
            };
            let mut result = chars.to_vec();
            result[i] = vriddhi;
            return Some(result);
        }
        if is_vyanjan(c) {
            let next = chars.get(i + 1).copied();
            if next.is_some_and(is_matra) {
                continue;
            }
            if next == Some('्') {
                continue;
            }
            let mut result = chars.to_vec();
            result.insert(i + 1, 'ा');
            return Some(result);
        }
    }
    None
}

// आदिवृद्धि-based normalization for `-िक` derivatives when the base lexeme is attested.
pub fn rule_aadhi_vriddhi(input: &str) -> Option<Prakriya> {
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    if len < 3 {
        return None;
    }
    let has_ik = (chars[len - 2] == 'ि' || chars[len - 2] == 'इ') && chars[len - 1] == 'क';
    if !has_ik {
        return None;
    }
    let root: String = chars[..len - 2].iter().collect();
    if root.is_empty() {
        return None;
    }
    let kosha = varnavinyas_kosha::kosha();
    if !kosha.contains(&root) {
        return None;
    }
    let corrected_chars = apply_vriddhi(&chars)?;
    let output: String = corrected_chars.into_iter().collect();
    if output == input {
        return None;
    }
    Some(Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam("3(क)"),
            "इक प्रत्ययमा आदिवृद्धि: प्रथम स्वरमा वृद्धि हुन्छ",
            input,
            &output,
        )],
    ))
}
