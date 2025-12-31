mod tests_ast;
mod tests_countingvisitor;
mod tests_sysml_parsing;
mod tests_types_comment;
mod tests_types_import;
mod tests_types_namespacedeclaration;
mod tests_types_usage;
mod tests_utils_all_refs_from;
mod tests_utils_ref_from;

// Re-export CountingVisitor for use by other test modules
pub(crate) use tests_ast::CountingVisitor;
