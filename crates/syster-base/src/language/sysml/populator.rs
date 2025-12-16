use crate::core::visitor::AstVisitor;
use crate::language::sysml::syntax::{
    Alias, Comment, Definition, Element, Import, NamespaceDeclaration, Package, SysMLFile, Usage,
};
use crate::semantic::error::SemanticError;
use crate::semantic::graph::RelationshipGraph;
use crate::semantic::symbol_table::{Symbol, SymbolTable};

// SysML relationship type constants
pub const REL_SPECIALIZATION: &str = "specialization";
pub const REL_REDEFINITION: &str = "redefinition";
pub const REL_SUBSETTING: &str = "subsetting";
pub const REL_TYPING: &str = "typing";
pub const REL_REFERENCE_SUBSETTING: &str = "reference_subsetting";
pub const REL_CROSS_SUBSETTING: &str = "cross_subsetting";

// Domain-specific SysML relationships
pub const REL_SATISFY: &str = "satisfy";
pub const REL_PERFORM: &str = "perform";
pub const REL_EXHIBIT: &str = "exhibit";
pub const REL_INCLUDE: &str = "include";
pub const REL_ASSERT: &str = "assert";
pub const REL_VERIFY: &str = "verify";

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

    /// Populates the symbol table by visiting all elements in the SysML file.
    ///
    /// # Errors
    ///
    /// Returns a vector of `SemanticError` if any semantic errors are encountered
    /// during population, such as duplicate symbol definitions.
    pub fn populate(&mut self, file: &SysMLFile) -> Result<(), Vec<SemanticError>> {
        // If there's a file-level namespace, enter it first
        let namespace_name = if let Some(ref ns) = file.namespace {
            self.visit_namespace(ns);
            Some(ns.name.clone())
        } else {
            None
        };

        // Process all elements in the file
        for element in &file.elements {
            // Skip Package element if it's the same as the file-level namespace
            // (we've already processed it via visit_namespace above)
            if let Element::Package(p) = element
                && let Some(ref ns_name) = namespace_name
                && p.name.as_ref() == Some(ns_name)
            {
                // This is the file-level package - skip it, we've already entered its namespace
                // But still process its children
                for child in &p.elements {
                    self.visit_element_with_lifecycle(child);
                }
                continue;
            }

            self.visit_element_with_lifecycle(element);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    fn visit_element_with_lifecycle(&mut self, element: &Element) {
        match element {
            Element::Package(p) => {
                self.visit_package(p);
                for child in &p.elements {
                    self.visit_element_with_lifecycle(child);
                }
                if p.name.is_some() {
                    self.exit_namespace();
                }
            }
            Element::Definition(d) => self.visit_definition(d),
            Element::Usage(u) => self.visit_usage(u),
            Element::Comment(c) => self.visit_comment(c),
            Element::Import(i) => self.visit_import(i),
            Element::Alias(a) => self.visit_alias(a),
        }
    }

    fn qualified_name(&self, name: &str) -> String {
        if self.current_namespace.is_empty() {
            name.to_string()
        } else {
            format!("{}::{}", self.current_namespace.join("::"), name)
        }
    }

    fn enter_namespace(&mut self, name: String) {
        self.current_namespace.push(name);
        self.symbol_table.enter_scope();
    }

    fn exit_namespace(&mut self) {
        self.current_namespace.pop();
        self.symbol_table.exit_scope();
    }

    fn insert_symbol(&mut self, name: String, symbol: Symbol) {
        if let Err(e) = self.symbol_table.insert(name.clone(), symbol) {
            self.errors
                .push(SemanticError::duplicate_definition(name, None));
            if let Some(last_error) = self.errors.last_mut() {
                last_error.message = e;
            }
        }
    }

    fn map_definition_kind(kind: &crate::language::sysml::syntax::DefinitionKind) -> String {
        match kind {
            crate::language::sysml::syntax::DefinitionKind::Part => "Part".to_string(),
            crate::language::sysml::syntax::DefinitionKind::Port => "Port".to_string(),
            crate::language::sysml::syntax::DefinitionKind::Action => "Action".to_string(),
            crate::language::sysml::syntax::DefinitionKind::State => "State".to_string(),
            crate::language::sysml::syntax::DefinitionKind::Item => "Item".to_string(),
            crate::language::sysml::syntax::DefinitionKind::Attribute => "Attribute".to_string(),
            crate::language::sysml::syntax::DefinitionKind::Requirement => {
                "Requirement".to_string()
            }
            crate::language::sysml::syntax::DefinitionKind::Concern => "UseCase".to_string(),
            crate::language::sysml::syntax::DefinitionKind::Case => "UseCase".to_string(),
            crate::language::sysml::syntax::DefinitionKind::AnalysisCase => "UseCase".to_string(),
            crate::language::sysml::syntax::DefinitionKind::VerificationCase => {
                "VerificationCase".to_string()
            }
            crate::language::sysml::syntax::DefinitionKind::UseCase => "UseCase".to_string(),
            crate::language::sysml::syntax::DefinitionKind::View => "View".to_string(),
            crate::language::sysml::syntax::DefinitionKind::Viewpoint => "Viewpoint".to_string(),
            crate::language::sysml::syntax::DefinitionKind::Rendering => "Rendering".to_string(),
        }
    }

    fn map_usage_kind(kind: &crate::language::sysml::syntax::UsageKind) -> String {
        match kind {
            crate::language::sysml::syntax::UsageKind::Part => "Part".to_string(),
            crate::language::sysml::syntax::UsageKind::Port => "Port".to_string(),
            crate::language::sysml::syntax::UsageKind::Action => "Action".to_string(),
            crate::language::sysml::syntax::UsageKind::Item => "Item".to_string(),
            crate::language::sysml::syntax::UsageKind::Attribute => "Attribute".to_string(),
            crate::language::sysml::syntax::UsageKind::Requirement => "Requirement".to_string(),
            crate::language::sysml::syntax::UsageKind::Concern => "Concern".to_string(),
            crate::language::sysml::syntax::UsageKind::Case => "Case".to_string(),
            crate::language::sysml::syntax::UsageKind::View => "View".to_string(),
            crate::language::sysml::syntax::UsageKind::SatisfyRequirement => {
                "SatisfyRequirement".to_string()
            }
            crate::language::sysml::syntax::UsageKind::PerformAction => "PerformAction".to_string(),
            crate::language::sysml::syntax::UsageKind::ExhibitState => "ExhibitState".to_string(),
            crate::language::sysml::syntax::UsageKind::IncludeUseCase => {
                "IncludeUseCase".to_string()
            }
        }
    }
}

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
                    if let crate::language::sysml::syntax::enums::DefinitionMember::Usage(usage) =
                        member
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
                if let crate::language::sysml::syntax::enums::DefinitionMember::Usage(usage) =
                    member
                {
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
                scope_id,
                source_file: self.symbol_table.current_file().map(String::from),
                span: usage.span,
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
            };
            self.insert_symbol(name.clone(), symbol);
        }
    }
}

#[cfg(test)]
#[path = "populator/tests.rs"]
mod tests;
