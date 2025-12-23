use crate::syntax::kerml::KerMLFile;
use crate::syntax::sysml::ast::SysMLFile;

/// A parsed syntax file that can be either SysML or KerML
#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxFile {
    SysML(SysMLFile),
    KerML(KerMLFile),
}

// Implement ParsedFile trait for semantic layer
impl crate::semantic::ParsedFile for SyntaxFile {
    fn extract_imports(&self) -> Vec<String> {
        match self {
            SyntaxFile::SysML(sysml_file) => crate::semantic::extract_imports(sysml_file),
            SyntaxFile::KerML(kerml_file) => crate::semantic::extract_kerml_imports(kerml_file),
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
