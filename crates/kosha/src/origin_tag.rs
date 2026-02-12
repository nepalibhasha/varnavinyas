//! Parse word origin from dictionary POS/tag fields.
//!
//! Maps abbreviation tags from the Nepali Brihat Shabdakosha (सङ्केतसूची)
//! to the four-category origin classification used by the orthography rules.

/// Origin classification (mirrors `shabda::Origin` but avoids circular dep).
/// Converted to shabda::Origin at the shabda crate boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OriginTag {
    Tatsam,
    Tadbhav,
    Deshaj,
    Aagantuk,
}

/// Extract the first origin tag from a headword's POS/metadata field.
///
/// The field format varies:
///   - `[सं.] ना.`       — tag before POS
///   - `ना. [फा.]`       — tag after POS
///   - `ना. [अ.],वि. [सं.]` — multiple tags (we take the first)
///
/// # Bracket invariant
///
/// This function assumes that in `headwords.tsv`, square brackets `[…]` are
/// used **exclusively** for origin/etymology annotations — never for POS
/// labels or other non-origin metadata. POS labels (ना., वि., क्रि.वि., etc.)
/// always appear unbracketed. This invariant is validated by the
/// `bracket_invariant_all_brackets_are_origin_or_known` test in `tests/lookup.rs`.
///
/// Returns `None` if no recognized origin tag is found.
pub fn parse_origin_tag(pos_field: &str) -> Option<OriginTag> {
    // Find first '[' ... ']' bracket pair (safe due to bracket invariant above)
    let start = pos_field.find('[')?;
    let end = pos_field[start..].find(']')? + start;
    let tag_content = pos_field[start + '['.len_utf8()..end].trim();

    classify_tag(tag_content)
}

