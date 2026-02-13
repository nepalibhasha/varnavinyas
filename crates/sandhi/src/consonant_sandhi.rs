use crate::{SandhiResult, SandhiType};

/// Apply consonant sandhi at the boundary of two morphemes.
pub fn apply_consonant_sandhi(first: &str, second: &str) -> Option<SandhiResult> {
    // Check known prefix assimilation patterns
    for &(prefix, second_start, merged, citation) in CONSONANT_ASSIMILATION_TABLE.iter() {
        if first == prefix {
            if let Some(rest) = second.strip_prefix(second_start) {
                let result = format!("{merged}{rest}");
                return Some(SandhiResult {
                    output: result,
                    sandhi_type: SandhiType::ConsonantSandhi,
                    rule_citation: citation,
                });
            }
        }
    }

    // General: halanta consonant + same consonant → gemination
    if first.ends_with('्') {
        let first_chars: Vec<char> = first.chars().collect();
        if first_chars.len() >= 2 {
            let base_consonant = first_chars[first_chars.len() - 2];
            let second_chars: Vec<char> = second.chars().collect();
            if !second_chars.is_empty() && second_chars[0] == base_consonant {
                // Same consonant: gemination (e.g., महत् + त्व → महत्त्व)
                let prefix: String = first_chars[..first_chars.len() - 2].iter().collect();
                let result = format!("{prefix}{base_consonant}्{second}");
                return Some(SandhiResult {
                    output: result,
                    sandhi_type: SandhiType::ConsonantSandhi,
                    rule_citation: "व्यञ्जन सन्धि: gemination (same consonant doubling)",
                });
            }

            // General nasalization: stop + nasal → panchham of stop's varga + nasal
            // e.g., वाक् + मय → वाङ्मय, षट् + मास → षण्मास
            // Must precede voicing rule: म is both voiced and nasal.
            if !second_chars.is_empty() && varnavinyas_akshar::is_panchham(second_chars[0]) {
                if let Some(v) = varnavinyas_akshar::varga(base_consonant) {
                    if let Some(nasal) = varnavinyas_akshar::panchham_of(v) {
                        let prefix: String = first_chars[..first_chars.len() - 2].iter().collect();
                        let result = format!("{prefix}{nasal}्{second}");
                        return Some(SandhiResult {
                            output: result,
                            sandhi_type: SandhiType::ConsonantSandhi,
                            rule_citation: "व्यञ्जन सन्धि: stop→nasal before nasal (panchham assimilation)",
                        });
                    }
                }
            }

            // General voicing: voiceless stop + voiced consonant → voiced counterpart
            // e.g., दिक् + गज → दिग्गज, वाक् + दान → वाग्दान
            if !second_chars.is_empty()
                && varnavinyas_akshar::is_voiceless(base_consonant)
                && varnavinyas_akshar::is_voiced(second_chars[0])
            {
                if let Some(voiced) = varnavinyas_akshar::voiced_counterpart(base_consonant) {
                    let prefix: String = first_chars[..first_chars.len() - 2].iter().collect();
                    let result = format!("{prefix}{voiced}्{second}");
                    return Some(SandhiResult {
                        output: result,
                        sandhi_type: SandhiType::ConsonantSandhi,
                        rule_citation: "व्यञ्जन सन्धि: voiceless→voiced before voiced consonant",
                    });
                }
            }
        }
    }

    None
}

/// Known consonant assimilation patterns.
/// (prefix, start_of_second, merged_form, citation)
static CONSONANT_ASSIMILATION_TABLE: &[(&str, &str, &str, &str)] = &[
    ("उत्", "ल", "उल्ल", "व्यञ्जन सन्धि: उत् + ल → उल्ल (assimilation)"),
    ("उत्", "च", "उच्च", "व्यञ्जन सन्धि: उत् + च → उच्च (assimilation)"),
    ("उत्", "न", "उन्न", "व्यञ्जन सन्धि: उत् + न → उन्न (assimilation)"),
    ("उत्", "स", "उत्स", "व्यञ्जन सन्धि: उत् + स → उत्स"),
    ("उत्", "थ", "उत्थ", "व्यञ्जन सन्धि: उत् + थ → उत्थ"),
    ("उत्", "प", "उत्प", "व्यञ्जन सन्धि: उत् + प → उत्प"),
    (
        "सम्",
        "क",
        "सङ्क",
        "व्यञ्जन सन्धि: सम् + क → सङ्क (panchham assimilation)",
    ),
    (
        "सम्",
        "ख",
        "सङ्ख",
        "व्यञ्जन सन्धि: सम् + ख → सङ्ख (panchham assimilation)",
    ),
    (
        "सम्",
        "ग",
        "सङ्ग",
        "व्यञ्जन सन्धि: सम् + ग → सङ्ग (panchham assimilation)",
    ),
    (
        "सम्",
        "घ",
        "सङ्घ",
        "व्यञ्जन सन्धि: सम् + घ → सङ्घ (panchham assimilation)",
    ),
    // निस् satva: स → श before palatal (च/छ)
    ("निस्", "च", "निश्च", "व्यञ्जन सन्धि: निस् + च → निश्च (satva)"),
    ("निस्", "छ", "निश्छ", "व्यञ्जन सन्धि: निस् + छ → निश्छ (satva)"),
    // दुस् satva: स → श before palatal (च/छ)
    ("दुस्", "च", "दुश्च", "व्यञ्जन सन्धि: दुस् + च → दुश्च (satva)"),
    ("दुस्", "छ", "दुश्छ", "व्यञ्जन सन्धि: दुस् + छ → दुश्छ (satva)"),
];
