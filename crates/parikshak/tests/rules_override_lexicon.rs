use varnavinyas_parikshak::check_word;

#[test]
fn rules_override_lexicon_rajnaitik() {
    // राजनैतिक is in the kosha lexicon but the Academy correction
    // table maps it to राजनीतिक. Rules must win over lexicon.
    let diag = check_word("राजनैतिक");
    assert!(diag.is_some(), "राजनैतिक must be flagged despite being in lexicon");
    let diag = diag.unwrap();
    assert_eq!(diag.correction, "राजनीतिक");
}
