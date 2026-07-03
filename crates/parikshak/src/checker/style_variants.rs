use std::collections::HashSet;

use varnavinyas_prakriya::{DiagnosticKind, Rule};

use crate::diagnostic::{Diagnostic, DiagnosticCategory};
use crate::tokenizer::AnalyzedToken;

use super::common::{is_word_boundary, overlaps_existing_span};

const INFERRED_KO_KA_RULE_CODE: &str = "section4-phrase-style-inferred-ko-ka";
const INFERRED_KO_KA_FOLLOWERS: &[(&str, &str)] = &[
    ("लागि", "लागि अघि -को भन्दा -का शैलीगत रूपमा उपयुक्त मानिन्छ"),
    ("निम्ति", "निम्ति अघि -को भन्दा -का शैलीगत रूपमा उपयुक्त मानिन्छ"),
    (
        "सम्बन्धमा",
        "सम्बन्धमा अघि -को भन्दा -का शैलीगत रूपमा उपयुक्त मानिन्छ",
    ),
    ("आधारमा", "आधारमा अघि -को भन्दा -का शैलीगत रूपमा उपयुक्त मानिन्छ"),
    ("बारेमा", "बारेमा अघि -को भन्दा -का शैलीगत रूपमा उपयुक्त मानिन्छ"),
];

/// Section 4 phrase/sentence-level style variants.
/// These are guidance suggestions, not hard errors.
const STYLE_VARIANT_CORRECTIONS: &[(&str, &str, &str)] = &[
    (
        "मर्माहित भएको",
        "मर्माहत भएको",
        "शब्द-रूपगत प्रयोगमा मर्माहत रूप उपयुक्त हुन्छ",
    ),
    (
        "निर्देशित गरेको",
        "निर्देशन गरेको",
        "पदावली प्रयोगमा निर्देशन रूप उपयुक्त हुन्छ",
    ),
    (
        "इमानदारिता देखाउनु",
        "इमानदारी देखाउनु",
        "पदावली प्रयोगमा इमानदारी रूप प्रचलित छ",
    ),
    (
        "भन्नुभएको कुरा",
        "भनेको कुरा",
        "पदावली प्रयोगमा भनेको रूप सिफारिस गरिन्छ",
    ),
    (
        "पढ्नुभएको किताब",
        "पढेको किताब",
        "पदावली प्रयोगमा पढेको रूप सिफारिस गरिन्छ",
    ),
    (
        "कार्यक्रमको सम्बन्धमा",
        "कार्यक्रमका सम्बन्धमा",
        "सम्बन्धमा अघि बहुवचन कारकमा का उपयुक्त हुन्छ",
    ),
    (
        "सूचनाको आधारमा",
        "सूचनाका आधारमा",
        "आधारमा अघि बहुवचन कारकमा का उपयुक्त हुन्छ",
    ),
    (
        "उपस्थितिको बारेमा",
        "उपस्थितिका बारेमा",
        "बारेमा अघि बहुवचन कारकमा का उपयुक्त हुन्छ",
    ),
    (
        "अपहरित भएको",
        "अपहरण भएको",
        "प्रयोगगत रूपमा अपहरण भएको सिफारिस गरिन्छ",
    ),
    (
        "संरक्षित गरिएको",
        "संरक्षण गरिएको",
        "प्रयोगगत रूपमा संरक्षण गरिएको सिफारिस गरिन्छ",
    ),
    (
        "प्रसारित गरिएको",
        "प्रसारण गरिएको",
        "प्रयोगगत रूपमा प्रसारण गरिएको सिफारिस गरिन्छ",
    ),
    (
        "कामको लागि",
        "कामका लागि",
        "प्रयोगगत रूपमा कामका लागि सिफारिस गरिन्छ",
    ),
    (
        "देशको निम्ति",
        "देशका निम्ति",
        "प्रयोगगत रूपमा देशका निम्ति सिफारिस गरिन्छ",
    ),
    (
        "म सबैलाई हार्दिक स्वागत गर्न चाहन्छु",
        "म सबैलाई हार्दिक स्वागत गर्छु",
        "वक्तव्य शैलीमा प्रत्यक्ष स्वागत गर्छु रूप स्पष्ट हुन्छ",
    ),
    (
        "म अब कार्यक्रम सञ्चालन गर्न गइरहेको छु वा जाँदै छु",
        "म अब कार्यक्रम सञ्चालन गर्दै छु",
        "वाक्यगत सटीकता: सञ्चालन गर्दै छु रूप स्पष्ट र संक्षिप्त हुन्छ",
    ),
    (
        "अब यो प्रसारणका प्रमुख समाचारहरू सुन्नुहोस्",
        "अब यस प्रसारणका प्रमुख समाचारहरू सुन्नुहोस्",
        "तिर्यक् कारक प्रसङ्गमा यो -> यस रूप उपयुक्त हुन्छ",
    ),
    (
        "म यस कार्यक्रम यहाँ अन्त्य गर्दछु",
        "म यो कार्यक्रम यहीँ अन्त्य गर्दछु",
        "सरल कारक प्रयोगमा यो/यहीँ रूप उपयुक्त हुन्छ",
    ),
    (
        "लाखौँ नेपालका जनता गरिबीको रेखामुनि छन्",
        "नेपालका लाखौँ जनता गरिबीको रेखामुनि छन्",
        "पदक्रम मिलाउन नेपालका लाखौँ जनता रूप उपयुक्त हुन्छ",
    ),
    (
        "नेपाल मानव अधिकार आयोगद्वारा आयोजित टीकापुर हत्याकाण्डसम्बन्धी छलफल कार्यक्रममा मन्त्रीज्यूले पनि बोल्नुभयो",
        "टीकापुर हत्याकाण्डसम्बन्धी नेपाल मानव अधिकार आयोगद्वारा आयोजित छलफल कार्यक्रममा मन्त्रीज्यूले पनि बोल्नुभयो",
        "वाक्यगत अर्थ-स्पष्टताका लागि घटकहरूको पदक्रम मिलाउनु उपयुक्त हुन्छ",
    ),
    (
        "स्थानीय जनशक्तिको श्रमदानबाट दश किलोमिटर लामो गाडी गुड्न सक्ने सडक निर्माण गरियो",
        "स्थानीय जनशक्तिको श्रमदानबाट गाडी गुड्न सक्ने दश किलोमिटर लामो सडक निर्माण गरियो",
        "वाक्यमा विशेषण/विशेष्यको सम्बन्ध स्पष्ट राख्न पदक्रम मिलाउनु उपयुक्त हुन्छ",
    ),
    (
        "यहाँको सहयोगप्रति म कृतघ्न छु",
        "यहाँको सहयोगप्रति म कृतज्ञ छु",
        "कृतघ्न र कृतज्ञ अर्थ भिन्न छन्",
    ),
    (
        "ऊ राजनीतिमा निर्लिप्त छ",
        "ऊ राजनीतिमा लिप्त छ",
        "निर्लिप्त र लिप्त अर्थ भिन्न छन्",
    ),
];

