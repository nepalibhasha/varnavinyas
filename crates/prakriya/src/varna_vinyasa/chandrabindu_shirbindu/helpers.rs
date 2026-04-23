use varnavinyas_akshar::is_vyanjan;
use varnavinyas_kosha::kosha;
use varnavinyas_shabda::OriginSource;

pub(super) fn nasalize_diphthong_suffix(input: &str) -> Option<(String, &'static str)> {
    let lex = kosha();

    const PATTERNS: &[(&str, &str, &str)] = &[
        ("ाउदा", "ाउँदा", "3(ख)(आ)-3"),
        ("ाउदै", "ाउँदै", "3(ख)(आ)-3"),
        ("िउदा", "िउँदा", "3(ख)(आ)-3"),
        ("िउदै", "िउँदै", "3(ख)(आ)-3"),
        ("ाउछ", "ाउँछ", "3(ख)(आ)-4"),
        ("ाउथ", "ाउँथ", "3(ख)(आ)-4"),
        ("िउछ", "िउँछ", "3(ख)(आ)-4"),
        ("िउथ", "िउँथ", "3(ख)(आ)-4"),
    ];

    for &(wrong, right, subrule) in PATTERNS {
        if input.contains(wrong) {
            let output = input.replacen(wrong, right, 1);
            if lex.contains(&output) {
                return Some((output, subrule));
            }
        }
    }
    None
}

pub(super) fn chandrabindu_subrule_for(output: &str) -> &'static str {
    if output.contains("ँछ") || output.contains("ँथ") {
        return "3(ख)(आ)-4";
    }
    if output.contains("ँदा") || output.contains("ँदै") {
        return "3(ख)(आ)-3";
    }
    if output.ends_with('ँ')
        || output.ends_with("ौँ")
        || output.ends_with("ौं")
        || output.ends_with("ुँ")
        || output.ends_with("ूँ")
        || output.contains("ँला")
    {
        return "3(ख)(आ)-2";
    }
    "3(ख)(आ)-1"
}

pub(super) fn is_stop_consonant(c: char) -> bool {
    is_vyanjan(c)
        && matches!(
            c,
            'क' | 'ख'
                | 'ग'
                | 'घ'
                | 'ङ'
                | 'च'
                | 'छ'
                | 'ज'
                | 'झ'
                | 'ञ'
                | 'ट'
                | 'ठ'
                | 'ड'
                | 'ढ'
                | 'ण'
                | 'त'
                | 'थ'
                | 'द'
                | 'ध'
                | 'न'
                | 'प'
                | 'फ'
                | 'ब'
                | 'भ'
                | 'म'
        )
}

pub(super) fn should_replace_shirbindu(
    input: &str,
    chars: &[char],
    idx: usize,
    _origin_source: OriginSource,
) -> bool {
    if idx + 1 == chars.len() && idx > 0 && matches!(chars[idx - 1], 'े' | 'ौ') {
        return true;
    }

    if kosha().contains(input) {
        return false;
    }

    let mut candidate_chars = chars.to_vec();
    candidate_chars[idx] = 'ँ';
    let candidate: String = candidate_chars.into_iter().collect();
    kosha().contains(&candidate)
}
