import java.io.File
import org.json.JSONArray
import org.json.JSONObject
import uniffi.varnavinyas_bindings_uniffi.*

fun main(args: Array<String>) {
    val cases = JSONObject(File(args[0]).readText()).getJSONArray("cases")
    for (index in 0 until cases.length()) {
        val entry = cases.getJSONObject(index)
        val options = entry.getJSONObject("options")
        val mode = if (options.getString("orthography_mode") == "academy-strict")
            OrthographyMode.ACADEMY_STRICT else OrthographyMode.COMMON_EDITORIAL
        val actual = JSONArray(checkTextWithAllOptions(entry.getString("text"),
            options.getBoolean("grammar"), PunctuationMode.STRICT, mode, false))
        check(actual.similar(entry.getJSONArray("expected_diagnostics"))) { "Fixture failed: ${entry.getString("id")}" }
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
