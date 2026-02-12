use crate::{apply, SandhiResult};
use varnavinyas_kosha::kosha;

/// Split a word at potential sandhi boundaries using general brute-force strategy.
///
/// Algorithm:
/// 1. Iterate over all valid character boundaries in the word.
/// 2. For each split (left, right), try to reconstruct original morphemes
///    that would result in `word` when combined via sandhi.
/// 3. Validate candidates against the kosha lexicon.
pub fn split(word: &str) -> Vec<(String, String, SandhiResult)> {
    let mut results = Vec::new();
    let lex = kosha();

    // Iterate over all internal character boundaries
    for (i, _) in word.char_indices().skip(1) {
        let (raw_left, raw_right) = word.split_at(i);

        // Strategy 1: Simple concatenation (Visarga retained, or no change)
        // Check if raw_left and raw_right are valid words
        if lex.contains(raw_left) && lex.contains(raw_right) {
             if let Ok(res) = apply(raw_left, raw_right) {
                 if res.output == word {
                     results.push((raw_left.to_string(), raw_right.to_string(), res));
                 }
             }
        }

        // Strategy 2: Vowel reconstruction on the right side
        // e.g. "महेन्द्र" split at "मह" + "न्द्र" -> try "मह" + "इन्द्र"
        // Try prepending every vowel to `raw_right`
        let vowels = ["अ", "आ", "इ", "ई", "उ", "ऊ", "ए", "ऐ", "ओ", "औ", "ऋ"];
        
        for v in vowels {
            let candidate_right = format!("{v}{raw_right}");
            
            // Optimization: check if candidate_right is a valid word first
            if !lex.contains(&candidate_right) {
                continue;
            }

            // Sub-strategy 2a: Left side is unchanged (e.g. Visarga -> R)
            // पुनः + आगमन = पुनरागमन. Split "पुन" + "रागमन" -> "पुन" + "आगमन"? 
            // Only if "पुन" is valid. "पुनः" is valid.
            
            // Try raw_left as-is
            if lex.contains(raw_left) {
                if let Ok(res) = apply(raw_left, &candidate_right) {
                    if res.output == word {
                        results.push((raw_left.to_string(), candidate_right.clone(), res.clone()));
                    }
                }
            }

            // Sub-strategy 2b: Left side needs reconstruction too (Vowel Sandhi)
            // "महेन्द्र" -> "मह" + "इन्द्र". "मह" might be "महा" or "मह".
            // Try appending vowels to raw_left (replacing last char if it's a matra? No, raw_left comes from split)
            
            // "महेन्द्र" split "मह" "न्द्र". right="इन्द्र". left="मह".
            // apply("मह", "इन्द्र") -> "महेन्द्र" (Guna: a+i=e).
            // So checking raw_left is enough IF raw_left ends in 'a' (implicit).
            
            // But what if left was "महा"? "महा" + "इन्द्र" -> "महेन्द्र".
            // "मह" split doesn't contain 'a' vowel? "मह" ends in 'h' which has implicit 'a'.
            // So "मह" works.
            
            // Try "महा" (append 'ा')
            let left_variants = vec![
                format!("{}ा", raw_left), // Aa
                format!("{}ः", raw_left), // Visarga
            ];
            
            for left in left_variants {
                if lex.contains(&left) {
                    if let Ok(res) = apply(&left, &candidate_right) {
                        if res.output == word {
                            results.push((left, candidate_right.clone(), res));
                        }
                    }
                }
            }
        }

        // Strategy 3: Yan Sandhi Reconstruction (इ/ई -> य, उ/ऊ -> व)
        // If left ends in ्य, try replacing with ि/ी and prepending vowel to right.
        if raw_left.ends_with("्य") {
            let base = &raw_left[..raw_left.len() - "्य".len()];
            let left_candidates = [format!("{}ि", base), format!("{}ी", base)];
            
            for left in left_candidates {
                if !lex.contains(&left) { continue; }
                
                // Try prepending vowels to right
                for v in vowels {
                    let right = format!("{v}{raw_right}");
                    if lex.contains(&right) {
                        if let Ok(res) = apply(&left, &right) {
                            if res.output == word {
                                results.push((left.clone(), right, res));
                            }
                        }
                    }
                }
            }
        }

        // If left ends in ्व, try replacing with ु/ू and prepending vowel to right.
        if raw_left.ends_with("्व") {
            let base = &raw_left[..raw_left.len() - "्व".len()];
            let left_candidates = [format!("{}ु", base), format!("{}ू", base)];
            
            for left in left_candidates {
                if !lex.contains(&left) { continue; }
                
                for v in vowels {
                    let right = format!("{v}{raw_right}");
                    if lex.contains(&right) {
                        if let Ok(res) = apply(&left, &right) {
                            if res.output == word {
                                results.push((left.clone(), right, res));
                            }
                        }
                    }
                }
            }
        }

        // Strategy 4: Visarga -> R Reconstruction
        // Case A: Visarga + Vowel (अ) -> र (whole)
        // e.g. "पुनरवलोकन" split at "पुन" | "रवलोकन"
        // right starts with 'र'. Try replacing 'र' with 'अ'.
        // left: append 'ः'.
        if raw_right.starts_with('र') {
            let left_candidate = format!("{}ः", raw_left);
            let right_candidate = format!("अ{}", &raw_right['र'.len_utf8()..]);
            
            if lex.contains(&left_candidate) && lex.contains(&right_candidate) {
                if let Ok(res) = apply(&left_candidate, &right_candidate) {
                    if res.output == word {
                        results.push((left_candidate, right_candidate, res));
                    }
                }
            }
        }

        // Case B: Visarga + Voiced Consonant -> र् (half)
        // e.g. "निर्धन" split at "नि" | "र्धन"
        // right starts with 'र्'. Try stripping 'र्'.
        // left: append 'ः'.
        if raw_right.starts_with("र्") {
             let left_candidate = format!("{}ः", raw_left);
             let right_candidate = raw_right["र्".len()..].to_string(); // Strip 'र्'
             
             if lex.contains(&left_candidate) && lex.contains(&right_candidate) {
                if let Ok(res) = apply(&left_candidate, &right_candidate) {
                    if res.output == word {
                        results.push((left_candidate, right_candidate, res));
                    }
                }
             }
        }
    }
    
    // Deduplicate results
    results.sort_by(|a, b| a.0.cmp(&b.0));
    results.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);

    results
}
