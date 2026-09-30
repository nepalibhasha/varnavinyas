import org.json.JSONArray
import uniffi.varnavinyas_bindings_uniffi.*
import java.nio.ByteBuffer
import java.nio.charset.CodingErrorAction

/** Prototype: retain the JSON; dictionary membership is a separate capability. */
data class OfflineDiagnostic(val start: Int, val end: Int, val incorrect: String,
                             val correction: String, val kind: String, val ruleCode: String)

data class OfflineSnapshot(val revision: Int, val text: String, val json: String,
                           val diagnostics: List<OfflineDiagnostic>) {
    /** Work in UTF-8 bytes, not Kotlin UTF-16 offsets, and reject stale snapshots. */
    fun applyingErrors(currentText: String, currentRevision: Int): String? {
        if (revision != currentRevision || text != currentText) return null
        val original = text.toByteArray(Charsets.UTF_8)
        var corrected = original
        var nextStart = original.size
        for (diagnostic in diagnostics.filter { it.kind == "Error" && it.ruleCode != "samasa-heuristic" }
            .sortedByDescending { it.start }) {
            val (start, end) = diagnostic
            if (start < 0 || end < start || end > nextStart ||
                !original.copyOfRange(start, end).contentEquals(diagnostic.incorrect.toByteArray(Charsets.UTF_8))) return null
            corrected = corrected.copyOfRange(0, start) + diagnostic.correction.toByteArray(Charsets.UTF_8) +
                corrected.copyOfRange(end, corrected.size)
            nextStart = start
        }
        return try {
            Charsets.UTF_8.newDecoder().onMalformedInput(CodingErrorAction.REPORT)
                .onUnmappableCharacter(CodingErrorAction.REPORT).decode(ByteBuffer.wrap(corrected)).toString()
        } catch (_: java.nio.charset.CharacterCodingException) { null }
    }
}

/** Run on Dispatchers.Default in the app; discard results when the revision changes. */
fun offlineCheck(text: String, revision: Int, mode: OrthographyMode = OrthographyMode.COMMON_EDITORIAL,
                 grammar: Boolean = false): OfflineSnapshot {
    val json = checkTextWithAllOptions(text, grammar, PunctuationMode.STRICT, mode, false)
    val array = JSONArray(json)
    val diagnostics = (0 until array.length()).map { index ->
        val d = array.getJSONObject(index)
        OfflineDiagnostic(d.getInt("span_start"), d.getInt("span_end"), d.getString("incorrect"),
            d.getString("correction"), d.getString("kind"), d.getString("rule_code"))
    }
    return OfflineSnapshot(revision, text, json, diagnostics)
}
