// Duck Language Server
// Provides IDE features for Duck programming language

use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

// Document storage - maps URI to rope (efficient string)
struct Backend {
    client: Client,
    documents: DashMap<String, Rope>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Backend {
            client,
            documents: DashMap::new(),
        }
    }

    // Analyze document and return diagnostics
    async fn analyze(&self, _uri: &Url, text: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = text.lines().collect();

        // Track bracket depth and quack state
        #[allow(unused_assignments)]
        let mut in_block = false;
        let mut block_start_line = 0;
        let mut bracket_depth: usize = 0;
        let mut last_quack_line: Option<usize> = None;
        let mut unquacked_blocks: Vec<(usize, usize, usize)> = Vec::new(); // (line, start_col, end_col)

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Skip comments and empty lines
            if trimmed.starts_with("--") || trimmed.is_empty() {
                continue;
            }

            // Check for quack keyword
            let chars: Vec<char> = line.chars().collect();
            let mut i = 0;
            let mut col = 0;

            while i < chars.len() {
                // Skip whitespace
                while i < chars.len() && chars[i].is_whitespace() {
                    i += 1;
                    col += 1;
                }

                if i >= chars.len() {
                    break;
                }

                // Check for comment
                if i + 1 < chars.len() && chars[i] == '-' && chars[i + 1] == '-' {
                    break;
                }

                // Check for quack keyword
                if i + 5 <= chars.len() {
                    let word: String = chars[i..i+5].iter().collect();
                    if word == "quack" && (i + 5 >= chars.len() || !chars[i + 5].is_alphanumeric()) {
                        last_quack_line = Some(line_num);
                        i += 5;
                        col += 5;
                        continue;
                    }
                }

                // Check for opening bracket (potential unquacked block)
                if chars[i] == '[' {
                    bracket_depth += 1;
                    if bracket_depth == 1 {
                        in_block = true;
                        block_start_line = line_num;

                        // Check if this block was quacked
                        if last_quack_line != Some(line_num) {
                            // Find the extent of the bracket
                            let start_col = col;
                            unquacked_blocks.push((line_num, start_col, start_col + 1));
                        }
                        last_quack_line = None; // Consume the quack
                    }
                    i += 1;
                    col += 1;
                    continue;
                }

                // Check for closing bracket
                if chars[i] == ']' {
                    bracket_depth = bracket_depth.saturating_sub(1);
                    if bracket_depth == 0 {
                        in_block = false;
                    }
                    i += 1;
                    col += 1;
                    continue;
                }

                // Check for common mistakes
                // Using = instead of be/becomes
                if chars[i] == '=' && (i == 0 || chars[i-1] != '!' && chars[i-1] != '<' && chars[i-1] != '>') {
                    if i + 1 < chars.len() && chars[i + 1] != '=' {
                        // Single = that's not == or != or <= or >=
                        let context_start = i.saturating_sub(10);
                        let context: String = chars[context_start..i].iter().collect();

                        // Check if it looks like assignment
                        if context.contains("let") || (i > 0 && chars[i-1].is_alphanumeric()) {
                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position { line: line_num as u32, character: col as u32 },
                                    end: Position { line: line_num as u32, character: (col + 1) as u32 },
                                },
                                severity: Some(DiagnosticSeverity::ERROR),
                                code: Some(NumberOrString::String("duck/use-be-becomes".into())),
                                source: Some("duck-lsp".into()),
                                message: "Use 'be' for declaration or 'becomes' for assignment, not '='".into(),
                                ..Default::default()
                            });
                        }
                    }
                }

                i += 1;
                col += 1;
            }
        }

        // Add diagnostics for unquacked blocks
        for (line, start, end) in unquacked_blocks {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position { line: line as u32, character: start as u32 },
                    end: Position { line: line as u32, character: end as u32 },
                },
                severity: Some(DiagnosticSeverity::WARNING),
                code: Some(NumberOrString::String("duck/missing-quack".into())),
                source: Some("duck-lsp".into()),
                message: "Block is missing 'quack' - the goose will refuse to run this".into(),
                ..Default::default()
            });
        }

        // Check for unclosed brackets
        if bracket_depth > 0 {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position { line: block_start_line as u32, character: 0 },
                    end: Position { line: block_start_line as u32, character: 1 },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("duck/unclosed-bracket".into())),
                source: Some("duck-lsp".into()),
                message: format!("Unclosed bracket - {} bracket(s) still open", bracket_depth),
                ..Default::default()
            });
        }

        // Check for common string mistakes
        for (line_num, line) in lines.iter().enumerate() {
            // Single quotes
            if line.contains("'") && !line.contains("\"") {
                if let Some(pos) = line.find("'") {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: line_num as u32, character: pos as u32 },
                            end: Position { line: line_num as u32, character: (pos + 1) as u32 },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: Some(NumberOrString::String("duck/single-quotes".into())),
                        source: Some("duck-lsp".into()),
                        message: "Duck uses double quotes for strings, not single quotes".into(),
                        ..Default::default()
                    });
                }
            }

            // C-style comments
            if line.contains("//") && !line.contains("://") {
                if let Some(pos) = line.find("//") {
                    // Make sure it's not part of a URL
                    if pos == 0 || !line[..pos].ends_with(':') {
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { line: line_num as u32, character: pos as u32 },
                                end: Position { line: line_num as u32, character: (pos + 2) as u32 },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: Some(NumberOrString::String("duck/wrong-comment".into())),
                            source: Some("duck-lsp".into()),
                            message: "Duck uses -- for comments, not //".into(),
                            ..Default::default()
                        });
                    }
                }
            }

            // Array indexing with brackets
            if line.contains("[") && line.contains("]") {
                // Look for pattern like `something[0]` or `list[i]`
                let chars: Vec<char> = line.chars().collect();
                for i in 1..chars.len() {
                    if chars[i] == '[' && chars[i-1].is_alphanumeric() {
                        // Check if this looks like array indexing
                        let mut j = i + 1;
                        while j < chars.len() && (chars[j].is_alphanumeric() || chars[j] == ' ') {
                            j += 1;
                        }
                        if j < chars.len() && chars[j] == ']' {
                            diagnostics.push(Diagnostic {
                                range: Range {
                                    start: Position { line: line_num as u32, character: i as u32 },
                                    end: Position { line: line_num as u32, character: (j + 1) as u32 },
                                },
                                severity: Some(DiagnosticSeverity::ERROR),
                                code: Some(NumberOrString::String("duck/use-at".into())),
                                source: Some("duck-lsp".into()),
                                message: "Use 'list at index' for indexing, not 'list[index]'".into(),
                                ..Default::default()
                            });
                        }
                    }
                }
            }
        }

        diagnostics
    }
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
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".into(), "[".into()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "duck-lsp".into(),
                version: Some("0.1.0".into()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Duck LSP initialized! The goose is watching.")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        self.documents.insert(uri.to_string(), Rope::from_str(&text));

        let diagnostics = self.analyze(&uri, &text).await;
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;

        if let Some(change) = params.content_changes.into_iter().next() {
            let text = change.text;
            self.documents.insert(uri.to_string(), Rope::from_str(&text));

            let diagnostics = self.analyze(&uri, &text).await;
            self.client
                .publish_diagnostics(uri, diagnostics, None)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.remove(&params.text_document.uri.to_string());
        // Clear diagnostics when file is closed
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(doc) = self.documents.get(&uri.to_string()) {
            let line_idx = position.line as usize;
            if let Some(line) = doc.get_line(line_idx) {
                let line_str: String = line.chars().collect();
                let char_idx = position.character as usize;

                // Find the word at cursor
                let word = get_word_at_position(&line_str, char_idx);

                // Provide hover info for keywords and builtins
                if let Some(info) = get_hover_info(&word) {
                    return Ok(Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: info,
                        }),
                        range: None,
                    }));
                }
            }
        }

        Ok(None)
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let mut completions = Vec::new();

        // Keywords
        let keywords = vec![
            ("quack", "Execute a block", CompletionItemKind::KEYWORD),
            ("let", "Declare a variable", CompletionItemKind::KEYWORD),
            ("be", "Initialize a variable value", CompletionItemKind::KEYWORD),
            ("becomes", "Assign a new value", CompletionItemKind::KEYWORD),
            ("define", "Define a function", CompletionItemKind::KEYWORD),
            ("taking", "Function parameters", CompletionItemKind::KEYWORD),
            ("as", "Function body", CompletionItemKind::KEYWORD),
            ("return", "Return from function", CompletionItemKind::KEYWORD),
            ("if", "Conditional statement", CompletionItemKind::KEYWORD),
            ("then", "If body", CompletionItemKind::KEYWORD),
            ("otherwise", "Else branch", CompletionItemKind::KEYWORD),
            ("while", "While loop", CompletionItemKind::KEYWORD),
            ("do", "Loop body", CompletionItemKind::KEYWORD),
            ("for", "For loop", CompletionItemKind::KEYWORD),
            ("each", "For each iteration", CompletionItemKind::KEYWORD),
            ("in", "Loop collection", CompletionItemKind::KEYWORD),
            ("repeat", "Repeat N times", CompletionItemKind::KEYWORD),
            ("times", "Repeat count", CompletionItemKind::KEYWORD),
            ("break", "Exit loop", CompletionItemKind::KEYWORD),
            ("continue", "Next iteration", CompletionItemKind::KEYWORD),
            ("struct", "Define a struct", CompletionItemKind::KEYWORD),
            ("with", "Struct fields", CompletionItemKind::KEYWORD),
            ("attempt", "Try block", CompletionItemKind::KEYWORD),
            ("rescue", "Catch errors", CompletionItemKind::KEYWORD),
            ("migrate", "Import library", CompletionItemKind::KEYWORD),
            ("and", "Logical AND", CompletionItemKind::OPERATOR),
            ("or", "Logical OR", CompletionItemKind::OPERATOR),
            ("not", "Logical NOT", CompletionItemKind::OPERATOR),
            ("true", "Boolean true", CompletionItemKind::VALUE),
            ("false", "Boolean false", CompletionItemKind::VALUE),
            ("nil", "Null value", CompletionItemKind::VALUE),
            ("at", "Index access", CompletionItemKind::KEYWORD),
        ];

        for (name, detail, kind) in keywords {
            completions.push(CompletionItem {
                label: name.into(),
                kind: Some(kind),
                detail: Some(detail.into()),
                ..Default::default()
            });
        }

        // Builtins
        let builtins = vec![
            ("print", "Print values to console"),
            ("input", "Read line from user"),
            ("len", "Get length of string or list"),
            ("type-of", "Get type name"),
            ("string", "Convert to string"),
            ("number", "Convert to number"),
            ("list", "Create a list"),
            ("range", "Create range of numbers"),
            ("push", "Add to list"),
            ("pop", "Remove from list"),
            ("map", "Transform list elements"),
            ("filter", "Filter list elements"),
            ("fold", "Reduce list to value"),
            ("find", "Find element in list"),
            ("any", "Check if any match"),
            ("all", "Check if all match"),
            ("sort", "Sort list"),
            ("reverse", "Reverse list/string"),
            ("join", "Join list to string"),
            ("split", "Split string to list"),
            ("contains", "Check membership"),
            ("trim", "Remove whitespace"),
            ("uppercase", "Convert to uppercase"),
            ("lowercase", "Convert to lowercase"),
            ("abs", "Absolute value"),
            ("floor", "Round down"),
            ("ceil", "Round up"),
            ("sqrt", "Square root"),
            ("pow", "Power"),
            ("min", "Minimum value"),
            ("max", "Maximum value"),
            ("random", "Random 0-1"),
            ("read-file", "Read file contents"),
            ("write-file", "Write to file"),
            ("append-file", "Append to file"),
            ("file-exists", "Check file exists"),
            ("env", "Get environment variable"),
            ("sleep", "Pause execution"),
            ("json-parse", "Parse JSON string"),
            ("json-stringify", "Convert to JSON"),
            ("http-get", "HTTP GET request"),
            ("http-post", "HTTP POST request"),
            ("base64-encode", "Encode to base64"),
            ("base64-decode", "Decode from base64"),
            ("ws-connect", "Connect to WebSocket"),
            ("ws-send", "Send WebSocket message"),
            ("ws-receive", "Receive WebSocket message"),
            ("ws-close", "Close WebSocket"),
            ("ws-connected", "Check WebSocket status"),
            ("keys", "Get struct field names"),
            ("values", "Get struct field values"),
        ];

        for (name, detail) in builtins {
            completions.push(CompletionItem {
                label: name.into(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(detail.into()),
                ..Default::default()
            });
        }

        Ok(Some(CompletionResponse::Array(completions)))
    }
}

