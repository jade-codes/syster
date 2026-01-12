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
            };
            self.insert_symbol(name.clone(), symbol);
            self.enter_namespace(name.clone());
        }
    }

    pub(super) fn visit_import(&mut self, import: &Import) {
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
        let symbol = Symbol::Import {
            path: import.path.clone(),
            path_span: import.path_span,
            qualified_name,
            is_recursive: import.is_recursive,
            scope_id,
            source_file: current_file,
            span: import.span,
        };
        let key = format!("import::{}", import.path);
        self.insert_symbol(key, symbol);
    }

    pub(super) fn visit_classifier(&mut self, classifier: &Classifier) {
        if let Some(name) = &classifier.name {
            let qualified_name = self.qualified_name(name);
            let scope_id = self.symbol_table.current_scope_id();

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
                }
            };
            self.insert_symbol(name.clone(), symbol);
            self.enter_namespace(name.clone());
            for member in &classifier.body {
                self.visit_classifier_member(member);
            }
        } else {
            for member in &classifier.body {
                self.visit_classifier_member(member);
            }
        }
    }

    pub(super) fn visit_classifier_member(&mut self, member: &ClassifierMember) {
        match member {
            ClassifierMember::Feature(feature) => self.visit_feature(feature),
            ClassifierMember::Specialization(spec) => {
                let source_qname = self.current_namespace.join("::");
                if !source_qname.is_empty() {
                    self.index_reference(&source_qname, &spec.general, spec.span);
                }
            }
            ClassifierMember::Import(import) => {
                self.visit_import(import);
            }
            ClassifierMember::Comment(_) => {}
        }
    }

    pub(super) fn visit_feature(&mut self, feature: &Feature) {
        if let Some(name) = &feature.name {
            let qualified_name = self.qualified_name(name);
            let scope_id = self.symbol_table.current_scope_id();
            let symbol = Symbol::Feature {
                name: name.clone(),
                qualified_name: qualified_name.clone(),
                scope_id,
                feature_type: None,
                source_file: self.symbol_table.current_file().map(String::from),
                span: feature.span,
            };
            self.insert_symbol(name.clone(), symbol);
            self.enter_namespace(name.clone());

            for member in &feature.body {
                self.visit_feature_member(&qualified_name, member);
            }

            self.exit_namespace();
        } else {
            for member in &feature.body {
                self.visit_feature_member("", member);
            }
        }
    }

    pub(super) fn visit_feature_member(&mut self, feature_name: &str, member: &FeatureMember) {
        match member {
            FeatureMember::Typing(typing) => {
                self.index_reference(feature_name, &typing.typed, typing.span);
            }
            FeatureMember::Redefinition(redef) => {
                self.index_reference(feature_name, &redef.redefined, redef.span);
            }
            FeatureMember::Subsetting(subset) => {
                self.index_reference(feature_name, &subset.subset, subset.span);
            }
            FeatureMember::Comment(_) => {}
        }
    }

    pub(super) fn visit_element(&mut self, element: &Element) {
        match element {
            Element::Package(package) => {
                self.visit_package(package);
                for child in &package.elements {
                    self.visit_element(child);
                }
                if package.name.is_some() {
                    self.exit_namespace();
                }
            }
            Element::Classifier(classifier) => {
                self.visit_classifier(classifier);
                if classifier.name.is_some() {
                    self.exit_namespace();
                }
            }
            Element::Feature(feature) => {
                self.visit_feature(feature);
                if feature.name.is_some() {
                    self.exit_namespace();
                }
            }
            Element::Import(import) => {
                self.visit_import(import);
            }
            Element::Annotation(_) | Element::Comment(_) => {}
        }
    }
}
