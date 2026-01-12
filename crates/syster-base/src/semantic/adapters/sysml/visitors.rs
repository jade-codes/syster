use crate::semantic::symbol_table::Symbol;
use crate::semantic::types::TokenType;
use crate::syntax::sysml::ast::{
    Alias, Comment, Definition, Import, NamespaceDeclaration, Package, Usage,
};
use crate::syntax::sysml::visitor::AstVisitor;

use crate::semantic::adapters::SysmlAdapter;

impl<'a> AstVisitor for SysmlAdapter<'a> {
    fn visit_namespace(&mut self, namespace: &NamespaceDeclaration) {
        let qualified_name = self.qualified_name(&namespace.name);
        let scope_id = self.symbol_table.current_scope_id();
        let current_file = self.symbol_table.current_file().map(String::from);
        let symbol = Symbol::Package {
            name: namespace.name.clone(),
            qualified_name,
            scope_id,
            source_file: current_file,
            span: namespace.span,
        };
        self.insert_symbol(namespace.name.clone(), symbol);
        self.enter_namespace(namespace.name.clone());
    }

    fn visit_package(&mut self, package: &Package) {
        if let Some(name) = &package.name {
            let qualified_name = self.qualified_name(name);
            let scope_id = self.symbol_table.current_scope_id();
            let source_file = self.symbol_table.current_file().map(String::from);
            let symbol = Symbol::Package {
                name: name.clone(),
                qualified_name,
                scope_id,
                source_file,
                span: package.span,
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
                span: definition.span,
            };
            self.insert_symbol(name.clone(), symbol);

            // If there's a short name, create an alias for it
            if let Some(ref short_name) = definition.short_name {
                let short_qualified_name = self.qualified_name(short_name);
                let alias_symbol = Symbol::Alias {
                    name: short_name.clone(),
                    qualified_name: short_qualified_name,
                    target: qualified_name.clone(),
                    target_span: definition.span,
                    scope_id,
                    source_file: self.symbol_table.current_file().map(String::from),
                    span: definition.short_name_span,
                };
                self.insert_symbol(short_name.clone(), alias_symbol);
            }

            // Index all relationship references for reverse lookups
            for spec in &definition.relationships.specializes {
                self.index_reference(&qualified_name, &spec.target, spec.span);
            }
            for redef in &definition.relationships.redefines {
                self.index_reference(&qualified_name, &redef.target, redef.span);
            }
            for include in &definition.relationships.includes {
                self.index_reference(&qualified_name, &include.target, include.span);
            }

            // Index domain relationships from nested usages
            for member in &definition.body {
                if let crate::syntax::sysml::ast::enums::DefinitionMember::Usage(usage) = member {
                    if let Some((target, span)) = usage.domain_target() {
                        self.index_reference(&qualified_name, target, span);
                    }
                    // Also index explicit relationship targets
                    for satisfy in &usage.relationships.satisfies {
                        self.index_reference(&qualified_name, &satisfy.target, satisfy.span);
                    }
                    for perform in &usage.relationships.performs {
                        self.index_reference(&qualified_name, &perform.target, perform.span);
                    }
                    for exhibit in &usage.relationships.exhibits {
                        self.index_reference(&qualified_name, &exhibit.target, exhibit.span);
                    }
                    for include in &usage.relationships.includes {
                        self.index_reference(&qualified_name, &include.target, include.span);
                    }
                }
            }

            // Visit nested members in the body
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
        // Determine the name and span: prefer explicit name, fall back to first redefinition or subsetting target
        let (name, name_span, is_anonymous) = if let Some(name) = &usage.name {
            (name.clone(), usage.span, false)
        } else if let Some(first_redef) = usage.relationships.redefines.first() {
            // Use the redefinition target as the name, with its span (for :>>)
            (first_redef.target.clone(), first_redef.span, true)
        } else if let Some(first_subset) = usage.relationships.subsets.first() {
            // Use the subsetting target as the name, with its span (for :>)
            (first_subset.target.clone(), first_subset.span, true)
        } else {
            return;
        };

        let qualified_name = self.qualified_name(&name);

        // Always index relationship references, even for duplicate anonymous usages.
        // This ensures semantic tokens are generated for type references like
        // `ref :> annotatedElement : SysML::ConnectionDefinition;`
        //
        // Token types are determined by what the reference points to:
        // - redefines/subsets → Property (they reference usages/features)
        // - typed_by → Type (they reference definitions/classifiers)
        for rel in &usage.relationships.redefines {
            self.index_reference_with_type(
                &qualified_name,
                &rel.target,
                rel.span,
                Some(TokenType::Property),
            );
        }
        for subset in &usage.relationships.subsets {
            self.index_reference_with_type(
                &qualified_name,
                &subset.target,
                subset.span,
                Some(TokenType::Property),
            );
        }
        if let Some(ref target) = usage.relationships.typed_by {
            self.index_reference_with_type(
                &qualified_name,
                target,
                usage.relationships.typed_by_span,
                Some(TokenType::Type),
            );
        }
        // references (::>) target usages, so Property token
        for reference in &usage.relationships.references {
            self.index_reference_with_type(
                &qualified_name,
                &reference.target,
                reference.span,
                Some(TokenType::Property),
            );
        }
        // crosses (=>) target usages, so Property token
        for cross in &usage.relationships.crosses {
            self.index_reference_with_type(
                &qualified_name,
                &cross.target,
                cross.span,
                Some(TokenType::Property),
            );
        }
        for meta in &usage.relationships.meta {
            self.index_reference(&qualified_name, &meta.target, meta.span);
        }

        // Skip duplicate anonymous usages (don't add to symbol table twice)
        if is_anonymous
            && self
                .symbol_table
                .find_by_qualified_name(&qualified_name)
                .is_some()
        {
            return;
        }

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
            span: name_span,
        };
        self.insert_symbol(name.clone(), symbol);

        // If there's a short name, create an alias
        if let Some(ref short_name) = usage.short_name {
            let short_qualified_name = self.qualified_name(short_name);
            let alias_symbol = Symbol::Alias {
                name: short_name.clone(),
                qualified_name: short_qualified_name,
                target: qualified_name.clone(),
                target_span: usage.span,
                scope_id,
                source_file: self.symbol_table.current_file().map(String::from),
                span: usage.short_name_span,
            };
            self.insert_symbol(short_name.clone(), alias_symbol);
        }

        // Visit nested members
        // Use the actual name for named usages, or the generated anonymous name for anonymous usages
        if !usage.body.is_empty() {
            self.enter_namespace(name.clone());
            for member in &usage.body {
                match member {
                    crate::syntax::sysml::ast::enums::UsageMember::Usage(nested_usage) => {
                        self.visit_usage(nested_usage);
                    }
                    crate::syntax::sysml::ast::enums::UsageMember::Comment(_) => {}
                }
            }
            self.exit_namespace();
        }
    }

    fn visit_import(&mut self, import: &Import) {
        let current_file = self.symbol_table.current_file().map(String::from);
        self.symbol_table.add_import(
            import.path.clone(),
            import.is_recursive,
            import.is_public,
            import.span,
            current_file.clone(),
        );

        let scope_id = self.symbol_table.current_scope_id();
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

    fn visit_comment(&mut self, _comment: &Comment) {}

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
            };
            self.insert_symbol(name.clone(), symbol);
        }
    }
}
