mod import_resolver;
mod name_resolver;

pub use name_resolver::Resolver;

// Re-export import utility functions
pub fn extract_imports(file: &crate::syntax::sysml::ast::SysMLFile) -> Vec<String> {
    Resolver::extract_imports(file)
}

pub fn extract_kerml_imports(file: &crate::syntax::kerml::ast::KerMLFile) -> Vec<String> {
    file.elements
        .iter()
        .filter_map(|element| {
            if let crate::syntax::kerml::ast::Element::Import(import) = element {
                Some(import.path.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn parse_import_path(path: &str) -> Vec<String> {
    Resolver::parse_import_path(path)
}

pub fn is_wildcard_import(path: &str) -> bool {
    Resolver::is_wildcard_import(path)
}

#[cfg(test)]
#[path = "resolver/tests.rs"]
mod tests;
