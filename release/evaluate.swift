import Foundation

@main struct Evaluation {
    static func main() throws {
        let data = try Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1]))
        let fixtures = try JSONSerialization.jsonObject(with: data) as! [String: Any]
        let cases = fixtures["cases"] as! [[String: Any]]
        for entry in cases {
            let options = entry["options"] as! [String: Any]
            let mode: OrthographyMode = options["orthography_mode"] as! String == "academy-strict" ? .academyStrict : .commonEditorial
            let output = checkTextWithAllOptions(text: entry["text"] as! String,
                grammar: options["grammar"] as! Bool, punctuationMode: .strict,
                orthographyMode: mode, includeNoopHeuristics: false)
            let actual = try JSONSerialization.jsonObject(with: Data(output.utf8)) as! NSArray
            precondition(actual.isEqual(to: entry["expected_diagnostics"] as! [Any]), "Fixture failed: \(entry["id"]!)")
        }
        print("Swift generated bindings: \(cases.count) shared diagnostic fixtures passed")
        let originData = try Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[2]))
        let originFixtures = try JSONSerialization.jsonObject(with: originData) as! [String: Any]
        let originCases = originFixtures["cases"] as! [[String: Any]]
        for entry in originCases {
            let word = entry["word"] as! String
            let expected = entry["expected"] as! [String: Any]
            let actual = classifyWithProvenance(word: word)
            precondition(actual.origin.map { String(describing: $0).lowercased() } == expected["origin"] as? String, "Origin: \(entry["id"]!)")
            precondition(String(describing: actual.source).lowercased() == expected["source"] as! String)
            precondition(abs(Double(actual.confidence) - (expected["confidence"] as! NSNumber).doubleValue) < 1e-6)
            precondition(String(describing: classify(word: word)).lowercased() == entry["legacy_origin"] as! String)
        }
        print("Swift generated bindings: \(originCases.count) shared origin fixtures passed")
    }
}
