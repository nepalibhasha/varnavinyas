//! Parse word origin from dictionary POS/tag fields.
//!
//! Maps abbreviation tags from the Nepali Brihat Shabdakosha (सङ्केतसूची)
//! to the four-category origin classification used by the orthography rules.

/// Shared origin classification type.
pub use varnavinyas_types::Origin as OriginTag;

struct TagEntry {
    prefixes: &'static [&'static str],
    origin: OriginTag,
    source_language: &'static str,
}

// INVARIANT: ordered by longest/specific prefixes first to avoid overlaps
// (e.g., "भो. ब" before "भो", "अङ्" before "अ").
static TAG_TABLE: &[TagEntry] = &[
    TagEntry {
        prefixes: &["भो. ब", "भो.ब"],
        origin: OriginTag::Tadbhav,
        source_language: "भोट-बर्मेली",
    },
    TagEntry {
        prefixes: &["अङ्", "अङ.", "अङ", "अड्"],
        origin: OriginTag::Aagantuk,
        source_language: "अङ्ग्रेजी",
    },
    TagEntry {
        prefixes: &["भा. इ", "भा.इ"],
        origin: OriginTag::Aagantuk,
        source_language: "भारत-इरानेली",
    },
    TagEntry {
        prefixes: &["फ्रा", "फ्रे"],
        origin: OriginTag::Aagantuk,
        source_language: "फ्रान्सेली",
    },
    TagEntry {
        prefixes: &["पोर्त"],
        origin: OriginTag::Aagantuk,
        source_language: "पोर्तगाली",
    },
    TagEntry {
        prefixes: &["जापा"],
        origin: OriginTag::Aagantuk,
        source_language: "जापानी",
    },
    TagEntry {
        prefixes: &["चिनि"],
        origin: OriginTag::Aagantuk,
        source_language: "चिनियाँ",
    },
    TagEntry {
        prefixes: &["स्पे"],
        origin: OriginTag::Aagantuk,
        source_language: "स्पेनिस",
    },
    TagEntry {
        prefixes: &["ग्री"],
        origin: OriginTag::Aagantuk,
        source_language: "ग्रीक",
    },
    TagEntry {
        prefixes: &["तामा"],
        origin: OriginTag::Deshaj,
        source_language: "तामाङ्गी",
    },
    TagEntry {
        prefixes: &["धिमा"],
        origin: OriginTag::Deshaj,
        source_language: "धिमाल",
    },
    TagEntry {
        prefixes: &["नेवा"],
        origin: OriginTag::Deshaj,
        source_language: "नेवारी",
    },
    TagEntry {
        prefixes: &["मरा"],
        origin: OriginTag::Tadbhav,
        source_language: "मराठी",
    },
    TagEntry {
        prefixes: &["उडि"],
        origin: OriginTag::Tadbhav,
        source_language: "उडिया",
    },
    TagEntry {
        prefixes: &["प्रा"],
        origin: OriginTag::Tadbhav,
        source_language: "प्राकृत",
    },
    TagEntry {
        prefixes: &["मै"],
        origin: OriginTag::Tadbhav,
        source_language: "मैथिली",
    },
    TagEntry {
        prefixes: &["फा"],
        origin: OriginTag::Aagantuk,
        source_language: "फारसी",
    },
    TagEntry {
        prefixes: &["तु"],
        origin: OriginTag::Aagantuk,
        source_language: "तुर्की",
    },
    TagEntry {
        prefixes: &["था"],
        origin: OriginTag::Aagantuk,
        source_language: "थारू",
    },
    TagEntry {
        prefixes: &["भो"],
        origin: OriginTag::Tadbhav,
        source_language: "भोजपुरी",
    },
    TagEntry {
        prefixes: &["हि"],
        origin: OriginTag::Tadbhav,
        source_language: "हिन्दी",
    },
    TagEntry {
        prefixes: &["मग"],
        origin: OriginTag::Deshaj,
        source_language: "मगराँती",
    },
    TagEntry {
        prefixes: &["डो"],
        origin: OriginTag::Deshaj,
        source_language: "डोटेली",
    },
    TagEntry {
        prefixes: &["दङ"],
        origin: OriginTag::Deshaj,
        source_language: "दङाली",
    },
    TagEntry {
        prefixes: &["लि"],
        origin: OriginTag::Deshaj,
        source_language: "लिम्बू",
    },
    TagEntry {
        prefixes: &["मो"],
        origin: OriginTag::Deshaj,
        source_language: "भोट-बर्मेली",
    },
    TagEntry {
        prefixes: &["बा"],
        origin: OriginTag::Deshaj,
        source_language: "बालबोली",
    },
    TagEntry {
        prefixes: &["अ.", "अ ", "अ"],
        origin: OriginTag::Aagantuk,
        source_language: "अरबी",
    },
];

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
    parse_tag_metadata(tag).map(|(origin, _)| origin)
}

