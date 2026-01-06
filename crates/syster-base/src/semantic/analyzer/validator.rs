use super::AnalysisContext;
use crate::semantic::SemanticAnalyzer;
use crate::semantic::resolver::Resolver;
use crate::semantic::symbol_table::Symbol;
use crate::semantic::types::SemanticError;

impl SemanticAnalyzer {
    pub(super) fn validate_types(&self, context: &mut AnalysisContext) {
        let resolver = Resolver::new(&self.symbol_table);
        for (_name, symbol) in self.symbol_table.all_symbols() {
            if let Some(type_ref) = symbol.type_reference() {
                let scope_id = symbol.scope_id();
                let resolved = resolver.resolve_in_scope(type_ref, scope_id);

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

    pub(super) fn validate_relationships(&self, context: &mut AnalysisContext) {
        let resolver = Resolver::new(&self.symbol_table);

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
                            &resolver,
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
                    self.validate_single_relationship(
                        &resolver,
                        &relationship_type,
                        symbol,
                        target,
                        context,
                    );
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
        resolver: &Resolver,
        relationship_type: &str,
        source: &Symbol,
        target_name: &str,
        context: &mut AnalysisContext,
    ) {
        match resolver.resolve(target_name) {
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
