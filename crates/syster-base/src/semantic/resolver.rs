mod import_resolver;
mod name_resolver;

pub use name_resolver::Resolver;

// Re-export import utility functions - these delegate to the syntax layer
pub fn extract_imports(file: &crate::syntax::sysml::ast::SysMLFile) -> Vec<String> {
    crate::syntax::file::extract_sysml_imports(file)
}

pub fn extract_kerml_imports(file: &crate::syntax::kerml::ast::KerMLFile) -> Vec<String> {
    crate::syntax::file::extract_kerml_imports(file)
}

pub fn parse_import_path(path: &str) -> Vec<String> {
    Resolver::parse_import_path(path)
}

pub fn is_wildcard_import(path: &str) -> bool {
    Resolver::is_wildcard_import(path)
}

#[cfg(test)]
mod tests;
