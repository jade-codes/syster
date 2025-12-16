use std::collections::HashMap;
use std::path::PathBuf;
use syster::core::constants::{KERML_EXT, SYSML_EXT};
use syster::project::ParseError;
use syster::semantic::Workspace;
use syster::semantic::symbol_table::Symbol;
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, Hover, HoverContents, Location, MarkedString, Position, Range,
    Url,
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
    /// Uses AST span tracking to find the exact element under the cursor,
    /// then provides rich information including relationships and documentation.
    pub fn get_hover(&self, uri: &Url, position: Position) -> Option<Hover> {
        let path = uri.to_file_path().ok()?;

        // Find symbol at position using AST spans
        let (symbol_name, hover_range) = self.find_symbol_at_position(&path, position)?;

        // Look up symbol in workspace (try qualified name first, then simple name)
        let symbol = self
            .workspace
            .symbol_table()
            .lookup_qualified(&symbol_name)
            .or_else(|| self.workspace.symbol_table().lookup(&symbol_name))?;

        // Format rich hover content with relationships
        let content = format_rich_hover(symbol, self.workspace());

        Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String(content)),
            range: Some(hover_range),
        })
    }

    /// Get the definition location for a symbol at the given position
    ///
    /// This method:
    /// 1. Finds the symbol at the cursor position using AST spans
    /// 2. Looks up the symbol in the symbol table
    /// 3. Returns the location where the symbol is defined
    ///
    /// If the cursor is on a type reference, this returns the definition of that type.
    /// If the cursor is on a definition itself, this returns the location of that definition.
    pub fn get_definition(&self, uri: &Url, position: Position) -> Option<Location> {
        let path = uri.to_file_path().ok()?;

        // Get document text to extract word at position
        let text = self.document_texts.get(&path)?;

        // Find symbol at position using AST spans (this gives us the containing element)
        let (element_name, _hover_range) = self.find_symbol_at_position(&path, position)?;

        // Extract the actual word under the cursor - this might be different from element_name
        // if the cursor is on a type reference (e.g., ": Car" in "part myCar : Car")
        let cursor_word = extract_word_at_cursor(text, position)?;

        // Try to look up the word under cursor first (handles type references)
        // Then fall back to the element name (handles hovering on the element itself)
        let lookup_name = if cursor_word != element_name {
            // Cursor is on something other than the element name (likely a type reference)
            &cursor_word
        } else {
            // Cursor is on the element itself
            &element_name
        };

        // Look up symbol in workspace
        // Try qualified lookup first, then simple name lookup, then search all symbols
        let symbol = self
            .workspace
            .symbol_table()
            .lookup_qualified(lookup_name)
            .or_else(|| self.workspace.symbol_table().lookup(lookup_name))
            .or_else(|| {
                // Fallback: search all symbols for matching simple name
                self.workspace
                    .symbol_table()
                    .all_symbols()
                    .into_iter()
                    .find(|(_key, sym)| sym.name() == lookup_name)
                    .map(|(_, sym)| sym)
            })?;

        // Get definition location from symbol
        let source_file = symbol.source_file()?;
        let span = symbol.span()?;

        // Convert file path to URI
        let def_uri = Url::from_file_path(source_file).ok()?;

        Some(Location {
            uri: def_uri,
            range: span_to_lsp_range(&span),
        })
    }

    /// Find all references to a symbol at the given position
    ///
    /// This method:
    /// 1. Identifies the symbol at the cursor position
    /// 2. Looks up the symbol in the symbol table
    /// 3. Searches all files in the workspace for references to that symbol
    /// 4. Returns locations of all references (optionally including the declaration)
    ///
    /// # Parameters
    /// - `uri`: The file containing the symbol
    /// - `position`: The cursor position on the symbol
    /// - `include_declaration`: Whether to include the symbol's definition in results
    pub fn get_references(
        &self,
        uri: &Url,
        position: Position,
        include_declaration: bool,
    ) -> Option<Vec<Location>> {
        let path = uri.to_file_path().ok()?;

        // Get document text to extract word at position
        let text = self.document_texts.get(&path)?;

        // Find symbol at position (this gives us the containing element)
        let (element_name, _range) = self.find_symbol_at_position(&path, position)?;

        // Extract the actual word under cursor (handles type references)
        let cursor_word = extract_word_at_cursor(text, position)?;

        // Determine which name to search for
        let lookup_name = if cursor_word != element_name {
            &cursor_word
        } else {
            &element_name
        };

        // Look up the symbol to get its qualified name
        let symbol = self
            .workspace
            .symbol_table()
            .lookup_qualified(lookup_name)
            .or_else(|| self.workspace.symbol_table().lookup(lookup_name))
            .or_else(|| {
                self.workspace
                    .symbol_table()
                    .all_symbols()
                    .into_iter()
                    .find(|(_key, sym)| sym.name() == lookup_name)
                    .map(|(_, sym)| sym)
            })?;

        let _qualified_name = symbol.qualified_name();
        let simple_name = symbol.name();

        // Search all files for references
        let mut locations = Vec::new();

        for file_path in self.workspace.files().keys() {
            let file_uri = Url::from_file_path(file_path).ok()?;
            let file_text = self.document_texts.get(file_path)?;

            // Use text-based search to find all occurrences of the symbol name
            // This is more accurate than AST spans which cover entire elements
            find_text_references(file_text, simple_name, &file_uri, &mut locations);
        }

        // Filter out declaration if requested
        if !include_declaration
            && let (Some(def_file), Some(def_span)) = (symbol.source_file(), symbol.span())
            && let Ok(def_uri) = Url::from_file_path(def_file)
        {
            locations.retain(|loc| {
                // Keep locations that are NOT within the definition span
                !(loc.uri == def_uri
                    && loc.range.start.line >= def_span.start.line as u32
                    && loc.range.end.line <= def_span.end.line as u32
                    && (loc.range.start.line > def_span.start.line as u32
                        || loc.range.start.character >= def_span.start.column as u32)
                    && (loc.range.end.line < def_span.end.line as u32
                        || loc.range.end.character <= def_span.end.column as u32))
            });
        }

        // Deduplicate locations (same file + same position)
        locations.sort_by(|a, b| {
            a.uri
                .as_str()
                .cmp(b.uri.as_str())
                .then(a.range.start.line.cmp(&b.range.start.line))
                .then(a.range.start.character.cmp(&b.range.start.character))
        });
        locations.dedup_by(|a, b| {
            a.uri == b.uri
                && a.range.start.line == b.range.start.line
                && a.range.start.character == b.range.start.character
        });

        Some(locations)
    }

    /// Find the symbol name and range at the given position by querying the AST
    fn find_symbol_at_position(
        &self,
        path: &PathBuf,
        position: Position,
    ) -> Option<(String, Range)> {
        use syster::core::Position as CorePosition;

        // Get the SysML file from workspace
        let workspace_file = self.workspace.files().get(path)?;
        let file = workspace_file.content();

        // Convert LSP position to our 0-indexed position
        let core_pos = CorePosition::new(position.line as usize, position.character as usize);

        // Search elements for one containing this position
        for element in &file.elements {
            if let Some((name, span)) = find_element_at_position(element, core_pos) {
                return Some((name, span_to_lsp_range(&span)));
            }
        }

        None
    }
}

