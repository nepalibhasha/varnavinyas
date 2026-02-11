use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use varnavinyas_parikshak::{self as parikshak};

use crate::config::Config;
use crate::convert::LineIndex;

/// Cached state for an open document.
struct DocumentState {
    text: String,
    line_index: LineIndex,
    diagnostics: Vec<parikshak::Diagnostic>,
}

pub struct Backend {
    client: Client,
    documents: Arc<RwLock<HashMap<Url, DocumentState>>>,
    config: Arc<RwLock<Config>>,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(Config::default())),
        }
    }

    /// Run diagnostics on a document and publish results.
    async fn update_diagnostics(&self, uri: Url, text: &str) {
        let line_index = LineIndex::new(text);
        let raw_diagnostics = parikshak::check_text(text);
        let config = self.config.read().await;

        let lsp_diags: Vec<tower_lsp::lsp_types::Diagnostic> = raw_diagnostics
            .iter()
            .filter(|d| config.categories.is_enabled(d.category))
            .map(|d| {
                let range = line_index.byte_span_to_range(d.span);
                tower_lsp::lsp_types::Diagnostic {
                    range,
                    severity: Some(DiagnosticSeverity::WARNING),
                    source: Some("varnavinyas".to_string()),
                    code: Some(NumberOrString::String(d.rule.code().to_string())),
                    message: format!("{} → {} ({})", d.incorrect, d.correction, d.category),
                    ..Default::default()
                }
            })
            .collect();

        self.client
            .publish_diagnostics(uri.clone(), lsp_diags, None)
            .await;

        let mut docs = self.documents.write().await;
        docs.insert(
            uri,
            DocumentState {
                text: text.to_string(),
                line_index,
                diagnostics: raw_diagnostics,
            },
        );
    }

    /// Re-diagnose all open documents (e.g., after config change).
    async fn rediagnose_all(&self) {
        let snapshots: Vec<(Url, String)> = {
            let docs = self.documents.read().await;
            docs.iter()
                .map(|(uri, state)| (uri.clone(), state.text.clone()))
                .collect()
        };

        for (uri, text) in snapshots {
            self.update_diagnostics(uri, &text).await;
        }
    }
}

/// Find diagnostics whose span contains the given byte offset.
/// Spans are end-exclusive: (start, end) where start is inclusive, end is exclusive.
fn diagnostics_at_byte<'a>(
    diagnostics: &'a [parikshak::Diagnostic],
    byte_offset: usize,
    config: &Config,
) -> Vec<&'a parikshak::Diagnostic> {
    diagnostics
        .iter()
        .filter(|d| {
            d.span.0 <= byte_offset
                && byte_offset < d.span.1
                && config.categories.is_enabled(d.category)
        })
        .collect()
}

/// Find diagnostics overlapping an LSP range.
fn diagnostics_in_range(
    diagnostics: &[parikshak::Diagnostic],
    range: &Range,
    line_index: &LineIndex,
    config: &Config,
) -> Vec<parikshak::Diagnostic> {
    let range_start = line_index.position_to_byte_offset(range.start);
    let range_end = line_index.position_to_byte_offset(range.end);

    diagnostics
        .iter()
        .filter(|d| {
            d.span.0 < range_end
                && d.span.1 > range_start
                && config.categories.is_enabled(d.category)
        })
        .cloned()
        .collect()
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "varnavinyas-lsp initialized")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.update_diagnostics(uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().next() {
            self.update_diagnostics(uri, &change.text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut docs = self.documents.write().await;
        docs.remove(&uri);
        // Clear diagnostics for closed document
        self.client.publish_diagnostics(uri, vec![], None).await;
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        // Try to extract varnavinyas settings
        let settings = match params.settings {
            Value::Object(ref map) => map
                .get("varnavinyas")
                .cloned()
                .unwrap_or(params.settings.clone()),
            _ => params.settings,
        };

        if let Ok(new_config) = serde_json::from_value::<Config>(settings) {
            let mut config = self.config.write().await;
            *config = new_config;
        }

        // Re-diagnose all open docs
        self.rediagnose_all().await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let docs = self.documents.read().await;
        let Some(doc) = docs.get(uri) else {
            return Ok(None);
        };

        let byte_offset = doc.line_index.position_to_byte_offset(position);
        let config = self.config.read().await;
        let hits = diagnostics_at_byte(&doc.diagnostics, byte_offset, &config);

        if hits.is_empty() {
            return Ok(None);
        }

        let mut parts = Vec::new();
        for diag in hits {
            parts.push(format!(
                "## {} → {}\n\n\
                 **Rule:** {}: {}\n\n\
                 **Category:** {}\n\n\
                 {}\n\n\
                 ---\n\n\
                 *Source: Nepal Academy Orthography Standard*",
                diag.incorrect,
                diag.correction,
                diag.rule.source_name(),
                diag.rule.code(),
                diag.category,
                diag.explanation,
            ));
        }

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: parts.join("\n\n"),
            }),
            range: None,
        }))
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = &params.text_document.uri;
        let range = &params.range;

        let docs = self.documents.read().await;
        let Some(doc) = docs.get(uri) else {
            return Ok(None);
        };

        let config = self.config.read().await;
        let hits = diagnostics_in_range(&doc.diagnostics, range, &doc.line_index, &config);

        if hits.is_empty() {
            return Ok(None);
        }

        let mut actions = Vec::new();
        for diag in &hits {
            let diag_range = doc.line_index.byte_span_to_range(diag.span);
            let edit = TextEdit {
                range: diag_range,
                new_text: diag.correction.clone(),
            };

            let mut changes = HashMap::new();
            changes.insert(uri.clone(), vec![edit]);

            let action = CodeAction {
                title: format!("{} → {}", diag.incorrect, diag.correction),
                kind: Some(CodeActionKind::QUICKFIX),
                diagnostics: Some(vec![tower_lsp::lsp_types::Diagnostic {
                    range: diag_range,
                    severity: Some(DiagnosticSeverity::WARNING),
                    source: Some("varnavinyas".to_string()),
                    code: Some(NumberOrString::String(diag.rule.code().to_string())),
                    message: format!(
                        "{} → {} ({})",
                        diag.incorrect, diag.correction, diag.category
                    ),
                    ..Default::default()
                }]),
                edit: Some(WorkspaceEdit {
                    changes: Some(changes),
                    ..Default::default()
                }),
                is_preferred: Some(true),
                ..Default::default()
            };
            actions.push(CodeActionOrCommand::CodeAction(action));
        }

        Ok(Some(actions))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use varnavinyas_parikshak::DiagnosticCategory;

    #[test]
    fn diagnostics_at_byte_filters_by_config() {
        let diag = parikshak::Diagnostic {
            span: (0, 30),
            incorrect: "अत्याधिक".to_string(),
            correction: "अत्यधिक".to_string(),
            rule: varnavinyas_prakriya::Rule::ShuddhaAshuddha("Section 4"),
            explanation: "test".to_string(),
            category: DiagnosticCategory::ShuddhaTable,
        };

        // Enabled — should find it
        let config = Config::default();
        let diags = [diag.clone()];
        let hits = diagnostics_at_byte(&diags, 5, &config);
        assert_eq!(hits.len(), 1);

        // Disabled — should filter it out
        let mut config2 = Config::default();
        config2.categories.shuddha_table = false;
        let diags2 = [diag];
        let hits2 = diagnostics_at_byte(&diags2, 5, &config2);
        assert!(hits2.is_empty());
    }
}
