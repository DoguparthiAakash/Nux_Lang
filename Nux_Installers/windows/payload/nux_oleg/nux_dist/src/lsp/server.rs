// Language Server Protocol (LSP) - IDE support for Nux
// Provides auto-completion, go-to-definition, diagnostics, etc.

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// LSP server for Nux
pub struct NuxLSP {
    documents: HashMap<String, Document>,
    symbols: HashMap<String, Vec<Symbol>>,
    diagnostics: HashMap<String, Vec<Diagnostic>>,
}

/// Document representation
#[derive(Debug, Clone)]
struct Document {
    uri: String,
    version: i32,
    content: String,
    lines: Vec<String>,
}

/// Symbol (function, variable, class, etc.)
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Function,
    Variable,
    Class,
    Module,
    Constant,
}

/// Location in source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

/// Diagnostic (error, warning, info)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

/// Completion item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionItemKind {
    Function,
    Variable,
    Class,
    Module,
    Keyword,
    Snippet,
}

impl NuxLSP {
    pub fn new() -> Self {
        NuxLSP {
            documents: HashMap::new(),
            symbols: HashMap::new(),
            diagnostics: HashMap::new(),
        }
    }

    /// Initialize LSP server
    pub fn initialize(&mut self) -> Result<(), LSPError> {
        println!("[LSP] Nux Language Server initialized");
        Ok(())
    }

    /// Open document
    pub fn did_open(&mut self, uri: String, version: i32, content: String) -> Result<(), LSPError> {
        println!("[LSP] Document opened: {}", uri);

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        let document = Document {
            uri: uri.clone(),
            version,
            content: content.clone(),
            lines,
        };

        self.documents.insert(uri.clone(), document);

        // Parse document and extract symbols
        self.parse_document(&uri)?;

        // Run diagnostics
        self.run_diagnostics(&uri)?;

        Ok(())
    }

    /// Document changed
    pub fn did_change(&mut self, uri: &str, version: i32, content: String) -> Result<(), LSPError> {
        println!("[LSP] Document changed: {}", uri);

        if let Some(doc) = self.documents.get_mut(uri) {
            doc.version = version;
            doc.content = content.clone();
            doc.lines = content.lines().map(|s| s.to_string()).collect();

            // Re-parse and re-diagnose
            self.parse_document(uri)?;
            self.run_diagnostics(uri)?;
        }

        Ok(())
    }

    /// Provide completions
    pub fn completion(&self, uri: &str, position: Position) -> Result<Vec<CompletionItem>, LSPError> {
        println!("[LSP] Completion requested at {}:{}", position.line, position.character);

        let mut completions = Vec::new();

        // Add keywords
        for keyword in &["fn", "let", "if", "else", "while", "for", "return", "import"] {
            completions.push(CompletionItem {
                label: keyword.to_string(),
                kind: CompletionItemKind::Keyword,
                detail: Some("keyword".to_string()),
                documentation: None,
                insert_text: Some(keyword.to_string()),
            });
        }

        // Add symbols from current document
        if let Some(symbols) = self.symbols.get(uri) {
            for symbol in symbols {
                completions.push(CompletionItem {
                    label: symbol.name.clone(),
                    kind: match symbol.kind {
                        SymbolKind::Function => CompletionItemKind::Function,
                        SymbolKind::Variable => CompletionItemKind::Variable,
                        SymbolKind::Class => CompletionItemKind::Class,
                        SymbolKind::Module => CompletionItemKind::Module,
                        SymbolKind::Constant => CompletionItemKind::Variable,
                    },
                    detail: symbol.detail.clone(),
                    documentation: None,
                    insert_text: Some(symbol.name.clone()),
                });
            }
        }

        Ok(completions)
    }

