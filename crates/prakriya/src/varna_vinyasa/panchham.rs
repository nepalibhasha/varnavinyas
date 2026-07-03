use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::rule_spec::{DiagnosticKind, RuleCategory, RuleSpec};
use crate::model::step::Step;
use varnavinyas_kosha::kosha;
use varnavinyas_shabda::{Origin, OriginSource, classify, classify_with_provenance};

pub const SPEC_PANCHHAM: RuleSpec = RuleSpec {
    id: "struct-panchham",
    category: RuleCategory::Structural,
    kind: DiagnosticKind::Error,
    priority: 120,
    citation: Rule::VarnaVinyasNiyam("3(ख)-पञ्चम"),
    examples: &[("संकेत", "सङ्केत"), ("संघीय", "सङ्घीय")],
};

const PS_FINAL_NGA_WITHOUT_GA_RULE_CODE: &str = "3(ख)-PS-Saisanik-6(घ)-पदान्त-ङ";
const PS_FINAL_NGA_WITHOUT_GA_FORMS: &[&str] = &[
    "मनाङ",
    "मोरङ",
    "गुरुङ",
    "तामाङ",
    "बागलुङ",
    "दाङ",
    "बझाङ",
    "दार्जिलिङ",
    "हङकङ",
    "करङ",
    "इन्जिनियरिङ",
    "बोर्डिङ",
    "बिल्डिङ",
    "ट्रेनिङ",
    "रङ",
    "छर्लङ",
    "तुर्लुङ",
    "फुरुङ",
    "भुङ",
    "डङ",
    "खटङ",
    "छ्याङ",
];

/// Academy 3(ख)(अ): panchham varna rules for तत्सम words.
/// In तत्सम words, anusvara (ं) before stop consonants -> panchham varna.
pub fn rule_panchham_varna(input: &str) -> Option<Prakriya> {
    if has_supported_non_tatsam_chandrabindu_variant(input) {
        return None;
    }

    if let Some(output) = normalize_final_nga_without_ga(input) {
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam(PS_FINAL_NGA_WITHOUT_GA_RULE_CODE),
                "शैक्षणिक व्याकरण ६(घ): हलन्त ङ मात्र उच्चारण हुने शब्दमा ङ्ग होइन ङ लेखिन्छ",
                input,
                &output,
            )],
        ));
    }

    let origin = classify(input);

    if !matches!(origin, Origin::Tatsam) {
        if let Some(output) = normalize_non_tatsam_panchham(input) {
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ख)(अ)-3"),
                    "तद्भव/आगन्तुक शब्दमा शिरविन्दु वा तत्सम पञ्चमवर्णको नियम लाग्दैन; उच्चारणअनुसार लेखिन्छ",
                    input,
                    &output,
                )],
            ));
        }
    }

    if !input.contains('ं') {
        return None;
    }

    let chars: Vec<char> = input.chars().collect();
    let mut result = String::with_capacity(input.len() + 8);
    let mut changed = false;
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == 'ं' {
            if let Some(&next) = chars.get(i + 1) {
                if next == 'ज'
                    && chars.get(i + 2).copied() == Some('्')
                    && chars.get(i + 3).copied() == Some('ञ')
                {
                    result.push(chars[i]);
                    i += 1;
                    continue;
                }
                if let Some(panchham) = get_panchham_for(next) {
                    result.push(panchham);
                    result.push('्');
                    changed = true;
                    i += 1;
                    continue;
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    if changed {
        if !matches!(origin, Origin::Tatsam) && !kosha().contains(&result) {
            return None;
        }
        let citation = first_panchham_citation(input).unwrap_or("3(ख)(अ)-2");
        return Some(Prakriya::corrected(
            input,
            &result,
            vec![Step::new(
                Rule::VarnaVinyasNiyam(citation),
                "तत्सम शब्दमा स्पर्श व्यञ्जन अघि पञ्चम वर्ण प्रयोग",
                input,
                &result,
            )],
        ));
    }

    None
}

fn has_supported_non_tatsam_chandrabindu_variant(input: &str) -> bool {
    if !input.contains('ं') {
        return false;
    }

    let lex = kosha();
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        if chars[i] != 'ं' {
            continue;
        }

        let mut candidate_chars = chars.clone();
        candidate_chars[i] = 'ँ';
        let candidate: String = candidate_chars.into_iter().collect();
        if chandrabindu_override_is_productive(&candidate) {
            continue;
        }
        let decision = classify_with_provenance(&candidate);

        if matches!(decision.origin, Origin::Tatsam) {
            continue;
        }

        let Some(entry) = lex.lookup(&candidate) else {
            continue;
        };
        let pos = entry.pos;
        if pos.contains("अव्य")
            || pos.contains("क्रि.वि.")
            || pos.contains("क्रियाविशेषण")
            || pos.contains("नामयोगी")
            || pos.contains("ना.यो.")
        {
            return true;
        }
    }

    false
}

fn normalize_final_nga_without_ga(input: &str) -> Option<String> {
    let stem = input.strip_suffix("ङ्ग")?;
    let output = format!("{stem}ङ");
    if PS_FINAL_NGA_WITHOUT_GA_FORMS.contains(&output.as_str()) {
        Some(output)
    } else {
        None
    }
}

fn chandrabindu_override_is_productive(candidate: &str) -> bool {
    candidate.contains("ँछ")
        || candidate.contains("ँथ")
        || candidate.contains("ँदा")
        || candidate.contains("ँदै")
        || candidate.ends_with('ँ')
        || candidate.ends_with("ौँ")
        || candidate.ends_with("ौं")
        || candidate.ends_with("ुँ")
        || candidate.ends_with("ूँ")
        || candidate.contains("ँला")
}

fn panchham_subrule_citation(next: char) -> &'static str {
    match next {
        'क' | 'ख' | 'ग' | 'घ' => "3(ख)(अ)-2-ङ्",
        'च' | 'छ' | 'ज' | 'झ' => "3(ख)(अ)-2-ञ्",
        'ट' | 'ठ' | 'ड' | 'ढ' | 'ण' => "3(ख)(अ)-2-ण्",
        'त' | 'थ' | 'द' | 'ध' | 'न' => "3(ख)(अ)-2-न्",
        'प' | 'फ' | 'ब' | 'भ' | 'म' => "3(ख)(अ)-2-म्",
        _ => "3(ख)(अ)-2",
    }
}

