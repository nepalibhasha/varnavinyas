use crate::{SandhiResult, SandhiType};
use varnavinyas_akshar::{is_matra, is_svar};

/// Apply vowel sandhi at the boundary of two morphemes.
pub fn apply_vowel_sandhi(first: &str, second: &str) -> Option<SandhiResult> {
    let first_chars: Vec<char> = first.chars().collect();
    let second_chars: Vec<char> = second.chars().collect();

    if first_chars.is_empty() || second_chars.is_empty() {
        return None;
    }

    let last = *first_chars.last().unwrap();
    let first_of_second = second_chars[0];

    // दीर्घ sandhi: इ/ई + इ/ई → ई (same-vowel lengthening for i-class)
    // Must precede Yan sandhi to avoid इ+इ being treated as Yan.
    if matches!(last, 'इ' | 'ई' | 'ि' | 'ी') && matches!(first_of_second, 'इ' | 'ई') {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let rest: String = second_chars[1..].iter().collect();
        let result = if prefix.is_empty() || is_svar(prefix.chars().last().unwrap_or('\0')) {
            format!("{prefix}ई{rest}")
        } else {
            format!("{prefix}ी{rest}")
        };
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "दीर्घ सन्धि: इ/ई + इ/ई → ई",
        });
    }

    // दीर्घ sandhi: उ/ऊ + उ/ऊ → ऊ (same-vowel lengthening for u-class)
    // Must precede Yan sandhi to avoid उ+उ being treated as Yan.
    if matches!(last, 'उ' | 'ऊ' | 'ु' | 'ू') && matches!(first_of_second, 'उ' | 'ऊ') {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let rest: String = second_chars[1..].iter().collect();
        let result = if prefix.is_empty() || is_svar(prefix.chars().last().unwrap_or('\0')) {
            format!("{prefix}ऊ{rest}")
        } else {
            format!("{prefix}ू{rest}")
        };
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "दीर्घ सन्धि: उ/ऊ + उ/ऊ → ऊ",
        });
    }

    // यण् sandhi: इ/ई + vowel → य + (vowel as matra, or consumed if अ)
    if matches!(last, 'ि' | 'ी' | 'इ' | 'ई') && is_vowel_start(first_of_second) {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let second_remainder: String = if first_of_second == 'अ' {
            second_chars[1..].iter().collect()
        } else {
            second.to_string()
        };
        let ya_form = if is_matra(last) { "्य" } else { "य" };
        let result = format!("{prefix}{ya_form}{second_remainder}");
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "यण् सन्धि: इ/ई + स्वर → य",
        });
    }

    // यण् sandhi: उ/ऊ + vowel → व + vowel
    if matches!(last, 'ु' | 'ू' | 'उ' | 'ऊ') && is_vowel_start(first_of_second) {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let second_remainder: String = if first_of_second == 'अ' {
            second_chars[1..].iter().collect()
        } else {
            second.to_string()
        };
        let va_form = if is_matra(last) { "्व" } else { "व" };
        let result = format!("{prefix}{va_form}{second_remainder}");
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "यण् सन्धि: उ/ऊ + स्वर → व",
        });
    }

    // दीर्घ sandhi: अ/आ + अ/आ → आ
    if matches!(last, 'अ' | 'आ' | 'ा') && matches!(first_of_second, 'अ' | 'आ') {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let rest: String = second_chars[1..].iter().collect();
        let result = if prefix.is_empty() || is_svar(prefix.chars().last().unwrap_or('\0')) {
            format!("{prefix}आ{rest}")
        } else {
            format!("{prefix}ा{rest}")
        };
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "दीर्घ सन्धि: अ/आ + अ/आ → आ",
        });
    }

    // गुण sandhi: अ/आ + इ/ई → ए
    if matches!(last, 'अ' | 'आ' | 'ा') && matches!(first_of_second, 'इ' | 'ई') {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let rest: String = second_chars[1..].iter().collect();
        let result = if prefix.is_empty() {
            format!("ए{rest}")
        } else {
            format!("{prefix}े{rest}")
        };
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "गुण सन्धि: अ/आ + इ/ई → ए",
        });
    }

    // गुण sandhi: अ/आ + उ/ऊ → ओ
    if matches!(last, 'अ' | 'आ' | 'ा') && matches!(first_of_second, 'उ' | 'ऊ') {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let rest: String = second_chars[1..].iter().collect();
        let result = if prefix.is_empty() {
            format!("ओ{rest}")
        } else {
            format!("{prefix}ो{rest}")
        };
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "गुण सन्धि: अ/आ + उ/ऊ → ओ",
        });
    }

    // गुण sandhi: अ/आ + ऋ → अर्
    if matches!(last, 'अ' | 'आ' | 'ा') && first_of_second == 'ऋ' {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let rest: String = second_chars[1..].iter().collect();
        let result = if prefix.is_empty() {
            format!("अर्{rest}")
        } else {
            format!("{prefix}र्{rest}")
        };
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "गुण सन्धि: अ/आ + ऋ → अर्",
        });
    }

    // वृद्धि सन्धि: अ/आ + ए/ऐ → ऐ
    if matches!(last, 'अ' | 'आ' | 'ा') && matches!(first_of_second, 'ए' | 'ऐ') {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let rest: String = second_chars[1..].iter().collect();
        let result = if prefix.is_empty() {
            format!("ऐ{rest}")
        } else {
            format!("{prefix}ै{rest}")
        };
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "वृद्धि सन्धि: अ/आ + ए/ऐ → ऐ",
        });
    }

    // वृद्धि सन्धि: अ/आ + ओ/औ → औ
    if matches!(last, 'अ' | 'आ' | 'ा') && matches!(first_of_second, 'ओ' | 'औ') {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let rest: String = second_chars[1..].iter().collect();
        let result = if prefix.is_empty() {
            format!("औ{rest}")
        } else {
            format!("{prefix}ौ{rest}")
        };
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "वृद्धि सन्धि: अ/आ + ओ/औ → औ",
        });
    }

    // अयादि सन्धि: ए/े + vowel → अय + vowel
    if matches!(last, 'ए' | 'े') && is_vowel_start(first_of_second) {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let result = format!("{prefix}य{second}");
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "अयादि सन्धि: ए + स्वर → अय्",
        });
    }

    // अयादि सन्धि: ऐ/ै + vowel → आय + vowel
    if matches!(last, 'ऐ' | 'ै') && is_vowel_start(first_of_second) {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let result = format!("{prefix}ाय{second}");
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "अयादि सन्धि: ऐ + स्वर → आय्",
        });
    }

    // अयादि सन्धि: ओ/ो + vowel → अव + vowel
    if matches!(last, 'ओ' | 'ो') && is_vowel_start(first_of_second) {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let result = format!("{prefix}व{second}");
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "अयादि सन्धि: ओ + स्वर → अव्",
        });
    }

    // अयादि सन्धि: औ/ौ + vowel → आव + vowel
    if matches!(last, 'औ' | 'ौ') && is_vowel_start(first_of_second) {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        let result = format!("{prefix}ाव{second}");
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VowelSandhi,
            rule_citation: "अयादि सन्धि: औ + स्वर → आव्",
        });
    }

    None
}

fn is_vowel_start(c: char) -> bool {
    is_svar(c)
}
