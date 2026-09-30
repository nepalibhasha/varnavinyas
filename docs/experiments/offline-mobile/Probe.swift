import Foundation

@main struct Probe {
    static func main() throws {
        setbuf(stdout, nil)
        let start = DispatchTime.now().uptimeNanoseconds
        _ = try offlineCheck("हामि", revision: 0)
        print("cold_check_ms=\(Double(DispatchTime.now().uptimeNanoseconds - start) / 1e6)")
        try Evaluation.main()
        let text = "😀 हामि\nहामि संघीय"
        let snapshot = try offlineCheck(text, revision: 1)
        precondition(snapshot.applyingErrors(to: text, revision: 1) == "😀 हामी\nहामी संघीय")
        precondition(snapshot.applyingErrors(to: "नेपाल", revision: 1) == nil)
        precondition(snapshot.applyingErrors(to: text, revision: 2) == nil)
        let strict = try offlineCheck("संघीय", revision: 3, mode: .academyStrict)
        precondition(strict.applyingErrors(to: strict.text, revision: 3) == "सङ्घीय")
        let grammar = try offlineCheck("सूर्योदय भयो।", revision: 4, grammar: true)
        precondition(grammar.applyingErrors(to: grammar.text, revision: 4) == grammar.text)
        let whitespace = try offlineCheck("नेपाल , राम्रो", revision: 5)
        precondition(whitespace.applyingErrors(to: whitespace.text, revision: 5) == "नेपाल, राम्रो")
        let invalid = OfflineSnapshot(revision: 6, text: "नेपाल", json: "[]", diagnostics: [
            OfflineDiagnostic(span_start: 0, span_end: 12, incorrect: "हामि", correction: "हामी", kind: "Error", rule_code: "test")])
        precondition(invalid.applyingErrors(to: invalid.text, revision: 6) == nil)
        let unknown = try offlineCheck("नेपाले", revision: 7)
        precondition(unknown.diagnostics.isEmpty && classifyWithProvenance(word: "नेपाले").origin == nil)
        print("offline_adapter_checks=8 passed")
        for repetitions in [1, 30, 300] {
            let sample = String(repeating: "हामि नेपाल संघीय सरकार। ", count: repetitions)
            var samples: [Double] = []
            for _ in 0..<20 {
                let start = DispatchTime.now().uptimeNanoseconds
                _ = try offlineCheck(sample, revision: 8)
                samples.append(Double(DispatchTime.now().uptimeNanoseconds - start) / 1e6)
            }
            samples.sort()
            print("utf8_bytes=\(sample.utf8.count) median_ms=\(samples[10]) p95_ms=\(samples[18])")
        }
    }
}
