use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::step::Step;
use varnavinyas_shabda::{Origin, classify};

/// Apply orthographic pattern rules (chandrabindu, sibilant, ri/kri, halanta, ya/e, ksha/chhya).
pub fn apply_orthographic_rules(input: &str) -> Option<Prakriya> {
    // chandrabindu/shirbindu rules
    if let Some(p) = rule_chandrabindu_shirbindu(input) {
        return Some(p);
    }

    // sibilant rules (श/ष/स)
    if let Some(p) = rule_sibilant(input) {
        return Some(p);
    }

    // ऋ/कृ rules
    if let Some(p) = rule_ri_kri(input) {
        return Some(p);
    }

    // halanta rules
    if let Some(p) = rule_halanta(input) {
        return Some(p);
    }

    None
}

fn rule_chandrabindu_shirbindu(_input: &str) -> Option<Prakriya> {
    // tatsam words: chandrabindu (ँ) → shirbindu (ं)
    // This is a pattern rule but needs word origin knowledge.
    // Handled primarily by correction table.
    None
}

fn rule_sibilant(_input: &str) -> Option<Prakriya> {
    // Pattern: aagantuk words should use स not ष
    // Pattern: aagantuk words should use न not ण
    // These need origin classification — handled by correction table.
    None
}

fn rule_ri_kri(input: &str) -> Option<Prakriya> {
    // Only apply ऋ/कृ rules to words that classify as tatsam.
    // Foreign words like क्रिकेट must not be mutated.
    let origin = classify(input);
    if !matches!(origin, Origin::Tatsam) {
        return None;
    }

    // Pattern: रि at start of tatsam word → ऋ
    if let Some(rest) = input.strip_prefix("रि") {
        // Known patterns: रिषि→ऋषि, रितु→ऋतु
        if rest.starts_with('ष') || rest.starts_with('त') {
            let output = format!("ऋ{rest}");
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ग)-ऋ"),
                    "tatsam uses ऋ (not रि)",
                    input,
                    &output,
                )],
            ));
        }
    }

    // Pattern: क्रि in tatsam → कृ
    if input.contains("क्रि") {
        let output = input.replace("क्रि", "कृ");
        if output != input {
            return Some(Prakriya::corrected(
                input,
                &output,
                vec![Step::new(
                    Rule::VarnaVinyasNiyam("3(ग)-कृ"),
                    "tatsam uses कृ (not क्रि)",
                    input,
                    &output,
                )],
            ));
        }
    }

    None
}

fn rule_halanta(_input: &str) -> Option<Prakriya> {
    // Halanta rules need a list of words requiring halanta
    // Handled by correction table
    None
}
