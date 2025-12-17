use crate::semantic::types::SemanticError;
use crate::syntax::sysml::ast::{Element, SysMLFile};
use crate::syntax::sysml::visitor::AstVisitor;

use super::SymbolTablePopulator;

impl<'a> SymbolTablePopulator<'a> {
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

    pub(super) fn visit_element_with_lifecycle(&mut self, element: &Element) {
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
}
