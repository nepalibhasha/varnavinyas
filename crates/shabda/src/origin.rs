use crate::tables;
pub use varnavinyas_types::Origin;

/// शब्दउत्पत्ति वर्गीकरणको स्रोत (provenance)।
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OriginSource {
    /// स्थानीय override तालिकाबाट।
    Override,
    /// शब्दकोश origin tag बाट।
    Kosha,
    /// heuristic fallback नियमबाट।
    Heuristic,
}

/// शब्दउत्पत्ति निर्णय र provenance metadata।
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OriginDecision {
    pub origin: Origin,
    pub source: OriginSource,
    pub confidence: f32,
}

/// नेपाली शब्दलाई उत्पत्तिका आधारमा वर्गीकृत गर्ने।
///
/// चार-स्तरीय lookup:
/// 1. Override तालिका (dictionary/heuristic ले छुटाउन सक्ने किनाराका केस)
/// 2. Kosha lookup (~130K headwords, Brihat + Pragya Shabdakosha metadata सहित)
/// 2b. Suffix-stripped kosha lookup (विभक्ति/बहुवचन रूपका लागि)
/// 3. Heuristic वर्गीकरण (ध्वन्यात्मक ढाँचा)
pub fn classify(word: &str) -> Origin {
    classify_with_provenance(word).origin
}

/// विभक्ति र बहुवचन suffix हरू जुन stem lookup का लागि छुटाइन्छ।
/// लामो suffix पहिले राख्नु ताकि "हरूले" → stem सही निस्किओस् (greedy)।
const STRIP_SUFFIXES: &[&str] = &[
    "हरूलाई",
    "हरूको",
    "हरूले",
    "हरूबाट",
    "हरूमा",
    "हरूसँग",
    "भित्र",
    "प्रति",
    "देखि",
    "सँगै",
    "सम्म",
    "हरु",
    "हरू",
    "लाई",
    "बाट",
    "सँग",
    "तिर",
    "का",
    "की",
    "ले",
    "को",
    "मा",
];

fn suffix_rest_candidates<'a>(word: &'a str, suffix: &str) -> Vec<&'a str> {
    let mut candidates = Vec::new();
    let word_len = word.chars().count();
    let suffix_len = suffix.chars().count();

    if let Some(stem) = word.strip_suffix(suffix) {
        if !stem.is_empty() {
            candidates.push(stem);
        }
    }

    let mut suffix_chars = suffix.chars();
    let Some(onset) = suffix_chars.next() else {
        return candidates;
    };
    let tail = suffix_chars.as_str();
    if tail.is_empty() {
        return candidates;
    }

    if word_len > suffix_len {
        if let Some(stem) = word.strip_suffix(tail) {
            if !stem.is_empty() && stem.ends_with(onset) && !candidates.contains(&stem) {
                candidates.push(stem);
            }
        }
    }

    candidates
}

/// provenance र confidence सहित वर्गीकरण।
pub fn classify_with_provenance(word: &str) -> OriginDecision {
    if word.is_empty() {
        return OriginDecision {
            origin: Origin::Deshaj,
            source: OriginSource::Heuristic,
            confidence: 0.0,
        };
    }

    // 1. Override तालिका (हातैले प्रमाणीकरण गरिएका किनाराका केस)
    if let Some(origin) = tables::lookup_origin(word) {
        return OriginDecision {
            origin,
            source: OriginSource::Override,
            confidence: 1.0,
        };
    }

    // 2. Kosha lookup (~130K headwords; origin tag available when present)
    let kosha = varnavinyas_kosha::kosha();
    if let Some(tag) = kosha.origin_of(word) {
        return OriginDecision {
            origin: tag,
            source: OriginSource::Kosha,
            confidence: 0.95,
        };
    }
    // If the word is a known headword but has no explicit origin tag,
    // treat it as Deshaj by default (dictionary-backed fallback).
    if kosha.lookup(word).is_some() {
        return OriginDecision {
            origin: Origin::Deshaj,
            source: OriginSource::Kosha,
            confidence: 0.85,
        };
    }

    // 2b. Suffix-stripped kosha lookup — विभक्ति/बहुवचन रूपलाई stem मा घटाएर हेर्ने।
    // उदाहरण: "संस्कृतमा" → "संस्कृत" ([सं.] → Tatsam), confidence थोरै कम।
    for suffix in STRIP_SUFFIXES {
        for stem in suffix_rest_candidates(word, suffix) {
            // कम्तिमा २ अक्षर नभए stem ग्राह्य छैन
            if stem.chars().count() > 1 {
                if let Some(tag) = kosha.origin_of(stem) {
                    return OriginDecision {
                        origin: tag,
                        source: OriginSource::Kosha,
                        confidence: 0.90,
                    };
                }
                if kosha.lookup(stem).is_some() {
                    return OriginDecision {
                        origin: Origin::Deshaj,
                        source: OriginSource::Kosha,
                        confidence: 0.80,
                    };
                }
            }
        }
    }

    // 3. Heuristic वर्गीकरण
    OriginDecision {
        origin: classify_heuristic(word),
        source: OriginSource::Heuristic,
        confidence: 0.65,
    }
}

