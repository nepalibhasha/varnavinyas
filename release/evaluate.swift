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
    }
}
