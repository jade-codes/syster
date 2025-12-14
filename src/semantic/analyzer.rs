use crate::semantic::RelationshipGraph;
use crate::semantic::error::{SemanticError, SemanticResult};
use crate::semantic::resolver::NameResolver;
use crate::semantic::symbol_table::{Symbol, SymbolTable};

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

        // Pass 1: Validate type references
        self.validate_types(&mut context);

        // Pass 2: Validate relationships (specialization, redefinition, etc.)
        self.validate_relationships(&mut context);

        context.into_result(())
    }

    fn validate_types(&self, context: &mut AnalysisContext) {
        for (_name, symbol) in self.symbol_table.all_symbols() {
            if let Some(type_ref) = symbol.type_reference() {
                let scope_id = symbol.scope_id();
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
        // Validate that all relationship targets exist
        for relationship_type in self.relationship_graph.relationship_types() {
            for (_name, symbol) in self.symbol_table.all_symbols() {
                let qualified_name = symbol.qualified_name();

                if let Some(targets) = self
                    .relationship_graph
                    .get_one_to_many(&relationship_type, qualified_name)
                {
                    for target in targets {
                        if let Some(target_symbol) = self.symbol_table.lookup(target) {
                            // Validate domain-specific relationship constraints
                            self.validate_domain_relationship_constraints(
                                &relationship_type,
                                symbol,
                                target_symbol,
                                context,
                            );
                        } else {
                            context.add_error(SemanticError::undefined_reference(format!(
                                "'{}' has {} relationship to undefined target '{}'",
                                qualified_name, relationship_type, target
                            )));
                        }
                    }
                }

                if let Some(target) = self
                    .relationship_graph
                    .get_one_to_one(&relationship_type, qualified_name)
                {
                    if let Some(target_symbol) = self.symbol_table.lookup(target) {
                        self.validate_domain_relationship_constraints(
                            &relationship_type,
                            symbol,
                            target_symbol,
                            context,
                        );
                    } else {
                        context.add_error(SemanticError::undefined_reference(format!(
                            "'{}' has {} relationship to undefined target '{}'",
                            qualified_name, relationship_type, target
                        )));
                    }
                }
            }
        }

        // Check for circular specialization dependencies
        for (_name, symbol) in self.symbol_table.all_symbols() {
            let qualified_name = symbol.qualified_name();

            if let Some(targets) = self
                .relationship_graph
                .get_one_to_many("specialization", qualified_name)
            {
                for target in targets {
                    if self.relationship_graph.has_transitive_path(
                        "specialization",
                        target,
                        qualified_name,
                    ) {
                        let cycle = vec![qualified_name.to_string(), target.to_string()];
                        context.add_error(SemanticError::circular_dependency(cycle));
                        break;
                    }
                }
            }
        }
    }

    fn validate_domain_relationship_constraints(
        &self,
        relationship_type: &str,
        _source: &Symbol,
        target: &Symbol,
        context: &mut AnalysisContext,
    ) {
        match relationship_type {
            "satisfy" => {
                // Satisfy relationships should target requirements
                if let Symbol::Definition { kind, .. } = target {
                    if kind != "Requirement" {
                        context.add_error(SemanticError::invalid_type(format!(
                            "satisfy relationship must target a requirement, but '{}' is a {}",
                            target.qualified_name(),
                            kind
                        )));
                    }
                } else if !matches!(target, Symbol::Usage { kind, .. } if kind == "Requirement") {
                    context.add_error(SemanticError::invalid_type(format!(
                        "satisfy relationship must target a requirement, but '{}' is not",
                        target.qualified_name()
                    )));
                }
            }
            "perform" => {
                // Perform relationships should target actions
                if let Symbol::Definition { kind, .. } = target
                    && kind != "Action"
                {
                    context.add_error(SemanticError::invalid_type(format!(
                        "perform relationship must target an action, but '{}' is a {}",
                        target.qualified_name(),
                        kind
                    )));
                }
            }
            "exhibit" => {
                // Exhibit relationships should target states
                if let Symbol::Definition { kind, .. } = target
                    && kind != "State"
                {
                    context.add_error(SemanticError::invalid_type(format!(
                        "exhibit relationship must target a state, but '{}' is a {}",
                        target.qualified_name(),
                        kind
                    )));
                }
            }
            "include" => {
                // Include relationships should target use cases
                if let Symbol::Definition { kind, .. } = target
                    && kind != "UseCase"
                {
                    context.add_error(SemanticError::invalid_type(format!(
                        "include relationship must target a use case, but '{}' is a {}",
                        target.qualified_name(),
                        kind
                    )));
                }
            }
            _ => {
                // Other relationships don't have specific type constraints yet
            }
        }
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
