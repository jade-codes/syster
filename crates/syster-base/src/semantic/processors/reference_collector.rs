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
                let fallback_file = symbol.source_file()?.to_string();
                let fallback_span = symbol.span();

                // Get all relationship targets with their spans
                let mut refs = Vec::new();

                // Typing relationship - use the span of the type reference
                if let Some((target, loc)) = self
                    .relationship_graph
                    .get_one_to_one_with_location(REL_TYPING, &qname)
                {
                    // Use location from graph if available, otherwise fall back
                    let (file, span) = match loc {
                        Some(l) => (l.file.to_string(), l.span),
                        None => (fallback_file.clone(), fallback_span?),
                    };
                    refs.push((target.to_string(), SymbolReference { file, span }));
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
                        .get_one_to_many_with_locations(rel_type, &qname)
                    {
                        for (target, loc) in targets {
                            // Use location from graph if available, otherwise fall back
                            let (file, span) = match loc {
                                Some(l) => (l.file.to_string(), l.span),
                                None => (fallback_file.clone(), fallback_span?),
                            };
                            refs.push((target.to_string(), SymbolReference { file, span }));
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
            // Use find_by_qualified_name for data lookup during population
            if let Some(symbol) = self.symbol_table.find_by_qualified_name(&target_name) {
                let qualified_name = symbol.qualified_name().to_string();
                self.symbol_table
                    .add_references_to_symbol(&qualified_name, refs);
            }
        }
    }

    /// Collect references from import statements and populate the reverse index
    fn collect_import_references(
        &mut self,
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
                let target_qname = self
                    .symbol_table
                    .find_by_qualified_name(&import.path)
                    .map(|s| s.qualified_name().to_string());

                if let Some(target_qname) = target_qname {
                    // Create a reference for the import statement itself
                    if let (Some(span), Some(file)) = (import.span, import.file.clone()) {
                        // Add to the references HashMap for legacy symbol references
                        let reference = SymbolReference {
                            file: file.clone(),
                            span,
                        };
                        references_by_target
                            .entry(target_qname.clone())
                            .or_default()
                            .push(reference);

                        // Also populate the reverse index for O(1) lookup
                        self.symbol_table
                            .add_import_reference(target_qname, file, span);
                    }
                }
            }
        }
    }
}
