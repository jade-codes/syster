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

use crate::language::sysml::populator::{
    REL_REDEFINITION, REL_REFERENCE_SUBSETTING, REL_SPECIALIZATION, REL_SUBSETTING, REL_TYPING,
};
use crate::semantic::graph::RelationshipGraph;
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
        // Build name resolution map (key -> name) once
        let symbol_names: HashMap<String, String> = self
            .symbol_table
            .all_symbols()
            .into_iter()
            .map(|(k, sym)| (k.clone(), sym.name().to_string()))
            .collect();

        // Collect all references grouped by target
        let references_by_target: HashMap<String, Vec<SymbolReference>> = self
            .symbol_table
            .all_symbols()
            .into_iter()
            .filter_map(|(_, symbol)| {
                let span = symbol.span()?;
                let file = symbol.source_file()?;
                let reference = SymbolReference {
                    file: file.to_string(),
                    span,
                };
                Some((symbol.qualified_name().to_string(), reference))
            })
            .flat_map(|(qname, reference)| {
                // Get all relationship targets for this symbol
                self.get_all_targets(&qname)
                    .into_iter()
                    .map(move |target| (target, reference.clone()))
            })
            .fold(HashMap::new(), |mut acc, (target, reference)| {
                acc.entry(target).or_default().push(reference);
                acc
            });

        // Apply references to symbols
        for (target_name, refs) in references_by_target {
            // Resolve target name (handle both qualified and simple names)
            let key = symbol_names
                .iter()
                .find(|(k, name)| *k == &target_name || *name == &target_name)
                .map(|(k, _)| k.clone());

            if let Some(key) = key
                && let Some(symbol) = self.symbol_table.lookup_global_mut(&key)
            {
                for reference in refs {
                    symbol.add_reference(reference);
                }
            }
        }
    }

    /// Get all relationship targets for a symbol
    fn get_all_targets(&self, qualified_name: &str) -> Vec<String> {
        let mut targets = Vec::new();

        // Typing relationship (: or "typed by")
        if let Some(target) = self
            .relationship_graph
            .get_one_to_one(REL_TYPING, qualified_name)
        {
            targets.push(target.clone());
        }

        // One-to-many relationships
        for rel_type in [
            REL_SPECIALIZATION,
            REL_REDEFINITION,
            REL_SUBSETTING,
            REL_REFERENCE_SUBSETTING,
        ] {
            if let Some(rel_targets) = self
                .relationship_graph
                .get_one_to_many(rel_type, qualified_name)
            {
                targets.extend(rel_targets.iter().cloned());
            }
        }

        targets
    }
}

#[cfg(test)]
mod tests;
