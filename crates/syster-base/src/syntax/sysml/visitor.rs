//! Visitor pattern for SysML AST traversal.
//!
//! Note: AstVisitor is also re-exported from semantic::adapters::sysml for convenience.

use super::ast::{
    Alias, Comment, Definition, Element, Import, NamespaceDeclaration, Package, SysMLFile, Usage,
};

/// Visitor trait for SysML AST nodes.
///
/// Implement this trait to define custom behavior when traversing
/// SysML model elements. Default implementations are no-ops.
pub trait AstVisitor {
    fn visit_file(&mut self, _file: &SysMLFile) {}
    fn visit_namespace(&mut self, _namespace: &NamespaceDeclaration) {}
    fn visit_element(&mut self, _element: &Element) {}
    fn visit_package(&mut self, _package: &Package) {}
    fn visit_definition(&mut self, _definition: &Definition) {}
    fn visit_usage(&mut self, _usage: &Usage) {}
    fn visit_comment(&mut self, _comment: &Comment) {}
    fn visit_import(&mut self, _import: &Import) {}
    fn visit_alias(&mut self, _alias: &Alias) {}
}
