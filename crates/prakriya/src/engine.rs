use std::sync::LazyLock;

use crate::correction_table;
use crate::hrasva_dirgha;
use crate::orthographic;
use crate::prakriya::Prakriya;
use crate::rule_spec::PatternRule;
use crate::step::Step;
use crate::structural;

/// All pattern rules, sorted by priority (lower = higher priority).
static PATTERN_RULES: LazyLock<Vec<PatternRule>> = LazyLock::new(|| {
    let mut rules = vec![
        // Structural (100–120)
        PatternRule { spec: structural::SPEC_SHRI, apply: structural::rule_shri_correction },
        PatternRule { spec: structural::SPEC_REDUNDANT_SUFFIX, apply: structural::rule_redundant_suffix },
        PatternRule { spec: structural::SPEC_PANCHHAM, apply: structural::rule_panchham_varna },
        // Hrasva/Dirgha (200–260)
        PatternRule { spec: hrasva_dirgha::SPEC_SUFFIX_NU, apply: hrasva_dirgha::rule_suffix_nu_hrasva },
        PatternRule { spec: hrasva_dirgha::SPEC_SUFFIX_ELI, apply: hrasva_dirgha::rule_suffix_eli_hrasva },
        PatternRule { spec: hrasva_dirgha::SPEC_SUFFIX_PRESERVES, apply: hrasva_dirgha::rule_suffix_preserves_dirgha },
        PatternRule { spec: hrasva_dirgha::SPEC_TADBHAV, apply: hrasva_dirgha::rule_tadbhav_hrasva },
        PatternRule { spec: hrasva_dirgha::SPEC_DIRGHA_ENDINGS, apply: hrasva_dirgha::rule_dirgha_endings },
        PatternRule { spec: hrasva_dirgha::SPEC_KINSHIP, apply: hrasva_dirgha::rule_kinship_tadbhav },
        PatternRule { spec: hrasva_dirgha::SPEC_KOSHA_BACKED, apply: hrasva_dirgha::kosha_backed_dirgha_correction },
        // Orthographic (300–330)
        PatternRule { spec: orthographic::SPEC_CHANDRABINDU, apply: orthographic::rule_chandrabindu },
        PatternRule { spec: orthographic::SPEC_SIBILANT, apply: orthographic::rule_sibilant },
        PatternRule { spec: orthographic::SPEC_RI_KRI, apply: orthographic::rule_ri_kri },
        PatternRule { spec: orthographic::SPEC_HALANTA, apply: orthographic::rule_halanta },
        // Orthographic kosha-backed (340–360)
        PatternRule { spec: orthographic::SPEC_AADHI_VRIDDHI, apply: orthographic::rule_aadhi_vriddhi },
        PatternRule { spec: orthographic::SPEC_YA_E, apply: orthographic::rule_ya_e },
        PatternRule { spec: orthographic::SPEC_KSHA_CHHYA, apply: orthographic::rule_ksha_chhya },
    ];
    rules.sort_by_key(|r| r.spec.priority);
    rules
});

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
    for rule in PATTERN_RULES.iter() {
        if let Some(p) = (rule.apply)(input) {
            return Some(p);
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::Rule;

    #[test]
    fn pattern_rules_sorted_by_priority() {
        let rules = &*PATTERN_RULES;
        for window in rules.windows(2) {
            assert!(
                window[0].spec.priority <= window[1].spec.priority,
                "Rules out of order: {} (priority {}) before {} (priority {})",
                window[0].spec.id,
                window[0].spec.priority,
                window[1].spec.id,
                window[1].spec.priority,
            );
        }
    }

    #[test]
    fn pattern_rules_have_unique_ids() {
        let rules = &*PATTERN_RULES;
        for (i, a) in rules.iter().enumerate() {
            for b in rules.iter().skip(i + 1) {
                assert_ne!(
                    a.spec.id, b.spec.id,
                    "Duplicate rule id: {}",
                    a.spec.id,
                );
            }
        }
    }

    /// Exercises the production `try_pattern_rules` path: structural rules (priority 100)
    /// must beat orthographic rules (priority 300+) for an input both would match.
    /// "श्रृङ्गार" triggers struct-shri (100). If priority ordering broke, a later
    /// rule could intercept it instead.
    #[test]
    fn production_priority_structural_beats_orthographic() {
        // श्रृङ्गार matches struct-shri (priority 100).
        // Verify the production path returns the structural correction.
        let p = try_pattern_rules("श्रृङ्गार").expect("should fire a pattern rule");
        assert_eq!(p.output, "शृङ्गार");
        // The first step should cite ShuddhaAshuddha (structural rule), not VarnaVinyasNiyam.
        assert!(
            matches!(p.steps[0].rule, Rule::ShuddhaAshuddha(_)),
            "Expected structural rule citation, got {:?}",
            p.steps[0].rule,
        );
    }

    /// Guard against silent omissions: every known rule ID must be present in the registry.
    /// If you add a new SPEC_* + rule fn in a module, add its ID here — the test will
    /// fail until you also register it in PATTERN_RULES.
    #[test]
    fn all_expected_rule_ids_registered() {
        const EXPECTED_IDS: &[&str] = &[
            // structural
            "struct-shri",
            "struct-redundant-suffix",
            "struct-panchham",
            // hrasva-dirgha
            "hd-suffix-nu",
            "hd-suffix-eli",
            "hd-suffix-preserves",
            "hd-tadbhav",
            "hd-dirgha-endings",
            "hd-kinship",
            "hd-kosha-backed",
            // orthographic
            "ortho-chandrabindu",
            "ortho-sibilant",
            "ortho-ri-kri",
            "ortho-halanta",
            "ortho-aadhi-vriddhi",
            "ortho-ya-e",
            "ortho-ksha-chhya",
        ];

        let registered: Vec<&str> = PATTERN_RULES.iter().map(|r| r.spec.id).collect();
        for &id in EXPECTED_IDS {
            assert!(
                registered.contains(&id),
                "Rule '{}' is expected but not registered in PATTERN_RULES",
                id,
            );
        }
        assert_eq!(
            registered.len(),
            EXPECTED_IDS.len(),
            "PATTERN_RULES has {} entries but EXPECTED_IDS has {} — update both when adding rules",
            registered.len(),
            EXPECTED_IDS.len(),
        );
    }
}
