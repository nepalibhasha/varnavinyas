use varnavinyas_akshar::{CharType, Varga, classify, split_aksharas};

pub fn run(text: &str) {
    let aksharas = split_aksharas(text);

    // Syllable breakdown
    let syllable_texts: Vec<&str> = aksharas.iter().map(|a| a.text.as_str()).collect();
    println!("Text: {text}");
    println!(
        "Aksharas ({}): {}",
        aksharas.len(),
        syllable_texts.join(" | ")
    );

    // Character-level analysis
    println!("Characters:");
    for c in text.chars() {
        match classify(c) {
            Some(dc) => {
                let type_label = char_type_label(dc.char_type);
                let mut parts = vec![format!("  {c} (U+{:04X}) \u{2014} {type_label}", c as u32)];
                if let Some(v) = dc.varga {
                    parts.push(format!(", {}", varga_label(v)));
                }
                if dc.is_panchham {
                    parts.push(", पञ्चम".to_string());
                }
                println!("{}", parts.concat());
            }
            None => {
                println!("  {c} (U+{:04X}) \u{2014} non-Devanagari", c as u32);
            }
        }
    }
}

fn char_type_label(ct: CharType) -> &'static str {
    match ct {
        CharType::Svar => "स्वर (vowel)",
        CharType::Vyanjan => "व्यञ्जन (consonant)",
        CharType::Matra => "मात्रा (vowel sign)",
        CharType::Halanta => "हलन्त (virama)",
        CharType::Chandrabindu => "चन्द्रबिन्दु",
        CharType::Shirbindu => "शिरबिन्दु (anusvara)",
        CharType::Visarga => "विसर्ग",
        CharType::Nukta => "नुक्ता",
        CharType::Avagraha => "अवग्रह",
        CharType::Numeral => "अंक (numeral)",
        CharType::Danda => "दण्ड (punctuation)",
        CharType::OtherMark => "चिह्न (mark)",
    }
}

fn varga_label(v: Varga) -> &'static str {
    match v {
        Varga::KaVarga => "क-वर्ग",
        Varga::ChaVarga => "च-वर्ग",
        Varga::TaVarga => "ट-वर्ग",
        Varga::TaVarga2 => "त-वर्ग",
        Varga::PaVarga => "प-वर्ग",
        Varga::Antastha => "अन्तस्थ",
        Varga::Ushma => "ऊष्म",
        Varga::Other => "अन्य",
    }
}
