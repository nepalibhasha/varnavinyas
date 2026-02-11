use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::step::Step;

/// Apply hrasva/dirgha pattern rules.
/// Returns Some(Prakriya) if a rule fired, None otherwise.
pub fn apply_hrasva_dirgha_rules(input: &str) -> Option<Prakriya> {
    // Rule: suffix -नु triggers hrasva on root vowel
    if input.ends_with("नु") || input.ends_with("र्नु") {
        if let Some(p) = rule_suffix_nu_hrasva(input) {
            return Some(p);
        }
    }

    // Rule: suffix -एली triggers hrasva on root vowel
    if input.ends_with("एली") || input.ends_with("ेली") {
        if let Some(p) = rule_suffix_eli_hrasva(input) {
            return Some(p);
        }
    }

    // Rule: suffix -ई preserves dirgha
    // Rule: suffix -ईय preserves dirgha
    // (These fix hrasva→dirgha, the reverse direction)
    if let Some(p) = rule_suffix_preserves_dirgha(input) {
        return Some(p);
    }

    // Rule: tadbhav single-meaning word takes hrasva
    if let Some(p) = rule_tadbhav_hrasva(input) {
        return Some(p);
    }

    // Rule: feminine/pronoun/postposition/absolutive takes dirgha
    if let Some(p) = rule_dirgha_endings(input) {
        return Some(p);
    }

    // Rule: kinship tadbhav patterns
    if let Some(p) = rule_kinship_tadbhav(input) {
        return Some(p);
    }

    None
}

fn rule_suffix_nu_hrasva(input: &str) -> Option<Prakriya> {
    // स्वीकार्नु → स्विकार्नु
    // Only replace the LAST dirgha ई before the suffix, not all occurrences.
    if !input.contains('ी') {
        return None;
    }

    // Find the suffix position to scope our search
    let suffix_start = input.rfind("कार्नु").or_else(|| input.rfind("नु"))?;
    let prefix_part = &input[..suffix_start];

    // Find the last ई in the part before the suffix
    let last_ii_pos = prefix_part.rfind('ी')?;
    let mut output = String::with_capacity(input.len());
    let mut pos = 0;
    for ch in input.chars() {
        let byte_pos = pos;
        pos += ch.len_utf8();
        if byte_pos == last_ii_pos {
            output.push('ि');
        } else {
            output.push(ch);
        }
    }

    if output != input {
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)-suffix-नु"),
                "suffix -नु triggers hrasva on root vowel",
                input,
                &output,
            )],
        ));
    }
    None
}

fn rule_suffix_eli_hrasva(input: &str) -> Option<Prakriya> {
    // पूर्वेली → पुर्वेली
    // Only replace the LAST dirgha ू before the suffix, not all occurrences.
    if !input.contains('ू') {
        return None;
    }

    let suffix_start = input.rfind("ेली")?;
    let prefix_part = &input[..suffix_start];

    // Find the last ू in the part before the suffix
    let last_uu_pos = prefix_part.rfind('ू')?;
    let mut output = String::with_capacity(input.len());
    let mut pos = 0;
    for ch in input.chars() {
        let byte_pos = pos;
        pos += ch.len_utf8();
        if byte_pos == last_uu_pos {
            output.push('ु');
        } else {
            output.push(ch);
        }
    }

    if output != input {
        return Some(Prakriya::corrected(
            input,
            &output,
            vec![Step::new(
                Rule::VarnaVinyasNiyam("3(क)-suffix-एली"),
                "suffix -एली triggers hrasva on root vowel",
                input,
                &output,
            )],
        ));
    }
    None
}

fn rule_suffix_preserves_dirgha(_input: &str) -> Option<Prakriya> {
    // पुर्वी → पूर्वी (suffix -ई preserves dirgha)
    // पुर्वीय → पूर्वीय (suffix -ईय preserves dirgha)
    // These need very specific patterns — handled by correction table
    None
}

fn rule_tadbhav_hrasva(_input: &str) -> Option<Prakriya> {
    // Generic tadbhav hrasva rule — mostly handled by correction table
    // Pattern rules would need a tadbhav word list to be effective
    None
}

fn rule_dirgha_endings(_input: &str) -> Option<Prakriya> {
    // Generic dirgha ending rules — mostly handled by correction table
    None
}

fn rule_kinship_tadbhav(_input: &str) -> Option<Prakriya> {
    // Kinship tadbhav patterns — mostly handled by correction table
    None
}
