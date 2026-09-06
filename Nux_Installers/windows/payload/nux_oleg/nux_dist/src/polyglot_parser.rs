// Polyglot Parser - Handles parsing of multi-language code blocks
// This module extends the Nux parser to support embedded foreign language code

/// Supported foreign languages
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForeignLanguage {
    Python,
    JavaScript,
    Rust,
    C,
    Cpp,
    Java,
    Go,
    Haskell,
}

impl ForeignLanguage {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "python" | "py" => Some(ForeignLanguage::Python),
            "javascript" | "js" => Some(ForeignLanguage::JavaScript),
            "rust" | "rs" => Some(ForeignLanguage::Rust),
            "c" => Some(ForeignLanguage::C),
            "cpp" | "c++" | "cxx" => Some(ForeignLanguage::Cpp),
            "java" => Some(ForeignLanguage::Java),
            "go" | "golang" => Some(ForeignLanguage::Go),
            "haskell" | "hs" => Some(ForeignLanguage::Haskell),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ForeignLanguage::Python => "python",
            ForeignLanguage::JavaScript => "javascript",
            ForeignLanguage::Rust => "rust",
            ForeignLanguage::C => "c",
            ForeignLanguage::Cpp => "cpp",
            ForeignLanguage::Java => "java",
            ForeignLanguage::Go => "go",
            ForeignLanguage::Haskell => "haskell",
        }
    }
}

/// Represents a foreign language code block
#[derive(Debug, Clone)]
pub struct LanguageBlock {
    pub language: ForeignLanguage,
    pub code: String,
    pub line: usize,
    pub column: usize,
}

/// Represents an external library import
#[derive(Debug, Clone)]
pub struct ExternalImport {
    pub language: ForeignLanguage,
    pub module_path: String,
    pub items: Vec<ImportItem>,
    pub alias: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImportItem {
    pub name: String,
    pub alias: Option<String>,
}

/// Represents an inline foreign expression
#[derive(Debug, Clone)]
pub struct InlineForeignExpr {
    pub language: ForeignLanguage,
    pub expression: String,
    pub line: usize,
    pub column: usize,
}

/// Polyglot parser state
pub struct PolyglotParser {
    language_blocks: Vec<LanguageBlock>,
    external_imports: Vec<ExternalImport>,
    inline_expressions: Vec<InlineForeignExpr>,
}

impl PolyglotParser {
    pub fn new() -> Self {
        PolyglotParser {
            language_blocks: Vec::new(),
            external_imports: Vec::new(),
            inline_expressions: Vec::new(),
        }
    }

    /// Parse a language block: @language { code }
    pub fn parse_language_block(
        &mut self,
        language_str: &str,
        code: &str,
        line: usize,
        column: usize,
    ) -> Result<usize, String> {
        let language = ForeignLanguage::from_str(language_str)
            .ok_or_else(|| format!("Unsupported language: {}", language_str))?;

        let block = LanguageBlock {
            language,
            code: code.to_string(),
            line,
            column,
        };

        self.language_blocks.push(block);
        Ok(self.language_blocks.len() - 1)
    }

    /// Parse an external import: import python:numpy as np
    pub fn parse_external_import(
        &mut self,
        language_str: &str,
        module_path: &str,
        alias: Option<String>,
    ) -> Result<usize, String> {
        let language = ForeignLanguage::from_str(language_str)
            .ok_or_else(|| format!("Unsupported language: {}", language_str))?;

        let import = ExternalImport {
            language,
            module_path: module_path.to_string(),
            items: Vec::new(),
            alias,
        };

        self.external_imports.push(import);
        Ok(self.external_imports.len() - 1)
    }

    /// Parse a from-import: from python:pandas import DataFrame, Series
    pub fn parse_from_import(
        &mut self,
        language_str: &str,
        module_path: &str,
        items: Vec<(String, Option<String>)>,
    ) -> Result<usize, String> {
        let language = ForeignLanguage::from_str(language_str)
            .ok_or_else(|| format!("Unsupported language: {}", language_str))?;

        let import_items: Vec<ImportItem> = items
            .into_iter()
            .map(|(name, alias)| ImportItem { name, alias })
            .collect();

        let import = ExternalImport {
            language,
            module_path: module_path.to_string(),
            items: import_items,
            alias: None,
        };

        self.external_imports.push(import);
        Ok(self.external_imports.len() - 1)
    }

