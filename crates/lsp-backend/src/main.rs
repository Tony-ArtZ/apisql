use core_lib::errors::ParseError;
use core_lib::parser::parse_program;
use runtime::exec::ExecutionRuntime;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    documents: Arc<Mutex<HashMap<Url, String>>>,
    last_response: Arc<Mutex<HashMap<Url, Value>>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![
                        ".".to_string(),
                        " ".to_string(),
                        ",".to_string(),
                    ]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.documents.lock().unwrap().insert(
            params.text_document.uri.clone(),
            params.text_document.text.clone(),
        );
        self.validate_document(params.text_document.uri, &params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.first() {
            self.documents
                .lock()
                .unwrap()
                .insert(params.text_document.uri.clone(), change.text.clone());
            self.validate_document(params.text_document.uri, &change.text)
                .await;
        }
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let mut items = Vec::new();

        // Add Keywords
        let keywords = vec![
            "USING", "REQUEST", "RESPONSE", "GET", "POST", "PUT", "DELETE", "PATCH", "AND", "OR",
            "SELECT", "FROM", "WHERE", "LIMIT",
        ];
        for kw in keywords {
            items.push(CompletionItem {
                label: kw.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..Default::default()
            });
        }

        // Add Fields from JSON
        let responses = self.last_response.lock().unwrap();
        if let Some(json) = responses.get(&uri) {
            let doc_text = self
                .documents
                .lock()
                .unwrap()
                .get(&uri)
                .cloned()
                .unwrap_or_default();
            let from_path = self.extract_from_path(&doc_text);

            let root = self.resolve_json_path(json, &from_path);

            let target = if let Value::Array(arr) = root {
                arr.first().unwrap_or(root)
            } else {
                root
            };

            if let Value::Object(map) = target {
                for (k, _v) in map {
                    items.push(CompletionItem {
                        label: k.clone(),
                        kind: Some(CompletionItemKind::FIELD),
                        detail: Some("Field from API response".to_string()),
                        ..Default::default()
                    });
                }
            }
        }

        Ok(Some(CompletionResponse::Array(items)))
    }
}

impl Backend {
    fn extract_from_path(&self, text: &str) -> Vec<String> {
        use regex::Regex;
        // Find "FROM <path>"
        let re = Regex::new(r"(?i)FROM\s+([a-zA-Z0-9_.]+)").unwrap();
        if let Some(caps) = re.captures(text) {
            if let Some(path_str) = caps.get(1) {
                return path_str
                    .as_str()
                    .split('.')
                    .map(|s| s.to_string())
                    .collect();
            }
        }
        vec![]
    }

    fn resolve_json_path<'a>(&self, json: &'a Value, path: &[String]) -> &'a Value {
        let mut current = json;
        for segment in path {
            if segment == "body" {
                continue;
            }
            match current {
                Value::Object(map) => {
                    if let Some(val) = map.get(segment) {
                        current = val;
                    } else {
                        // If path segment not found, just return current to be safe
                        break;
                    }
                }
                _ => break,
            }
        }
        current
    }

    async fn validate_document(&self, uri: Url, text: &str) {
        let mut diagnostics = Vec::new();

        match parse_program(text) {
            Ok(program) => {
                // Try to fetch data
                if let Some(req) = program.request_blocks.first() {
                    let mut runtime = ExecutionRuntime::new();
                    match runtime.fetch_data(req) {
                        Ok(json) => {
                            self.last_response.lock().unwrap().insert(uri.clone(), json);
                        }
                        Err(e) => {
                            let diag = Diagnostic {
                                range: Range {
                                    start: Position {
                                        line: 0,
                                        character: 0,
                                    },
                                    end: Position {
                                        line: 0,
                                        character: 0,
                                    },
                                },
                                severity: Some(DiagnosticSeverity::WARNING),
                                message: format!("Failed to fetch data: {}", e),
                                ..Default::default()
                            };
                            diagnostics.push(diag);
                        }
                    }
                }
            }
            Err(e) => {
                if let ParseError::Syntax {
                    line,
                    column,
                    message,
                } = e
                {
                    let diag = Diagnostic {
                        range: Range {
                            start: Position {
                                line: (line as u32).saturating_sub(1),
                                character: (column as u32).saturating_sub(1),
                            },
                            end: Position {
                                line: (line as u32).saturating_sub(1),
                                character: (column as u32),
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: format!("{}", message),
                        source: Some("apisql".to_string()),
                        ..Default::default()
                    };
                    diagnostics.push(diag);
                }
            }
        }

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: Arc::new(Mutex::new(HashMap::new())),
        last_response: Arc::new(Mutex::new(HashMap::new())),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
