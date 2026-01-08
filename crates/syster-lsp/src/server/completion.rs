use crate::server::core::LspServer;
use async_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionResponse, Documentation, Position,
};

impl LspServer {
    /// Get completion items at a position
    pub fn get_completions(
        &self,
        path: &std::path::Path,
        position: Position,
    ) -> CompletionResponse {
        let mut items = Vec::new();

        // Get document text to determine context
        let text = self.document_texts.get(path).map(|s| s.as_str());

        // Determine completion context
        let context = if let Some(text) = text {
            Self::determine_completion_context(text, position)
        } else {
            CompletionContext::General
        };

        match context {
            CompletionContext::AfterColon => {
                // After ":", suggest types from symbol table
                self.add_type_completions(&mut items);
            }
            CompletionContext::AfterRelationshipKeyword => {
                // After "specializes", "subsets", etc., suggest types
                self.add_type_completions(&mut items);
            }
            CompletionContext::AfterDef => {
                // After "part def", "action def", etc., don't suggest anything
                // User is typing the name
                return CompletionResponse::Array(items);
            }
            CompletionContext::General => {
                // General context: suggest keywords and symbols
                self.add_keyword_completions(&mut items, path);
                self.add_symbol_completions(&mut items);
            }
        }

        CompletionResponse::Array(items)
    }

    /// Determine completion context based on text before cursor
    fn determine_completion_context(text: &str, position: Position) -> CompletionContext {
        let lines: Vec<&str> = text.lines().collect();
        if (position.line as usize) >= lines.len() {
            return CompletionContext::General;
        }

        let line = lines[position.line as usize];
        let text_before_cursor = &line[..position.character.min(line.len() as u32) as usize];
        let text_before_cursor = text_before_cursor.trim_end();

        // Check for typing relationship (after ":")
        if text_before_cursor.ends_with(':') && !text_before_cursor.ends_with("::") {
            return CompletionContext::AfterColon;
        }

        // Check for relationship keywords
        for keyword in &["specializes", "subsets", "redefines", "references"] {
            if text_before_cursor.ends_with(keyword) {
                return CompletionContext::AfterRelationshipKeyword;
            }
        }

        // Check if we're after "def" keyword
        if text_before_cursor.ends_with("def") {
            return CompletionContext::AfterDef;
        }

        CompletionContext::General
    }

    /// Add keyword completions
    fn add_keyword_completions(&self, items: &mut Vec<CompletionItem>, path: &std::path::Path) {
        let keywords = syster::keywords::get_keywords_for_file(path);

        for keyword in keywords {
            items.push(CompletionItem {
                label: keyword.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some(format!("Keyword: {keyword}")),
                ..Default::default()
            });
        }

        // Add relationship operators (same for both KerML and SysML)
        for op in syster::keywords::RELATIONSHIP_OPERATORS {
            items.push(CompletionItem {
                label: op.to_string(),
                kind: Some(CompletionItemKind::OPERATOR),
                detail: Some("Relationship operator".to_string()),
                ..Default::default()
            });
        }
    }

    /// Add type completions from symbol table
    fn add_type_completions(&self, items: &mut Vec<CompletionItem>) {
        let symbol_table = self.workspace.symbol_table();

        for symbol in symbol_table.iter_symbols() {
            // Only suggest classifiers/definitions, not usages
            if matches!(
                symbol,
                syster::semantic::symbol_table::Symbol::Classifier { .. }
                    | syster::semantic::symbol_table::Symbol::Definition { .. }
            ) {
                let qualified_name = symbol.qualified_name();
                items.push(CompletionItem {
                    label: symbol.name().to_string(),
                    kind: Some(CompletionItemKind::CLASS),
                    detail: Some(qualified_name.to_string()),
                    documentation: Some(Documentation::String(format!("Type: {qualified_name}"))),
                    ..Default::default()
                });
            }
        }
    }

    /// Add all symbols for general completion
    fn add_symbol_completions(&self, items: &mut Vec<CompletionItem>) {
        let symbol_table = self.workspace.symbol_table();

        for symbol in symbol_table.iter_symbols() {
            let (kind, icon) = match symbol {
                syster::semantic::symbol_table::Symbol::Package { .. } => {
                    (CompletionItemKind::MODULE, "📦")
                }
                syster::semantic::symbol_table::Symbol::Classifier { .. }
                | syster::semantic::symbol_table::Symbol::Definition { .. } => {
                    (CompletionItemKind::CLASS, "🔷")
                }
                syster::semantic::symbol_table::Symbol::Feature { .. } => {
                    (CompletionItemKind::PROPERTY, "🔸")
                }
                syster::semantic::symbol_table::Symbol::Usage { .. } => {
                    (CompletionItemKind::VARIABLE, "📍")
                }
                syster::semantic::symbol_table::Symbol::Alias { .. } => {
                    (CompletionItemKind::REFERENCE, "🔗")
                }
                syster::semantic::symbol_table::Symbol::Import { .. } => continue, // Skip imports in completions
            };

            let qualified_name = symbol.qualified_name();
            items.push(CompletionItem {
                label: symbol.name().to_string(),
                kind: Some(kind),
                detail: Some(format!("{icon} {qualified_name}")),
                ..Default::default()
            });
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompletionContext {
    /// After ":" for typing relationship
    AfterColon,
    /// After "specializes", "subsets", etc.
    AfterRelationshipKeyword,
    /// After "def" keyword (user is typing definition name)
    AfterDef,
    /// General context
    General,
}
