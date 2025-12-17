use crate::language::kerml::KerMLFile;
use crate::language::sysml::syntax::SysMLFile;

/// A parsed language file that can be either SysML or KerML
#[derive(Debug, Clone, PartialEq)]
pub enum LanguageFile {
    SysML(SysMLFile),
    KerML(KerMLFile),
}

impl LanguageFile {
    /// Extracts import statements from the file
    ///
    /// Returns a vector of qualified import paths found in the file.
    /// For KerML files, returns an empty vector until KerML import extraction is implemented.
    pub fn extract_imports(&self) -> Vec<String> {
        match self {
            LanguageFile::SysML(sysml_file) => {
                crate::semantic::resolver::extract_imports(sysml_file)
            }
            LanguageFile::KerML(_) => {
                // TODO: Extract KerML imports when implemented
                vec![]
            }
        }
    }

    /// Returns a reference to the SysML file if this is a SysML file
    pub fn as_sysml(&self) -> Option<&SysMLFile> {
        match self {
            LanguageFile::SysML(sysml_file) => Some(sysml_file),
            LanguageFile::KerML(_) => None,
        }
    }

    /// Returns a reference to the KerML file if this is a KerML file
    pub fn as_kerml(&self) -> Option<&KerMLFile> {
        match self {
            LanguageFile::SysML(_) => None,
            LanguageFile::KerML(kerml_file) => Some(kerml_file),
        }
    }
}
