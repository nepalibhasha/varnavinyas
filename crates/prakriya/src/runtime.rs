use std::sync::LazyLock;

use crate::model::rule_spec::PatternRule;
use crate::niyama_registry;

static PATTERN_RULES: LazyLock<Vec<PatternRule>> = LazyLock::new(|| {
    let mut rules = niyama_registry::usage_fix_rules();
    rules.extend(niyama_registry::varna_vinyasa_rules());
    rules.sort_by_key(|r| r.spec.priority);
    rules
});

pub fn pattern_rules() -> &'static [PatternRule] {
    &PATTERN_RULES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_rules_sorted_by_priority() {
        let rules = pattern_rules();
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
        let rules = pattern_rules();
        for (i, a) in rules.iter().enumerate() {
            for b in rules.iter().skip(i + 1) {
                assert_ne!(a.spec.id, b.spec.id, "Duplicate rule id: {}", a.spec.id,);
            }
        }
    }
}
