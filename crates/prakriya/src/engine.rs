use crate::correction_table;
use crate::model::prakriya::{Prakriya, RuleHit};
use crate::model::rule_spec::{DiagnosticKind, RuleCategory};
use crate::model::step::Step;
use crate::runtime;

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

    if let Some(hit) = collect_rule_hits(input).into_iter().next() {
        return hit.into_prakriya();
    }

    // No correction needed — word is considered correct
    Prakriya::correct(input)
}

/// Collect all applicable rule hits for a word, sorted by production priority.
///
/// This is a non-breaking companion to `derive()`: callers that need the
/// current single best correction should keep using `derive()`, while tools
/// that want to surface alternate applicable rules can use this API.
pub fn collect_rule_hits(input: &str) -> Vec<RuleHit> {
    if input.is_empty() {
        return Vec::new();
    }

    let mut hits = Vec::new();
    if let Some(hit) = try_correction_table_hit(input) {
        hits.push(hit);
    }
    hits.extend(collect_pattern_rule_hits(input));
    hits.sort_by_key(|hit| hit.priority);
    hits.dedup_by(rule_hits_equivalent);
    hits
}

fn rule_hits_equivalent(a: &mut RuleHit, b: &mut RuleHit) -> bool {
    a.category == b.category
        && a.kind == b.kind
        && a.prakriya.output == b.prakriya.output
        && step_signatures(&a.prakriya.steps) == step_signatures(&b.prakriya.steps)
}

fn step_signatures(steps: &[Step]) -> Vec<(crate::model::rule::Rule, &str, &str, &str)> {
    steps
        .iter()
        .map(|s| {
            (
                s.rule,
                s.description.as_str(),
                s.before.as_str(),
                s.after.as_str(),
            )
        })
        .collect()
}

/// Try all pattern-based rules in priority order.
fn collect_pattern_rule_hits(input: &str) -> Vec<RuleHit> {
    runtime::pattern_rules()
        .iter()
        .filter_map(|rule| {
            (rule.apply)(input).map(|p| RuleHit {
                spec_id: Some(rule.spec.id),
                priority: rule.spec.priority,
                category: rule.spec.category,
                kind: rule.spec.kind,
                prakriya: p.with_metadata(rule.spec.category, rule.spec.kind),
            })
        })
        .collect()
}

