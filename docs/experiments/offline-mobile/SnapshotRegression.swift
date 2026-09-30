import Foundation

// Swift String equality is canonically equivalent, while diagnostic spans refer
// to exact UTF-8 bytes. Verify that equivalence cannot bypass the snapshot guard.
@main struct SnapshotRegression {
    static func main() {
        let composed = "\u{00E9}"
        let decomposed = "e\u{0301}"
        precondition(composed == decomposed)
        let snapshot = OfflineSnapshot(revision: 0, text: composed, json: "[]", diagnostics: [])
        precondition(snapshot.applyingErrors(to: composed, revision: 0) == composed)
        precondition(snapshot.applyingErrors(to: decomposed, revision: 0) == nil)
        print("Swift exact UTF-8 snapshot regression passed")
    }
}
