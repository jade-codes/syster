use crate::core::constants::*;
use crate::semantic::symbol_table::Symbol;
use crate::syntax::sysml::ast::{
    Alias, Comment, Definition, Import, NamespaceDeclaration, Package, Usage,
};
use crate::syntax::sysml::visitor::AstVisitor;

use super::SymbolTablePopulator;

impl<'a> AstVisitor for SymbolTablePopulator<'a> {
    fn visit_namespace(&mut self, namespace: &NamespaceDeclaration) {
        // Create the Package symbol for the file-level namespace
        let qualified_name = self.qualified_name(&namespace.name);
        let scope_id = self.symbol_table.current_scope_id();
        let symbol = Symbol::Package {
            name: namespace.name.clone(),
            qualified_name,
            scope_id,
            source_file: self.symbol_table.current_file().map(String::from),
            span: namespace.span,
            references: Vec::new(),
        };
        self.insert_symbol(namespace.name.clone(), symbol);

        // Enter the file-level namespace
        // This ensures all subsequent elements at file level are qualified with the namespace
        self.enter_namespace(namespace.name.clone());
    }

    fn visit_package(&mut self, package: &Package) {
        if let Some(name) = &package.name {
            let qualified_name = self.qualified_name(name);
            let scope_id = self.symbol_table.current_scope_id();
            let symbol = Symbol::Package {
                name: name.clone(),
                qualified_name,
                scope_id,
                source_file: self.symbol_table.current_file().map(String::from),
                span: package.span,
                references: Vec::new(),
            };
            self.insert_symbol(name.clone(), symbol);
            self.enter_namespace(name.clone());
        }
    }

    fn visit_definition(&mut self, definition: &Definition) {
        if let Some(name) = &definition.name {
            let qualified_name = self.qualified_name(name);
            let kind = Self::map_definition_kind(&definition.kind);
            let scope_id = self.symbol_table.current_scope_id();
            let symbol = Symbol::Definition {
                name: name.clone(),
                qualified_name: qualified_name.clone(),
                kind,
                scope_id,
                source_file: self.symbol_table.current_file().map(String::from),
                span: definition.span,
                references: Vec::new(),
            };
            self.insert_symbol(name.clone(), symbol);

            if let Some(ref mut graph) = self.relationship_graph {
                for target in &definition.relationships.specializes {
                    graph.add_one_to_many(
                        REL_SPECIALIZATION,
                        qualified_name.clone(),
                        target.clone(),
                    );
                }

                // Extract top-level domain relationships (e.g., include in use case definitions)
                // Note: exhibit/perform/satisfy are handled as nested usages below
                for target in &definition.relationships.includes {
                    graph.add_one_to_many(REL_INCLUDE, qualified_name.clone(), target.clone());
                }

                // Extract domain relationships from nested usages in the body
                // Note: include relationships are at the definition level, not in nested usages
                for member in &definition.body {
                    if let crate::syntax::sysml::ast::enums::DefinitionMember::Usage(usage) = member
                    {
                        // Extract satisfy relationships
                        for target in &usage.relationships.satisfies {
                            graph.add_one_to_many(
                                REL_SATISFY,
                                qualified_name.clone(),
                                target.clone(),
                            );
                        }
                        // Extract perform relationships
                        for target in &usage.relationships.performs {
                            graph.add_one_to_many(
                                REL_PERFORM,
                                qualified_name.clone(),
                                target.clone(),
                            );
                        }
                        // Extract exhibit relationships
                        for target in &usage.relationships.exhibits {
                            graph.add_one_to_many(
                                REL_EXHIBIT,
                                qualified_name.clone(),
                                target.clone(),
                            );
                        }
                        // Note: include relationships are handled at the definition level above
                    }
                }
            }

            // Visit nested members in the body (add them to symbol table)
            self.enter_namespace(name.clone());
            for member in &definition.body {
                if let crate::syntax::sysml::ast::enums::DefinitionMember::Usage(usage) = member {
                    self.visit_usage(usage);
                }
            }
            self.exit_namespace();
        }
    }

    fn visit_usage(&mut self, usage: &Usage) {
        if let Some(name) = &usage.name {
            let qualified_name = self.qualified_name(name);
            let kind = Self::map_usage_kind(&usage.kind);
            let scope_id = self.symbol_table.current_scope_id();
            let symbol = Symbol::Usage {
                name: name.clone(),
                qualified_name: qualified_name.clone(),
                kind,
                usage_type: usage.relationships.typed_by.clone(),
                scope_id,
                source_file: self.symbol_table.current_file().map(String::from),
                span: usage.span,
                references: Vec::new(),
            };
            self.insert_symbol(name.clone(), symbol);

            if let Some(ref mut graph) = self.relationship_graph {
                // Redefinitions (:>>)
                for target in &usage.relationships.redefines {
                    graph.add_one_to_many(REL_REDEFINITION, qualified_name.clone(), target.clone());
                }
                // Subsetting (:>)
                for target in &usage.relationships.subsets {
                    graph.add_one_to_many(REL_SUBSETTING, qualified_name.clone(), target.clone());
                }
                // Feature typing (:)
                if let Some(ref target) = usage.relationships.typed_by {
                    graph.add_one_to_one(REL_TYPING, qualified_name.clone(), target.clone());
                }
                // References (::>)
                for target in &usage.relationships.references {
                    graph.add_one_to_many(
                        REL_REFERENCE_SUBSETTING,
                        qualified_name.clone(),
                        target.clone(),
                    );
                }
                // Cross (=>)
                for target in &usage.relationships.crosses {
                    graph.add_one_to_many(
                        REL_CROSS_SUBSETTING,
                        qualified_name.clone(),
                        target.clone(),
                    );
                }
            }
        }
    }

    fn visit_import(&mut self, import: &Import) {
        // Record the import in the current scope
        self.symbol_table
            .add_import(import.path.clone(), import.is_recursive);
    }

    fn visit_comment(&mut self, _comment: &Comment) {
        // Comments don't affect symbol table
    }

    fn visit_alias(&mut self, alias: &Alias) {
        if let Some(name) = &alias.name {
            let qualified_name = self.qualified_name(name);
            let scope_id = self.symbol_table.current_scope_id();
            let symbol = Symbol::Alias {
                name: name.clone(),
                qualified_name,
                target: alias.target.clone(),
                scope_id,
                source_file: self.symbol_table.current_file().map(String::from),
                span: alias.span,
                references: Vec::new(),
            };
            self.insert_symbol(name.clone(), symbol);
        }
    }
}
