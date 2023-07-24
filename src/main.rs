use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use serde_json::Value;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

const FAILED_TO_ACQUIRE_LOCK_MSG: &str = "failed to acquire lock";

fn get_char_index_from_position(s: &str, position: Position) -> usize {
    let line_start = s
        .lines()
        .take(position.line as usize)
        .map(|line| line.len() + 1)
        .sum::<usize>();

    let char_index = line_start + position.character as usize;

    if char_index > s.len() {
        s.len()
    } else {
        char_index
    }
}

#[derive(Debug)]
struct Backend {
    client: Client,
    document_text: Arc<Mutex<String>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::INFO, "workspace folders changed!")
            .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::INFO, "configuration changed!")
            .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::INFO, "watched files have changed!")
            .await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client
            .log_message(MessageType::INFO, "command executed!")
            .await;

        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        *self.document_text.lock().expect(FAILED_TO_ACQUIRE_LOCK_MSG) = params.text_document.text;

        self.client
            .log_message(MessageType::INFO, "file opened!")
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        for change in params.content_changes {
            match change.range {
                Some(range) => {
                    let mut text = self.document_text.lock().expect(FAILED_TO_ACQUIRE_LOCK_MSG);

                    let start = get_char_index_from_position(text.as_str(), range.start);
                    let end = get_char_index_from_position(text.as_str(), range.end);

                    text.replace_range(start..end, change.text.as_str());
                }
                None => {
                    *self.document_text.lock().expect(FAILED_TO_ACQUIRE_LOCK_MSG) = change.text;
                }
            }
        }

        self.client
            .log_message(MessageType::INFO, "file changed!")
            .await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let text = self.document_text.lock().expect("failed to acquire lock");

        let position = params.text_document_position.position;
        let end = get_char_index_from_position(text.as_str(), position);

        let distance = text
            .chars()
            .rev()
            .skip(text.len() - end)
            .enumerate()
            .find(|(_, ch)| ch.is_whitespace() || ch.is_ascii_punctuation())
            .map_or(end, |t| t.0);

        let start = end - distance;
        let current_word = &text[start..end];

        let words = text.split(|ch: char| ch.is_whitespace() || ch.is_ascii_punctuation());

        Ok(Some(CompletionResponse::Array(
            HashSet::<&str>::from_iter(words)
                .into_iter()
                .filter_map(|word| {
                    if word == current_word {
                        return None;
                    }

                    Some(CompletionItem {
                        label: word.to_string(),
                        detail: None,
                        kind: Some(CompletionItemKind::TEXT),
                        ..CompletionItem::default()
                    })
                })
                .collect(),
        )))
    }
}

#[tokio::main]
async fn main() {
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    let (service, socket) = LspService::new(|client| Backend {
        client,
        document_text: Arc::new(Mutex::new(String::new())),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
