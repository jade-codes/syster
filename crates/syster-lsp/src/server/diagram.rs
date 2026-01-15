//! Diagram data provider for VS Code webview integration.
//!
//! Provides diagram data (symbols + relationships) in a format consumable
//! by the diagram-core TypeScript package.

use super::LspServer;
use async_lsp::lsp_types::request::Request;
use serde::{Deserialize, Serialize};
use std::path::Path;
use syster::semantic::symbol_table::Symbol;

/// Custom LSP request: syster/getDiagram
pub enum GetDiagramRequest {}

impl Request for GetDiagramRequest {
    type Params = GetDiagramParams;
    type Result = DiagramData;
    const METHOD: &'static str = "syster/getDiagram";
}

/// Request parameters for syster/getDiagram
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagramParams {
    /// URI of the file to get diagram for (optional - if None, returns whole workspace)
    pub uri: Option<String>,
}

/// Symbol data for diagram visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagramSymbol {
    pub name: String,
    pub qualified_name: String,
    pub kind: String,

    // Definition-specific
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_kind: Option<String>,

    // Usage-specific
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_kind: Option<String>,

    // Common optional fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

/// Relationship data for diagram edges
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagramRelationship {
    #[serde(rename = "type")]
    pub rel_type: String,
    pub source: String,
    pub target: String,
}

/// Complete diagram data response
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagramData {
    pub symbols: Vec<DiagramSymbol>,
    pub relationships: Vec<DiagramRelationship>,
}

impl LspServer {
    /// Get diagram data for the workspace or a specific file
    pub fn get_diagram(&self, file_path: Option<&Path>) -> DiagramData {
        let mut symbols = Vec::new();
        let mut relationships = Vec::new();

        // Collect symbols based on file path or whole workspace
        let symbol_iter: Box<dyn Iterator<Item = &Symbol>> = if let Some(path) = file_path {
            let path_str = path.to_str().unwrap_or("");
            Box::new(
                self.workspace
                    .symbol_table()
                    .get_symbols_for_file(path_str)
                    .into_iter(),
            )
        } else {
            Box::new(self.workspace.symbol_table().iter_symbols())
        };

        // Convert symbols and extract relationships from symbol data
        for symbol in symbol_iter {
            if let Some(diagram_symbol) = convert_symbol_to_diagram(symbol) {
                // Extract typing relationship from the symbol itself
                if let Some(ref typed_by) = diagram_symbol.typed_by {
                    relationships.push(DiagramRelationship {
                        rel_type: "typing".to_string(),
                        source: diagram_symbol.qualified_name.clone(),
                        target: typed_by.clone(),
                    });
                }
                symbols.push(diagram_symbol);
            }
        }

        DiagramData {
            symbols,
            relationships,
        }
    }
}

/// Convert a Symbol to DiagramSymbol
fn convert_symbol_to_diagram(symbol: &Symbol) -> Option<DiagramSymbol> {
    match symbol {
        Symbol::Definition {
            name,
            qualified_name,
            kind,
            ..
        } => Some(DiagramSymbol {
            name: name.clone(),
            qualified_name: qualified_name.clone(),
            kind: "Definition".to_string(),
            definition_kind: Some(kind.clone()),
            usage_kind: None,
            features: None,
            typed_by: None,
            direction: None,
        }),
        Symbol::Usage {
            name,
            qualified_name,
            kind,
            usage_type,
            ..
        } => Some(DiagramSymbol {
            name: name.clone(),
            qualified_name: qualified_name.clone(),
            kind: "Usage".to_string(),
            definition_kind: None,
            usage_kind: Some(kind.clone()),
            features: None,
            typed_by: usage_type.clone(),
            direction: None,
        }),
        Symbol::Package {
            name,
            qualified_name,
            ..
        } => Some(DiagramSymbol {
            name: name.clone(),
            qualified_name: qualified_name.clone(),
            kind: "Package".to_string(),
            definition_kind: None,
            usage_kind: None,
            features: None,
            typed_by: None,
            direction: None,
        }),
        Symbol::Feature {
            name,
            qualified_name,
            feature_type,
            ..
        } => Some(DiagramSymbol {
            name: name.clone(),
            qualified_name: qualified_name.clone(),
            kind: "Feature".to_string(),
            definition_kind: None,
            usage_kind: None,
            features: None,
            typed_by: feature_type.clone(),
            direction: None,
        }),
        Symbol::Classifier {
            name,
            qualified_name,
            kind,
            ..
        } => Some(DiagramSymbol {
            name: name.clone(),
            qualified_name: qualified_name.clone(),
            kind: "Classifier".to_string(),
            definition_kind: Some(kind.clone()),
            usage_kind: None,
            features: None,
            typed_by: None,
            direction: None,
        }),
        // Skip Alias and Import - not useful for diagrams
        Symbol::Alias { .. } | Symbol::Import { .. } => None,
    }
}