/// Extract the human-readable source language name from a headword's POS/tag field.
///
/// For example: `[फा.]` → `"फारसी"`, `[अङ्.]` → `"अङ्ग्रेजी"`, `[सं.]` → `"संस्कृत"`.
/// Returns `None` if no recognized language tag is found.
pub fn parse_source_language(pos_field: &str) -> Option<&'static str> {
    let start = pos_field.find('[')?;
    let end = pos_field[start..].find(']')? + start;
    let tag = pos_field[start + '['.len_utf8()..end].trim();
    source_language_from_tag(tag)
}

fn source_language_from_tag(tag: &str) -> Option<&'static str> {
    parse_tag_metadata(tag).map(|(_, source)| source)
}

fn parse_tag_metadata(tag: &str) -> Option<(OriginTag, &'static str)> {
    let normalized = tag.trim().trim_end_matches('.');

    // Sanskrit origin tags:
    // - "सं" or "सं." alone → Tatsam (direct Sanskrit, unchanged form)
    // - "सं. X" with etymological root → Tadbhav (derived from Sanskrit X)
    if normalized == "सं" {
        return Some((OriginTag::Tatsam, "संस्कृत"));
    }
    if normalized.starts_with("सं.") || normalized.starts_with("सं ") {
        let after = normalized
            .strip_prefix("सं.")
            .or_else(|| normalized.strip_prefix("सं "))
            .unwrap_or("")
            .trim();

        if after.is_empty() {
            return Some((OriginTag::Tatsam, "संस्कृत"));
        } else {
            return Some((OriginTag::Tadbhav, "संस्कृत"));
        }
    }

    if let Some(entry) = match_tag_table(normalized) {
        return Some((entry.origin, entry.source_language));
    }

    // Etymological derivation tags: "X ८ सं. Y" means derived from Sanskrit.
    if tag.contains("८ सं") || tag.contains("८सं") {
        return Some((OriginTag::Tadbhav, "संस्कृत"));
    }
    if tag.contains("८ अ") {
        return Some((OriginTag::Aagantuk, "अरबी"));
    }
    if tag.contains("८ फा") {
        return Some((OriginTag::Aagantuk, "फारसी"));
    }
    if tag.contains("८ पोर्त") {
        // Keep legacy spelling for compatibility with existing output behavior.
        return Some((OriginTag::Aagantuk, "पोर्तुगाली"));
    }

    None
}

fn match_tag_table(normalized: &str) -> Option<&'static TagEntry> {
    TAG_TABLE.iter().find(|entry| {
        entry
            .prefixes
            .iter()
            .any(|prefix| normalized == *prefix || normalized.starts_with(prefix))
    })
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

    #[test]
    fn test_overlap_prefers_bho_b_before_bho() {
        assert_eq!(parse_origin_tag("ना. [भो. ब.]"), Some(OriginTag::Tadbhav));
        assert_eq!(parse_source_language("ना. [भो. ब.]"), Some("भोट-बर्मेली"));
    }

    #[test]
    fn test_overlap_prefers_ang_before_a() {
        assert_eq!(parse_origin_tag("ना. [अङ्.]"), Some(OriginTag::Aagantuk));
        assert_eq!(parse_source_language("ना. [अङ्.]"), Some("अङ्ग्रेजी"));
    }

    // --- parse_source_language tests ---

    #[test]
    fn test_source_language_persian() {
        assert_eq!(parse_source_language("ना. [फा.]"), Some("फारसी"));
    }

    #[test]
    fn test_source_language_arabic() {
        assert_eq!(parse_source_language("ना. [अ.]"), Some("अरबी"));
    }

    #[test]
    fn test_source_language_english() {
        assert_eq!(parse_source_language("ना. [अङ्.]"), Some("अङ्ग्रेजी"));
    }

    #[test]
    fn test_source_language_sanskrit_tatsam() {
        assert_eq!(parse_source_language("[सं.] ना."), Some("संस्कृत"));
    }

    #[test]
    fn test_source_language_sanskrit_tadbhav() {
        assert_eq!(parse_source_language("[सं. दण्ड] ना."), Some("संस्कृत"));
    }

    #[test]
    fn test_source_language_newari() {
        assert_eq!(parse_source_language("ना. [नेवा.]"), Some("नेवारी"));
    }

    #[test]
    fn test_source_language_hindi() {
        assert_eq!(parse_source_language("क्रि.वि. [हि.]"), Some("हिन्दी"));
    }

    #[test]
    fn test_source_language_none() {
        assert_eq!(parse_source_language("ना."), None);
        assert_eq!(parse_source_language(""), None);
    }
}