pub(crate) fn add_style_variant_diagnostics(
    text: &str,
    tokens: &[AnalyzedToken],
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for &(incorrect, correct, explanation) in STYLE_VARIANT_CORRECTIONS {
        for (start, _) in text.match_indices(incorrect) {
            let end = start + incorrect.len();
            let span = (start, end);

            if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
                continue;
            }
            if !is_word_boundary(text, start, end) {
                continue;
            }

            diagnostics.push(Diagnostic {
                span,
                incorrect: incorrect.to_string(),
                correction: correct.to_string(),
                rule: Rule::Vyakaran("section4-phrase-style"),
                explanation: format!("Section 4 शैली सुझाव: {explanation}"),
                category: DiagnosticCategory::ShuddhaTable,
                kind: DiagnosticKind::Variant,
                confidence: 0.78,
                alternate_reasons: Vec::new(),
            });
            blocked_spans.insert(span);
        }
    }

    add_inferred_ko_ka_style_variants(text, tokens, blocked_spans, diagnostics);
}

fn add_inferred_ko_ka_style_variants(
    text: &str,
    tokens: &[AnalyzedToken],
    blocked_spans: &mut HashSet<(usize, usize)>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for pair in tokens.windows(2) {
        let left = &pair[0];
        let right = &pair[1];
        if !left.is_devanagari_word() || !right.is_devanagari_word() {
            continue;
        }
        if !text[left.end..right.start].chars().any(char::is_whitespace) {
            continue;
        }

        let left_surface = left.surface();
        let right_surface = right.surface();
        let Some(base) = left_surface.strip_suffix("को") else {
            continue;
        };
        if base.is_empty() {
            continue;
        }
        let Some((_, explanation)) = INFERRED_KO_KA_FOLLOWERS
            .iter()
            .find(|(follower, _)| right_surface.as_ref() == *follower)
        else {
            continue;
        };

        let span = (left.start, right.end);
        if blocked_spans.contains(&span) || overlaps_existing_span(diagnostics, span) {
            continue;
        }
        if !is_word_boundary(text, span.0, span.1) {
            continue;
        }

        diagnostics.push(Diagnostic {
            span,
            incorrect: text[span.0..span.1].to_string(),
            correction: format!("{base}का {right_surface}"),
            rule: Rule::Vyakaran(INFERRED_KO_KA_RULE_CODE),
            explanation: format!("Section 4 पदावली शैली सुझाव (अनुमित): {explanation}"),
            category: DiagnosticCategory::ShuddhaTable,
            kind: DiagnosticKind::Variant,
            confidence: 0.7,
            alternate_reasons: Vec::new(),
        });
        blocked_spans.insert(span);
    }
}