    /// Go to definition
    pub fn goto_definition(&self, uri: &str, position: Position) -> Result<Option<Location>, LSPError> {
        println!("[LSP] Go to definition at {}:{}", position.line, position.character);

        // Get word at position
        if let Some(doc) = self.documents.get(uri) {
            if let Some(line) = doc.lines.get(position.line) {
                // Extract word at cursor (simplified)
                let word = self.extract_word(line, position.character);

                // Find symbol definition
                if let Some(symbols) = self.symbols.get(uri) {
                    for symbol in symbols {
                        if symbol.name == word {
                            return Ok(Some(symbol.location.clone()));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    /// Find references
    pub fn find_references(&self, uri: &str, position: Position) -> Result<Vec<Location>, LSPError> {
        println!("[LSP] Find references at {}:{}", position.line, position.character);

        // Simplified - would scan all documents for references
        Ok(vec![])
    }

    /// Get diagnostics
    pub fn get_diagnostics(&self, uri: &str) -> Vec<Diagnostic> {
        self.diagnostics.get(uri).cloned().unwrap_or_default()
    }

    /// Parse document and extract symbols
    fn parse_document(&mut self, uri: &str) -> Result<(), LSPError> {
        let mut symbols = Vec::new();

        if let Some(doc) = self.documents.get(uri) {
            for (line_num, line) in doc.lines.iter().enumerate() {
                // Simple parsing - detect function definitions
                if line.trim().starts_with("fn ") {
                    if let Some(name) = self.extract_function_name(line) {
                        symbols.push(Symbol {
                            name: name.clone(),
                            kind: SymbolKind::Function,
                            location: Location {
                                uri: uri.to_string(),
                                range: Range {
                                    start: Position { line: line_num, character: 0 },
                                    end: Position { line: line_num, character: line.len() },
                                },
                            },
                            detail: Some("function".to_string()),
                        });
                    }
                }

                // Detect variable declarations
                if line.trim().starts_with("let ") {
                    if let Some(name) = self.extract_variable_name(line) {
                        symbols.push(Symbol {
                            name: name.clone(),
                            kind: SymbolKind::Variable,
                            location: Location {
                                uri: uri.to_string(),
                                range: Range {
                                    start: Position { line: line_num, character: 0 },
                                    end: Position { line: line_num, character: line.len() },
                                },
                            },
                            detail: Some("variable".to_string()),
                        });
                    }
                }
            }
        }

        self.symbols.insert(uri.to_string(), symbols);
        Ok(())
    }

    /// Run diagnostics on document
    fn run_diagnostics(&mut self, uri: &str) -> Result<(), LSPError> {
        let mut diagnostics = Vec::new();

        if let Some(doc) = self.documents.get(uri) {
            for (line_num, line) in doc.lines.iter().enumerate() {
                // Check for common errors (simplified)
                
                // Unclosed string
                if line.contains('"') && line.matches('"').count() % 2 != 0 {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: line_num, character: 0 },
                            end: Position { line: line_num, character: line.len() },
                        },
                        severity: DiagnosticSeverity::Error,
                        message: "Unclosed string literal".to_string(),
                        source: "nux".to_string(),
                    });
                }

                // Unused variable warning (simplified)
                if line.trim().starts_with("let ") && !line.contains("=") {
                    diagnostics.push(Diagnostic {
                        range: Range {
                            start: Position { line: line_num, character: 0 },
                            end: Position { line: line_num, character: line.len() },
                        },
                        severity: DiagnosticSeverity::Warning,
                        message: "Variable declared but not initialized".to_string(),
                        source: "nux".to_string(),
                    });
                }
            }
        }

        self.diagnostics.insert(uri.to_string(), diagnostics);
        Ok(())
    }

    // Helper methods

    fn extract_word(&self, line: &str, pos: usize) -> String {
        // Simplified word extraction
        let chars: Vec<char> = line.chars().collect();
        let mut start = pos;
        let mut end = pos;

        while start > 0 && chars[start - 1].is_alphanumeric() {
            start -= 1;
        }

        while end < chars.len() && chars[end].is_alphanumeric() {
            end += 1;
        }

        chars[start..end].iter().collect()
    }

    fn extract_function_name(&self, line: &str) -> Option<String> {
        // Extract function name from "fn name(...)"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            Some(parts[1].trim_end_matches('(').to_string())
        } else {
            None
        }
    }

    fn extract_variable_name(&self, line: &str) -> Option<String> {
        // Extract variable name from "let name = ..."
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            Some(parts[1].trim_end_matches('=').to_string())
        } else {
            None
        }
    }
}

/// LSP errors
#[derive(Debug)]
pub enum LSPError {
    DocumentNotFound(String),
    ParseError(String),
}

impl std::fmt::Display for LSPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LSPError::DocumentNotFound(uri) => write!(f, "Document not found: {}", uri),
            LSPError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for LSPError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_creation() {
        let lsp = NuxLSP::new();
        assert_eq!(lsp.documents.len(), 0);
    }

    #[test]
    fn test_document_open() {
        let mut lsp = NuxLSP::new();
        let result = lsp.did_open(
            "file:///test.nux".to_string(),
            1,
            "fn main() {}".to_string()
        );
        assert!(result.is_ok());
        assert_eq!(lsp.documents.len(), 1);
    }

    #[test]
    fn test_completion() {
        let mut lsp = NuxLSP::new();
        lsp.did_open(
            "file:///test.nux".to_string(),
            1,
            "fn main() {}".to_string()
        ).unwrap();

        let completions = lsp.completion(
            "file:///test.nux",
            Position { line: 0, character: 0 }
        ).unwrap();

        assert!(completions.len() > 0);
    }
}
