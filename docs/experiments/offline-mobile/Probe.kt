import uniffi.varnavinyas_bindings_uniffi.*

fun main(args: Array<String>) {
    val start = System.nanoTime()
    offlineCheck("हामि", 0)
    println("cold_check_ms=${(System.nanoTime() - start) / 1e6}")
    evaluateFixtures(args)
    val text = "😀 हामि\nहामि संघीय"
    val snapshot = offlineCheck(text, 1)
    check(snapshot.applyingErrors(text, 1) == "😀 हामी\nहामी संघीय")
    check(snapshot.applyingErrors("नेपाल", 1) == null)
    check(snapshot.applyingErrors(text, 2) == null)
    val strict = offlineCheck("संघीय", 3, OrthographyMode.ACADEMY_STRICT)
    check(strict.applyingErrors(strict.text, 3) == "सङ्घीय")
    val grammar = offlineCheck("सूर्योदय भयो।", 4, grammar = true)
    check(grammar.applyingErrors(grammar.text, 4) == grammar.text)
    val whitespace = offlineCheck("नेपाल , राम्रो", 5)
    check(whitespace.applyingErrors(whitespace.text, 5) == "नेपाल, राम्रो")
    val invalid = OfflineSnapshot(6, "नेपाल", "[]", listOf(OfflineDiagnostic(0, 12, "हामि", "हामी", "Error", "test")))
    check(invalid.applyingErrors(invalid.text, 6) == null)
    val unknown = offlineCheck("नेपाले", 7)
    check(unknown.diagnostics.isEmpty() && classifyWithProvenance("नेपाले").origin == null)
    println("offline_adapter_checks=8 passed")
    for (repetitions in listOf(1, 30, 300)) {
        val sample = "हामि नेपाल संघीय सरकार। ".repeat(repetitions)
        val samples = (0 until 20).map {
            val start = System.nanoTime()
            offlineCheck(sample, 8)
            (System.nanoTime() - start) / 1e6
        }.sorted()
        println("utf8_bytes=${sample.toByteArray(Charsets.UTF_8).size} median_ms=${samples[10]} p95_ms=${samples[18]}")
    }
}
