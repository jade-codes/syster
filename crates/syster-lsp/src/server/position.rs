use super::LspServer;
use super::helpers::find_element_at_position;
use std::path::PathBuf;
use tower_lsp::lsp_types::{Position, Range};

impl LspServer {
    /// Find the symbol name and range at the given position by querying the AST
    pub(super) fn find_symbol_at_position(
        &self,
        path: &PathBuf,
        position: Position,
    ) -> Option<(String, Range)> {
        use super::helpers::span_to_lsp_range;
        use syster::core::Position as CorePosition;

        // Get the SysML file from workspace
        let workspace_file = self.workspace.files().get(path)?;

        // Only process SysML files for now
        let file = match workspace_file.content() {
            syster::language::LanguageFile::SysML(sysml_file) => sysml_file,
            syster::language::LanguageFile::KerML(_) => return None,
        };

        // Convert LSP position to our 0-indexed position
        let core_pos = CorePosition::new(position.line as usize, position.character as usize);

        // Search elements for one containing this position
        for element in &file.elements {
            if let Some((name, span)) = find_element_at_position(element, core_pos) {
                return Some((name, span_to_lsp_range(&span)));
            }
        }

        None
    }
}