/// Extract the word at the cursor position from the document text
fn extract_word_at_cursor(text: &str, position: Position) -> Option<String> {
    let lines: Vec<&str> = text.lines().collect();
    let line = lines.get(position.line as usize)?;
    let chars: Vec<char> = line.chars().collect();

    let pos = position.character as usize;
    if pos >= chars.len() {
        return None;
    }

    // Find word boundaries - identifiers are alphanumeric + underscore
    let is_word_char = |c: char| c.is_alphanumeric() || c == '_';

    // Find start of word
    let mut start = pos;
    while start > 0 && is_word_char(chars[start - 1]) {
        start -= 1;
    }

    // Find end of word
    let mut end = pos;
    while end < chars.len() && is_word_char(chars[end]) {
        end += 1;
    }

    if start == end {
        return None;
    }

    Some(chars[start..end].iter().collect())
}

/// Find text-based references to a symbol
fn find_text_references(
    text: &str,
    symbol_name: &str,
    file_uri: &Url,
    locations: &mut Vec<Location>,
) {
    // Search for the symbol name in the text
    for (line_num, line) in text.lines().enumerate() {
        let mut col = 0;
        while let Some(pos) = line[col..].find(symbol_name) {
            let actual_col = col + pos;

            // Check if this is a complete word (not part of another identifier)
            let is_word_char = |c: char| c.is_alphanumeric() || c == '_';

            let before_ok =
                actual_col == 0 || !line.chars().nth(actual_col - 1).is_some_and(is_word_char);

            let after_pos = actual_col + symbol_name.len();
            let after_ok =
                after_pos >= line.len() || !line.chars().nth(after_pos).is_some_and(is_word_char);

            if before_ok && after_ok {
                locations.push(Location {
                    uri: file_uri.clone(),
                    range: Range {
                        start: Position {
                            line: line_num as u32,
                            character: actual_col as u32,
                        },
                        end: Position {
                            line: line_num as u32,
                            character: (actual_col + symbol_name.len()) as u32,
                        },
                    },
                });
            }

            col = actual_col + 1;
        }
    }
}

