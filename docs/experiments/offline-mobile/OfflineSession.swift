import Foundation

/// Prototype: keep raw diagnostics and provenance separate from dictionary lookup results.
struct OfflineDiagnostic: Decodable {
    let span_start: Int
    let span_end: Int
    let incorrect: String
    let correction: String
    let kind: String
    let rule_code: String
}

struct OfflineSnapshot {
    let revision: Int
    let text: String
    let json: String
    let diagnostics: [OfflineDiagnostic]

    /// Byte splicing avoids confusing Swift graphemes with Rust UTF-8 offsets.
    /// Reject the entire operation if the editor changed or any range is invalid.
    func applyingErrors(to currentText: String, revision currentRevision: Int) -> String? {
        guard revision == currentRevision, Array(text.utf8) == Array(currentText.utf8) else { return nil }
        let original = Array(text.utf8)
        var corrected = original
        var nextStart = original.count
        for diagnostic in diagnostics.filter({ $0.kind == "Error" && $0.rule_code != "samasa-heuristic" })
            .sorted(by: { $0.span_start > $1.span_start }) {
            let start = diagnostic.span_start
            let end = diagnostic.span_end
            guard start >= 0, end >= start, end <= nextStart,
                Array(original[start..<end]) == Array(diagnostic.incorrect.utf8) else { return nil }
            corrected.replaceSubrange(start..<end, with: diagnostic.correction.utf8)
            nextStart = start
        }
        return String(bytes: corrected, encoding: .utf8)
    }
}

/// Call from a background executor in the app. A native call is synchronous;
/// cancellation should discard its result rather than assume it interrupts Rust.
func offlineCheck(_ text: String, revision: Int, mode: OrthographyMode = .commonEditorial,
                  grammar: Bool = false) throws -> OfflineSnapshot {
    let json = checkTextWithAllOptions(text: text, grammar: grammar, punctuationMode: .strict,
        orthographyMode: mode, includeNoopHeuristics: false)
    let diagnostics = try JSONDecoder().decode([OfflineDiagnostic].self, from: Data(json.utf8))
    return OfflineSnapshot(revision: revision, text: text, json: json, diagnostics: diagnostics)
}
