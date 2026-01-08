use super::LspServer;
use super::helpers::{span_to_lsp_range, uri_to_path};
use async_lsp::lsp_types::{DocumentLink, Url};

impl LspServer {
    /// Get document links for imports and qualified references in the document
    ///
    /// Returns a list of clickable links that navigate to:
    /// 1. Import statements - links to the definition of the imported symbol
    /// 2. Type references - links to specialized types, typed definitions, etc.
    pub fn get_document_links(&self, uri: &Url) -> Vec<DocumentLink> {
        let mut links = Vec::new();

        let path = match uri_to_path(uri) {
            Some(p) => p,
            None => return links,
        };

        let file_path_str = path.to_string_lossy().to_string();

        // Get all imports from this file using the symbol table
        let imports = self
            .workspace
            .symbol_table()
            .get_file_imports(&file_path_str);

        // Create document links for each import
        for (import_path, span) in imports {
            if let Some(target_uri) = self.resolve_import_to_uri(&import_path) {
                links.push(DocumentLink {
                    range: span_to_lsp_range(&span),
                    target: Some(target_uri),
                    tooltip: Some(format!("Go to {import_path}")),
                    data: None,
                });
            }
        }

        links
    }

    /// Resolve an import path to a file URI
    ///
    /// Given an import path like "Base::DataValue", this resolves to the file
    /// that contains the DataValue definition in the Base package.
    fn resolve_import_to_uri(&self, import_path: &str) -> Option<Url> {
        // Parse the import path to get the base package/namespace
        let parts: Vec<&str> = import_path.split("::").collect();
        if parts.is_empty() {
            return None;
        }

        // For wildcard imports (Base::* or Base::**), use the base package
        let symbol_name = if parts.last() == Some(&"*") || parts.last() == Some(&"**") {
            // Join all parts except the wildcard
            parts[..parts.len() - 1].join("::")
        } else {
            // Use the full path for specific imports
            import_path.to_string()
        };

        if symbol_name.is_empty() {
            return None;
        }

        // Look up the symbol using resolver
        let resolver = self.resolver();
        let symbol = resolver.resolve(&symbol_name)?;

        // Get the source file for this symbol
        let source_file = symbol.source_file()?;

        // Convert to URI
        Url::from_file_path(source_file).ok()
    }
}
