use async_lsp::lsp_types::{DocumentLink, Url};
use std::collections::HashMap;
use syster::core::constants::{REL_SPECIALIZATION, REL_SUBSETTING, REL_TYPING};
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

        // Check specialization relationships (e.g., Car :> Vehicle)
        // This is one-to-many, so a symbol can specialize multiple types
        if let Some(targets) = graph.get_one_to_many_with_locations(REL_SPECIALIZATION, qname) {
            for (target, ref_location) in targets {
                let Some(ref_loc) = ref_location else {
                    continue;
                };
                add_link_if_resolvable(
                    target,
                    &ref_loc.span,
                    scope_id,
                    symbol_table,
                    &mut url_cache,
                    &mut links,
                );
            }
        }

        // Check typing relationships (e.g., myPart : PartDef)
        // This is one-to-one
        if let Some((target, ref_location)) = graph.get_one_to_one_with_location(REL_TYPING, qname)
            && let Some(ref_loc) = ref_location
        {
            add_link_if_resolvable(
                target,
                &ref_loc.span,
                scope_id,
                symbol_table,
                &mut url_cache,
                &mut links,
            );
        }

        // Check subsetting relationships (e.g., wheels subsets components)
        // This is one-to-many
        if let Some(targets) = graph.get_one_to_many_with_locations(REL_SUBSETTING, qname) {
            for (target, ref_location) in targets {
                let Some(ref_loc) = ref_location else {
                    continue;
                };
                add_link_if_resolvable(
                    target,
                    &ref_loc.span,
                    scope_id,
                    symbol_table,
                    &mut url_cache,
                    &mut links,
                );
            }
        }
    }

    links
}

/// Helper to add a document link if the target can be resolved
fn add_link_if_resolvable<'a>(
    target: &str,
    span: &syster::core::Span,
    scope_id: usize,
    symbol_table: &'a syster::semantic::SymbolTable,
    url_cache: &mut HashMap<&'a str, Url>,
    links: &mut Vec<DocumentLink>,
) {
    let Some(target_symbol) = symbol_table.resolve_in_scope(target, scope_id) else {
        return;
    };
    let Some(source_file) = target_symbol.source_file() else {
        return;
    };

    // Use cached URL or create and cache new one
    let target_uri = match url_cache.get(source_file) {
        Some(url) => url.clone(),
        None => {
            let Ok(url) = Url::from_file_path(source_file) else {
                return;
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
