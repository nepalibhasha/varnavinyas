use varnavinyas_akshar::{
    is_matra, is_panchham, is_svar, is_voiced, is_voiceless, is_vyanjan, panchham_of,
    svar_to_matra, varga, voiced_counterpart,
};
use varnavinyas_kosha::Kosha;
use varnavinyas_kosha::origin_tag::OriginTag;

use crate::{RuleFamily, SandhiCandidate, SandhiResult, SandhiType};

pub struct SandhiRule {
    pub apply: fn(&SandhiRule, &str, &str) -> Option<SandhiResult>,
    pub forward: bool,
    pub reverse: bool,
    pub id: &'static str,
    pub reverse_apply: fn(&SandhiRule, &str, &str, &str, &Kosha, &mut Vec<SandhiCandidate>),
}

struct ReverseSuffixSpec {
    strip_suffix: &'static str,
    replacements: &'static [&'static str],
}

struct ReverseMatraSpec {
    matra: char,
    vowels: &'static [&'static str],
}

struct ReverseExpansionSpec {
    left_suffixes: &'static [&'static str],
    right_prefixes: &'static [&'static str],
}

struct ReverseRightRewriteSpec {
    strip_prefix: &'static str,
    replacement_prefix: &'static str,
}

const VOWELS: &[&str] = &["अ", "आ", "इ", "ई", "उ", "ऊ", "ए", "ऐ", "ओ", "औ", "ऋ"];
const RECONSTRUCTION_LEFT_SUFFIXES: &[&str] = &["", "ा", "ः"];
const EXACT_LEFT_SUFFIXES: &[&str] = &[""];
const DIRECT_JOIN_UPASARGAS: &[&str] = &["प्र", "वि"];
const SIBILANT_RULES: &[(char, &[char])] =
    &[('श', &['च', 'छ']), ('ष', &['ट', 'ठ']), ('स', &['त', 'थ'])];
const CONSONANT_ASSIMILATION_TABLE: &[(&str, &str, &str, &str)] = &[
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
    ("निस्", "च", "निश्च", "व्यञ्जन सन्धि: निस् + च → निश्च (satva)"),
    ("निस्", "छ", "निश्छ", "व्यञ्जन सन्धि: निस् + छ → निश्छ (satva)"),
    ("दुस्", "च", "दुश्च", "व्यञ्जन सन्धि: दुस् + च → दुश्च (satva)"),
    ("दुस्", "छ", "दुश्छ", "व्यञ्जन सन्धि: दुस् + छ → दुश्छ (satva)"),
];
const YAN_RULES: &[ReverseSuffixSpec] = &[
    ReverseSuffixSpec {
        strip_suffix: "्य",
        replacements: &["ि", "ी"],
    },
    ReverseSuffixSpec {
        strip_suffix: "्व",
        replacements: &["ु", "ू"],
    },
];
const AYADI_RULES: &[ReverseSuffixSpec] = &[
    ReverseSuffixSpec {
        strip_suffix: "ाय",
        replacements: &["ै", "ऐ"],
    },
    ReverseSuffixSpec {
        strip_suffix: "य",
        replacements: &["े", "ए"],
    },
    ReverseSuffixSpec {
        strip_suffix: "ाव",
        replacements: &["ौ", "औ"],
    },
    ReverseSuffixSpec {
        strip_suffix: "व",
        replacements: &["ो", "ओ"],
    },
];
const MATRA_REVERSE_RULES: &[ReverseMatraSpec] = &[
    ReverseMatraSpec {
        matra: 'ा',
        vowels: &["अ", "आ"],
    },
    ReverseMatraSpec {
        matra: 'े',
        vowels: &["इ", "ई"],
    },
    ReverseMatraSpec {
        matra: 'ो',
        vowels: &["उ", "ऊ"],
    },
    ReverseMatraSpec {
        matra: 'ै',
        vowels: &["ए", "ऐ"],
    },
    ReverseMatraSpec {
        matra: 'ौ',
        vowels: &["ओ", "औ"],
    },
];
const DIRECT_VERIFIED_SPEC: ReverseExpansionSpec = ReverseExpansionSpec {
    left_suffixes: EXACT_LEFT_SUFFIXES,
    right_prefixes: &[""],
};
const VOWEL_RECONSTRUCTION_SPEC: ReverseExpansionSpec = ReverseExpansionSpec {
    left_suffixes: RECONSTRUCTION_LEFT_SUFFIXES,
    right_prefixes: VOWELS,
};
const VISARGA_R_REWRITE_SPECS: &[ReverseRightRewriteSpec] = &[
    ReverseRightRewriteSpec {
        strip_prefix: "र",
        replacement_prefix: "अ",
    },
    ReverseRightRewriteSpec {
        strip_prefix: "र्",
        replacement_prefix: "",
    },
];