    /// Parse an inline foreign expression: @python(sum([1, 2, 3]))
    pub fn parse_inline_expression(
        &mut self,
        language_str: &str,
        expression: &str,
        line: usize,
        column: usize,
    ) -> Result<usize, String> {
        let language = ForeignLanguage::from_str(language_str)
            .ok_or_else(|| format!("Unsupported language: {}", language_str))?;

        let inline_expr = InlineForeignExpr {
            language,
            expression: expression.to_string(),
            line,
            column,
        };

        self.inline_expressions.push(inline_expr);
        Ok(self.inline_expressions.len() - 1)
    }

    /// Get all parsed language blocks
    pub fn get_language_blocks(&self) -> &[LanguageBlock] {
        &self.language_blocks
    }

    /// Get all parsed external imports
    pub fn get_external_imports(&self) -> &[ExternalImport] {
        &self.external_imports
    }

    /// Get all parsed inline expressions
    pub fn get_inline_expressions(&self) -> &[InlineForeignExpr] {
        &self.inline_expressions
    }

    /// Validate a language block's syntax (basic validation)
    pub fn validate_block(&self, block_id: usize) -> Result<(), String> {
        if block_id >= self.language_blocks.len() {
            return Err("Invalid block ID".to_string());
        }

        let block = &self.language_blocks[block_id];

        // Basic validation - check for balanced braces, quotes, etc.
        match block.language {
            ForeignLanguage::Python => self.validate_python(&block.code),
            ForeignLanguage::JavaScript => self.validate_javascript(&block.code),
            ForeignLanguage::Rust => self.validate_rust(&block.code),
            ForeignLanguage::C | ForeignLanguage::Cpp => self.validate_c_cpp(&block.code),
            _ => Ok(()), // Other languages validated at runtime
        }
    }

    fn validate_python(&self, code: &str) -> Result<(), String> {
        // Basic Python validation - check indentation consistency
        let lines: Vec<&str> = code.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            // Check for tabs mixed with spaces
            if line.contains('\t') && line.contains("    ") {
                return Err(format!("Line {}: Mixed tabs and spaces in Python code", i + 1));
            }
        }
        Ok(())
    }

    fn validate_javascript(&self, code: &str) -> Result<(), String> {
        // Basic JavaScript validation - check for balanced braces
        let mut brace_count = 0;
        let mut paren_count = 0;
        let mut bracket_count = 0;

        for ch in code.chars() {
            match ch {
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                '(' => paren_count += 1,
                ')' => paren_count -= 1,
                '[' => bracket_count += 1,
                ']' => bracket_count -= 1,
                _ => {}
            }
        }

        if brace_count != 0 {
            return Err("Unbalanced braces in JavaScript code".to_string());
        }
        if paren_count != 0 {
            return Err("Unbalanced parentheses in JavaScript code".to_string());
        }
        if bracket_count != 0 {
            return Err("Unbalanced brackets in JavaScript code".to_string());
        }

        Ok(())
    }

    fn validate_rust(&self, code: &str) -> Result<(), String> {
        // Basic Rust validation - check for balanced braces
        let mut brace_count = 0;

        for ch in code.chars() {
            match ch {
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                _ => {}
            }
        }

        if brace_count != 0 {
            return Err("Unbalanced braces in Rust code".to_string());
        }

        Ok(())
    }

    fn validate_c_cpp(&self, code: &str) -> Result<(), String> {
        // Basic C/C++ validation - check for balanced braces and semicolons
        let mut brace_count = 0;

        for ch in code.chars() {
            match ch {
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                _ => {}
            }
        }

        if brace_count != 0 {
            return Err("Unbalanced braces in C/C++ code".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_language_block() {
        let mut parser = PolyglotParser::new();
        let result = parser.parse_language_block(
            "python",
            "def hello():\n    print('Hello')",
            1,
            1,
        );
        assert!(result.is_ok());
        assert_eq!(parser.get_language_blocks().len(), 1);
    }

    #[test]
    fn test_parse_external_import() {
        let mut parser = PolyglotParser::new();
        let result = parser.parse_external_import("python", "numpy", Some("np".to_string()));
        assert!(result.is_ok());
        assert_eq!(parser.get_external_imports().len(), 1);
    }

    #[test]
    fn test_validate_python() {
        let mut parser = PolyglotParser::new();
        parser.parse_language_block("python", "def foo():\n    pass", 1, 1).unwrap();
        assert!(parser.validate_block(0).is_ok());
    }

    #[test]
    fn test_validate_javascript() {
        let mut parser = PolyglotParser::new();
        parser.parse_language_block("javascript", "function foo() { return 42; }", 1, 1).unwrap();
        assert!(parser.validate_block(0).is_ok());
    }
}
