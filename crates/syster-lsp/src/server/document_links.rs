use super::LspServer;
use super::helpers::{span_to_lsp_range, uri_to_path};
use async_lsp::lsp_types::{DocumentLink, Url};
use syster::core::Span;
use syster::core::constants::{REL_SPECIALIZATION, REL_SUBSETTING, REL_TYPING};
use tracing::{debug, info};

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
                    tooltip: Some(format!("Go to {}", import_path)),
                    data: None,
                });
            }
        }

        // Add links for type references (specializations, typing, subsetting)
        self.add_type_reference_links(&file_path_str, &mut links);

        links
    }

    /// Add document links for type references in relationships
    fn add_type_reference_links(&self, file_path: &str, links: &mut Vec<DocumentLink>) {
        let graph = self.workspace.relationship_graph();
        let symbol_table = self.workspace.symbol_table();

        debug!("add_type_reference_links for file: {}", file_path);
        debug!(
            "  Total symbols in table: {}",
            symbol_table.all_symbols().len()
        );

        // Track how many links we add (for logging)
        let initial_link_count = links.len();

        // Get all symbols defined in this file
        let mut matched_count = 0;
        for (_, symbol) in symbol_table.all_symbols() {
            let Some(source_file) = symbol.source_file() else {
                continue;
            };

            // Normalize paths for comparison
            let source_path = std::path::Path::new(source_file);
            let request_path = std::path::Path::new(file_path);

            if source_path != request_path {
                continue;
            }

            matched_count += 1;

            let qname = symbol.qualified_name();
            let scope_id = symbol.scope_id();

            // Check specialization relationships (e.g., Car :> Vehicle)
            if let Some(targets) = graph.get_one_to_many_with_spans(REL_SPECIALIZATION, qname) {
                debug!(
                    "  Found {} specialization targets for {}",
                    targets.len(),
                    qname
                );
                for (target, span) in targets {
                    self.try_add_type_link(target, span, scope_id, links);
                }
            }

            // Check typing relationships (e.g., myPart : PartDef)
            if let Some((target, span)) = graph.get_one_to_one_with_span(REL_TYPING, qname) {
                self.try_add_type_link(target, span, scope_id, links);
            }

            // Check subsetting relationships (e.g., wheels subsets components)
            if let Some(targets) = graph.get_one_to_many_with_spans(REL_SUBSETTING, qname) {
                for (target, span) in targets {
                    self.try_add_type_link(target, span, scope_id, links);
                }
            }
        }

        let added_count = links.len() - initial_link_count;
        info!(
            "Document links for {}: matched {} symbols, added {} type reference links",
            file_path, matched_count, added_count
        );
    }

    /// Try to add a document link for a type reference
    ///
    /// If the span and target URI can be resolved, adds a DocumentLink to the list.
    fn try_add_type_link(
        &self,
        target: &str,
        span: Option<&Span>,
        scope_id: usize,
        links: &mut Vec<DocumentLink>,
    ) {
        if let Some(span) = span
            && let Some(target_uri) = self.resolve_symbol_to_uri_in_scope(target, scope_id)
        {
            debug!("    Adding link to {} at {:?}", target, span);
            links.push(DocumentLink {
                range: span_to_lsp_range(span),
                target: Some(target_uri),
                tooltip: Some(format!("Go to {}", target)),
                data: None,
            });
        }
    }

    /// Resolve a symbol name to a file URI using scope-aware resolution
    ///
    /// This resolves names in the context of a specific scope's imports,
    /// handling both qualified names (e.g., "Base::Vehicle") and
    /// simple names (e.g., "Vehicle") imported via `import Base::*`.
    fn resolve_symbol_to_uri_in_scope(&self, name: &str, scope_id: usize) -> Option<Url> {
        let symbol_table = self.workspace.symbol_table();

        // Use scope-aware resolution which tries:
        // 1. Qualified lookup (for fully qualified names)
        // 2. Direct lookup in scope hierarchy
        // 3. Lookup via imports from the given scope
        let symbol = symbol_table.resolve_in_scope(name, scope_id)?;
        let source_file = symbol.source_file()?;
        Url::from_file_path(source_file).ok()
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

        // Look up the symbol in the symbol table
        let symbol = self.workspace.symbol_table().resolve(&symbol_name)?;

        // Get the source file for this symbol
        let source_file = symbol.source_file()?;

        // Convert to URI
        Url::from_file_path(source_file).ok()
    }
}
