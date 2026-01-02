use crate::core::constants::*;
use crate::semantic::symbol_table::Symbol;
use crate::syntax::sysml::ast::{
    Alias, Comment, Definition, Import, NamespaceDeclaration, Package, Usage,
};
use crate::syntax::sysml::visitor::AstVisitor;

use crate::semantic::adapters::SysmlAdapter;

impl<'a> AstVisitor for SysmlAdapter<'a> {
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
            let semantic_role = Self::definition_kind_to_semantic_role(&definition.kind);
            let scope_id = self.symbol_table.current_scope_id();
            let symbol = Symbol::Definition {
                name: name.clone(),
                qualified_name: qualified_name.clone(),
                kind,
                semantic_role: Some(semantic_role),
                scope_id,
                source_file: self.symbol_table.current_file().map(String::from),
                // Use name_span if available, fallback to full span
                span: definition.span,
                references: Vec::new(),
            };
            self.insert_symbol(name.clone(), symbol);

            if let Some(ref mut graph) = self.relationship_graph {
                for spec in &definition.relationships.specializes {
                    graph.add_one_to_many(
                        REL_SPECIALIZATION,
                        qualified_name.clone(),
                        spec.target.clone(),
                        spec.span,
                    );
                }

                for redef in &definition.relationships.redefines {
                    graph.add_one_to_many(
                        REL_REDEFINITION,
                        qualified_name.clone(),
                        redef.target.clone(),
                        redef.span,
                    );
                }

                // Extract top-level domain relationships (e.g., include in use case definitions)
                // Note: exhibit/perform/satisfy are handled as nested usages below
                for include in &definition.relationships.includes {
                    graph.add_one_to_many(
                        REL_INCLUDE,
                        qualified_name.clone(),
                        include.target.clone(),
                        include.span,
                    );
                }

                // Extract domain relationships from nested usages in the body
                for member in &definition.body {
                    if let crate::syntax::sysml::ast::enums::DefinitionMember::Usage(usage) = member
                    {
                        // Extract satisfy relationships
                        for satisfy in &usage.relationships.satisfies {
                            graph.add_one_to_many(
                                REL_SATISFY,
                                qualified_name.clone(),
                                satisfy.target.clone(),
                                satisfy.span,
                            );
                        }
                        // Extract perform relationships
                        for perform in &usage.relationships.performs {
                            graph.add_one_to_many(
                                REL_PERFORM,
                                qualified_name.clone(),
                                perform.target.clone(),
                                perform.span,
                            );
                        }
                        // Extract exhibit relationships
                        for exhibit in &usage.relationships.exhibits {
                            graph.add_one_to_many(
                                REL_EXHIBIT,
                                qualified_name.clone(),
                                exhibit.target.clone(),
                                exhibit.span,
                            );
                        }
                        // Extract include relationships (from use case bodies)
                        for include in &usage.relationships.includes {
                            graph.add_one_to_many(
                                REL_INCLUDE,
                                qualified_name.clone(),
                                include.target.clone(),
                                include.span,
                            );
                        }
                    }
                }
            }

