use crate::{SandhiResult, SandhiType};
use varnavinyas_akshar::{is_svar, svar_to_matra};

/// Apply visarga sandhi at the boundary of two morphemes.
pub fn apply_visarga_sandhi(first: &str, second: &str) -> Option<SandhiResult> {
    // First must end with visarga ः
    if !first.ends_with('ः') {
        return None;
    }

    let prefix = &first[..first.len() - 'ः'.len_utf8()];
    let second_chars: Vec<char> = second.chars().collect();
    let first_of_second = *second_chars.first()?;

    // Visarga retained before sibilants (स, श, ष) and unvoiced stops
    if matches!(first_of_second, 'स' | 'श' | 'ष' | 'क' | 'ख' | 'प' | 'फ') {
        let result = format!("{first}{second}");
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VisargaSandhi,
            rule_citation: "विसर्ग सन्धि: विसर्ग retained before स/श/ष/unvoiced stops",
        });
    }

    // Visarga → र before vowel
    // When the second word starts with अ, the अ is consumed
    // because र already carries inherent अ.
    // For other vowels, they become matras attached to र.
    if is_svar(first_of_second) {
        let second_remainder: String;
        let ra_form: String;

        if first_of_second == 'अ' {
            // अ is consumed — inherent vowel of र
            second_remainder = second_chars[1..].iter().collect();
            ra_form = "र".to_string();
        } else {
            // Other vowels become matra attached to र
            second_remainder = second_chars[1..].iter().collect();
            let matra = svar_to_matra(first_of_second).unwrap_or(first_of_second);
            ra_form = format!("र{matra}");
        };

        let result = format!("{prefix}{ra_form}{second_remainder}");
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VisargaSandhi,
            rule_citation: "विसर्ग सन्धि: विसर्ग → र before vowel",
        });
    }

    // Visarga → र before voiced consonants
    if is_voiced_consonant(first_of_second) {
        let result = format!("{prefix}र{second}");
        return Some(SandhiResult {
            output: result,
            sandhi_type: SandhiType::VisargaSandhi,
            rule_citation: "विसर्ग सन्धि: विसर्ग → र before voiced consonant",
        });
    }

    None
}

fn is_voiced_consonant(c: char) -> bool {
    matches!(
        c,
        'ग' | 'घ'
            | 'ङ'
            | 'ज'
            | 'झ'
            | 'ञ'
            | 'ड'
            | 'ढ'
            | 'ण'
            | 'द'
            | 'ध'
            | 'न'
            | 'ब'
            | 'भ'
            | 'म'
            | 'य'
            | 'र'
            | 'ल'
            | 'व'
            | 'ह'
    )
}
