use super::LspServer;
use super::helpers::span_to_lsp_range;
use async_lsp::lsp_types::{Location, OneOf, Position, Range, SymbolKind, Url, WorkspaceSymbol};
use syster::semantic::symbol_table::Symbol;

impl LspServer {
    /// Get workspace-wide symbols filtered by the user's query.
    pub fn get_workspace_symbols(&mut self, query: &str) -> Vec<WorkspaceSymbol> {
        if self.ensure_workspace_loaded().is_err() {
            return Vec::new();
        }

        let query = query.trim();
        let query_lower = query.to_lowercase();

        let mut results = Vec::new();
        for symbol in self.workspace.symbol_table().iter_symbols() {
            if matches!(symbol, Symbol::Import { .. }) {
                continue;
            }

            if !query_lower.is_empty() && !symbol_matches_query(symbol, &query_lower) {
                continue;
            }

            let Some(source_file) = symbol.source_file() else {
                continue;
            };
            let Ok(uri) = Url::from_file_path(source_file) else {
                continue;
            };

            let range = symbol
                .span()
                .map(|span| span_to_lsp_range(&span))
                .unwrap_or_else(default_lsp_range);

            results.push(WorkspaceSymbol {
                name: symbol.name().to_string(),
                kind: symbol_kind(symbol),
                tags: None,
                location: OneOf::Left(Location { uri, range }),
                container_name: container_name(symbol.qualified_name()),
                data: None,
            });
        }

        results.sort_by(|a, b| a.name.cmp(&b.name));
        results
    }
}

fn symbol_matches_query(symbol: &Symbol, query_lower: &str) -> bool {
    let name = symbol.name().to_lowercase();
    if name.contains(query_lower) {
        return true;
    }

    let qualified = symbol.qualified_name().to_lowercase();
    qualified.contains(query_lower)
}

fn container_name(qualified_name: &str) -> Option<String> {
    let (container, _leaf) = qualified_name.rsplit_once("::")?;
    Some(container.to_string())
}

fn symbol_kind(symbol: &Symbol) -> SymbolKind {
    match symbol {
        Symbol::Package { .. } => SymbolKind::NAMESPACE,
        Symbol::Classifier { .. } | Symbol::Definition { .. } => SymbolKind::CLASS,
        Symbol::Feature { .. } | Symbol::Usage { .. } => SymbolKind::PROPERTY,
        Symbol::Alias { .. } => SymbolKind::VARIABLE,
        Symbol::Import { .. } => SymbolKind::NAMESPACE,
    }
}

fn default_lsp_range() -> Range {
    let position = Position {
        line: 0,
        character: 0,
    };
    Range {
        start: position,
        end: position,
    }
}
