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
/// त्रिस्तरीय lookup:
/// 1. Override तालिका (dictionary/heuristic ले छुटाउन सक्ने किनाराका केस)
/// 2. Kosha lookup (~26K शब्द, Brihat Shabdakosha origin tag सहित)
/// 3. Heuristic वर्गीकरण (ध्वन्यात्मक ढाँचा)
pub fn classify(word: &str) -> Origin {
    classify_with_provenance(word).origin
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

    // 2. Kosha lookup (~26K शब्दमा origin tag)
    if let Some(tag) = varnavinyas_kosha::kosha().origin_of(word) {
        return OriginDecision {
            origin: tag,
            source: OriginSource::Kosha,
            confidence: 0.95,
        };
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
