use varnavinyas_samasa::analyze_compound;

#[test]
fn known_compound_has_candidate() {
    let candidates = analyze_compound("सूर्योदय");
    assert!(
        candidates
            .iter()
            .any(|c| c.left == "सूर्य" && c.right == "उदय")
    );
}