/// Try the static correction table.
fn try_correction_table_hit(input: &str) -> Option<RuleHit> {
    let entry = correction_table::lookup(input)?;

    // Handle multi-answer entries (e.g., "धीरता/धैर्य")
    // Return the first alternative
    let output = entry.correct.split('/').next().unwrap_or(entry.correct);

    Some(RuleHit {
        spec_id: None,
        priority: 0,
        category: RuleCategory::ShuddhaTable,
        kind: DiagnosticKind::Error,
        prakriya: Prakriya::corrected(
            input,
            output,
            vec![Step::new(entry.rule, entry.description, input, output)],
        )
        .with_metadata(RuleCategory::ShuddhaTable, DiagnosticKind::Error),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::rule::Rule;

    /// Exercises the production `try_pattern_rules` path: structural rules (priority 100)
    /// must beat orthographic rules (priority 300+) for an input both would match.
    /// "श्रृङ्गार" triggers struct-shri (100). If priority ordering broke, a later
    /// rule could intercept it instead.
    #[test]
    fn production_priority_structural_beats_orthographic() {
        // श्रृङ्गार matches struct-shri (priority 100).
        // Verify the production path returns the structural correction.
        let p = collect_pattern_rule_hits("श्रृङ्गार")
            .into_iter()
            .next()
            .expect("should fire a pattern rule")
            .into_prakriya();
        assert_eq!(p.output, "शृङ्गार");
        // The first step should cite ShuddhaAshuddha (structural rule), not VarnaVinyasNiyam.
        assert!(
            matches!(p.steps[0].rule, Rule::ShuddhaAshuddha(_)),
            "Expected structural rule citation, got {:?}",
            p.steps[0].rule,
        );
    }

    #[test]
    fn collect_rule_hits_keeps_derive_winner_stable() {
        let hits = collect_rule_hits("बिद्वान");
        assert!(
            !hits.is_empty(),
            "Expected at least one hit for बिद्वान, got: {hits:?}"
        );
        assert_eq!(
            derive("बिद्वान").output,
            hits[0].prakriya.output,
            "derive() should still pick the top-priority hit"
        );
        assert_eq!(
            hits[0].prakriya.output, "विद्वान्",
            "Authoritative correction-table winner should remain stable"
        );
    }

    #[test]
    fn collect_rule_hits_deduplicates_equivalent_hits() {
        let hits = collect_rule_hits("नेपालि");
        assert_eq!(
            hits.len(),
            1,
            "Equivalent duplicate hits should be collapsed for नेपालि, got: {hits:?}"
        );
        assert_eq!(hits[0].prakriya.output, "नेपाली");
    }

    #[test]
    fn collect_rule_hits_prefers_specific_rules_over_hd_tadbhav_fallback() {
        let cases = [
            ("सूमार्ग", "3(क)(अ)-1"),
            ("अभीमान", "3(क)(आ)-1"),
            ("ऊन्नाइस", "3(क)(अ)-7"),
            ("कीसान", "3(क)(अ)-3"),
        ];

        for (input, expected_rule) in cases {
            let hits = collect_rule_hits(input);
            assert_eq!(
                hits.len(),
                1,
                "Specific numbered rule should suppress generic hd-tadbhav alternate for {input}: {hits:?}"
            );
            assert_eq!(
                hits[0].prakriya.steps[0].rule,
                Rule::VarnaVinyasNiyam(expected_rule)
            );
        }
    }

    #[test]
    fn collect_rule_hits_prefers_specific_final_dirgha_rules_over_kosha_fallback() {
        let hits = collect_rule_hits("भाउजु");
        assert_eq!(
            hits.len(),
            1,
            "Specific final-dirgha rule should suppress kosha-backed generic alternate: {hits:?}"
        );
        assert_eq!(hits[0].prakriya.output, "भाउजू");
        assert_eq!(
            hits[0].prakriya.steps[0].rule,
            Rule::VarnaVinyasNiyam("3(क)(ऊ)-3")
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
            "hd-prefix-hrasva",
            "hd-dvi-tri-hrasva",
            "hd-initial-name-hrasva",
            "hd-initial-aagantuk-hrasva",
            "hd-suffix-nu",
            "hd-suffix-eli",
            "hd-su-prefix-preserves-dirgha",
            "hd-suffix-preserves",
            "hd-suffix-family-preserves-dirgha",
            "hd-tadbhav",
            "hd-pronoun",
            "hd-initial-adjective-hrasva",
            "hd-initial-number-hrasva",
            "hd-initial-avyaya-hrasva",
            "hd-initial-onomatopoeic-hrasva",
            "hd-medial-prefix-hrasva",
            "hd-medial-suffix-hrasva",
            "hd-medial-derived-name-hrasva",
            "hd-medial-underived-name-hrasva",
            "hd-medial-aagantuk-name-hrasva",
            "hd-medial-adjective-hrasva",
            "hd-medial-avyaya-hrasva",
            "hd-medial-onomatopoeic-hrasva",
            "hd-final-ii-suffix-dirgha",
            "hd-dirgha-endings",
            "hd-kinship",
            "hd-final-hrasva-endings",
            "hd-kosha-backed",
            // orthographic
            "ortho-chandrabindu",
            "ortho-sibilant",
            "ortho-ri-kri",
            "ortho-ba-va",
            "ortho-halanta",
            "ortho-aadhi-vriddhi",
            "ortho-ya-e",
            "ortho-ksha-chhya",
            "ortho-gya-gyan",
        ];

        let registered: Vec<&str> = crate::runtime::pattern_rules()
            .iter()
            .map(|r| r.spec.id)
            .collect();
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
