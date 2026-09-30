use varnavinyas_prakriya::{analyze, collect_rule_hits, derive};

#[test]
fn all_reviewed_semantic_examples_have_specific_corrections_and_accepted_notes() {
    let data = include_str!("../../../data/rule_inventories/final_ii_semantic_classes.tsv");
    for row in data.lines().skip(1) {
        let fields: Vec<_> = row.split('\t').collect();
        let correct = fields[0];
        let code = match fields[1] {
            "feminine_adjective" => "3(क)(ऊ)-4",
            "inanimate_noun" => "3(क)(ऊ)-6",
            "ps_inanimate_noun" => "PS-Saisanik-ह्रस्वदीर्घ-(थ)",
            other => panic!("unknown class: {other}"),
        };
        let wrong = format!("{}ि", correct.strip_suffix('ी').unwrap());
        let p = derive(&wrong);
        assert_eq!(p.output, correct, "{wrong}");
        assert_eq!(p.steps[0].rule.code(), code, "{wrong}");
        let hits = collect_rule_hits(&wrong);
        assert_eq!(
            hits.iter().filter(|h| h.prakriya.output == correct).count(),
            1,
            "{wrong}: {hits:?}"
        );
        assert_eq!(derive(correct).output, correct, "{correct}");
        assert!(
            analyze(correct)
                .rule_notes
                .iter()
                .any(|n| n.rule.code() == code),
            "{correct}"
        );
    }
}

#[test]
fn ending_alone_does_not_establish_a_semantic_class() {
    for word in [
        "माथि",
        "पछि",
        "अघि",
        "जति",
        "चोटि",
        "नाति",
        "प्रति",
        "स्थिति",
        "समिति",
        "गति",
    ] {
        assert_eq!(derive(word).output, word, "{word}");
        assert!(!analyze(word).rule_notes.iter().any(|n| {
            ["3(क)(ऊ)-4", "3(क)(ऊ)-6", "PS-Saisanik-ह्रस्वदीर्घ-(थ)"].contains(&n.rule.code())
        }));
    }
}
