use std::collections::HashMap;
use std::path::PathBuf;
use syster::core::ParseError;
use syster::semantic::Workspace;
use syster::syntax::SyntaxFile;

/// LspServer manages the workspace state for the LSP server
pub struct LspServer {
    pub(super) workspace: Workspace<SyntaxFile>,
    /// Track parse errors for each file (keyed by file path)
    pub(super) parse_errors: HashMap<PathBuf, Vec<ParseError>>,
    /// Track document text for hover and other features (keyed by file path)
    pub(super) document_texts: HashMap<PathBuf, String>,
}

impl LspServer {
    pub fn new() -> Self {
        Self {
            workspace: Workspace<SyntaxFile>::new(),
            parse_errors: HashMap::new(),
            document_texts: HashMap::new(),
        }
    }

    pub fn workspace(&self) -> &Workspace<SyntaxFile> {
        &self.workspace
    }
}