fn classify_heuristic(word: &str) -> Origin {
    let chars: Vec<char> = word.chars().collect();

    // आगन्तुक संकेत: विदेशी व्यञ्जन समूह, नुक्ता रूप
    if has_aagantuk_markers(&chars) {
        return Origin::Aagantuk;
    }

    // तत्सम संकेत: ऋ, ष, क्ष, ज्ञ, विसर्ग, विशेष संयुक्ताक्षर
    if has_tatsam_markers(word, &chars) {
        return Origin::Tatsam;
    }

    // तद्भव ढाँचा: सरल ध्वनि-रूपान्तरण
    if has_tadbhav_markers(word, &chars) {
        return Origin::Tadbhav;
    }

    // Default: देशज (मूल नेपाली)
    Origin::Deshaj
}

fn has_aagantuk_markers(chars: &[char]) -> bool {
    // नुक्ता रूप (क़ ख़ ग़ ज़ ड़ ढ़ फ़)
    for c in chars {
        if matches!(
            c,
            '\u{0958}'..='\u{095F}' // precomposed नुक्ता व्यञ्जन
        ) {
            return true;
        }
    }

    // combining नुक्ता चिह्न जाँच्ने
    for window in chars.windows(2) {
        if window[1] == '\u{093C}' {
            // व्यञ्जनपछि आउने ़ (नुक्ता)
            return true;
        }
    }

    false
}

fn has_tatsam_markers(word: &str, chars: &[char]) -> bool {
    // प्रत्यक्ष तत्सम स्वर: ऋ
    if chars.contains(&'ऋ') || chars.contains(&'ृ') {
        return true;
    }

    // ष (मूर्धन्य सिबिलेन्ट) — प्रबल तत्सम संकेत
    if chars.contains(&'ष') {
        return true;
    }

    // विसर्ग ः
    if chars.contains(&'ः') {
        return true;
    }

    // संयुक्ताक्षर: क्ष, ज्ञ
    if word.contains("क्ष") || word.contains("ज्ञ") || word.contains("क्त") || word.contains("त्म")
    {
        return true;
    }

    // श्र/त्र/त्त/द्ध/द्य/द्व जस्ता तत्सम-उन्मुख संयुक्ताक्षर
    // नोट: ध्य यहाँ राखिएको छैन — अँध्यार जस्ता देशज परिवारमा पनि यो क्लस्टर आउँछ।
    if word.contains("त्र")
        || word.contains("त्त")
        || word.contains("द्ध")
        || word.contains("द्य")
        || word.contains("द्व")
    {
        return true;
    }

    false
}

/// शब्दको स्रोत भाषा खोज्ने (जस्तै: "फारसी", "अरबी", "संस्कृत")।
///
/// kosha को origin tag प्रयोग हुन्छ। शब्दमा मान्य भाषा tag नभए `None`।
pub fn source_language(word: &str) -> Option<&'static str> {
    varnavinyas_kosha::kosha().source_language_of(word)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// अँध्यार परिवार देशज हो — ध्य क्लस्टर भए पनि तत्सम होइन।
    /// Regression: adding ध्य to tatsam markers caused false positives here.
    #[test]
    fn andhyaar_family_is_deshaj_not_tatsam() {
        // Base form (in words.txt; kosha lookup handles this)
        let d = classify_with_provenance("अँध्यार");
        assert_ne!(
            d.origin,
            Origin::Tatsam,
            "अँध्यार should not be Tatsam (got {:?})",
            d
        );

        // Inflected forms that fall through to heuristic
        for word in &["अँध्यारोपन", "अँध्यारिन्थ्यो"]
        {
            let d = classify_with_provenance(word);
            assert_ne!(
                d.origin,
                Origin::Tatsam,
                "{word} should not be Tatsam (got {:?})",
                d
            );
        }
    }

    /// अध्ययन kosha मा [सं.] tag सहित छ — heuristic बिनै पनि Tatsam निस्कन्छ।
    #[test]
    fn adhyayan_is_tatsam_via_kosha() {
        let d = classify_with_provenance("अध्ययन");
        assert_eq!(d.origin, Origin::Tatsam, "अध्ययन should be Tatsam");
        assert_eq!(d.source, OriginSource::Kosha, "should come from kosha");
    }
}

fn has_tadbhav_markers(word: &str, chars: &[char]) -> bool {
    // तद्भवमा देखिने प्रचलित अन्त्य: -ो, -ा
    let last = chars.last().copied().unwrap_or('\0');
    let second_last = if chars.len() >= 2 {
        chars[chars.len() - 2]
    } else {
        '\0'
    };

    // तद्भव क्रियामा देखिने प्रचलित अन्त्य
    if word.ends_with("नु") || word.ends_with("ने") || word.ends_with("को") {
        return true;
    }

    // तद्भवको बोलिचालि/लघुरूप अन्त्य
    if last == 'ो' || (second_last != '\0' && matches!(last, 'ो' | 'ा')) {
        // -ठो/-ठा अन्त्य धेरैजसो तद्भव रूप हुन्छ
        if matches!(second_last, 'ठ' | 'ड' | 'ढ') {
            return true;
        }
    }

    false
}
