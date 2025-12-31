use super::helpers::{span_to_lsp_range, uri_to_path};
use super::LspServer;
use async_lsp::lsp_types::{Position, SymbolKind, TypeHierarchyItem, Url};
use syster::core::constants::REL_SPECIALIZATION;
use syster::semantic::symbol_table::Symbol;

impl LspServer {
    /// Prepare type hierarchy by finding the symbol at the given position
    ///
    /// This is the entry point for type hierarchy. It returns a TypeHierarchyItem
    /// representing the type at the cursor position, which can then be used to
    /// query for supertypes and subtypes.
    pub fn prepare_type_hierarchy(
        &self,
        uri: &Url,
        position: Position,
    ) -> Option<Vec<TypeHierarchyItem>> {
        let path = uri_to_path(uri)?;
        let (qualified_name, _range) = self.find_symbol_at_position(&path, position)?;

        // Get the symbol from the symbol table
        let symbol = self.workspace.symbol_table().resolve(&qualified_name)?;

        // Create TypeHierarchyItem
        let item = self.symbol_to_type_hierarchy_item(symbol)?;

        Some(vec![item])
    }

    /// Get supertypes (types that this type specializes/inherits from)
    pub fn get_type_hierarchy_supertypes(
        &self,
        item: &TypeHierarchyItem,
    ) -> Option<Vec<TypeHierarchyItem>> {
        // Extract qualified name from the data field
        let qualified_name = self.extract_qualified_name_from_item(item)?;

        // Get specialization relationships (what this type specializes)
        let supertypes = self
            .workspace
            .relationship_graph()
            .get_one_to_many(REL_SPECIALIZATION, &qualified_name)?;

        // Convert each supertype to a TypeHierarchyItem
        let items: Vec<TypeHierarchyItem> = supertypes
            .iter()
            .filter_map(|supertype_qname| {
                let symbol = self.workspace.symbol_table().resolve(supertype_qname)?;
                self.symbol_to_type_hierarchy_item(symbol)
            })
            .collect();

        if items.is_empty() {
            None
        } else {
            Some(items)
        }
    }

    /// Get subtypes (types that specialize/inherit from this type)
    pub fn get_type_hierarchy_subtypes(
        &self,
        item: &TypeHierarchyItem,
    ) -> Option<Vec<TypeHierarchyItem>> {
        // Extract qualified name from the data field
        let qualified_name = self.extract_qualified_name_from_item(item)?;

        // Get reverse specialization relationships (what specializes this type)
        let subtypes = self
            .workspace
            .relationship_graph()
            .get_one_to_many_sources(REL_SPECIALIZATION, &qualified_name);

        if subtypes.is_empty() {
            return None;
        }

        // Convert each subtype to a TypeHierarchyItem
        let items: Vec<TypeHierarchyItem> = subtypes
            .iter()
            .filter_map(|subtype_qname| {
                let symbol = self.workspace.symbol_table().resolve(subtype_qname)?;
                self.symbol_to_type_hierarchy_item(symbol)
            })
            .collect();

        if items.is_empty() {
            None
        } else {
            Some(items)
        }
    }

    /// Convert a Symbol to a TypeHierarchyItem
    fn symbol_to_type_hierarchy_item(&self, symbol: &Symbol) -> Option<TypeHierarchyItem> {
        let source_file = symbol.source_file()?;
        let span = symbol.span()?;
        let uri = Url::from_file_path(source_file).ok()?;
        let range = span_to_lsp_range(&span);

        let kind = match symbol {
            Symbol::Package { .. } => SymbolKind::NAMESPACE,
            Symbol::Classifier { .. } | Symbol::Definition { .. } => SymbolKind::CLASS,
            Symbol::Feature { .. } | Symbol::Usage { .. } => SymbolKind::PROPERTY,
            Symbol::Alias { .. } => SymbolKind::VARIABLE,
        };

        // Store qualified name in data field for later retrieval
        let data = serde_json::to_value(symbol.qualified_name()).ok()?;

        Some(TypeHierarchyItem {
            name: symbol.name().to_string(),
            kind,
            tags: None,
            detail: Some(symbol.qualified_name().to_string()),
            uri,
            range,
            selection_range: range,
            data: Some(data),
        })
    }

    /// Extract qualified name from TypeHierarchyItem's data field
    fn extract_qualified_name_from_item(&self, item: &TypeHierarchyItem) -> Option<String> {
        let data = item.data.as_ref()?;
        data.as_str().map(|s| s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_prepare_type_hierarchy_basic() {
        // Create a test file with a simple class
        let mut server = LspServer::new();

        let test_file = PathBuf::from("/tmp/test_type_hierarchy.kerml");
        let source = r#"
classifier Vehicle;
classifier Car specializes Vehicle;
"#;

        // Add document to server
        let uri = Url::from_file_path(&test_file).unwrap();
        std::fs::write(&test_file, source).unwrap();
        let _ = server.open_document(&uri, source);

        // Position on "Car" (line 2, column 11)
        let position = Position {
            line: 2,
            character: 11,
        };

        let result = server.prepare_type_hierarchy(&uri, position);
        assert!(result.is_some());

        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Car");

        // Cleanup
        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_get_supertypes() {
        let mut server = LspServer::new();

        let test_file = PathBuf::from("/tmp/test_supertypes.kerml");
        let source = r#"
classifier Vehicle;
classifier LandVehicle specializes Vehicle;
classifier Car specializes LandVehicle;
"#;

        let uri = Url::from_file_path(&test_file).unwrap();
        std::fs::write(&test_file, source).unwrap();
        let _ = server.open_document(&uri, source);

        // Get Car's type hierarchy item
        let position = Position {
            line: 3,
            character: 11,
        };
        let prepare_result = server.prepare_type_hierarchy(&uri, position);
        assert!(prepare_result.is_some());

        let items = prepare_result.unwrap();
        let car_item = &items[0];

        // Get supertypes of Car
        let supertypes = server.get_type_hierarchy_supertypes(car_item);
        assert!(supertypes.is_some());

        let supertypes = supertypes.unwrap();
        assert_eq!(supertypes.len(), 1);
        assert_eq!(supertypes[0].name, "LandVehicle");

        // Cleanup
        std::fs::remove_file(&test_file).ok();
    }

    #[test]
    fn test_get_subtypes() {
        let mut server = LspServer::new();

        let test_file = PathBuf::from("/tmp/test_subtypes.kerml");
        let source = r#"
classifier Vehicle;
classifier Car specializes Vehicle;
classifier Truck specializes Vehicle;
"#;

        let uri = Url::from_file_path(&test_file).unwrap();
        std::fs::write(&test_file, source).unwrap();
        let _ = server.open_document(&uri, source);

        // Get Vehicle's type hierarchy item
        let position = Position {
            line: 1,
            character: 11,
        };
        let prepare_result = server.prepare_type_hierarchy(&uri, position);
        assert!(prepare_result.is_some());

        let items = prepare_result.unwrap();
        let vehicle_item = &items[0];

        // Get subtypes of Vehicle
        let subtypes = server.get_type_hierarchy_subtypes(vehicle_item);
        assert!(subtypes.is_some());

        let subtypes = subtypes.unwrap();
        assert_eq!(subtypes.len(), 2);

        let names: Vec<&str> = subtypes.iter().map(|item| item.name.as_str()).collect();
        assert!(names.contains(&"Car"));
        assert!(names.contains(&"Truck"));

        // Cleanup
        std::fs::remove_file(&test_file).ok();
    }
}
