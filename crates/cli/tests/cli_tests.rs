use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    assert_cmd::cargo::cargo_bin_cmd!("varnavinyas")
}

// ── check subcommand ────────────────────────────────────────────

#[test]
fn check_stdin_with_errors_exits_1() {
    cmd()
        .arg("check")
        .write_stdin("अत्याधिक\n")
        .assert()
        .code(1)
        .stdout(predicate::str::contains("\u{2192}")); // → arrow
}

#[test]
fn check_clean_text_exits_0() {
    cmd()
        .arg("check")
        .write_stdin("नेपाल\n")
        .assert()
        .code(0)
        .stdout(predicate::str::is_empty());
}

#[test]
fn check_dash_reads_stdin() {
    cmd()
        .args(["check", "-"])
        .write_stdin("अत्याधिक\n")
        .assert()
        .code(1);
}

#[test]
fn check_json_returns_valid_json() {
    let output = cmd()
        .args(["check", "--format", "json"])
        .write_stdin("अत्याधिक\n")
        .assert()
        .code(1)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value =
        serde_json::from_slice(&output).expect("stdout should be valid JSON");
    assert!(json.is_array());
    let arr = json.as_array().unwrap();
    assert!(!arr.is_empty());
    assert!(arr[0].get("line").is_some());
    assert!(arr[0].get("column").is_some());
    assert!(arr[0].get("incorrect").is_some());
    assert!(arr[0].get("correction").is_some());
}

#[test]
fn check_explain_includes_rule() {
    cmd()
        .args(["check", "--explain"])
        .write_stdin("अत्याधिक\n")
        .assert()
        .code(1)
        .stdout(predicate::str::contains("["));
}

#[test]
fn check_nonexistent_file_exits_2() {
    cmd()
        .args(["check", "/nonexistent/file.txt"])
        .assert()
        .code(2);
}

#[test]
fn check_json_column_is_char_based() {
    // "नेपाल " = 6 chars (न े प ा ल space), then "अत्याधिक" starts at char column 7.
    // In bytes "नेपाल " is 13 bytes — if column were byte-based it would be 14.
    let output = cmd()
        .args(["check", "--format", "json"])
        .write_stdin("नेपाल अत्याधिक\n")
        .assert()
        .code(1)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value =
        serde_json::from_slice(&output).expect("stdout should be valid JSON");
    let arr = json.as_array().unwrap();
    assert_eq!(arr[0]["line"], 1);
    assert_eq!(
        arr[0]["column"], 7,
        "column should be character-based, not byte-based"
    );
}

#[test]
fn check_json_clean_returns_empty_array() {
    let output = cmd()
        .args(["check", "--format", "json"])
        .write_stdin("नेपाल\n")
        .assert()
        .code(0)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value =
        serde_json::from_slice(&output).expect("clean JSON should still be valid JSON");
    assert_eq!(json, serde_json::json!([]));
}

// ── akshar subcommand ───────────────────────────────────────────

#[test]
fn akshar_prints_syllables() {
    cmd()
        .args(["akshar", "नमस्ते"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Aksharas"))
        .stdout(predicate::str::contains("Characters:"));
}

#[test]
fn akshar_shows_unicode_codepoints() {
    cmd()
        .args(["akshar", "क"])
        .assert()
        .success()
        .stdout(predicate::str::contains("U+0915"))
        .stdout(predicate::str::contains("व्यञ्जन"));
}

// ── lipi subcommand ─────────────────────────────────────────────

#[test]
fn lipi_devanagari_to_iast() {
    cmd()
        .args(["lipi", "नमस्ते", "--to", "iast"])
        .assert()
        .success()
        .stdout(predicate::str::contains("namaste"));
}

#[test]
fn lipi_iast_to_devanagari() {
    cmd()
        .args(["lipi", "namaste", "--from", "iast", "--to", "devanagari"])
        .assert()
        .success()
        .stdout(predicate::str::contains("नमस्ते"));
}

#[test]
fn lipi_invalid_scheme_exits_2() {
    cmd()
        .args(["lipi", "test", "--to", "bogus"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("unknown scheme"));
}

// ── general ─────────────────────────────────────────────────────

#[test]
fn no_args_shows_help() {
    cmd()
        .assert()
        .code(2)
        .stderr(predicate::str::contains("Usage"));
}