pub fn all_rules() -> &'static [SandhiRule] {
    static RULES: [SandhiRule; 32] = [
        SandhiRule {
            apply: match_visarga_rule,
            forward: true,
            reverse: false,
            id: "visarga-sh",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_visarga_rule,
            forward: true,
            reverse: false,
            id: "visarga-ssha",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_visarga_rule,
            forward: true,
            reverse: false,
            id: "visarga-s",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_visarga_rule,
            forward: true,
            reverse: false,
            id: "visarga-retained",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_visarga_rule,
            forward: true,
            reverse: false,
            id: "visarga-r-vowel",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_visarga_rule,
            forward: true,
            reverse: false,
            id: "visarga-o-ghosha",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_visarga_rule,
            forward: true,
            reverse: false,
            id: "visarga-r-voiced",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_consonant_rule,
            forward: true,
            reverse: false,
            id: "consonant-prefix-assimilation",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_consonant_rule,
            forward: true,
            reverse: false,
            id: "consonant-gemination",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_consonant_rule,
            forward: true,
            reverse: false,
            id: "consonant-panchham",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_consonant_rule,
            forward: true,
            reverse: false,
            id: "consonant-voicing",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-dirgha-i",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-dirgha-u",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-yan-i",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-yan-u",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-dirgha-a",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-guna-e",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-guna-o",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-guna-r",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-vriddhi-ai",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-vriddhi-au",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-ayadi-e",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-ayadi-ai",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-ayadi-o",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: match_vowel_rule,
            forward: true,
            reverse: false,
            id: "vowel-ayadi-au",
            reverse_apply: noop_reverse,
        },
        SandhiRule {
            apply: no_forward,
            forward: false,
            reverse: true,
            id: "direct-join-upasarga",
            reverse_apply: reverse_direct_join,
        },
        SandhiRule {
            apply: no_forward,
            forward: false,
            reverse: true,
            id: "direct-verified",
            reverse_apply: reverse_direct_verified,
        },
        SandhiRule {
            apply: no_forward,
            forward: false,
            reverse: true,
            id: "vowel-reconstruction",
            reverse_apply: reverse_vowel_reconstruction,
        },
        SandhiRule {
            apply: no_forward,
            forward: false,
            reverse: true,
            id: "yan-reconstruction",
            reverse_apply: reverse_yan,
        },
        SandhiRule {
            apply: no_forward,
            forward: false,
            reverse: true,
            id: "visarga-r",
            reverse_apply: reverse_visarga_r,
        },
        SandhiRule {
            apply: no_forward,
            forward: false,
            reverse: true,
            id: "visarga-sibilant",
            reverse_apply: reverse_visarga_sibilant,
        },
        SandhiRule {
            apply: no_forward,
            forward: false,
            reverse: true,
            id: "ayadi-and-matra",
            reverse_apply: reverse_ayadi_and_matra,
        },
    ];
    &RULES
}

