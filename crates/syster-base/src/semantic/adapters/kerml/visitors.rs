use crate::core::constants::{REL_REDEFINITION, REL_SPECIALIZATION, REL_SUBSETTING, REL_TYPING};
use crate::semantic::symbol_table::Symbol;
use crate::syntax::kerml::ast::{
    Classifier, ClassifierKind, ClassifierMember, Element, Feature, FeatureMember, Import,
    NamespaceDeclaration, Package,
};

use crate::semantic::adapters::KermlAdapter;

impl<'a> KermlAdapter<'a> {
    pub(super) fn visit_namespace(&mut self, namespace: &NamespaceDeclaration) {
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
        self.enter_namespace(namespace.name.clone());
    }

    pub(super) fn visit_package(&mut self, package: &Package) {
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
            // Don't exit here - let the caller manage lifecycle
        }
    }

    pub(super) fn visit_classifier(&mut self, classifier: &Classifier) {
        if let Some(name) = &classifier.name {
            let qualified_name = self.qualified_name(name);
            let scope_id = self.symbol_table.current_scope_id();

            // Determine the kind string and symbol type
            // Use Symbol::Classifier for "classifier" kind (tracks is_abstract)
            // Use Symbol::Definition for specific types (datatype, function, etc.)
            let (use_classifier_symbol, kind_str) = match classifier.kind {
                ClassifierKind::Classifier => (true, "Classifier"),
                ClassifierKind::DataType => (false, "Datatype"),
                ClassifierKind::Function => (false, "Function"),
                ClassifierKind::Class => (false, "Class"),
                ClassifierKind::Structure => (false, "Structure"),
                ClassifierKind::Behavior => (false, "Behavior"),
                ClassifierKind::Type => (false, "Type"),
                ClassifierKind::Association => (false, "Association"),
                ClassifierKind::AssociationStructure => (false, "AssociationStructure"),
                ClassifierKind::Metaclass => (false, "Metaclass"),
            };

            let symbol = if use_classifier_symbol {
                Symbol::Classifier {
                    name: name.clone(),
                    qualified_name,
                    kind: kind_str.to_string(),
                    is_abstract: classifier.is_abstract,
                    scope_id,
                    source_file: self.symbol_table.current_file().map(String::from),
                    span: classifier.span,
                    references: Vec::new(),
                }
            } else {
                Symbol::Definition {
                    name: name.clone(),
                    qualified_name,
                    kind: kind_str.to_string(),
                    semantic_role: None,
                    scope_id,
                    source_file: self.symbol_table.current_file().map(String::from),
                    span: classifier.span,
                    references: Vec::new(),
                }
            };

            self.insert_symbol(name.clone(), symbol);
            self.enter_namespace(name.clone());
            // Don't exit here - let the caller manage lifecycle

            // Process classifier members
            for member in &classifier.body {
                self.visit_classifier_member(member);
            }
        } else {
            // Anonymous classifier - still process its members
            for member in &classifier.body {
                self.visit_classifier_member(member);
            }
        }
    }

    pub(super) fn visit_classifier_member(&mut self, member: &ClassifierMember) {
        match member {
            ClassifierMember::Feature(feature) => self.visit_feature(feature),
            ClassifierMember::Specialization(spec) => {
                // Record relationship in graph if available
                // Use qualified name for source since relationships are stored with qualified names
                if let Some(graph) = &mut self.relationship_graph {
                    let source_qname = self.current_namespace.join("::");
                    if !source_qname.is_empty() {
                        graph.add_one_to_many(
                            REL_SPECIALIZATION,
                            source_qname,
                            spec.general.clone(),
                            spec.span,
                        );
                    }
                }
            }
            ClassifierMember::Import(import) => {
                // Handle imports in classifier bodies
                self.visit_import(import);
            }
            ClassifierMember::Comment(_) => {
                // Skip comments
            }
        }
    }

    pub(super) fn visit_feature(&mut self, feature: &Feature) {
        if let Some(name) = &feature.name {
            let qualified_name = self.qualified_name(name);
            let scope_id = self.symbol_table.current_scope_id();
            let symbol = Symbol::Feature {
                name: name.clone(),
                qualified_name,
                scope_id,
                feature_type: None,
                source_file: self.symbol_table.current_file().map(String::from),
                span: feature.span,
                references: Vec::new(),
            };
            self.insert_symbol(name.clone(), symbol);

            // Enter namespace for named features to support nested scopes
            // Features like steps, behaviors can contain nested elements
            self.enter_namespace(name.clone());
            // Don't exit here - let the caller manage lifecycle

            // Process feature members (typing, redefinition, subsetting)
            for member in &feature.body {
                self.visit_feature_member(name, member);
            }
        } else {
            // Anonymous feature - process members but don't create scope
            for member in &feature.body {
                self.visit_feature_member("", member);
            }
        }
    }

    pub(super) fn visit_feature_member(&mut self, feature_name: &str, member: &FeatureMember) {
        if let Some(graph) = &mut self.relationship_graph {
            match member {
                FeatureMember::Typing(typing) => {
                    graph.add_one_to_one(
                        REL_TYPING,
                        feature_name.to_string(),
                        typing.typed.clone(),
                        typing.span,
                    );
                }
                FeatureMember::Redefinition(redef) => {
                    graph.add_one_to_one(
                        REL_REDEFINITION,
                        feature_name.to_string(),
                        redef.redefined.clone(),
                        redef.span,
                    );
                }
                FeatureMember::Subsetting(subset) => {
                    graph.add_one_to_one(
                        REL_SUBSETTING,
                        feature_name.to_string(),
                        subset.subset.clone(),
                        subset.span,
                    );
                }
                FeatureMember::Comment(_) => {
                    // Skip
                }
            }
        }
    }

    pub(super) fn visit_element(&mut self, element: &Element) {
        match element {
            Element::Package(package) => {
                self.visit_package(package);
                // Process children
                for child in &package.elements {
                    self.visit_element(child);
                }
                // Exit namespace if package has a name
                if package.name.is_some() {
                    self.exit_namespace();
                }
            }
            Element::Classifier(classifier) => {
                self.visit_classifier(classifier);
                // Classifier already processes its members internally
                // Exit namespace if classifier has a name
                if classifier.name.is_some() {
                    self.exit_namespace();
                }
            }
            Element::Feature(feature) => {
                self.visit_feature(feature);
                // Exit namespace if feature has a name (for steps, etc with nested elements)
                if feature.name.is_some() {
                    self.exit_namespace();
                }
            }
            Element::Import(_) | Element::Annotation(_) | Element::Comment(_) => {
                // Skip annotations and comments for now
                // Imports are handled below
            }
        }

        // Handle imports separately to track them in symbol table
        if let Element::Import(import) = element {
            self.visit_import(import);
        }
    }

    pub(super) fn visit_import(&mut self, import: &Import) {
        // Record the import in the current scope
        let current_file = self.symbol_table.current_file().map(String::from);
        self.symbol_table.add_import(
            import.path.clone(),
            import.is_recursive,
            import.span,
            current_file,
        );
    }
}
