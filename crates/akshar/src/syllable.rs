use crate::devanagari::{self, CharType};

/// A single syllable unit (akshara).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Akshara {
    /// The text of this akshara.
    pub text: String,
    /// Starting byte offset in the original string.
    pub start: usize,
    /// Ending byte offset in the original string.
    pub end: usize,
}

/// Split text into akshara (syllable) units.
///
/// An akshara is the minimal pronounceable unit:
/// - A consonant + optional halanta + consonant chains + vowel sign
/// - A standalone vowel
/// - Anusvara/chandrabindu attach to the preceding akshara
/// - Coda consonants (C+halanta before a consonant with its own vowel)
///   attach to the preceding akshara
///
/// Non-Devanagari characters form their own akshara units.
///
/// # Examples
///
/// ```
/// use varnavinyas_akshar::split_aksharas;
///
/// let result = split_aksharas("नमस्ते");
/// let texts: Vec<&str> = result.iter().map(|a| a.text.as_str()).collect();
/// assert_eq!(texts, vec!["न", "मस्", "ते"]);
/// ```
pub fn split_aksharas(text: &str) -> Vec<Akshara> {
    let mut aksharas = Vec::new();
    let chars: Vec<(usize, char)> = text.char_indices().collect();

    if chars.is_empty() {
        return aksharas;
    }

    let mut i = 0;
    let len = chars.len();

    while i < len {
        let (start_byte, c) = chars[i];

        match devanagari::classify(c) {
            Some(dc) => match dc.char_type {
                CharType::Vyanjan => {
                    i += 1;

                    // Step 1: Consume conjuncts — halanta + consonant chains
                    // at the START of an akshara (onset cluster like प्र, त्त्व)
                    while i < len && is_char_type(&chars, i, CharType::Halanta) {
                        if i + 1 < len && is_vyanjan_at(&chars, i + 1) {
                            i += 2; // consume halanta + consonant
                        } else {
                            i += 1; // trailing halanta (word-final virama)
                            break;
                        }
                    }

                    // Step 2: Consume optional matra
                    if i < len && is_char_type(&chars, i, CharType::Matra) {
                        i += 1;
                    }

                    // Step 3: Consume optional nukta
                    if i < len && is_char_type(&chars, i, CharType::Nukta) {
                        i += 1;
                    }

                    // Step 4: Coda check — absorb C+halanta when the following
                    // consonant has its own vowel (not part of a longer chain).
                    // E.g., in "नमस्ते": after 'म' (inherent vowel), 'स्' is coda
                    // because 'त' has matra 'े'.
                    while i < len && is_vyanjan_at(&chars, i) {
                        if i + 1 < len && is_char_type(&chars, i + 1, CharType::Halanta) {
                            if i + 2 < len && is_vyanjan_at(&chars, i + 2) {
                                // Check: does C₂ continue a chain (another halanta)?
                                let c2_chains =
                                    i + 3 < len && is_char_type(&chars, i + 3, CharType::Halanta);
                                if c2_chains {
                                    // C₂ is part of a longer conjunct chain — don't
                                    // steal C_coda; the whole chain will be the next
                                    // akshara's onset.
                                    break;
                                }
                                // C₂ has its own vowel → C_coda + halanta is coda
                                i += 2;
                            } else {
                                // After halanta is end-of-string or non-consonant
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    // Step 5: Consume trailing anusvara/chandrabindu/visarga
                    while i < len {
                        match char_type_at(&chars, i) {
                            Some(
                                CharType::Shirbindu | CharType::Chandrabindu | CharType::Visarga,
                            ) => {
                                i += 1;
                            }
                            _ => break,
                        }
                    }

                    let end_byte = if i < len { chars[i].0 } else { text.len() };
                    aksharas.push(Akshara {
                        text: text[start_byte..end_byte].to_string(),
                        start: start_byte,
                        end: end_byte,
                    });
                }

                CharType::Svar => {
                    i += 1;

                    // Consume trailing anusvara/chandrabindu/visarga
                    while i < len {
                        match char_type_at(&chars, i) {
                            Some(
                                CharType::Shirbindu | CharType::Chandrabindu | CharType::Visarga,
                            ) => {
                                i += 1;
                            }
                            _ => break,
                        }
                    }

                    let end_byte = if i < len { chars[i].0 } else { text.len() };
                    aksharas.push(Akshara {
                        text: text[start_byte..end_byte].to_string(),
                        start: start_byte,
                        end: end_byte,
                    });
                }

                CharType::Shirbindu | CharType::Chandrabindu | CharType::Visarga => {
                    // Attach to preceding akshara if possible
                    if let Some(last) = aksharas.last_mut() {
                        i += 1;
                        let end_byte = if i < len { chars[i].0 } else { text.len() };
                        last.text = text[last.start..end_byte].to_string();
                        last.end = end_byte;
                    } else {
                        i += 1;
                        let end_byte = if i < len { chars[i].0 } else { text.len() };
                        aksharas.push(Akshara {
                            text: text[start_byte..end_byte].to_string(),
                            start: start_byte,
                            end: end_byte,
                        });
                    }
                }

                _ => {
                    i += 1;
                    let end_byte = if i < len { chars[i].0 } else { text.len() };
                    aksharas.push(Akshara {
                        text: text[start_byte..end_byte].to_string(),
                        start: start_byte,
                        end: end_byte,
                    });
                }
            },

            None => {
                i += 1;
                let end_byte = if i < len { chars[i].0 } else { text.len() };
                aksharas.push(Akshara {
                    text: text[start_byte..end_byte].to_string(),
                    start: start_byte,
                    end: end_byte,
                });
            }
        }
    }

    aksharas
}

fn char_type_at(chars: &[(usize, char)], idx: usize) -> Option<CharType> {
    devanagari::classify(chars[idx].1).map(|dc| dc.char_type)
}

fn is_char_type(chars: &[(usize, char)], idx: usize, ct: CharType) -> bool {
    char_type_at(chars, idx) == Some(ct)
}

fn is_vyanjan_at(chars: &[(usize, char)], idx: usize) -> bool {
    is_char_type(chars, idx, CharType::Vyanjan)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn texts(aksharas: &[Akshara]) -> Vec<&str> {
        aksharas.iter().map(|a| a.text.as_str()).collect()
    }

    #[test]
    fn test_simple_word_kathmandu() {
        // काठमाडौं → 4 aksharas
        let result = split_aksharas("काठमाडौं");
        assert_eq!(texts(&result), vec!["का", "ठ", "मा", "डौं"]);
    }

    #[test]
    fn test_conjunct_namaste() {
        // नमस्ते → 3 aksharas (स् is coda of म, त is onset of next)
        let result = split_aksharas("नमस्ते");
        assert_eq!(texts(&result), vec!["न", "मस्", "ते"]);
    }

    #[test]
    fn test_conjunct_prashasan() {
        // प्रशासन — प्र is one conjunct akshara (onset cluster)
        let result = split_aksharas("प्रशासन");
        assert_eq!(texts(&result), vec!["प्र", "शा", "स", "न"]);
    }

    #[test]
    fn test_standalone_vowel() {
        let result = split_aksharas("अ");
        assert_eq!(texts(&result), vec!["अ"]);
    }

    #[test]
    fn test_vowel_with_anusvara() {
        let result = split_aksharas("अं");
        assert_eq!(texts(&result), vec!["अं"]);
    }

    #[test]
    fn test_empty_string() {
        let result = split_aksharas("");
        assert!(result.is_empty());
    }

    #[test]
    fn test_halanta_at_end() {
        // संसद् — halanta at word end
        let result = split_aksharas("संसद्");
        assert_eq!(texts(&result), vec!["सं", "स", "द्"]);
    }

    #[test]
    fn test_mixed_script() {
        let result = split_aksharas("abcक");
        assert_eq!(texts(&result), vec!["a", "b", "c", "क"]);
    }

    #[test]
    fn test_byte_offsets() {
        let text = "नमस्ते";
        let result = split_aksharas(text);
        for a in &result {
            assert_eq!(&text[a.start..a.end], a.text);
        }
    }

    #[test]
    fn test_chandrabindu_attachment() {
        // जान्छौँ — chandrabindu attaches to the last akshara
        let result = split_aksharas("जान्छौँ");
        let t = texts(&result);
        assert!(t.last().unwrap().contains('ँ'));
    }

    #[test]
    fn test_nepal() {
        let result = split_aksharas("नेपाल");
        assert_eq!(texts(&result), vec!["ने", "पा", "ल"]);
    }

    #[test]
    fn test_mahattva_triple_conjunct() {
        // महत्त्व — triple conjunct stays together (chain: त्+त्+व)
        let result = split_aksharas("महत्त्व");
        assert_eq!(texts(&result), vec!["म", "ह", "त्त्व"]);
    }

    #[test]
    fn test_coda_vigyan() {
        // विज्ञान — ज् is coda of वि, ञ starts new akshara
        let result = split_aksharas("विज्ञान");
        assert_eq!(texts(&result), vec!["विज्", "ञा", "न"]);
    }
}
