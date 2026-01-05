use async_lsp::lsp_types::{DocumentLink, Url};
use std::collections::HashMap;
use syster::semantic::Workspace;
use syster::syntax::SyntaxFile;

use super::helpers::span_to_lsp_range;

/// Collect document links for all outgoing type references in a file
///
/// Returns DocumentLinks for specialization, typing, and subsetting relationships.
/// Each link navigates to the target type's definition.
pub fn collect_document_links(
    workspace: &Workspace<SyntaxFile>,
    file_path: &str,
) -> Vec<DocumentLink> {
    let mut links = Vec::new();
    let graph = workspace.relationship_graph();
    let symbol_table = workspace.symbol_table();

    // Cache file path → URL conversions (many symbols may reference same files)
    let mut url_cache: HashMap<&str, Url> = HashMap::new();

    for symbol in symbol_table.get_symbols_for_file(file_path) {
        let qname = symbol.qualified_name();
        let scope_id = symbol.scope_id();

        for (target, span) in graph.get_all_targets_with_spans(qname) {
            let Some(span) = span else { continue };

            let Some(target_symbol) = symbol_table.resolve_in_scope(target, scope_id) else {
                continue;
            };
            let Some(source_file) = target_symbol.source_file() else {
                continue;
            };

            // Use cached URL or create and cache new one
            let target_uri = match url_cache.get(source_file) {
                Some(url) => url.clone(),
                None => {
                    let Ok(url) = Url::from_file_path(source_file) else {
                        continue;
                    };
                    url_cache.insert(source_file, url.clone());
                    url
                }
            };

            links.push(DocumentLink {
                range: span_to_lsp_range(span),
                target: Some(target_uri),
                tooltip: Some(format!("Go to {}", target)),
                data: None,
            });
        }
    }

    links
}
