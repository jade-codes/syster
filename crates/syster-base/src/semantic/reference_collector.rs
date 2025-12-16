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

use crate::core::Span;
use crate::language::sysml::populator::{
    REL_REDEFINITION, REL_REFERENCE_SUBSETTING, REL_SPECIALIZATION, REL_SUBSETTING, REL_TYPING,
};
use crate::semantic::graph::RelationshipGraph;
use crate::semantic::symbol_table::SymbolTable;
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
        let references_to_add = self.collect_references();

        for (target_name, spans) in references_to_add {
            if let Some(symbol) = self.symbol_table.lookup_mut(&target_name) {
                for span in spans {
                    symbol.add_reference(span);
                }
            }
        }
    }

    /// Collect references by examining relationship graphs
    fn collect_references(&self) -> HashMap<String, Vec<Span>> {
        let mut references: HashMap<String, Vec<Span>> = HashMap::new();

        for (_key, symbol) in self.symbol_table.all_symbols() {
            let Some(span) = symbol.span() else {
                continue;
            };

            // Collect all relationship targets for this symbol
            let targets = self.get_all_targets(symbol.qualified_name());

            for target in targets {
                references.entry(target).or_default().push(span.clone());
            }
        }

        references
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
