use super::LspServer;
use async_lsp::lsp_types::{LinkedEditingRanges, Position, Url};

impl LspServer {
    /// Get linked editing ranges for a position
    ///
    /// Returns ranges that should be edited simultaneously when the user
    /// edits one of them. This is typically used for syntactically coupled
    /// constructs like opening/closing tags in XML/HTML.
    ///
    /// For SysML v2, this feature is currently not applicable as there are
    /// no syntactically coupled constructs that would benefit from linked editing.
    /// The semantic renaming of symbols is already handled by the rename feature.
    ///
    /// Returns None to indicate no linked ranges at this position.
    pub fn get_linked_editing_ranges(
        &self,
        _uri: &Url,
        _position: Position,
    ) -> Option<LinkedEditingRanges> {
        // SysML/KerML does not have syntactically coupled constructs like HTML tags
        // that would benefit from linked editing. Semantic symbol renaming is
        // handled by the rename feature instead.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_lsp::lsp_types::Url;

    #[test]
    fn test_linked_editing_returns_none() {
        let server = LspServer::new();
        let uri = Url::parse("file:///test.sysml").unwrap();
        let position = Position::new(0, 0);

        let result = server.get_linked_editing_ranges(&uri, position);
        assert!(result.is_none(), "Should return None for SysML files");
    }

    #[test]
    fn test_linked_editing_with_different_positions() {
        let server = LspServer::new();
        let uri = Url::parse("file:///test.kerml").unwrap();

        // Test various positions
        let positions = vec![
            Position::new(0, 0),
            Position::new(10, 5),
            Position::new(100, 50),
        ];

        for position in positions {
            let result = server.get_linked_editing_ranges(&uri, position);
            assert!(
                result.is_none(),
                "Should return None for position {:?}",
                position
            );
        }
    }
}