fn normalize_non_tatsam_panchham(input: &str) -> Option<String> {
    const PATTERNS: &[(&str, &str)] = &[
        ("ण्ट", "न्ट"),
        ("ण्ठ", "न्ठ"),
        ("ण्ड", "न्ड"),
        ("ण्ढ", "न्ढ"),
        ("ण्ण", "न्न"),
        ("ञ्च", "न्च"),
        ("ञ्छ", "न्छ"),
        ("ञ्ज", "न्ज"),
        ("ञ्झ", "न्झ"),
    ];

    for &(wrong, right) in PATTERNS {
        if !input.contains(wrong) {
            continue;
        }
        let output = input.replace(wrong, right);
        let output_origin = classify_with_provenance(&output);
        if matches!(output_origin.origin, Origin::Tatsam) {
            continue;
        }
        if matches!(output_origin.source, OriginSource::Heuristic) && !kosha().contains(&output) {
            continue;
        }
        return Some(output);
    }
    None
}

fn first_panchham_citation(input: &str) -> Option<&'static str> {
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        if chars[i] != 'ं' {
            continue;
        }
        let next = chars.get(i + 1).copied()?;
        if next == 'ज'
            && chars.get(i + 2).copied() == Some('्')
            && chars.get(i + 3).copied() == Some('ञ')
        {
            continue;
        }
        if get_panchham_for(next).is_some() {
            return Some(panchham_subrule_citation(next));
        }
    }
    None
}

fn get_panchham_for(c: char) -> Option<char> {
    match c {
        'क' | 'ख' | 'ग' | 'घ' => Some('ङ'),
        'च' | 'छ' | 'ज' | 'झ' => Some('ञ'),
        'ट' | 'ठ' | 'ड' | 'ढ' => Some('ण'),
        'त' | 'थ' | 'द' | 'ध' | 'न' => Some('न'),
        'प' | 'फ' | 'ब' | 'भ' | 'म' => Some('म'),
        _ => None,
    }
}