fn get_word_at_position(line: &str, char_idx: usize) -> String {
    let chars: Vec<char> = line.chars().collect();
    if char_idx >= chars.len() {
        return String::new();
    }

    // Find word boundaries (allow hyphens in identifiers)
    let mut start = char_idx;
    while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '-' || chars[start - 1] == '_') {
        start -= 1;
    }

    let mut end = char_idx;
    while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '-' || chars[end] == '_') {
        end += 1;
    }

    chars[start..end].iter().collect()
}

fn get_hover_info(word: &str) -> Option<String> {
    match word {
        "quack" => Some("**quack**\n\nAuthorize a block to execute. Without `quack`, blocks are parsed but not run.\n\n```duck\nquack [print \"Hello\"]\n```".into()),
        "let" => Some("**let**\n\nDeclare a new variable.\n\n```duck\nquack [let x be 42]\nquack [let name be \"Gerald\"]\n```".into()),
        "be" => Some("**be**\n\nInitialize a variable's value.\n\n```duck\nquack [let x be 42]\n```".into()),
        "becomes" => Some("**becomes**\n\nAssign a new value to an existing variable.\n\n```duck\nquack [x becomes x + 1]\n```".into()),
        "define" => Some("**define**\n\nCreate a function.\n\n```duck\nquack [define greet taking [name] as\n  quack [print f\"Hello, {name}!\"]\n]\n```".into()),
        "if" => Some("**if ... then ... otherwise**\n\nConditional execution.\n\n```duck\nquack [if x > 5 then\n  quack [print \"big\"]\notherwise\n  quack [print \"small\"]\n]\n```".into()),
        "while" => Some("**while ... do**\n\nLoop while condition is true.\n\n```duck\nquack [while x > 0 do\n  quack [print x]\n  quack [x becomes x - 1]\n]\n```".into()),
        "for" => Some("**for each [var] in collection do**\n\nIterate over a list.\n\n```duck\nquack [for each [item] in my-list do\n  quack [print item]\n]\n```".into()),
        "repeat" => Some("**repeat N times**\n\nRepeat a block N times.\n\n```duck\nquack [repeat 5 times\n  quack [print \"Quack!\"]\n]\n```".into()),
        "struct" => Some("**struct**\n\nDefine a custom data type.\n\n```duck\nquack [struct duck with [name, age]]\nquack [let d be duck(\"Gerald\", 5)]\n```".into()),
        "attempt" => Some("**attempt ... rescue**\n\nError handling.\n\n```duck\nquack [attempt\n  quack [risky-operation()]\nrescue err\n  quack [print f\"Error: {err}\"]\n]\n```".into()),
        "migrate" => Some("**migrate**\n\nImport a library.\n\n```duck\nquack [migrate \"git+konacodes/quests\" as quest]\nquack [quest.get(\"https://example.com\")]\n```".into()),
        "print" => Some("**print(...)**\n\nPrint values to the console.\n\n```duck\nquack [print \"Hello\"]\nquack [print x y z]\n```".into()),
        "list" => Some("**list(...)**\n\nCreate a new list.\n\n```duck\nquack [let nums be list(1, 2, 3)]\n```".into()),
        "map" => Some("**map(list, fn)**\n\nTransform each element.\n\n```duck\nquack [let doubled be map(nums, [x] -> x * 2)]\n```".into()),
        "filter" => Some("**filter(list, fn)**\n\nKeep matching elements.\n\n```duck\nquack [let evens be filter(nums, [x] -> x % 2 == 0)]\n```".into()),
        "http-get" => Some("**http-get(url, headers?)**\n\nMake an HTTP GET request.\n\n```duck\nquack [let response be http-get(\"https://api.example.com\")]\nquack [print response.status]\nquack [print response.body]\n```".into()),
        "http-post" => Some("**http-post(url, body, headers?)**\n\nMake an HTTP POST request.\n\n```duck\nquack [let response be http-post(url, body, headers)]\n```".into()),
        "ws-connect" => Some("**ws-connect(url)**\n\nConnect to a WebSocket server.\n\n```duck\nquack [let ws be ws-connect(\"wss://example.com\")]\n```".into()),
        "json-parse" => Some("**json-parse(string)**\n\nParse a JSON string into a Duck value.\n\n```duck\nquack [let data be json-parse(response.body)]\nquack [print data.name]\n```".into()),
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
