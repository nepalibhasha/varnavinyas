use crate::OutputFormat;
use varnavinyas_shabda::{OriginSource, classify_with_provenance};

pub fn run(word: &str, format: OutputFormat) {
    let decision = classify_with_provenance(word);
    let origin = decision
        .supported_origin()
        .map(|origin| origin.transliterated_label());
    match format {
        OutputFormat::Json => println!(
            "{}",
            serde_json::json!({
                "origin": origin,
                "source": decision.source.as_str(),
                "confidence": decision.confidence,
            })
        ),
        OutputFormat::Text => {
            let evidence = match decision.source {
                OriginSource::Override | OriginSource::Kosha => "documented",
                OriginSource::Heuristic => "inferred",
                OriginSource::Unknown => "no origin evidence",
            };
            println!(
                "{} ({evidence}; source: {}; confidence: {:.2})",
                origin.unwrap_or("Unknown"),
                decision.source.as_str(),
                decision.confidence,
            );
        }
    }
}
