//! # Semantic Analyzer
//!
//! Orchestrates semantic validation passes over the symbol table and relationship graphs,
//! detecting errors and enforcing SysML/KerML semantic rules.
//!
//! ## Analysis Phases
//!
//! The analyzer runs multiple validation passes:
//!
//! 1. **Type Reference Validation**: Ensure all type references resolve to valid types
//! 2. **Relationship Validation**: Check specialization cycles, redefinition correctness
//! 3. **Multiplicity Validation**: Enforce cardinality constraints (future)
//! 4. **Constraint Evaluation**: Evaluate OCL-like constraints (future)
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────┐
//! │ AnalysisContext  │ ← Holds references and collects errors
//! └────────┬─────────┘
//!          │
//!          ├─→ SymbolTable
//!          ├─→ RelationshipGraph
//!          ├─→ NameResolver
//!          └─→ Vec<SemanticError>
//!
//! ┌──────────────────┐
//! │ SemanticAnalyzer │ ← Runs validation passes
//! └────────┬─────────┘
//!          │
//!          ├─→ validate_type_references()
//!          ├─→ validate_specializations()
//!          ├─→ validate_redefinitions()
//!          └─→ (more passes...)
//! ```
//!
//! ## Usage Example
//!
//! ```rust
//! use syster::semantic::{SemanticAnalyzer, SymbolTable, RelationshipGraph};
//!
//! let symbol_table = SymbolTable::new();
//! let relationship_graph = RelationshipGraph::new();
//!
//! // Create analyzer with populated model
//! let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(
//!     symbol_table,
//!     relationship_graph,
//! );
//!
//! // Run all validation passes
//! match analyzer.analyze() {
//!     Ok(()) => println!("No errors found"),
//!     Err(errors) => {
//!         for error in errors {
//!             eprintln!("Error: {:?}", error);
//!         }
//!     }
//! }
//! ```
//!
//! ## Analysis Context
//!
//! The `AnalysisContext` provides:
//! - Immutable access to symbol table and graphs
//! - Name resolution via `NameResolver`
//! - Error collection via `add_error()`
//!
//! **Key pattern**: Validation passes mutate the context by adding errors,
//! but don't modify the model.
//!
//! ## Error Handling
//!
//! Errors are collected (not thrown) to report multiple issues at once.
//! Validation passes should continue checking even after finding errors
//! to report as many issues as possible in a single analysis run.
//!
//! ## Extensibility
//!
//! To add a new validation pass:
//!
//! 1. Add error kind to `SemanticErrorKind` enum
//! 2. Implement validation method on `SemanticAnalyzer`
//! 3. Call from `analyze()` method
//! 4. Add tests in `analyzer/tests.rs`
//!
//! See [Adding Semantic Checks](../../docs/CONTRIBUTING.md#adding-new-features) for guide.
//!
//! ## Performance
//!
//! - **Validation passes**: O(n) where n = symbols (each symbol checked once)
//! - **Cycle detection**: O(V + E) where V = vertices, E = edges (DFS)
//! - **Name resolution**: O(1) average per lookup (HashMap-backed)
//!
//! For large models (10,000+ symbols), validation typically completes in milliseconds.

use crate::semantic::RelationshipGraph;
use crate::semantic::error::{SemanticError, SemanticResult};
use crate::semantic::processors::{NoOpValidator, RelationshipValidator};
use crate::semantic::resolver::NameResolver;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use std::sync::Arc;

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
    validator: Arc<dyn RelationshipValidator>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            relationship_graph: RelationshipGraph::new(),
            validator: Arc::new(NoOpValidator),
        }
    }

    pub fn with_symbol_table(symbol_table: SymbolTable) -> Self {
        Self {
            symbol_table,
            relationship_graph: RelationshipGraph::new(),
            validator: Arc::new(NoOpValidator),
        }
    }

    pub fn with_symbol_table_and_relationships(
        symbol_table: SymbolTable,
        relationship_graph: RelationshipGraph,
    ) -> Self {
        Self {
            symbol_table,
            relationship_graph,
            validator: Arc::new(NoOpValidator),
        }
    }

    pub fn with_validator(
        symbol_table: SymbolTable,
        relationship_graph: RelationshipGraph,
        validator: Arc<dyn RelationshipValidator>,
    ) -> Self {
        Self {
            symbol_table,
            relationship_graph,
            validator,
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
        // Validate that all relationship targets exist and check domain constraints
        for relationship_type in self.relationship_graph.relationship_types() {
            for (_name, symbol) in self.symbol_table.all_symbols() {
                let qualified_name = symbol.qualified_name();

                // Check one-to-many relationships
                if let Some(targets) = self
                    .relationship_graph
                    .get_one_to_many(&relationship_type, qualified_name)
                {
                    for target in targets {
                        self.validate_single_relationship(
                            &relationship_type,
                            symbol,
                            target,
                            context,
                        );
                    }
                }

                // Check one-to-one relationships
                if let Some(target) = self
                    .relationship_graph
                    .get_one_to_one(&relationship_type, qualified_name)
                {
                    self.validate_single_relationship(&relationship_type, symbol, target, context);
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

    /// Validates a single relationship between source symbol and target name
    fn validate_single_relationship(
        &self,
        relationship_type: &str,
        source: &Symbol,
        target_name: &str,
        context: &mut AnalysisContext,
    ) {
        match self.symbol_table.lookup(target_name) {
            Some(target_symbol) => {
                // Validate using language-specific validator
                if let Err(error) =
                    self.validator
                        .validate_relationship(relationship_type, source, target_symbol)
                {
                    context.add_error(error);
                }
            }
            None => {
                context.add_error(SemanticError::undefined_reference(format!(
                    "'{}' has {} relationship to undefined target '{}'",
                    source.qualified_name(),
                    relationship_type,
                    target_name
                )));
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
