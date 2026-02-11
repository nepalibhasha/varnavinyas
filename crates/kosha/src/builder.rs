use fst::SetBuilder;

/// Build an FST Set from a sorted list of words (byte-order sorted).
pub fn build_fst_set(words: &[&str]) -> Vec<u8> {
    let mut builder = SetBuilder::memory();
    for word in words {
        // fst::SetBuilder requires keys in lexicographic (byte) order.
        // Our input is pre-sorted by UTF-8 bytes, so this is safe.
        builder.insert(word).expect("words must be sorted");
    }
    builder.into_inner().expect("FST build should succeed")
}