/// Map a bracket-interior tag string to an OriginTag.
fn classify_tag(tag: &str) -> Option<OriginTag> {
    // Normalize: strip trailing periods and whitespace
    let normalized = tag.trim().trim_end_matches('.');

    // Sanskrit origin tags:
    // - "सं" or "सं." alone → Tatsam (direct Sanskrit, unchanged form)
    // - "सं. X" with etymological root → Tadbhav (derived from Sanskrit X)
    //   e.g., यहाँ [सं. इह], अकास [सं. आकाश], अगाडि [सं. अग्र+आडि]
    if normalized == "सं" {
        return Some(OriginTag::Tatsam);
    }
    if normalized.starts_with("सं.") || normalized.starts_with("सं ") {
        // Check if there's an etymological root after "सं."
        let after = normalized
            .strip_prefix("सं.")
            .or_else(|| normalized.strip_prefix("सं "))
            .unwrap_or("")
            .trim();
        if after.is_empty() {
            // Just [सं.] — true tatsam
            return Some(OriginTag::Tatsam);
        } else {
            // [सं. इह] — has etymological root → tadbhav (derived from Sanskrit)
            return Some(OriginTag::Tadbhav);
        }
    }

    // Foreign / Aagantuk — exact match OR prefix + etymological root
    // e.g., [अ.] or [अ. इकवाल], [फा.] or [फा. अङ्गुर]
    static AAGANTUK_PREFIXES: &[&str] = &[
        "अ.",   // Arabic (अरबी) — must check before अङ्/अड्
        "अ ",   // Arabic without period
        "अङ्",   // English (अङ्ग्रेजी)
        "अङ.",  // English variant
        "अङ",   // English (after period stripping: अङ. → अङ)
        "अड्",   // English variant
        "फा",   // Persian (फारसी)
        "तु",    // Turkish (तुर्की)
        "था",   // Tharu (थारू)
        "फ्रा",  // French (फ्रान्सेली)
        "फ्रे",   // French variant
        "पोर्त", // Portuguese
        "ग्री",  // Greek
        "स्पे",   // Spanish
        "जापा", // Japanese
        "भा. इ",
        "भा.इ", // Indo-Iranian
    ];
    for prefix in AAGANTUK_PREFIXES {
        if normalized == *prefix || normalized.starts_with(prefix) {
            return Some(OriginTag::Aagantuk);
        }
    }
    // Bare "अ" (no period, no trailing content) — Arabic
    if normalized == "अ" {
        return Some(OriginTag::Aagantuk);
    }

    // Tadbhav (related Indic languages / Prakrit) — exact or prefix
    // e.g., [प्रा.] or [प्रा. अक्खआडा > सं. अक्षवाट]
    static TADBHAV_PREFIXES: &[&str] = &[
        "प्रा", // Prakrit (प्राकृत)
        "हि",  // Hindi
        "भो",  // Bhojpuri (भोजपुरी) — covers भो. ब, भो. पु., etc.
        "मरा", // Marathi
        "मै",   // Maithili
    ];
    for prefix in TADBHAV_PREFIXES {
        if normalized == *prefix || normalized.starts_with(prefix) {
            return Some(OriginTag::Tadbhav);
        }
    }

    // Deshaj (native / regional Nepali languages) — prefix match handles
    // both bare tags [नेवा.] and tags with etymological roots [नेवा. अलःकै]
    static DESHAJ_PREFIXES: &[&str] = &[
        "नेवा", // Newari (नेवारी)
        "लि",  // Limbu (लिम्बू)
        "मो",  // Tibeto-Burman (मो. ब = मोङ्गोल बर्मेली)
        "मगा", // Magar (मगार)
        "डो",  // Doteli (डोटेली)
        "बा",  // Children's language (बा. बो = बालबोली)
    ];
    for prefix in DESHAJ_PREFIXES {
        if normalized == *prefix || normalized.starts_with(prefix) {
            return Some(OriginTag::Deshaj);
        }
    }

    // Etymological derivation tags: "X ८ सं. Y" means "X derived from Sanskrit Y"
    // These are tadbhav (form has changed from the original)
    if tag.contains("८ सं") || tag.contains("८सं") {
        return Some(OriginTag::Tadbhav);
    }
    if tag.contains("८ अ") || tag.contains("८ फा") || tag.contains("८ पोर्त")
    {
        return Some(OriginTag::Aagantuk);
    }

    // Unknown or verb-root tags (√ ...) — no origin
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanskrit_tag() {
        assert_eq!(parse_origin_tag("[सं.] ना."), Some(OriginTag::Tatsam));
        assert_eq!(parse_origin_tag("[ सं.] ना."), Some(OriginTag::Tatsam));
    }

    #[test]
    fn test_persian_tag() {
        assert_eq!(parse_origin_tag("ना. [फा.]"), Some(OriginTag::Aagantuk));
    }

    #[test]
    fn test_english_tag() {
        assert_eq!(parse_origin_tag("ना. [अङ्.]"), Some(OriginTag::Aagantuk));
        assert_eq!(parse_origin_tag("ना. [अड्]"), Some(OriginTag::Aagantuk));
        assert_eq!(parse_origin_tag("ना. [अड्.]"), Some(OriginTag::Aagantuk));
    }

    #[test]
    fn test_hindi_tag() {
        assert_eq!(parse_origin_tag("क्रि.वि. [हि.]"), Some(OriginTag::Tadbhav));
    }

    #[test]
    fn test_newari_tag() {
        assert_eq!(parse_origin_tag("ना. [नेवा.]"), Some(OriginTag::Deshaj));
    }

    #[test]
    fn test_no_origin_tag() {
        assert_eq!(parse_origin_tag("ना."), None);
        assert_eq!(parse_origin_tag("क.क्रि."), None);
        assert_eq!(parse_origin_tag(""), None);
    }

    #[test]
    fn test_multiple_tags_takes_first() {
        // "ना. [अ.],वि. [सं.]" → first is Arabic → Aagantuk
        assert_eq!(
            parse_origin_tag("ना. [अ.],वि. [सं.]"),
            Some(OriginTag::Aagantuk)
        );
    }

    #[test]
    fn test_sanskrit_with_root_is_tadbhav() {
        // [सं. X] with etymological root = tadbhav (derived from Sanskrit)
        assert_eq!(parse_origin_tag("[सं. दण्ड] ना."), Some(OriginTag::Tadbhav));
        assert_eq!(
            parse_origin_tag("क्रि.वि. [सं. इह]"),
            Some(OriginTag::Tadbhav)
        );
        assert_eq!(parse_origin_tag("ना. [सं. आकाश]"), Some(OriginTag::Tadbhav));
    }

    #[test]
    fn test_etymological_derivation() {
        // "कपास ८ सं. कर्पास" — derived from Sanskrit → tadbhav
        assert_eq!(
            parse_origin_tag("[कपास ८ सं. कर्पास]"),
            Some(OriginTag::Tadbhav)
        );
    }
}