            // Visit nested members in the body (add them to symbol table)
            self.enter_namespace(name.clone());
            for member in &definition.body {
                match member {
                    crate::syntax::sysml::ast::enums::DefinitionMember::Usage(usage) => {
                        self.visit_usage(usage);
                    }
                    crate::syntax::sysml::ast::enums::DefinitionMember::Comment(_) => {}
                }
            }
            self.exit_namespace();
        }
    }

    fn visit_usage(&mut self, usage: &Usage) {
        // Get the name: explicit name, or inferred from first redefinition target
        // In SysML v2, `attribute :>> num` creates a feature named "num" that redefines the inherited one
        let (name, is_anonymous) = if let Some(name) = &usage.name {
            (name.clone(), false)
        } else if let Some(first_redef) = usage.relationships.redefines.first() {
            // Anonymous redefinition inherits name from redefined feature
            (first_redef.target.clone(), true)
        } else {
            // No name and no redefinition - skip
            return;
        };

        let qualified_name = self.qualified_name(&name);

        // For anonymous redefinitions, avoid symbol table collisions by checking if a symbol
        // with this qualified name already exists. This prevents duplicate symbols when the
        // same redefinition is processed multiple times (e.g., from different file paths).
        if is_anonymous
            && self
                .symbol_table
                .lookup_qualified(&qualified_name)
                .is_some()
        {
            return;
        }

        // Create a symbol for all usages (named and inferred from redefinition)
        let kind = Self::map_usage_kind(&usage.kind);
        let semantic_role = Self::usage_kind_to_semantic_role(&usage.kind);
        let scope_id = self.symbol_table.current_scope_id();

        let symbol = Symbol::Usage {
            name: name.clone(),
            qualified_name: qualified_name.clone(),
            kind,
            semantic_role: Some(semantic_role),
            usage_type: usage.relationships.typed_by.clone(),
            scope_id,
            source_file: self.symbol_table.current_file().map(String::from),
            span: usage.span,
            references: Vec::new(),
        };
        self.insert_symbol(name.clone(), symbol);

        // Store relationships for both named and anonymous usages
        if let Some(ref mut graph) = self.relationship_graph {
            // Redefinitions (:>>)
            for rel in &usage.relationships.redefines {
                graph.add_one_to_many(
                    REL_REDEFINITION,
                    qualified_name.clone(),
                    rel.target.clone(),
                    rel.span,
                );
            }
            // Subsetting (:>)
            for subset in &usage.relationships.subsets {
                graph.add_one_to_many(
                    REL_SUBSETTING,
                    qualified_name.clone(),
                    subset.target.clone(),
                    subset.span,
                );
            }
            // Feature typing (:)
            if let Some(ref target) = usage.relationships.typed_by {
                // Resolve target to fully qualified name using symbol table lookup
                let resolved_target = self
                    .symbol_table
                    .lookup(target)
                    .or_else(|| self.symbol_table.lookup_qualified(target))
                    .map(|s| s.qualified_name().to_string())
                    .unwrap_or_else(|| {
                        // If lookup fails and target contains ::, try prepending current namespace
                        if target.contains("::") {
                            format!("{}::{}", self.current_namespace.join("::"), target)
                        } else {
                            target.clone()
                        }
                    });

                graph.add_one_to_one(
                    REL_TYPING,
                    qualified_name.clone(),
                    resolved_target,
                    usage.relationships.typed_by_span,
                );
            }
            // References (::>)
            for reference in &usage.relationships.references {
                graph.add_one_to_many(
                    REL_REFERENCE_SUBSETTING,
                    qualified_name.clone(),
                    reference.target.clone(),
                    reference.span,
                );
            }
            // Cross (=>)
            for cross in &usage.relationships.crosses {
                graph.add_one_to_many(
                    REL_CROSS_SUBSETTING,
                    qualified_name.clone(),
                    cross.target.clone(),
                    cross.span,
                );
            }
        }

        // Visit nested members in the usage body (only for named usages)
        if let Some(name) = &usage.name {
            self.enter_namespace(name.clone());
            for member in &usage.body {
                match member {
                    crate::syntax::sysml::ast::enums::UsageMember::Usage(nested_usage) => {
                        self.visit_usage(nested_usage);
                    }
                    crate::syntax::sysml::ast::enums::UsageMember::Comment(_) => {
                        // Comments don't affect symbol table
                    }
                }
            }
            self.exit_namespace();
        }
    }

    fn visit_import(&mut self, import: &Import) {
        // Record the import in the current scope for resolution
        let current_file = self.symbol_table.current_file().map(String::from);
        self.symbol_table.add_import(
            import.path.clone(),
            import.is_recursive,
            import.span,
            current_file.clone(),
        );

        // Also create a Symbol::Import for semantic token highlighting
        let scope_id = self.symbol_table.current_scope_id();

        // Note: Two different identifier formats are used here:
        // 1. qualified_name: "import::scope_id::path" - Globally unique identifier for the symbol,
        //    includes scope_id to distinguish same import path used in different scopes
        // 2. key: "import::path" - Symbol table insertion key, simpler format without scope_id
        //    used to store the symbol in the current scope without conflicts
        let qualified_name = format!("import::{}::{}", scope_id, import.path);
        let key = format!("import::{}", import.path);

        let symbol = Symbol::Import {
            path: import.path.clone(),
            path_span: import.path_span,
            qualified_name,
            is_recursive: import.is_recursive,
            scope_id,
            source_file: current_file,
            span: import.span,
        };
        self.insert_symbol(key, symbol);
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
                target_span: alias.target_span,
                scope_id,
                source_file: self.symbol_table.current_file().map(String::from),
                span: alias.span,
                references: Vec::new(),
            };
            self.insert_symbol(name.clone(), symbol);
        }
    }
}
