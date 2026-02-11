use std::io::Read;
use std::process::ExitCode;

use serde::Serialize;
use varnavinyas_parikshak::{Diagnostic, check_text};

use crate::OutputFormat;

/// JSON-serializable diagnostic output.
#[derive(Serialize)]
struct JsonDiagnostic {
    line: usize,
    column: usize,
    incorrect: String,
    correction: String,
    rule: String,
    category: String,
    explanation: String,
}

pub fn run(input: Option<String>, explain: bool, format: OutputFormat) -> ExitCode {
    let (source_name, text) = match read_input(input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };

    let diagnostics = check_text(&text);

    let line_offsets = build_line_offsets(&text);

    match format {
        OutputFormat::Text => {
            print_text(&diagnostics, &source_name, &text, &line_offsets, explain);
        }
        OutputFormat::Json => {
            print_json(&diagnostics, &text, &line_offsets);
        }
    }

    if diagnostics.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

/// Read input from stdin or a file. Returns (source_name, text).
fn read_input(input: Option<String>) -> Result<(String, String), String> {
    match input.as_deref() {
        None | Some("-") => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .map_err(|e| format!("failed to read stdin: {e}"))?;
            Ok(("<stdin>".to_string(), buf))
        }
        Some(path) => {
            let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
            Ok((path.to_string(), text))
        }
    }
}

/// Build a sorted list of byte offsets where each line starts.
/// line_offsets[0] = 0 (line 1 starts at byte 0).
fn build_line_offsets(text: &str) -> Vec<usize> {
    let mut offsets = vec![0];
    for (i, b) in text.bytes().enumerate() {
        if b == b'\n' {
            offsets.push(i + 1);
        }
    }
    offsets
}

/// Convert a byte offset to 1-based (line, column).
/// Column is character-based (not byte-based) for editor compatibility.
fn byte_to_line_col(byte_offset: usize, text: &str, line_offsets: &[usize]) -> (usize, usize) {
    let line_idx = match line_offsets.binary_search(&byte_offset) {
        Ok(i) => i,
        Err(i) => i.saturating_sub(1),
    };
    let line_start = line_offsets[line_idx];
    let col = text[line_start..byte_offset].chars().count() + 1;
    (line_idx + 1, col)
}

fn print_text(
    diagnostics: &[Diagnostic],
    source: &str,
    text: &str,
    line_offsets: &[usize],
    explain: bool,
) {
    for diag in diagnostics {
        let (line, col) = byte_to_line_col(diag.span.0, text, line_offsets);
        println!(
            "{source}:{line}:{col}: {} \u{2192} {}",
            diag.incorrect, diag.correction
        );
        if explain {
            println!("  [{}] {}", diag.category, diag.explanation);
        }
    }
}

fn print_json(diagnostics: &[Diagnostic], text: &str, line_offsets: &[usize]) {
    let entries: Vec<JsonDiagnostic> = diagnostics
        .iter()
        .map(|diag| {
            let (line, column) = byte_to_line_col(diag.span.0, text, line_offsets);
            JsonDiagnostic {
                line,
                column,
                incorrect: diag.incorrect.clone(),
                correction: diag.correction.clone(),
                rule: diag.rule.to_string(),
                category: diag.category.to_string(),
                explanation: diag.explanation.clone(),
            }
        })
        .collect();

    // Unwrap is safe: our struct is always serializable
    println!("{}", serde_json::to_string_pretty(&entries).unwrap());
}
