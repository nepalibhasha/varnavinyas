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
}
