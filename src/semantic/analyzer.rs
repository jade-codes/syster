use crate::semantic::RelationshipGraph;
use crate::semantic::error::{SemanticError, SemanticResult};
use crate::semantic::resolver::NameResolver;
use crate::semantic::symbol_table::{Symbol, SymbolTable};

/// Context for semantic analysis passes
pub struct AnalysisContext<'a> {
    pub symbol_table: &'a SymbolTable,
    pub relationship_graph: &'a RelationshipGraph,
    pub resolver: NameResolver<'a>,
    pub errors: Vec<SemanticError>,
}

impl<'a> AnalysisContext<'a> {
    pub fn new(symbol_table: &'a SymbolTable, relationship_graph: &'a RelationshipGraph) -> Self {
        Self {
            symbol_table,
            relationship_graph,
            resolver: NameResolver::new(symbol_table),
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: SemanticError) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Converts this context into a `Result`, returning the value if no errors were collected.
    ///
    /// # Errors
    ///
    /// Returns `Err` containing all collected semantic errors if any errors were added to this context.
    pub fn into_result<T>(self, value: T) -> SemanticResult<T> {
        if self.errors.is_empty() {
            Ok(value)
        } else {
            Err(self.errors)
        }
    }
}

/// Main semantic analyzer that orchestrates analysis passes
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    relationship_graph: RelationshipGraph,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            relationship_graph: RelationshipGraph::new(),
        }
    }

    pub fn with_symbol_table(symbol_table: SymbolTable) -> Self {
        Self {
            symbol_table,
            relationship_graph: RelationshipGraph::new(),
        }
    }

    pub fn with_symbol_table_and_relationships(
        symbol_table: SymbolTable,
        relationship_graph: RelationshipGraph,
    ) -> Self {
        Self {
            symbol_table,
            relationship_graph,
        }
    }

    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

    pub fn symbol_table_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    pub fn relationship_graph(&self) -> &RelationshipGraph {
        &self.relationship_graph
    }

    pub fn relationship_graph_mut(&mut self) -> &mut RelationshipGraph {
        &mut self.relationship_graph
    }

    /// Run all analysis passes on the symbol table
    /// # Errors
    ///
    /// Returns a `Vec<SemanticError>` if any semantic errors are detected during:
    /// - Symbol table structure validation
    /// - Type reference validation
    /// - Relationship validation (specialization, redefinition, etc.)
    pub fn analyze(&self) -> SemanticResult<()> {
        let mut context = AnalysisContext::new(&self.symbol_table, &self.relationship_graph);

        // Pass 1: Validate symbol table structure (scoping, duplicates)
        self.validate_symbol_table(&mut context);

        // Pass 2: Validate type references
        self.validate_types(&mut context);

        // Pass 3: Validate relationships (specialization, redefinition, etc.)
        self.validate_relationships(&mut context);

        context.into_result(())
    }

    fn validate_symbol_table(&self, _context: &mut AnalysisContext) {
        // Symbol table validation happens during insertion
        // This pass can check for additional structural issues
    }

    fn validate_types(&self, context: &mut AnalysisContext) {
        // Validate that all type references resolve to valid type symbols
        // Use scope-aware resolution: resolve from the scope where the symbol was defined
        for (_name, symbol) in self.symbol_table.all_symbols() {
            if let Some(type_ref) = symbol.type_reference() {
                let scope_id = symbol.scope_id();
                // Resolve the type reference from the symbol's defining scope
                let resolved = self.symbol_table.lookup_from_scope(type_ref, scope_id);

                match resolved {
                    Some(resolved_symbol) => {
                        if !resolved_symbol.is_type() {
                            context.add_error(SemanticError::invalid_type(format!(
                                "'{}' references '{}' which is not a valid type",
                                symbol.qualified_name(),
                                type_ref
                            )));
                        }
                    }
                    None => {
                        context.add_error(SemanticError::undefined_reference(format!(
                            "{} (referenced by '{}')",
                            type_ref,
                            symbol.qualified_name()
                        )));
                    }
                }
            }
        }
    }

    fn validate_relationships(&self, context: &mut AnalysisContext) {
        // Relationship validation checks semantic constraints between symbols
        // Future: This will validate specialization chains, redefinition rules, etc.
        // when relationship information is added to the Symbol structure.

        // For now, validate basic structural constraints:

        // Check for potentially problematic patterns
        for (_name, symbol) in self.symbol_table.all_symbols() {
            match symbol {
                Symbol::Classifier {
                    is_abstract: true, ..
                } => {
                    // Abstract classifiers shouldn't be directly instantiated
                    // This check would require tracking usages/instantiations
                }
                Symbol::Definition { kind, .. } => {
                    // Verify definition kind constraints
                    // For example: certain definition kinds may have specific requirements
                    self.validate_definition_constraints(kind, symbol, context);
                }
                _ => {}
            }
        }
    }

    fn validate_definition_constraints(
        &self,
        _kind: &crate::semantic::symbol_table::DefinitionKind,
        _symbol: &Symbol,
        _context: &mut AnalysisContext,
    ) {
        // Placeholder for definition-specific constraint validation
        // Future: Validate that Requirements have proper structure,
        // ViewPoints have Views, etc.
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "analyzer/tests.rs"]
mod tests;
