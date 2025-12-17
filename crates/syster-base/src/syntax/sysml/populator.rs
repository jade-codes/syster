mod helpers;
mod population;
mod visitors;

use crate::semantic::graphs::RelationshipGraph;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::types::SemanticError;

pub struct SymbolTablePopulator<'a> {
    symbol_table: &'a mut SymbolTable,
    relationship_graph: Option<&'a mut RelationshipGraph>,
    current_namespace: Vec<String>,
    errors: Vec<SemanticError>,
}

impl<'a> SymbolTablePopulator<'a> {
    pub fn new(symbol_table: &'a mut SymbolTable) -> Self {
        Self {
            symbol_table,
            relationship_graph: None,
            current_namespace: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn with_relationships(
        symbol_table: &'a mut SymbolTable,
        relationship_graph: &'a mut RelationshipGraph,
    ) -> Self {
        Self {
            symbol_table,
            relationship_graph: Some(relationship_graph),
            current_namespace: Vec::new(),
            errors: Vec::new(),
        }
    }
}

#[cfg(test)]
#[path = "populator/tests.rs"]
mod tests;
