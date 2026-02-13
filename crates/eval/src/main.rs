use colored::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use varnavinyas_parikshak::check_word;

#[derive(Debug, Deserialize)]
struct TestCase {
    incorrect: String,
    correct: String,
    #[serde(default)]
    #[allow(dead_code)]
    description: String,
}

type GoldData = HashMap<String, Vec<TestCase>>;

struct CategoryResult {
    name: String,
    total: usize,
    tp: usize,
    fn_: usize,
    fp: usize,
}

impl CategoryResult {
    fn accuracy(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (self.tp as f64 / self.total as f64) * 100.0
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Varnavinyas Evaluation Harness".bold().cyan());

    // Path relative to workspace root (CWD when running `cargo run`)
    let path = Path::new("docs/tests/gold.toml");
    if !path.exists() {
        eprintln!("Error: Gold dataset not found at {}", path.display());
        std::process::exit(1);
    }

    let content = std::fs::read_to_string(path)?;
    let data: GoldData = toml::from_str(&content)?;

    let mut results = Vec::new();
    let mut grand_total = 0;
    let mut grand_tp = 0;
    let mut grand_fn = 0;
    let mut grand_fp = 0;

    // Process each category
    let mut categories: Vec<_> = data.keys().collect();
    categories.sort();

    for category in categories {
        let cases = &data[category];
        let mut tp = 0;
        let mut fn_ = 0;
        let mut fp = 0;

        for case in cases {
            // Check incorrect form -> should suggest correct form
            if let Some(diag) = check_word(&case.incorrect) {
                if diag.correction == case.correct {
                    tp += 1;
                } else {
                    fn_ += 1; // Flagged but wrong correction
                    // println!("  [FN] {} -> {} (Expected: {})", case.incorrect, diag.correction, case.correct);
                }
            } else {
                fn_ += 1; // Not flagged
                // println!("  [FN] {} -> (No correction)", case.incorrect);
            }

            // Check correct form -> should NOT be flagged (False Positive check)
            if let Some(_diag) = check_word(&case.correct) {
                fp += 1;
                // println!("  [FP] {} -> flagged as error", case.correct);
            }
        }

        results.push(CategoryResult {
            name: category.clone(),
            total: cases.len(),
            tp,
            fn_,
            fp,
        });

        grand_total += cases.len();
        grand_tp += tp;
        grand_fn += fn_;
        grand_fp += fp;
    }

    // Print Table
    println!();
    println!(
        "{:<20} | {:>4} | {:>4} | {:>4} | {:>8}",
        "Category", "TP", "FN", "FP", "Accuracy"
    );
    println!(
        "{:-<20}-+-{:-<4}-+-{:-<4}-+-{:-<4}-+-{:-<8}",
        "", "", "", "", ""
    );

    for res in &results {
        let acc = res.accuracy();
        let color_acc = if acc > 90.0 {
            format!("{:.1}%", acc).green()
        } else if acc > 70.0 {
            format!("{:.1}%", acc).yellow()
        } else {
            format!("{:.1}%", acc).red()
        };

        println!(
            "{:<20} | {:>4} | {:>4} | {:>4} | {:>8}",
            res.name, res.tp, res.fn_, res.fp, color_acc
        );
    }

    println!(
        "{:-<20}-+-{:-<4}-+-{:-<4}-+-{:-<4}-+-{:-<8}",
        "", "", "", "", ""
    );
    let grand_acc = if grand_total > 0 {
        (grand_tp as f64 / grand_total as f64) * 100.0
    } else {
        0.0
    };

    println!(
        "{:<20} | {:>4} | {:>4} | {:>4} | {:>8}",
        "TOTAL".bold(),
        grand_tp,
        grand_fn,
        grand_fp,
        format!("{:.1}%", grand_acc).bold()
    );

    println!();

    if grand_acc < 80.0 {
        println!("{}", "Warning: Overall accuracy below 80%".yellow());
    } else {
        println!("{}", "Evaluation passed.".green());
    }

    Ok(())
}
