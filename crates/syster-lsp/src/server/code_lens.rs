use super::LspServer;
use super::helpers::{collect_reference_locations, span_to_lsp_range, uri_to_path};
use async_lsp::lsp_types::{CodeLens, Command, Position, Url};
use syster::semantic::symbol_table::Symbol;

impl LspServer {
    /// Get code lenses for a document
    ///
    /// Shows inline commands above definitions:
    /// - "N references" - clickable to show all references
    /// - "N implementations" - for abstract definitions (future)
    pub fn get_code_lenses(&self, uri: &Url) -> Vec<CodeLens> {
        let Some(path) = uri_to_path(uri) else {
            return Vec::new();
        };

        let mut lenses = Vec::new();

        // Collect all symbols from this file
        for (_, symbol) in self.workspace.symbol_table().all_symbols() {
            // Only include symbols defined in this file
            let Some(symbol_path) = symbol.source_file() else {
                continue;
            };
            if symbol_path != path.to_str().unwrap_or("") {
                continue;
            }

            // Only show code lens for top-level definitions (not features/usages nested deeply)
            if !self.should_show_code_lens(symbol) {
                continue;
            }

            if let Some(span) = symbol.span() {
                let range = span_to_lsp_range(&span);

                // Get reference count
                let qualified_name = symbol.qualified_name();
                let reference_count = self.count_references(qualified_name);

                // Only show code lens if there are references
                if reference_count > 0 {
                    // Serialize command arguments (these are basic LSP types and should not fail)
                    let Ok(uri_value) = serde_json::to_value(uri) else {
                        continue;
                    };
                    let Ok(position_value) = serde_json::to_value(Position {
                        line: range.start.line,
                        character: range.start.character,
                    }) else {
                        continue;
                    };
                    let Ok(locations_value) = serde_json::to_value(collect_reference_locations(
                        &self.workspace,
                        qualified_name,
                    )) else {
                        continue;
                    };

                    let lens = CodeLens {
                        range,
                        command: Some(Command {
                            title: format!(
                                "{} reference{}",
                                reference_count,
                                if reference_count == 1 { "" } else { "s" }
                            ),
                            command: "editor.action.showReferences".to_string(),
                            arguments: Some(vec![uri_value, position_value, locations_value]),
                        }),
                        data: None,
                    };
                    lenses.push(lens);
                }
            }
        }

        lenses
    }

    /// Determine if a symbol should show a code lens
    ///
    /// We show code lens for:
    /// - Packages
    /// - Classifiers
    /// - Definitions (part def, port def, etc.)
    ///
    /// We don't show for:
    /// - Features (nested properties)
    /// - Usages (instances)
    /// - Aliases
    fn should_show_code_lens(&self, symbol: &Symbol) -> bool {
        match symbol {
            Symbol::Package { .. } => true,
            Symbol::Classifier { .. } => true,
            Symbol::Definition { .. } => true,
            Symbol::Feature { .. } => false,
            Symbol::Usage { .. } => false,
            Symbol::Alias { .. } => false,
        }
    }

    /// Count the number of references to a symbol
    fn count_references(&self, qualified_name: &str) -> usize {
        let locations = collect_reference_locations(&self.workspace, qualified_name);
        locations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_lens_basic() {
        let mut server = LspServer::new();
        let uri = Url::parse("file:///test.sysml").unwrap();
        let text = r#"
part def Vehicle;
part car : Vehicle;
        "#;

        server.open_document(&uri, text).unwrap();

        let lenses = server.get_code_lenses(&uri);

        // Should have one code lens for Vehicle showing 1 reference
        assert_eq!(lenses.len(), 1);
        assert!(lenses[0].command.is_some());
        let cmd = lenses[0].command.as_ref().unwrap();
        assert_eq!(cmd.title, "1 reference");
        assert_eq!(cmd.command, "editor.action.showReferences");
    }

    #[test]
    fn test_code_lens_multiple_references() {
        let mut server = LspServer::new();
        let uri = Url::parse("file:///test.sysml").unwrap();
        let text = r#"
part def Vehicle;
part car : Vehicle;
part truck : Vehicle;
part bike : Vehicle;
        "#;

        server.open_document(&uri, text).unwrap();

        let lenses = server.get_code_lenses(&uri);

        // Should have one code lens for Vehicle showing 3 references
        assert_eq!(lenses.len(), 1);
        let cmd = lenses[0].command.as_ref().unwrap();
        assert_eq!(cmd.title, "3 references");
    }

    #[test]
    fn test_code_lens_no_references() {
        let mut server = LspServer::new();
        let uri = Url::parse("file:///test.sysml").unwrap();
        let text = r#"
part def Vehicle;
part def Bike;
        "#;

        server.open_document(&uri, text).unwrap();

        let lenses = server.get_code_lenses(&uri);

        // Should have no code lenses since there are no references
        assert_eq!(lenses.len(), 0);
    }

    #[test]
    fn test_code_lens_classifier() {
        let mut server = LspServer::new();
        let uri = Url::parse("file:///test.kerml").unwrap();
        let text = r#"
classifier Vehicle;
classifier Car specializes Vehicle;
        "#;

        server.open_document(&uri, text).unwrap();

        let lenses = server.get_code_lenses(&uri);

        // Should have one code lens for Vehicle showing the specialization reference
        assert_eq!(lenses.len(), 1);
        let cmd = lenses[0].command.as_ref().unwrap();
        assert_eq!(cmd.title, "1 reference");
    }

    #[test]
    fn test_code_lens_excludes_features() {
        let mut server = LspServer::new();
        let uri = Url::parse("file:///test.sysml").unwrap();
        let text = r#"
part def Vehicle {
    attribute speed : Real;
}
        "#;

        server.open_document(&uri, text).unwrap();

        let lenses = server.get_code_lenses(&uri);

        // Should have no code lenses (Vehicle has no references, and features are excluded)
        assert_eq!(lenses.len(), 0);
    }

    #[test]
    fn test_code_lens_invalid_uri() {
        let server = LspServer::new();
        let uri = Url::parse("http://example.com/not-a-file").unwrap();

        let lenses = server.get_code_lenses(&uri);

        // Should return empty vec for invalid file URI
        assert_eq!(lenses.len(), 0);
    }
}
