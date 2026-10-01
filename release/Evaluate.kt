import java.io.File
import org.json.JSONArray
import org.json.JSONObject
import uniffi.varnavinyas_bindings_uniffi.*

// Android's platform org.json lacks JSONArray.similar(). Compare recursively
// so the same harness runs on both Android and the host JVM.
fun equivalentJson(actual: Any?, expected: Any?): Boolean = when {
    actual is Number && expected is Number ->
        kotlin.math.abs(actual.toDouble() - expected.toDouble()) < 1e-6
    actual is JSONObject && expected is JSONObject -> {
        val keys = actual.keys().asSequence().toSet()
        keys == expected.keys().asSequence().toSet() &&
            keys.all { equivalentJson(actual.get(it), expected.get(it)) }
    }
    actual is JSONArray && expected is JSONArray ->
        actual.length() == expected.length() &&
            (0 until actual.length()).all { equivalentJson(actual.get(it), expected.get(it)) }
    else -> actual == expected
}

fun main(args: Array<String>) {
    for ((word, form, lemma) in listOf(Triple("खोजिरहेको", "खोजि", "खोज्नु"),
        Triple("गइरहेकी", "गइ", "जानु"), Triple("भइरहेका", "भइ", "हुनु"),
        Triple("दिइरहेको", "दिइ", "दिनु"))) {
        val reading = analyzeProgressive(word)!!
        check(reading.mainForm == form && reading.mainLemma == lemma)
        check(reading.auxiliaryLemma == "रहनु")
        check(reading.mainForm + reading.auxiliaryForm == word)
    }
    check(analyzeProgressive("झझझिरहेकी") == null)
    println("Kotlin generated bindings: progressive analysis checks passed")
    check(equivalentJson(JSONObject("{\"a\":1,\"b\":[null,2]}"), JSONObject("{\"b\":[null,2.0],\"a\":1.0}")))
    check(!equivalentJson(JSONArray("[1,2]"), JSONArray("[2,1]")))
    check(!equivalentJson(JSONObject("{\"a\":1}"), JSONObject("{\"a\":1,\"b\":2}")))
    val cases = JSONObject(File(args[0]).readText()).getJSONArray("cases")
    for (index in 0 until cases.length()) {
        val entry = cases.getJSONObject(index)
        val options = entry.getJSONObject("options")
        val mode = if (options.getString("orthography_mode") == "academy-strict")
            OrthographyMode.ACADEMY_STRICT else OrthographyMode.COMMON_EDITORIAL
        val actual = JSONArray(checkTextWithAllOptions(entry.getString("text"),
            options.getBoolean("grammar"), PunctuationMode.STRICT, mode, false))
        check(equivalentJson(actual, entry.getJSONArray("expected_diagnostics"))) { "Fixture failed: ${entry.getString("id")}" }
    }
    println("Kotlin generated bindings: ${cases.length()} shared diagnostic fixtures passed")
    val originCases = JSONObject(File(args[1]).readText()).getJSONArray("cases")
    for (index in 0 until originCases.length()) {
        val entry = originCases.getJSONObject(index)
        val expected = entry.getJSONObject("expected")
        val actual = classifyWithProvenance(entry.getString("word"))
        val expectedOrigin = if (expected.isNull("origin")) null else expected.getString("origin")
        check(actual.origin?.name?.lowercase() == expectedOrigin) { "Origin: ${entry.getString("id")}" }
        check(actual.source.name.lowercase() == expected.getString("source"))
        check(kotlin.math.abs(actual.confidence.toDouble() - expected.getDouble("confidence")) < 1e-6)
        check(classify(entry.getString("word")).name.lowercase() == entry.getString("legacy_origin"))
    }
    println("Kotlin generated bindings: ${originCases.length()} shared origin fixtures passed")
}