/// Find an element at the given position in the AST
fn find_element_at_position(
    element: &syster::language::sysml::syntax::Element,
    position: syster::core::Position,
) -> Option<(String, syster::core::Span)> {
    use syster::language::sysml::syntax::Element;

    match element {
        Element::Package(pkg) => {
            // First, check nested elements (most specific match)
            for child in &pkg.elements {
                if let Some(result) = find_element_at_position(child, position) {
                    return Some(result);
                }
            }
            // If no nested element matched, check if position is in package itself
            if let (Some(name), Some(span)) = (&pkg.name, pkg.span)
                && span.contains(position)
            {
                return Some((name.clone(), span));
            }
        }
        Element::Definition(def) => {
            if let (Some(name), Some(span)) = (&def.name, def.span)
                && span.contains(position)
            {
                return Some((name.clone(), span));
            }
        }
        Element::Usage(usage) => {
            if let (Some(name), Some(span)) = (&usage.name, usage.span)
                && span.contains(position)
            {
                return Some((name.clone(), span));
            }
        }
        Element::Alias(alias) => {
            if let (Some(name), Some(span)) = (&alias.name, alias.span)
                && span.contains(position)
            {
                return Some((name.clone(), span));
            }
        }
        _ => {}
    }

    None
}

/// Convert our Span to LSP Range
fn span_to_lsp_range(span: &syster::core::Span) -> Range {
    Range {
        start: Position {
            line: span.start.line as u32,
            character: span.start.column as u32,
        },
        end: Position {
            line: span.end.line as u32,
            character: span.end.column as u32,
        },
    }
}

/// Format rich hover information with relationships and documentation
fn format_rich_hover(symbol: &Symbol, workspace: &syster::semantic::Workspace) -> String {
    let mut result = String::new();

    // Main declaration
    result.push_str("```sysml\n");
    result.push_str(&format_symbol_declaration(symbol));
    result.push_str("\n```\n");

    // Qualified name
    result.push_str(&format!(
        "\n**Qualified Name:** `{}`\n",
        symbol.qualified_name()
    ));

    // Source file
    if let Some(file) = symbol.source_file() {
        result.push_str(&format!("\n**Defined in:** `{}`\n", file));
    }

    // Relationships (using relationship graph)
    if let Some(relationships) = get_symbol_relationships(symbol, workspace)
        && !relationships.is_empty()
    {
        result.push_str("\n**Relationships:**\n");
        for rel in relationships {
            result.push_str(&format!("- {}\n", rel));
        }
    }

    result
}

/// Format the symbol declaration
fn format_symbol_declaration(symbol: &Symbol) -> String {
    match symbol {
        Symbol::Alias { name, target, .. } => format!("alias {} for {}", name, target),
        Symbol::Package { name, .. } => format!("package {}", name),
        Symbol::Classifier {
            name,
            kind,
            is_abstract,
            ..
        } => {
            let prefix = if *is_abstract { "abstract " } else { "" };
            format!("{}{} {}", prefix, kind, name)
        }
        Symbol::Definition { name, kind, .. } => format!("{} def {}", kind, name),
        Symbol::Usage { name, kind, .. } => format!("{} {}", kind, name),
        Symbol::Feature {
            name, feature_type, ..
        } => {
            let type_str = feature_type
                .as_ref()
                .map(|t| format!(": {}", t))
                .unwrap_or_default();
            format!("feature {}{}", name, type_str)
        }
    }
}

/// Get relationships for a symbol from the workspace
fn get_symbol_relationships(
    symbol: &Symbol,
    workspace: &syster::semantic::Workspace,
) -> Option<Vec<String>> {
    let mut relationships = Vec::new();
    let qname = symbol.qualified_name();
    let graph = workspace.relationship_graph();

    // Specializations
    if let Some(bases) = graph.get_one_to_many("specialization", qname) {
        for base in bases {
            relationships.push(format!("Specializes `{}`", base));
        }
    }

    // Redefinitions
    if let Some(redefs) = graph.get_one_to_many("redefinition", qname) {
        for redef in redefs {
            relationships.push(format!("Redefines `{}`", redef));
        }
    }

    // Subsettings
    if let Some(subsets) = graph.get_one_to_many("subsetting", qname) {
        for subset in subsets {
            relationships.push(format!("Subsets `{}`", subset));
        }
    }

    // Typing (for usages)
    if let Some(typ) = graph.get_one_to_one("typing", qname) {
        relationships.push(format!("Typed by `{}`", typ));
    }

    if relationships.is_empty() {
        None
    } else {
        Some(relationships)
    }
}

#[cfg(test)]
mod tests;
