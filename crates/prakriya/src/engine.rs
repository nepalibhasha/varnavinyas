use crate::correction_table;
use crate::hrasva_dirgha;
use crate::orthographic;
use crate::prakriya::Prakriya;
use crate::step::Step;
use crate::structural;

/// Derive the correct form of a word, with step-by-step rule tracing.
///
/// This is the main entry point for the correction engine.
/// It uses a hybrid approach:
/// 1. Correction table lookup (authoritative Academy standard entries)
/// 2. Pattern-based rules as fallback (generalizable heuristics)
/// 3. If neither fires, the word is considered correct.
pub fn derive(input: &str) -> Prakriya {
    if input.is_empty() {
        return Prakriya::correct("");
    }

    // Phase A: Correction table lookup (Authoritative)
    if let Some(p) = try_correction_table(input) {
        return p;
    }

    // Phase B: Try pattern rules (Heuristics)
    if let Some(p) = try_pattern_rules(input) {
        return p;
    }

    // No correction needed — word is considered correct
    Prakriya::correct(input)
}

/// Try all pattern-based rules in priority order.
fn try_pattern_rules(input: &str) -> Option<Prakriya> {
    // 1. Structural rules (श्रृ→शृ, redundant suffix, etc.)
    if let Some(p) = structural::apply_structural_rules(input) {
        return Some(p);
    }

    // 2. Hrasva/dirgha rules
    if let Some(p) = hrasva_dirgha::apply_hrasva_dirgha_rules(input) {
        return Some(p);
    }

    // 3. Orthographic rules (chandrabindu, sibilant, ri/kri, etc.)
    if let Some(p) = orthographic::apply_orthographic_rules(input) {
        return Some(p);
    }

    None
}

/// Try the static correction table.
fn try_correction_table(input: &str) -> Option<Prakriya> {
    let entry = correction_table::lookup(input)?;

    // Handle multi-answer entries (e.g., "धीरता/धैर्य")
    // Return the first alternative
    let output = entry.correct.split('/').next().unwrap_or(entry.correct);

    Some(Prakriya::corrected(
        input,
        output,
        vec![Step::new(entry.rule, entry.description, input, output)],
    ))
}
