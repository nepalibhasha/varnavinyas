use varnavinyas_prakriya::{Rule, analyze, collect_rule_hits, derive};

#[test]
fn every_reviewed_source_example_has_a_stable_specific_winner() {
    let data = include_str!("../../../data/rule_inventories/ps_final_u_hrasva.tsv");
    for row in data.lines().skip(1) {
        let correct = row.split('\t').next().unwrap();
        let wrong = format!("{}ू", correct.strip_suffix('ु').unwrap());
        let p = derive(&wrong);
        assert_eq!(p.output, correct, "{wrong}");
        assert_eq!(p.steps[0].rule.code(), "3(क)(इ)-PS-Saisanik-(छ)");
        let hits = collect_rule_hits(&wrong);
        assert_eq!(
            hits.iter()
                .filter(|hit| hit.prakriya.output == correct)
                .count(),
            1,
            "{wrong}: {hits:?}"
        );
        assert_eq!(derive(correct).output, correct);
        assert!(
            analyze(correct).rule_notes.iter().any(|note| {
                note.rule == Rule::VarnaVinyasNiyam("3(क)(इ)-PS-Saisanik-(छ)")
            }),
            "{correct}"
        );
    }
}

#[test]
fn final_u_inventory_does_not_shorten_other_words() {
    for word in ["वधू", "भाउजू", "सासू", "थारू", "काजू", "हिन्दू", "जाऊ"]
    {
        assert_eq!(derive(word).output, word, "{word}");
    }
}

#[test]
fn reviewed_vadhu_policy_beats_raw_headword_attestation() {
    let p = derive("बधू");
    assert_eq!(p.output, "वधू");
    assert_eq!(p.steps[0].rule.code(), "3(ग)(आ)-PS-Saisanik-वधू");
    assert_eq!(collect_rule_hits("बधू").len(), 1);
    assert_eq!(derive("वधू").output, "वधू");
}