fn no_forward(_rule: &SandhiRule, _first: &str, _second: &str) -> Option<SandhiResult> {
    None
}

fn match_visarga_rule(rule: &SandhiRule, first: &str, second: &str) -> Option<SandhiResult> {
    let prefix = first.strip_suffix('ः')?;
    if prefix.is_empty() {
        return None;
    }

    let second_chars: Vec<char> = second.chars().collect();
    let first_of_second = *second_chars.first()?;

    match rule.id {
        "visarga-sh" if matches!(first_of_second, 'च' | 'छ') => Some(visarga_result(
            format!("{prefix}श्{second}"),
            RuleFamily::VisargaSibilant,
            rule.id,
            "विसर्ग सन्धि: ः → श् before palatal (च/छ)",
        )),
        "visarga-ssha" if matches!(first_of_second, 'ट' | 'ठ') => Some(visarga_result(
            format!("{prefix}ष्{second}"),
            RuleFamily::VisargaSibilant,
            rule.id,
            "विसर्ग सन्धि: ः → ष् before retroflex (ट/ठ)",
        )),
        "visarga-s" if matches!(first_of_second, 'त' | 'थ') => Some(visarga_result(
            format!("{prefix}स्{second}"),
            RuleFamily::VisargaSibilant,
            rule.id,
            "विसर्ग सन्धि: ः → स् before dental (त/थ)",
        )),
        "visarga-retained"
            if matches!(first_of_second, 'स' | 'श' | 'ष' | 'क' | 'ख' | 'प' | 'फ') =>
        {
            Some(visarga_result(
                format!("{first}{second}"),
                RuleFamily::VisargaSibilant,
                rule.id,
                "विसर्ग सन्धि: विसर्ग retained before स/श/ष/guttural/labial stops",
            ))
        }
        "visarga-r-vowel" if is_svar(first_of_second) => {
            let second_remainder: String = second_chars[1..].iter().collect();
            let ra_form = if first_of_second == 'अ' {
                "र".to_string()
            } else {
                let matra = svar_to_matra(first_of_second).unwrap_or(first_of_second);
                format!("र{matra}")
            };
            Some(visarga_result(
                format!("{prefix}{ra_form}{second_remainder}"),
                RuleFamily::VisargaR,
                rule.id,
                "विसर्ग सन्धि: विसर्ग → र before vowel",
            ))
        }
        "visarga-o-ghosha" | "visarga-r-voiced" if is_voiced_consonant(first_of_second) => {
            let prefix_chars: Vec<char> = prefix.chars().collect();
            let last_char = *prefix_chars.last()?;
            let is_implicit_a = !is_matra(last_char) && !is_svar(last_char) && last_char != '्';
            let is_punah_antah = prefix == "पुन" || prefix == "अन्त";
            if rule.id == "visarga-o-ghosha" {
                if is_implicit_a && !is_punah_antah {
                    Some(visarga_result(
                        format!("{prefix}ो{second}"),
                        RuleFamily::VisargaR,
                        rule.id,
                        "विसर्ग सन्धि: अः + घोष वर्ण → ओ",
                    ))
                } else {
                    None
                }
            } else {
                Some(visarga_result(
                    format!("{prefix}र{second}"),
                    RuleFamily::VisargaR,
                    rule.id,
                    "विसर्ग सन्धि: विसर्ग → र before voiced consonant",
                ))
            }
        }
        _ => None,
    }
}

