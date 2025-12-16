use std::collections::HashMap;
use std::path::PathBuf;
use syster::core::constants::{KERML_EXT, SYSML_EXT};
use syster::project::ParseError;
use syster::semantic::Workspace;
use syster::semantic::symbol_table::Symbol;
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, Hover, HoverContents, MarkedString, Position, Range, Url,
};

/// Backend manages the workspace state for the LSP server
pub struct Backend {
    workspace: Workspace,
    /// Track parse errors for each file (keyed by file path)
    parse_errors: HashMap<PathBuf, Vec<ParseError>>,
    /// Track document text for hover and other features (keyed by file path)
    document_texts: HashMap<PathBuf, String>,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            workspace: Workspace::new(),
            parse_errors: HashMap::new(),
            document_texts: HashMap::new(),
        }
    }

    pub fn workspace(&self) -> &Workspace {
        &self.workspace
    }

    pub fn workspace_mut(&mut self) -> &mut Workspace {
        &mut self.workspace
    }

    /// Parse and update a document in the workspace
    ///
    /// This is a helper method that handles:
    /// - Storing document text
    /// - Parsing the file
    /// - Storing parse errors
    /// - Updating the workspace
    /// - Repopulating symbols
    fn parse_and_update(&mut self, uri: &Url, text: &str, is_update: bool) -> Result<(), String> {
        let path = uri
            .to_file_path()
            .map_err(|_| format!("Invalid file URI: {}", uri))?;

        // Store document text
        self.document_texts.insert(path.clone(), text.to_string());

        // Parse the file based on extension
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| "File has no extension".to_string())?;

        let parse_result = match ext {
            SYSML_EXT => syster::project::file_loader::parse_with_result(text, &path),
            KERML_EXT => return Err("KerML files not yet fully supported".to_string()),
            _ => return Err(format!("Unsupported file extension: {}", ext)),
        };

        // Store parse errors
        self.parse_errors.insert(path.clone(), parse_result.errors);

        // If updating, remove old file first
        if is_update {
            self.workspace.remove_file(&path);
        }

        // If parsing succeeded, add to workspace
        if let Some(file) = parse_result.content {
            self.workspace.add_file(path, file);
            // Populate symbols - ignore semantic errors for now
            let _ = self.workspace.populate_all();
        }

        Ok(())
    }

    /// Open a document and add it to the workspace
    pub fn open_document(&mut self, uri: &Url, text: &str) -> Result<(), String> {
        self.parse_and_update(uri, text, false)
    }

    /// Update an open document with new content
    pub fn change_document(&mut self, uri: &Url, text: &str) -> Result<(), String> {
        self.parse_and_update(uri, text, true)
    }

    /// Close a document - optionally remove from workspace
    /// For now, we keep documents in workspace even after close
    /// to maintain cross-file references
    pub fn close_document(&mut self, _uri: &Url) -> Result<(), String> {
        // We don't remove from workspace to keep cross-file references working
        // In the future, might want to track "open" vs "workspace" files separately
        Ok(())
    }

    /// Get LSP diagnostics for a given file
    pub fn get_diagnostics(&self, uri: &Url) -> Vec<Diagnostic> {
        let path = match uri.to_file_path() {
            Ok(p) => p,
            Err(_) => return vec![],
        };

        // Convert parse errors to LSP diagnostics
        self.parse_errors
            .get(&path)
            .map(|errors| {
                errors
                    .iter()
                    .map(|e| Diagnostic {
                        range: Range {
                            start: Position {
                                line: e.position.line as u32,
                                character: e.position.column as u32,
                            },
                            end: Position {
                                line: e.position.line as u32,
                                character: (e.position.column + 1) as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: e.message.clone(),
                        ..Default::default()
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get hover information for a symbol at the given position
    ///
    /// For now, this is a simple implementation that extracts a word at the cursor
    /// and looks it up in the symbol table. A more sophisticated implementation would
    /// parse the AST to find the exact symbol at the position.
    pub fn get_hover(&self, uri: &Url, position: Position) -> Option<Hover> {
        let path = uri.to_file_path().ok()?;

        // Get document text
        let text = self.document_texts.get(&path)?;

        // Extract word at position
        let word = extract_word_at_position(text, position)?;

        // Look up symbol in workspace
        let symbol = self.workspace.symbol_table().lookup(&word)?;

        // Format hover content based on symbol type
        let content = format_symbol_hover(symbol);

        Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(content)),
            range: None,
        })
    }
}

/// Extract a word (identifier) at the given position from text
/// TODO: Remove this once AST position tracking is implemented (task 8+9)
/// This is a temporary workaround for hover - proper implementation will query AST directly
fn extract_word_at_position(text: &str, position: Position) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    let line = lines.get(position.line as usize)?;
    let col = position.character as usize;

    if col >= line.len() {
        return None;
    }

    // Find word boundaries
    let chars: Vec<char> = line.chars().collect();

    // Check if we're on a word character
    if !chars[col].is_alphanumeric() && chars[col] != '_' {
        return None;
    }

    // Find start of word
    let mut start = col;
    while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_') {
        start -= 1;
    }

    // Find end of word
    let mut end = col;
    while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
        end += 1;
    }

    Some(chars[start..end].iter().collect())
}

/// Format hover content with consistent markdown styling
/// TODO: Remove this once AST position tracking is implemented (task 8+9)
/// Temporary helper for basic hover - will be replaced with rich contextual hover
fn format_hover_content(syntax: &str, qualified_name: Option<&str>) -> String {
    let mut result = format!("```sysml\n{}\n```", syntax);
    if let Some(qname) = qualified_name {
        result.push_str(&format!("\n\nQualified: `{}`", qname));
    }
    result
}

/// Format a symbol for hover display
/// TODO: Remove this once AST position tracking is implemented (task 8+9)
/// Temporary implementation - will be replaced with rich hover showing docs, signatures, relationships
fn format_symbol_hover(symbol: &Symbol) -> String {
    match symbol {
        Symbol::Alias { name, target, .. } => {
            format_hover_content(&format!("alias {} = {}", name, target), None)
        }
        Symbol::Package {
            name,
            qualified_name,
            ..
        } => format_hover_content(&format!("package {}", name), Some(qualified_name)),
        Symbol::Classifier {
            name,
            qualified_name,
            kind,
            is_abstract,
            ..
        } => {
            let prefix = if *is_abstract { "abstract " } else { "" };
            format_hover_content(
                &format!("{}{} {}", prefix, kind, name),
                Some(qualified_name),
            )
        }
        Symbol::Definition {
            name,
            qualified_name,
            kind,
            ..
        } => format_hover_content(&format!("{} def {}", kind, name), Some(qualified_name)),
        Symbol::Usage {
            name,
            qualified_name,
            kind,
            ..
        } => format_hover_content(&format!("{} {}", kind, name), Some(qualified_name)),
        Symbol::Feature {
            name,
            qualified_name,
            feature_type,
            ..
        } => {
            let type_str = feature_type
                .as_ref()
                .map(|t| format!(": {}", t))
                .unwrap_or_default();
            format_hover_content(
                &format!("feature {}{}", name, type_str),
                Some(qualified_name),
            )
        }
    }
}

#[cfg(test)]
mod tests;
