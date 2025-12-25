//! # Reference Collector
//!
//! Collects all references to symbols by analyzing relationship graphs.
//! Populates the `references` field in Symbol instances for LSP "Find References".
//!
//! ## How it works
//!
//! 1. Iterate through all symbols in the symbol table
//! 2. For each symbol with relationships (typed_by, specializes, etc.):
//!    - Get the target symbol name from the relationship graph
//!    - Add the symbol's span to the target's `references` list
//! 3. Result: Each symbol knows all locations where it's referenced

use crate::core::constants::{
    REL_REDEFINITION, REL_REFERENCE_SUBSETTING, REL_SPECIALIZATION, REL_SUBSETTING, REL_TYPING,
};
use crate::semantic::graphs::RelationshipGraph;
use crate::semantic::symbol_table::{SymbolReference, SymbolTable};
use std::collections::HashMap;

pub struct ReferenceCollector<'a> {
    symbol_table: &'a mut SymbolTable,
    relationship_graph: &'a RelationshipGraph,
}

impl<'a> ReferenceCollector<'a> {
    pub fn new(
        symbol_table: &'a mut SymbolTable,
        relationship_graph: &'a RelationshipGraph,
    ) -> Self {
        Self {
            symbol_table,
            relationship_graph,
        }
    }

    /// Collect all references and populate the references field in symbols
    pub fn collect(&mut self) {
        // Collect all references grouped by target
        let mut references_by_target: HashMap<String, Vec<SymbolReference>> = self
            .symbol_table
            .all_symbols()
            .into_iter()
            .flat_map(|(_, symbol)| {
                let qname = symbol.qualified_name().to_string();
                let file = symbol.source_file()?.to_string();

                // Get all relationship targets with their spans
                let mut refs = Vec::new();

                // Typing relationship - use the span of the type reference
                if let Some((target, span)) = self
                    .relationship_graph
                    .get_one_to_one_with_span(REL_TYPING, &qname)
                {
                    let reference_span = span.copied().or(symbol.span())?;
                    refs.push((
                        target.clone(),
                        SymbolReference {
                            file: file.clone(),
                            span: reference_span,
                        },
                    ));
                }

                // One-to-many relationships
                for rel_type in [
                    REL_SPECIALIZATION,
                    REL_REDEFINITION,
                    REL_SUBSETTING,
                    REL_REFERENCE_SUBSETTING,
                ] {
                    if let Some(targets) = self
                        .relationship_graph
                        .get_one_to_many_with_spans(rel_type, &qname)
                    {
                        for (target, span) in targets {
                            let reference_span = span.copied().or(symbol.span())?;
                            refs.push((
                                target.clone(),
                                SymbolReference {
                                    file: file.clone(),
                                    span: reference_span,
                                },
                            ));
                        }
                    }
                }

                Some(refs)
            })
            .flatten()
            .fold(HashMap::new(), |mut acc, (target, reference)| {
                acc.entry(target).or_default().push(reference);
                acc
            });

        // Collect import references
        self.collect_import_references(&mut references_by_target);

        // Apply references to symbols
        for (target_name, refs) in references_by_target {
            let resolved_symbol = self
                .symbol_table
                .lookup_qualified(&target_name)
                .or_else(|| self.symbol_table.lookup(&target_name));

            if let Some(symbol) = resolved_symbol {
                let qualified_name = symbol.qualified_name().to_string();
                self.symbol_table
                    .add_references_to_symbol(&qualified_name, refs);
            }
        }
    }

    /// Collect references from import statements
    fn collect_import_references(
        &self,
        references_by_target: &mut HashMap<String, Vec<SymbolReference>>,
    ) {
        // Iterate through all scopes and their imports
        for scope_id in 0..self.symbol_table.scope_count() {
            let imports = self.symbol_table.get_scope_imports(scope_id);

            for import in imports {
                // Parse the import path - skip wildcard imports
                if import.path.ends_with("::*") || import.path.ends_with("::**") {
                    continue;
                }

                // Try to resolve the imported path to a fully qualified name
                let resolved = self
                    .symbol_table
                    .lookup_qualified(&import.path)
                    .or_else(|| self.symbol_table.lookup(&import.path))
                    .map(|s| s.qualified_name().to_string());

                if let Some(target_qname) = resolved {
                    // Create a reference for the import statement itself
                    if let (Some(span), Some(file)) = (import.span, import.file) {
                        let reference = SymbolReference { file, span };
                        references_by_target
                            .entry(target_qname)
                            .or_default()
                            .push(reference);
                    }
                }
            }
        }
    }
}