fn match_consonant_rule(rule: &SandhiRule, first: &str, second: &str) -> Option<SandhiResult> {
    match rule.id {
        "consonant-prefix-assimilation" => {
            for &(prefix, second_start, merged, citation) in CONSONANT_ASSIMILATION_TABLE {
                if first == prefix {
                    if let Some(rest) = second.strip_prefix(second_start) {
                        return Some(consonant_result(
                            format!("{merged}{rest}"),
                            rule.id,
                            citation,
                        ));
                    }
                }
            }
            None
        }
        "consonant-gemination" | "consonant-panchham" | "consonant-voicing" => {
            if !first.ends_with('्') {
                return None;
            }
            let first_chars: Vec<char> = first.chars().collect();
            if first_chars.len() < 2 {
                return None;
            }
            let base_consonant = first_chars[first_chars.len() - 2];
            let second_chars: Vec<char> = second.chars().collect();
            let first_of_second = *second_chars.first()?;
            let prefix: String = first_chars[..first_chars.len() - 2].iter().collect();

            match rule.id {
                "consonant-gemination" if first_of_second == base_consonant => {
                    Some(consonant_result(
                        format!("{prefix}{base_consonant}्{second}"),
                        rule.id,
                        "व्यञ्जन सन्धि: gemination (same consonant doubling)",
                    ))
                }
                "consonant-panchham" if is_panchham(first_of_second) => {
                    let nasal = panchham_of(varga(base_consonant)?)?;
                    Some(consonant_result(
                        format!("{prefix}{nasal}्{second}"),
                        rule.id,
                        "व्यञ्जन सन्धि: stop→nasal before nasal (panchham assimilation)",
                    ))
                }
                "consonant-voicing"
                    if is_voiceless(base_consonant) && is_voiced(first_of_second) =>
                {
                    let voiced = voiced_counterpart(base_consonant)?;
                    Some(consonant_result(
                        format!("{prefix}{voiced}्{second}"),
                        rule.id,
                        "व्यञ्जन सन्धि: voiceless→voiced before voiced consonant",
                    ))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn match_vowel_rule(rule: &SandhiRule, first: &str, second: &str) -> Option<SandhiResult> {
    let first_chars: Vec<char> = first.chars().collect();
    let second_chars: Vec<char> = second.chars().collect();
    if first_chars.is_empty() || second_chars.is_empty() {
        return None;
    }

    let last_char = *first_chars.last()?;
    let (last, inherent) = if is_vyanjan(last_char) {
        ('अ', true)
    } else {
        (last_char, false)
    };
    let first_of_second = second_chars[0];
    let rest: String = second_chars[1..].iter().collect();

    match rule.id {
        "vowel-dirgha-i"
            if matches!(last, 'इ' | 'ई' | 'ि' | 'ी') && matches!(first_of_second, 'इ' | 'ई') =>
        {
            let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
            let result = if prefix.is_empty() || is_svar(prefix.chars().last().unwrap_or('\0')) {
                format!("{prefix}ई{rest}")
            } else {
                format!("{prefix}ी{rest}")
            };
            Some(vowel_result(
                result,
                RuleFamily::VowelGuna,
                rule.id,
                "दीर्घ सन्धि: इ/ई + इ/ई → ई",
            ))
        }
        "vowel-dirgha-u"
            if matches!(last, 'उ' | 'ऊ' | 'ु' | 'ू') && matches!(first_of_second, 'उ' | 'ऊ') =>
        {
            let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
            let result = if prefix.is_empty() || is_svar(prefix.chars().last().unwrap_or('\0')) {
                format!("{prefix}ऊ{rest}")
            } else {
                format!("{prefix}ू{rest}")
            };
            Some(vowel_result(
                result,
                RuleFamily::VowelGuna,
                rule.id,
                "दीर्घ सन्धि: उ/ऊ + उ/ऊ → ऊ",
            ))
        }
        "vowel-yan-i" if matches!(last, 'ि' | 'ी' | 'इ' | 'ई') && is_svar(first_of_second) =>
        {
            let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
            let second_remainder: String = if first_of_second == 'अ' {
                second_chars[1..].iter().collect()
            } else {
                second.to_string()
            };
            let ya_form = if is_matra(last) { "्य" } else { "य" };
            Some(vowel_result(
                format!("{prefix}{ya_form}{second_remainder}"),
                RuleFamily::Yan,
                rule.id,
                "यण् सन्धि: इ/ई + स्वर → य",
            ))
        }
        "vowel-yan-u" if matches!(last, 'ु' | 'ू' | 'उ' | 'ऊ') && is_svar(first_of_second) =>
        {
            let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
            let second_remainder: String = if first_of_second == 'अ' {
                second_chars[1..].iter().collect()
            } else {
                second.to_string()
            };
            let va_form = if is_matra(last) { "्व" } else { "व" };
            Some(vowel_result(
                format!("{prefix}{va_form}{second_remainder}"),
                RuleFamily::Yan,
                rule.id,
                "यण् सन्धि: उ/ऊ + स्वर → व",
            ))
        }
        "vowel-dirgha-a"
            if matches!(last, 'अ' | 'आ' | 'ा') && matches!(first_of_second, 'अ' | 'आ') =>
        {
            Some(vowel_result(
                emit_a_sandhi(first, &first_chars, inherent, &rest, "आ", "ा"),
                RuleFamily::VowelGuna,
                rule.id,
                "दीर्घ सन्धि: अ/आ + अ/आ → आ",
            ))
        }
        "vowel-guna-e"
            if matches!(last, 'अ' | 'आ' | 'ा') && matches!(first_of_second, 'इ' | 'ई') =>
        {
            Some(vowel_result(
                emit_a_sandhi(first, &first_chars, inherent, &rest, "ए", "े"),
                RuleFamily::VowelGuna,
                rule.id,
                "गुण सन्धि: अ/आ + इ/ई → ए",
            ))
        }
        "vowel-guna-o"
            if matches!(last, 'अ' | 'आ' | 'ा') && matches!(first_of_second, 'उ' | 'ऊ') =>
        {
            Some(vowel_result(
                emit_a_sandhi(first, &first_chars, inherent, &rest, "ओ", "ो"),
                RuleFamily::VowelGuna,
                rule.id,
                "गुण सन्धि: अ/आ + उ/ऊ → ओ",
            ))
        }
        "vowel-guna-r" if matches!(last, 'अ' | 'आ' | 'ा') && first_of_second == 'ऋ' => {
            Some(vowel_result(
                emit_a_sandhi(first, &first_chars, inherent, &rest, "अर्", "र्"),
                RuleFamily::VowelGuna,
                rule.id,
                "गुण सन्धि: अ/आ + ऋ → अर्",
            ))
        }
        "vowel-vriddhi-ai"
            if matches!(last, 'अ' | 'आ' | 'ा') && matches!(first_of_second, 'ए' | 'ऐ') =>
        {
            Some(vowel_result(
                emit_a_sandhi(first, &first_chars, inherent, &rest, "ऐ", "ै"),
                RuleFamily::VowelVriddhi,
                rule.id,
                "वृद्धि सन्धि: अ/आ + ए/ऐ → ऐ",
            ))
        }
        "vowel-vriddhi-au"
            if matches!(last, 'अ' | 'आ' | 'ा') && matches!(first_of_second, 'ओ' | 'औ') =>
        {
            Some(vowel_result(
                emit_a_sandhi(first, &first_chars, inherent, &rest, "औ", "ौ"),
                RuleFamily::VowelVriddhi,
                rule.id,
                "वृद्धि सन्धि: अ/आ + ओ/औ → औ",
            ))
        }
        "vowel-ayadi-e" if matches!(last, 'ए' | 'े') && is_svar(first_of_second) => {
            let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
            Some(vowel_result(
                format!("{prefix}य{second}"),
                RuleFamily::Ayadi,
                rule.id,
                "अयादि सन्धि: ए + स्वर → अय्",
            ))
        }
        "vowel-ayadi-ai" if matches!(last, 'ऐ' | 'ै') && is_svar(first_of_second) => {
            let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
            Some(vowel_result(
                format!("{prefix}ाय{second}"),
                RuleFamily::Ayadi,
                rule.id,
                "अयादि सन्धि: ऐ + स्वर → आय्",
            ))
        }
        "vowel-ayadi-o" if matches!(last, 'ओ' | 'ो') && is_svar(first_of_second) => {
            let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
            Some(vowel_result(
                format!("{prefix}व{second}"),
                RuleFamily::Ayadi,
                rule.id,
                "अयादि सन्धि: ओ + स्वर → अव्",
            ))
        }
        "vowel-ayadi-au" if matches!(last, 'औ' | 'ौ') && is_svar(first_of_second) => {
            let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
            Some(vowel_result(
                format!("{prefix}ाव{second}"),
                RuleFamily::Ayadi,
                rule.id,
                "अयादि सन्धि: औ + स्वर → आव्",
            ))
        }
        _ => None,
    }
}

fn emit_a_sandhi(
    first: &str,
    first_chars: &[char],
    inherent: bool,
    rest: &str,
    full_vowel: &str,
    matra: &str,
) -> String {
    if inherent {
        format!("{first}{matra}{rest}")
    } else {
        let prefix: String = first_chars[..first_chars.len() - 1].iter().collect();
        if prefix.is_empty() {
            format!("{full_vowel}{rest}")
        } else {
            format!("{prefix}{matra}{rest}")
        }
    }
}

fn vowel_result(
    output: String,
    family: RuleFamily,
    rule_id: &'static str,
    rule_citation: &'static str,
) -> SandhiResult {
    SandhiResult::new(
        output,
        SandhiType::VowelSandhi,
        family,
        rule_id,
        rule_citation,
    )
}

fn visarga_result(
    output: String,
    family: RuleFamily,
    rule_id: &'static str,
    rule_citation: &'static str,
) -> SandhiResult {
    SandhiResult::new(
        output,
        SandhiType::VisargaSandhi,
        family,
        rule_id,
        rule_citation,
    )
}

fn consonant_result(
    output: String,
    rule_id: &'static str,
    rule_citation: &'static str,
) -> SandhiResult {
    SandhiResult::new(
        output,
        SandhiType::ConsonantSandhi,
        RuleFamily::ConsonantAssimilation,
        rule_id,
        rule_citation,
    )
}

fn is_voiced_consonant(c: char) -> bool {
    matches!(
        c,
        'ग' | 'घ'
            | 'ङ'
            | 'ज'
            | 'झ'
            | 'ञ'
            | 'ड'
            | 'ढ'
            | 'ण'
            | 'द'
            | 'ध'
            | 'न'
            | 'ब'
            | 'भ'
            | 'म'
            | 'य'
            | 'र'
            | 'ल'
            | 'व'
            | 'ह'
    )
}

fn noop_reverse(
    _rule: &SandhiRule,
    _surface: &str,
    _raw_left: &str,
    _raw_right: &str,
    _lex: &Kosha,
    _out: &mut Vec<SandhiCandidate>,
) {
}

fn lexical_status(word: &str, lex: &Kosha, allow_bound_form: bool) -> crate::LexicalStatus {
    if lex.lookup(word).is_some() {
        crate::LexicalStatus::KnownHeadword
    } else if allow_bound_form {
        crate::LexicalStatus::KnownBoundForm
    } else if lex.contains(word) {
        crate::LexicalStatus::KnownSurface
    } else {
        crate::LexicalStatus::Unknown
    }
}

pub fn apply_reverse_rule(
    rule: &SandhiRule,
    surface: &str,
    raw_left: &str,
    raw_right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
) {
    (rule.reverse_apply)(rule, surface, raw_left, raw_right, lex, out);
}

pub fn build_candidate(
    surface: &str,
    left: String,
    right: String,
    result: SandhiResult,
    lex: &Kosha,
) -> SandhiCandidate {
    let left_bound = crate::decode::is_known_one_akshara_upasarga(&left);
    SandhiCandidate {
        surface: surface.to_string(),
        left: left.clone(),
        right: right.clone(),
        sandhi_type: result.sandhi_type,
        family: result.family,
        rule_id: result.rule_id,
        rule_citation: result.rule_citation,
        forward_verified: true,
        lexical_left: lexical_status(&left, lex, left_bound),
        lexical_right: lexical_status(&right, lex, false),
        authority: crate::AuthorityTier::Exploratory,
        confidence: 0.0,
    }
}

fn matched_forward_result(left: &str, right: &str, surface: &str) -> Option<SandhiResult> {
    crate::apply_all(left, right)
        .into_iter()
        .find(|res| res.output == surface)
}

fn reverse_direct_join(
    rule: &SandhiRule,
    surface: &str,
    raw_left: &str,
    raw_right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
) {
    if DIRECT_JOIN_UPASARGAS.contains(&raw_left)
        && lex.contains(raw_left)
        && lex.contains(raw_right)
        && lex.origin_of(surface) == Some(OriginTag::Tatsam)
        && lex.origin_of(raw_right) == Some(OriginTag::Tatsam)
    {
        out.push(SandhiCandidate {
            surface: surface.to_string(),
            left: raw_left.to_string(),
            right: raw_right.to_string(),
            sandhi_type: SandhiType::ConsonantSandhi,
            family: RuleFamily::DirectJoin,
            rule_id: rule.id,
            rule_citation: "उपसर्ग संयोग: direct prefix-stem concatenation",
            forward_verified: true,
            lexical_left: lexical_status(raw_left, lex, true),
            lexical_right: lexical_status(raw_right, lex, false),
            authority: crate::AuthorityTier::Exploratory,
            confidence: 0.0,
        });
    }
}

fn reverse_direct_verified(
    _rule: &SandhiRule,
    surface: &str,
    raw_left: &str,
    raw_right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
) {
    apply_expansion_spec(
        surface,
        raw_left,
        raw_right,
        lex,
        out,
        &DIRECT_VERIFIED_SPEC,
    );
}

fn reverse_vowel_reconstruction(
    _rule: &SandhiRule,
    surface: &str,
    raw_left: &str,
    raw_right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
) {
    apply_expansion_spec(
        surface,
        raw_left,
        raw_right,
        lex,
        out,
        &VOWEL_RECONSTRUCTION_SPEC,
    );
}

fn reverse_yan(
    _rule: &SandhiRule,
    surface: &str,
    raw_left: &str,
    raw_right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
) {
    apply_suffix_specs(surface, raw_left, raw_right, lex, out, YAN_RULES);
}

fn reverse_visarga_r(
    _rule: &SandhiRule,
    surface: &str,
    raw_left: &str,
    raw_right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
) {
    apply_right_rewrite_specs(
        surface,
        raw_left,
        raw_right,
        lex,
        out,
        "ः",
        VISARGA_R_REWRITE_SPECS,
    );
}

fn reverse_visarga_sibilant(
    _rule: &SandhiRule,
    surface: &str,
    raw_left: &str,
    raw_right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
) {
    for &(sibilant, stops) in SIBILANT_RULES {
        let suffix = format!("{sibilant}्");
        if let Some(base) = raw_left.strip_suffix(&suffix) {
            if let Some(first_char) = raw_right.chars().next() {
                if stops.contains(&first_char) {
                    let left_candidate = format!("{base}ः");
                    push_verified_candidate(surface, &left_candidate, raw_right, lex, out);
                }
            }
        }
    }
}

fn reverse_ayadi_and_matra(
    _rule: &SandhiRule,
    surface: &str,
    raw_left: &str,
    raw_right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
) {
    apply_suffix_specs(surface, raw_left, raw_right, lex, out, AYADI_RULES);
    apply_matra_specs(surface, raw_left, raw_right, lex, out, MATRA_REVERSE_RULES);
}

fn apply_suffix_specs(
    surface: &str,
    raw_left: &str,
    raw_right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
    specs: &[ReverseSuffixSpec],
) {
    for spec in specs {
        if let Some(base) = raw_left.strip_suffix(spec.strip_suffix) {
            for replacement in spec.replacements {
                let left = format!("{base}{replacement}");
                reverse_vowel_suffix(surface, &left, raw_right, lex, out);
            }
            break;
        }
    }
}

fn apply_matra_specs(
    surface: &str,
    raw_left: &str,
    raw_right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
    specs: &[ReverseMatraSpec],
) {
    let mut right_chars = raw_right.chars();
    let Some(first_char) = right_chars.next() else {
        return;
    };
    let Some(spec) = specs.iter().find(|spec| spec.matra == first_char) else {
        return;
    };

    let remainder = right_chars.as_str();
    for left in expand_left_suffixes(raw_left, RECONSTRUCTION_LEFT_SUFFIXES) {
        emit_matra_candidates(surface, &left, remainder, spec.vowels, lex, out);
    }
}

fn emit_matra_candidates(
    surface: &str,
    left: &str,
    right_remainder: &str,
    vowels: &[&str],
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
) {
    for v in vowels {
        let candidate_right = format!("{v}{right_remainder}");
        push_verified_candidate(surface, left, &candidate_right, lex, out);
    }
}

fn reverse_vowel_suffix(
    surface: &str,
    left: &str,
    raw_right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
) {
    for v in VOWELS {
        let right = format!("{v}{raw_right}");
        push_verified_candidate(surface, left, &right, lex, out);
    }
}

fn push_verified_with_left_suffixes(
    surface: &str,
    raw_left: &str,
    right: &str,
    suffixes: &[&str],
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
) {
    for left in expand_left_suffixes(raw_left, suffixes) {
        push_verified_candidate(surface, &left, right, lex, out);
    }
}

fn apply_expansion_spec(
    surface: &str,
    raw_left: &str,
    raw_right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
    spec: &ReverseExpansionSpec,
) {
    for right in expand_right_prefixes(raw_right, spec.right_prefixes) {
        push_verified_with_left_suffixes(surface, raw_left, &right, spec.left_suffixes, lex, out);
    }
}

fn apply_right_rewrite_specs(
    surface: &str,
    raw_left: &str,
    raw_right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
    left_suffix: &str,
    specs: &[ReverseRightRewriteSpec],
) {
    let left = format!("{raw_left}{left_suffix}");
    for spec in specs {
        if let Some(remainder) = raw_right.strip_prefix(spec.strip_prefix) {
            let right = format!("{}{}", spec.replacement_prefix, remainder);
            push_verified_candidate(surface, &left, &right, lex, out);
        }
    }
}

fn expand_left_suffixes(raw_left: &str, suffixes: &[&str]) -> Vec<String> {
    suffixes
        .iter()
        .map(|suffix| {
            if suffix.is_empty() {
                raw_left.to_string()
            } else {
                format!("{raw_left}{suffix}")
            }
        })
        .collect()
}

fn expand_right_prefixes(raw_right: &str, prefixes: &[&str]) -> Vec<String> {
    prefixes
        .iter()
        .map(|prefix| {
            if prefix.is_empty() {
                raw_right.to_string()
            } else {
                format!("{prefix}{raw_right}")
            }
        })
        .collect()
}

fn push_verified_candidate(
    surface: &str,
    left: &str,
    right: &str,
    lex: &Kosha,
    out: &mut Vec<SandhiCandidate>,
) {
    if !lex.contains(left) || !lex.contains(right) {
        return;
    }

    if let Some(res) = matched_forward_result(left, right, surface) {
        out.push(build_candidate(
            surface,
            left.to_string(),
            right.to_string(),
            res,
            lex,
        ));
    }
}
