use crate::syntax::kerml::KerMLFile;
use crate::syntax::kerml::ast::Element as KerMLElement;
use crate::syntax::sysml::ast::{Element as SysMLElement, SysMLFile};

/// A parsed syntax file that can be either SysML or KerML
#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxFile {
    SysML(SysMLFile),
    KerML(KerMLFile),
}

/// Extract import paths from a SysML file
pub fn extract_sysml_imports(file: &SysMLFile) -> Vec<String> {
    file.elements
        .iter()
        .filter_map(|element| {
            if let SysMLElement::Import(import) = element {
                Some(import.path.clone())
            } else {
                None
            }
        })
        .collect()
}

/// Extract import paths from a KerML file
pub fn extract_kerml_imports(file: &KerMLFile) -> Vec<String> {
    file.elements
        .iter()
        .filter_map(|element| {
            if let KerMLElement::Import(import) = element {
                Some(import.path.clone())
            } else {
                None
            }
        })
        .collect()
}

// Implement ParsedFile trait for semantic layer
impl crate::semantic::ParsedFile for SyntaxFile {
    fn extract_imports(&self) -> Vec<String> {
        match self {
            SyntaxFile::SysML(sysml_file) => extract_sysml_imports(sysml_file),
            SyntaxFile::KerML(kerml_file) => extract_kerml_imports(kerml_file),
        }
    }
}

impl SyntaxFile {
    /// Extracts import statements from the file
    ///
    /// Returns a vector of qualified import paths found in the file.
    pub fn extract_imports(&self) -> Vec<String> {
        crate::semantic::ParsedFile::extract_imports(self)
    }

    /// Returns a reference to the SysML file if this is a SysML file
    pub fn as_sysml(&self) -> Option<&SysMLFile> {
        match self {
            SyntaxFile::SysML(sysml_file) => Some(sysml_file),
            SyntaxFile::KerML(_) => None,
        }
    }

    /// Returns a reference to the KerML file if this is a KerML file
    pub fn as_kerml(&self) -> Option<&KerMLFile> {
        match self {
            SyntaxFile::SysML(_) => None,
            SyntaxFile::KerML(kerml_file) => Some(kerml_file),
        }
    }
}
