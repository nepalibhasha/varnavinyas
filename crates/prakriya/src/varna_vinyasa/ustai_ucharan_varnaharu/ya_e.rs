use crate::prakriya::Prakriya;
use crate::rule::Rule;
use crate::step::Step;
use varnavinyas_kosha::kosha;

// Academy 3(ग)(इ): first-letter ए<->य alternation with lexicon guard.
// -----------------------------------------------------------------------------
// 3(ग)(इ) 'ए' र 'य' को प्रयोग
// -----------------------------------------------------------------------------
pub fn rule_ya_e(input: &str) -> Option<Prakriya> {
    let chars: Vec<char> = input.chars().collect();
    if chars.is_empty() {
        return None;
    }
    let swap_char = match chars[0] {
        'ए' => 'य',
        'य' => 'ए',
        _ => return None,
    };
    let kosha = kosha();
    if kosha.contains(input) {
        return None;
    }
    let mut swapped = chars;
    swapped[0] = swap_char;
    let candidate: String = swapped.into_iter().collect();
    if kosha.contains(&candidate) {
        let citation = if input.starts_with('य') {
            // य -> ए correction path.
            if candidate.ends_with("एँ")
                || candidate.ends_with("ए")
                || candidate.ends_with("एछ")
                || candidate.ends_with("एछौ")
                || candidate.ends_with("एछु")
            {
                "3(ग)(इ)-ए-1"
            } else if candidate.contains("एर")
                || candidate.contains("एको")
                || candidate.contains("एका")
                || candidate.contains("एकी")
            {
                "3(ग)(इ)-ए-2"
            } else if candidate.starts_with("एक") || candidate.starts_with("एघार") {
                "3(ग)(इ)-ए-5"
            } else {
                "3(ग)(इ)-ए-6"
            }
        } else {
            // ए -> य correction path.
            if candidate.starts_with("यो")
                || candidate.starts_with("यो ")
                || candidate.starts_with("यत")
                || candidate.starts_with("यह")
                || candidate.starts_with("त्यो")
            {
                "3(ग)(इ)-य-1"
            } else if candidate.ends_with("यौ")
                || candidate.ends_with("यो")
                || candidate.ends_with("यौँ")
            {
                "3(ग)(इ)-य-2"
            } else if candidate.ends_with("िया")
                || candidate.ends_with("ैया")
                || candidate.ends_with("्यौली")
                || candidate.ends_with("्याइँ")
            {
                "3(ग)(इ)-य-3"
            } else if candidate.starts_with("यज्ञ")
                || candidate.starts_with("यक्ष")
                || candidate.starts_with("यथ")
                || candidate.starts_with("यति")
            {
                "3(ग)(इ)-य-4"
            } else {
                "3(ग)(इ)-य-5"
            }
        };
        return Some(Prakriya::corrected(
            input,
            &candidate,
            vec![Step::new(
                Rule::VarnaVinyasNiyam(citation),
                "ए/य भेद: शब्दादिमा ए र य फरक हुन्छ",
                input,
                &candidate,
            )],
        ));
    }
    None
}
