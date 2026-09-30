//! Conservative verb evidence for the numbered vowel rules.
use super::helpers::hrasva_helpers;
use crate::{DiagnosticKind, Prakriya, Rule, RuleCategory, RuleSpec, Step};
use varnavinyas_kosha::kosha;

pub const SPEC_VERB_HRASVA: RuleSpec = RuleSpec {
    id: "hd-verb-hrasva",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 224,
    citation: Rule::VarnaVinyasNiyam("3(क)(अ)-9/3(क)(आ)-8/3(क)(इ)-8"),
    examples: &[("कीन्ने", "किन्ने"), ("खेलीने", "खेलिने"), ("लिनूपर्ने", "लिनुपर्ने")],
};

fn is_infinitive(word: &str) -> bool {
    let lex = kosha();
    word.ends_with("नु")
        && lex.is_correction_target(word)
        && lex
            .lookup(word)
            .is_some_and(|entry| entry.pos.contains("क्रिया"))
}

pub(super) fn has_supported_infinitive(stem: &str) -> bool {
    if stem.is_empty() {
        return false;
    }
    [
        format!("{stem}नु"),
        format!("{stem}्नु"),
        format!("{stem}उनु"),
        format!("{stem}इनु"),
    ]
    .iter()
    .any(|candidate| is_infinitive(candidate))
}

fn has_supported_verb_form(word: &str) -> bool {
    is_infinitive(word)
        || ["ने", "न", "े", "दा", "दै"].iter().any(|ending| {
            word.strip_suffix(ending)
                .is_some_and(has_supported_infinitive)
        })
}

pub fn rule_verb_hrasva(input: &str) -> Option<Prakriya> {
    let lex = kosha();
    if lex.is_rule_protected(input) {
        return None;
    }
    if let Some(output) = hrasva_helpers::initial_dirgha_to_hrasva(input) {
        if has_supported_verb_form(&output) {
            return correction(
                input,
                &output,
                "3(क)(अ)-9",
                "क्रियापदको सुरुको इकार/उकार ह्रस्व हुन्छ",
            );
        }
    }
    // 3(क)(आ)-8: the passive इ/ि is short. The root must independently
    // support an infinitive; merely ending in ीने does not establish a verb.
    for (wrong, right) in [
        ("ीनु", "िनु"),
        ("ीने", "िने"),
        ("ीन", "िन"),
        ("ीन्छ", "िन्छ"),
        ("ीयो", "ियो"),
    ] {
        if let Some(stem) = input.strip_suffix(wrong) {
            let output = format!("{stem}{right}");
            if has_supported_infinitive(stem) && lex.is_correction_target(&output) {
                return correction(
                    input,
                    &output,
                    "3(क)(आ)-8",
                    "कर्म वा भाववाच्यका क्रियापदमा इकार ह्रस्व हुन्छ",
                );
            }
        }
    }
    // Bare पढ्नू/जानू are valid imperatives under the school grammar (म).
    // A joined obligation participle uses the infinitive नु, not imperative नू.
    if let Some(infinitive) = input.strip_suffix("पर्ने") {
        if let Some(stem) = infinitive.strip_suffix("नू") {
            let canonical = format!("{stem}नु");
            let output = format!("{canonical}पर्ने");
            if is_infinitive(&canonical) && lex.is_correction_target(&output) {
                return correction(
                    input,
                    &output,
                    "3(क)(इ)-8",
                    "नु प्रत्यय भएको क्रियापदमा उकार ह्रस्व हुन्छ; पर्ने जोडिँदा पनि नु कायम रहन्छ",
                );
            }
        }
    }
    None
}

fn correction(
    input: &str,
    output: &str,
    code: &'static str,
    explanation: &str,
) -> Option<Prakriya> {
    Some(Prakriya::corrected(
        input,
        output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam(code),
            explanation,
            input,
            output,
        )],
    ))
}

/// A long ई can mark a feminine predicate or an एर-equivalent converb under
/// PS (ब)/(भ), even when the short spelling is independently attested as a noun.
pub(super) fn is_supported_i_verb_form(word: &str) -> bool {
    fn positive(word: &str) -> bool {
        let Some(stem) = word.strip_suffix('ी').or_else(|| word.strip_suffix('ई')) else {
            return false;
        };
        has_supported_infinitive(stem)
            || match stem {
                "भ" => is_infinitive("हुनु"),
                "ग" => is_infinitive("जानु"),
                _ => false,
            }
    }
    positive(word) || word.strip_prefix('न').is_some_and(positive)
}

pub const SPEC_FINAL_I_VERB_DIRGHA: RuleSpec = RuleSpec {
    id: "hd-final-i-verb-dirgha",
    category: RuleCategory::HrasvaDirgha,
    kind: DiagnosticKind::Error,
    priority: 239,
    citation: Rule::VarnaVinyasNiyam("PS-Saisanik-ह्रस्वदीर्घ-(ब)/(भ)"),
    examples: &[("दिइ", "दिई"), ("नभइ", "नभई")],
};

/// Without sentence parsing, correct only unsupported short spellings. Known
/// nouns/adverbs such as भइ, मिलाइ, पारि and लेखि retain their own meanings.
pub fn rule_final_i_verb_dirgha(input: &str) -> Option<Prakriya> {
    let lex = kosha();
    if lex.is_rule_protected(input) {
        return None;
    }
    let output = if let Some(stem) = input.strip_suffix('ि') {
        format!("{stem}ी")
    } else if let Some(stem) = input.strip_suffix('इ') {
        format!("{stem}ई")
    } else {
        return None;
    };
    if !lex.is_correction_target(&output) || !is_supported_i_verb_form(&output) {
        return None;
    }
    // Existing numbered final-vowel rules retain their reason when they
    // already recover this form, rather than gaining a duplicate fallback.
    if super::rule_reviewed_final_dirgha(input).is_some_and(|hit| hit.output == output)
        || super::rule_pronoun_vowel_length(input).is_some_and(|hit| hit.output == output)
        || super::rule_final_ii_suffix_dirgha(input).is_some_and(|hit| hit.output == output)
        || super::rule_final_adjective_dirgha(input).is_some_and(|hit| hit.output == output)
        || super::rule_final_vati_vi_dirgha(input).is_some_and(|hit| hit.output == output)
        || super::rule_dirgha_endings(input).is_some_and(|hit| hit.output == output)
    {
        return None;
    }
    correction(
        input,
        &output,
        "PS-Saisanik-ह्रस्वदीर्घ-(ब)/(भ)",
        "शैक्षणिक व्याकरण (ब)/(भ): स्त्रीलिङ्गी समापक क्रिया र एरको सट्टा आउने कृदन्तको अन्त्यमा ईकार दीर्घ हुन्छ",
    )
}
