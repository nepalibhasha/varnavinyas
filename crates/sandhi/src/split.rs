use crate::{apply, SandhiResult};
use varnavinyas_akshar::split_aksharas;
use varnavinyas_kosha::kosha;

/// Split a word at potential sandhi boundaries using general brute-force strategy.
///
/// The caller should pass the **morphological root** (after stripping
/// agglutinative suffixes like case markers and plural markers) so that
/// sandhi analysis operates on the stem, not inflected forms.
///
/// Algorithm:
/// 1. Skip words shorter than 3 aksharas — short stems are atomic roots,
///    not sandhi compounds (e.g., "राम" is a name, not "रा + आम").
/// 2. Iterate over all valid character boundaries in the word.
/// 3. For each split (left, right), try to reconstruct original morphemes
///    that would result in `word` when combined via sandhi.
/// 4. Validate candidates against the kosha lexicon.
/// 5. Filter results where either part has fewer than 2 aksharas.
pub fn split(word: &str) -> Vec<(String, String, SandhiResult)> {
    // Guard: stems shorter than 3 aksharas are atomic roots, not compounds.
    if split_aksharas(word).len() < 3 {
        return Vec::new();
    }

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

        // Strategy 2: Vowel reconstruction on the right side.
        // Try prepending every vowel to raw_right to reconstruct the pre-sandhi form.
        // e.g., "मह"|"न्द्र" → try "मह" + "इन्द्र" (गुण: अ+इ=ए).
        let vowels = ["अ", "आ", "इ", "ई", "उ", "ऊ", "ए", "ऐ", "ओ", "औ", "ऋ"];

        for v in vowels {
            let candidate_right = format!("{v}{raw_right}");
            if !lex.contains(&candidate_right) {
                continue;
            }

            // 2a: Left as-is (inherent अ or explicit vowel ending).
            if lex.contains(raw_left) {
                if let Ok(res) = apply(raw_left, &candidate_right) {
                    if res.output == word {
                        results.push((raw_left.to_string(), candidate_right.clone(), res.clone()));
                    }
                }
            }

            // 2b: Left with आ or visarga appended (e.g., "महा" + "इन्द्र").
            for suffix in ["ा", "ः"] {
                let left = format!("{raw_left}{suffix}");
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
        if let Some(base) = raw_left.strip_suffix("्य") {
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
        if let Some(base) = raw_left.strip_suffix("्व") {
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
        if let Some(remainder) = raw_right.strip_prefix("र्") {
             let left_candidate = format!("{}ः", raw_left);
             let right_candidate = remainder.to_string();
             
             if lex.contains(&left_candidate) && lex.contains(&right_candidate) {
                if let Ok(res) = apply(&left_candidate, &right_candidate) {
                    if res.output == word {
                        results.push((left_candidate, right_candidate, res));
                    }
                }
             }
        }

        // Strategy 5: Visarga -> Sibilant Reconstruction (satva sandhi)
        // ः + च/छ → श्+च/छ, ः + ट/ठ → ष्+ट/ठ, ः + त/थ → स्+त/थ
        // Reverse: if raw_left ends in श्, ष्, or स् followed by the matching stop
        // at the start of raw_right, try reconstructing visarga form.
        // e.g. "निश्चय" split at "निश्" | "चय" → try "निः" + "चय"
        // Also handles: "निश" | "्चय" → skip (halanta at start of right is not useful)
        // We check: raw_left ends in sibilant+halanta, raw_right starts with matching stop.
        {
            let sibilant_map: &[(char, &[char])] = &[
                ('श', &['च', 'छ']),   // palatal
                ('ष', &['ट', 'ठ']),   // retroflex
                ('स', &['त', 'थ']),   // dental
            ];
            for &(sibilant, stops) in sibilant_map {
                let suffix = format!("{sibilant}्");
                if let Some(base) = raw_left.strip_suffix(&*suffix) {
                    if let Some(first_char) = raw_right.chars().next() {
                        if stops.contains(&first_char) {
                            let left_candidate = format!("{base}ः");
                            if lex.contains(&left_candidate) && lex.contains(raw_right) {
                                if let Ok(res) = apply(&left_candidate, raw_right) {
                                    if res.output == word {
                                        results.push((left_candidate, raw_right.to_string(), res));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Strategy 6: Ayadi Sandhi Reconstruction
        // ए+vowel→अय, ऐ+vowel→आय, ओ+vowel→अव, औ+vowel→आव
        // Reverse: if raw_left ends in य, try ए/े; if ends in ाय, try ऐ/ै;
        //          if raw_left ends in व, try ओ/ो; if ends in ाव, try औ/ौ.

        // ऐ→आय: raw_left ends in ाय (longer pattern, check first)
        if let Some(base) = raw_left.strip_suffix("ाय") {
            let left_candidates = [format!("{base}ै"), format!("{base}ऐ")];
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
        // ए→अय: raw_left ends in य (but not ाय, already handled above)
        else if let Some(base) = raw_left.strip_suffix('य') {
            let left_candidates = [format!("{base}े"), format!("{base}ए")];
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

        // औ→आव: raw_left ends in ाव (longer pattern, check first)
        if let Some(base) = raw_left.strip_suffix("ाव") {
            let left_candidates = [format!("{base}ौ"), format!("{base}औ")];
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
        // ओ→अव: raw_left ends in व (but not ाव, already handled above)
        else if let Some(base) = raw_left.strip_suffix('व') {
            let left_candidates = [format!("{base}ो"), format!("{base}ओ")];
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

        // Strategy 7: Guna/Vriddhi matra reconstruction.
        // When a sandhi merges अ/आ with another vowel, the result appears as a
        // matra on the preceding consonant: सूर्य+उदय → सूर्योदय (ो matra).
        // Splitting at "सूर्य"|"ोदय" gives raw_right starting with a matra.
        // Strip the matra and try prepending the original vowel.
        //
        // Matra → candidate pre-sandhi vowels:
        //   ा → अ, आ (दीर्घ)    े → इ, ई (गुण)    ो → उ, ऊ (गुण)
        //   ै → ए, ऐ (वृद्धि)   ौ → ओ, औ (वृद्धि)
        let mut right_chars = raw_right.chars();
        if let Some(first_char) = right_chars.next() {
            let candidate_vowels: Option<&[&str]> = match first_char {
                'ा' => Some(&["अ", "आ"]),
                'े' => Some(&["इ", "ई"]),
                'ो' => Some(&["उ", "ऊ"]),
                'ै' => Some(&["ए", "ऐ"]),
                'ौ' => Some(&["ओ", "औ"]),
                _ => None,
            };

            if let Some(vowels) = candidate_vowels {
                let remainder = right_chars.as_str();

                // Try raw_left as-is (inherent अ participates in sandhi).
                if lex.contains(raw_left) {
                    for v in vowels {
                        let candidate_right = format!("{v}{remainder}");
                        if lex.contains(&candidate_right) {
                            if let Ok(res) = apply(raw_left, &candidate_right) {
                                if res.output == word {
                                    results.push((raw_left.to_string(), candidate_right, res));
                                }
                            }
                        }
                    }
                }

                // Try left with आ or visarga appended (e.g., "महा" + "इन्द्र").
                for suffix in ["ा", "ः"] {
                    let left = format!("{raw_left}{suffix}");
                    if lex.contains(&left) {
                        for v in vowels {
                            let candidate_right = format!("{v}{remainder}");
                            if lex.contains(&candidate_right) {
                                if let Ok(res) = apply(&left, &candidate_right) {
                                    if res.output == word {
                                        results.push((left.clone(), candidate_right, res));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Filter out degenerate splits where either part has fewer than 2 aksharas.
    // e.g. "रा + आम → राम" is technically valid दीर्घ sandhi but linguistically
    // meaningless — "राम" is a single morpheme, not a compound.
    // Meaningful sandhi components are almost always multi-syllabic words.
    results.retain(|(left, right, _)| {
        split_aksharas(left).len() >= 2 && split_aksharas(right).len() >= 2
    });

    // Deduplicate results by (left, right) pair
    results.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
    results.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_duplicate_splits() {
        let results = split("विधान");
        // Check no duplicate (left, right) pairs
        for i in 0..results.len() {
            for j in (i + 1)..results.len() {
                assert!(
                    !(results[i].0 == results[j].0 && results[i].1 == results[j].1),
                    "Duplicate: {} + {}",
                    results[i].0,
                    results[i].1
                );
            }
        }
    }
}
