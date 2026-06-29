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
fn check_json_contract_required_fields_have_stable_types() {
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
    let diag = json
        .as_array()
        .and_then(|arr| arr.first())
        .expect("input should produce one diagnostic");

    assert!(diag["line"].is_u64());
    assert!(diag["column"].is_u64());
    assert!(diag["incorrect"].is_string());
    assert!(diag["correction"].is_string());
    assert!(diag["rule"].is_string());
    assert!(diag["rule_code"].is_string());
    assert!(diag["category"].is_string());
    assert!(diag["category_code"].is_string());
    assert!(diag["category_label"].is_string());
    assert!(diag["explanation"].is_string());
    assert!(diag["kind"].is_string());
    assert!(diag["confidence"].is_number());
}

#[test]
fn check_json_category_fields_use_stable_codes() {
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
    let diag = json
        .as_array()
        .and_then(|arr| arr.first())
        .expect("input should produce one diagnostic");

    assert_eq!(diag["category"], diag["category_code"]);
    assert_ne!(diag["category"], diag["category_label"]);
    assert!(
        diag["rule_code"]
            .as_str()
            .is_some_and(|code| !code.is_empty())
    );
}

#[test]
fn check_common_editorial_orthography_emits_variants() {
    let output = cmd()
        .args([
            "check",
            "--orthography-mode",
            "common-editorial",
            "--format",
            "json",
        ])
        .write_stdin("संघीय संसद नेपाल . नेपाली कांग्रेस\n")
        .assert()
        .code(1)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value =
        serde_json::from_slice(&output).expect("stdout should be valid JSON");
    let arr = json.as_array().expect("diagnostics should be an array");

    for incorrect in ["संघीय", "संसद", "कांग्रेस"] {
        let diag = arr
            .iter()
            .find(|diag| diag["incorrect"] == incorrect)
            .unwrap_or_else(|| panic!("Expected diagnostic for {incorrect}, got: {arr:?}"));
        assert_eq!(diag["kind"], "Variant");
    }

    assert!(
        arr.iter()
            .any(|diag| diag["incorrect"] == "." && diag["kind"] == "Error"),
        "Punctuation should remain an error in strict punctuation mode, got: {arr:?}"
    );
}

#[test]
fn check_grammar_flag_emits_grammar_pass_diagnostic() {
    let output = cmd()
        .args(["check", "--grammar", "--format", "json"])
        .write_stdin("सूर्योदय भयो।\n")
        .assert()
        .code(0)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value =
        serde_json::from_slice(&output).expect("stdout should be valid JSON");
    let arr = json.as_array().expect("stdout should be an array");
    assert!(
        arr.iter().any(
            |diag| diag.get("rule_code").and_then(serde_json::Value::as_str)
                == Some("samasa-heuristic")
        ),
        "--grammar should surface grammar-pass diagnostics, got: {arr:?}"
    );
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

#[test]
fn check_json_includes_alternate_reasons_for_multi_hit_word() {
    let output = cmd()
        .args(["check", "--format", "json"])
        .write_stdin("भौतीक\n")
        .assert()
        .code(1)
        .get_output()
        .stdout
        .clone();

    let json: serde_json::Value =
        serde_json::from_slice(&output).expect("stdout should be valid JSON");
    let arr = json.as_array().unwrap();
    assert!(!arr.is_empty());
    let alternate_reasons = arr[0]
        .get("alternate_reasons")
        .and_then(serde_json::Value::as_array)
        .expect("multi-hit word should expose alternate_reasons");
    assert!(!alternate_reasons.is_empty());
    assert_eq!(
        alternate_reasons[0]["category"],
        alternate_reasons[0]["category_code"]
    );
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
