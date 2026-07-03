use crate::model::prakriya::Prakriya;
use crate::model::rule::Rule;
use crate::model::step::Step;
use std::sync::LazyLock;
use varnavinyas_shabda::{Origin, classify};

const PS_LOANWORD_AJANTA_RULE_CODE: &str = "3(ङ)-PS-Saisanik-3(ग)-आगन्तुक";
const AJANTA_INVENTORY_DATA: &str =
    include_str!("../../../../../data/rule_inventories/ajanta_halanta.tsv");

static AJANTA_INVENTORY: LazyLock<Vec<AjantaInventoryEntry>> =
    LazyLock::new(|| parse_ajanta_inventory(AJANTA_INVENTORY_DATA));

#[derive(Debug, Clone, Copy)]
struct AjantaInventoryEntry {
    input: &'static str,
    output: &'static str,
    rule_code: &'static str,
}

fn corrected(
    input: &str,
    output: String,
    code: &'static str,
    explanation: &'static str,
) -> Prakriya {
    Prakriya::corrected(
        input,
        &output,
        vec![Step::new(
            Rule::VarnaVinyasNiyam(code),
            explanation,
            input,
            &output,
        )],
    )
}

// -----------------------------------------------------------------------------
// 3(ङ) अजन्त लेख्नुपर्ने रूप
// Implemented subrules:
// - 3(ङ)-अजन्त-1 .. 8
// - PS-Saisanik 3(ग) आगन्तुक अजन्त examples
// -----------------------------------------------------------------------------
pub(super) fn rule_ajanta_required(input: &str) -> Option<Prakriya> {
    let stem = input.strip_suffix('्')?;

    if input.ends_with("छस्") || input.ends_with("छन्") || input.ends_with("इस्") || input == "अर्थात्"
    {
        return None;
    }
    if (input.ends_with("मान्") || input.ends_with("वान्") || input.ends_with("वत्"))
        && matches!(classify(stem), Origin::Tatsam)
    {
        return None;
    }

    if let Some(entry) = AJANTA_INVENTORY.iter().find(|entry| entry.input == input) {
        return Some(corrected(
            input,
            entry.output.to_string(),
            entry.rule_code,
            ajanta_explanation(entry.rule_code),
        ));
    }

    let output = stem.to_string();
    if output.ends_with('छ') {
        return Some(corrected(
            input,
            output,
            "3(ङ)-अजन्त-5",
            "अन्त्यमा 'छ' आउने समापक क्रियापदमा हलन्त लेखिँदैन",
        ));
    }

    None
}

fn ajanta_explanation(rule_code: &str) -> &'static str {
    match rule_code {
        "3(ङ)-अजन्त-1" => "एकाक्षरी सर्वनाम/अव्ययमा हलन्त लेखिँदैन",
        "3(ङ)-अजन्त-2" => "स्वरान्त अव्ययमा हलन्त लेखिँदैन",
        "3(ङ)-अजन्त-3" => "सामान्य आदरार्थी आज्ञार्थ क्रियापदमा हलन्त लेखिँदैन",
        "3(ङ)-अजन्त-4" => "अन्त्यमा 'न' आउने अकरण क्रियापदमा हलन्त लेखिँदैन",
        "3(ङ)-अजन्त-6" => "असमापक क्रियापदमा हलन्त लेखिँदैन",
        "3(ङ)-अजन्त-7" => "अनुकरणात्मक शब्दको अन्त्यमा हलन्त लेखिँदैन",
        "3(ङ)-अजन्त-8" => "कतिपय नाम/सर्वनाम/विशेषणमा लेखन अजन्त हुन्छ",
        PS_LOANWORD_AJANTA_RULE_CODE => "शैक्षणिक व्याकरण ३(ग): उच्चारण हलन्त हुने आगन्तुक शब्द अजन्त लेखिन्छन्",
        _ => panic!("unknown ajanta inventory rule code `{rule_code}`"),
    }
}

fn parse_ajanta_inventory(data: &'static str) -> Vec<AjantaInventoryEntry> {
    let mut lines = data.lines();
    let header = lines.next().expect("ajanta inventory must have a header");
    assert_eq!(
        header, "input\toutput\tmatch_kind\trule_code\tsource\treview_status",
        "unexpected ajanta inventory header"
    );

    let mut entries = Vec::new();
    for (line_idx, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&'static str> = line.split('\t').collect();
        assert_eq!(
            fields.len(),
            6,
            "ajanta inventory line {} must have 6 TSV fields",
            line_idx + 2
        );
        let [input, output, match_kind, rule_code, source, review_status] =
            <[&'static str; 6]>::try_from(fields).expect("checked field count");
        assert!(
            !input.is_empty() && !output.is_empty(),
            "ajanta inventory line {} has empty input/output",
            line_idx + 2
        );
        assert_eq!(
            match_kind,
            "exact",
            "ajanta inventory line {} has unsupported match kind `{}`",
            line_idx + 2,
            match_kind
        );
        assert!(
            !rule_code.is_empty() && !source.is_empty() && !review_status.is_empty(),
            "ajanta inventory line {} must include provenance fields",
            line_idx + 2
        );
        let expected_input = format!("{output}्");
        assert_eq!(
            input,
            expected_input,
            "ajanta inventory line {} input must be output plus terminal halanta",
            line_idx + 2
        );
        let _ = ajanta_explanation(rule_code);
        entries.push(AjantaInventoryEntry {
            input,
            output,
            rule_code,
        });
    }

    for (idx, entry) in entries.iter().enumerate() {
        assert!(
            entries[..idx]
                .iter()
                .all(|other| other.input != entry.input),
            "duplicate ajanta inventory row for {}",
            entry.input
        );
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ajanta_inventory_schema_is_valid() {
        let entries = &*AJANTA_INVENTORY;
        assert_eq!(entries.len(), 56);
        assert_eq!(
            entries
                .iter()
                .filter(|entry| entry.rule_code == PS_LOANWORD_AJANTA_RULE_CODE)
                .count(),
            7
        );
    }
}
