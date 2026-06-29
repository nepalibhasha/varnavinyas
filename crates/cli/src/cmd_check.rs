use std::io::Read;
use std::process::ExitCode;

use serde::Serialize;
use varnavinyas_parikshak::{
    ApiDiagnostic, CheckOptions, Diagnostic, DiagnosticKind, OrthographyMode, PunctuationMode,
    check_text_with_options, diagnostic_reason_category,
};

use crate::{OrthographyModeArg, OutputFormat, PunctuationModeArg};

/// JSON-serializable diagnostic output.
#[derive(Serialize)]
struct JsonDiagnostic {
    line: usize,
    column: usize,
    incorrect: String,
    correction: String,
    rule: String,
    rule_code: String,
    category: String,
    category_code: String,
    category_label: String,
    explanation: String,
    kind: String,
    confidence: f32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    alternate_reasons: Vec<JsonAlternateReason>,
}

#[derive(Serialize)]
struct JsonAlternateReason {
    rule: String,
    rule_code: String,
    category: String,
    category_code: String,
    category_label: String,
    explanation: String,
    correction: String,
}

pub struct RunOptions {
    pub input: Option<String>,
    pub explain: bool,
    pub grammar: bool,
    pub punctuation_mode: PunctuationModeArg,
    pub orthography_mode: OrthographyModeArg,
    pub debug_include_noop_heuristics: bool,
    pub fail_on_suggestions: bool,
    pub format: OutputFormat,
}

pub fn run(options: RunOptions) -> ExitCode {
    let (source_name, text) = match read_input(options.input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };

    let diagnostics = check_text_with_options(
        &text,
        CheckOptions {
            grammar: options.grammar,
            orthography_mode: to_core_orthography_mode(options.orthography_mode),
            punctuation_mode: to_core_punctuation_mode(options.punctuation_mode),
            include_noop_heuristics: options.debug_include_noop_heuristics,
        },
    );

    let line_offsets = build_line_offsets(&text);

    match options.format {
        OutputFormat::Text => {
            print_text(
                &diagnostics,
                &source_name,
                &text,
                &line_offsets,
                options.explain,
            );
        }
        OutputFormat::Json => {
            print_json(&diagnostics, &text, &line_offsets);
        }
    }

    if has_blocking_diagnostics(&diagnostics, options.fail_on_suggestions) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

fn has_blocking_diagnostics(diagnostics: &[Diagnostic], fail_on_suggestions: bool) -> bool {
    if fail_on_suggestions {
        !diagnostics.is_empty()
    } else {
        diagnostics
            .iter()
            .any(|d| matches!(d.kind, DiagnosticKind::Error))
    }
}

fn to_core_punctuation_mode(mode: PunctuationModeArg) -> PunctuationMode {
    match mode {
        PunctuationModeArg::Strict => PunctuationMode::Strict,
        PunctuationModeArg::NormalizedEditorial => PunctuationMode::NormalizedEditorial,
    }
}

fn to_core_orthography_mode(mode: OrthographyModeArg) -> OrthographyMode {
    match mode {
        OrthographyModeArg::AcademyStrict => OrthographyMode::AcademyStrict,
        OrthographyModeArg::CommonEditorial => OrthographyMode::CommonEditorial,
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
            "{source}:{line}:{col}: {}{} \u{2192} {}",
            if matches!(diag.kind, DiagnosticKind::Error) {
                ""
            } else {
                "[suggestion] "
            },
            diag.incorrect,
            diag.correction
        );
        if explain {
            println!("  [{}] {}", diag.category, diag.explanation);
            for alt in &diag.alternate_reasons {
                let category = diagnostic_reason_category(alt);
                println!(
                    "  [other: {} | {}] {} -> {}",
                    category,
                    alt.rule,
                    alt.explanation,
                    alt.correction.as_deref().unwrap_or("")
                );
            }
        }
    }
}

fn print_json(diagnostics: &[Diagnostic], text: &str, line_offsets: &[usize]) {
    let entries: Vec<JsonDiagnostic> = diagnostics
        .iter()
        .map(|diag| {
            let (line, column) = byte_to_line_col(diag.span.0, text, line_offsets);
            let api = ApiDiagnostic::from(diag);
            JsonDiagnostic {
                line,
                column,
                incorrect: api.incorrect,
                correction: api.correction,
                rule: api.rule,
                rule_code: api.rule_code,
                category: api.category_code.clone(),
                category_code: api.category_code,
                category_label: api.category,
                explanation: api.explanation,
                kind: api.kind,
                confidence: api.confidence,
                alternate_reasons: api
                    .alternate_reasons
                    .into_iter()
                    .map(|alt| JsonAlternateReason {
                        rule: alt.rule,
                        rule_code: alt.rule_code,
                        category: alt.category_code.clone(),
                        category_code: alt.category_code,
                        category_label: alt.category,
                        explanation: alt.explanation,
                        correction: alt.correction,
                    })
                    .collect(),
            }
        })
        .collect();

    match serde_json::to_string_pretty(&entries) {
        Ok(json) => println!("{json}"),
        Err(e) => {
            eprintln!("error: failed to serialize diagnostics as JSON: {e}");
            println!("[]");
        }
    }
}
