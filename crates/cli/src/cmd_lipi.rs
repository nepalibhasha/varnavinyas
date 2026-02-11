use std::process::ExitCode;

use varnavinyas_lipi::{Scheme, transliterate};

pub fn run(text: &str, from: &str, to: &str) -> ExitCode {
    let from_scheme = match parse_scheme(from) {
        Some(s) => s,
        None => {
            eprintln!("error: unknown scheme '{from}'. Supported: devanagari, iast");
            return ExitCode::from(2);
        }
    };

    let to_scheme = match parse_scheme(to) {
        Some(s) => s,
        None => {
            eprintln!("error: unknown scheme '{to}'. Supported: devanagari, iast");
            return ExitCode::from(2);
        }
    };

    match transliterate(text, from_scheme, to_scheme) {
        Ok(result) => {
            println!("{result}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}

fn parse_scheme(s: &str) -> Option<Scheme> {
    match s.to_ascii_lowercase().as_str() {
        "devanagari" | "deva" => Some(Scheme::Devanagari),
        "iast" => Some(Scheme::Iast),
        _ => None,
    }
}
