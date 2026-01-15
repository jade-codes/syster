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

        // Collect symbols
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

        for symbol in symbol_iter {
            if let Some(diagram_symbol) = convert_symbol_to_diagram(symbol) {
                symbols.push(diagram_symbol);
            }
        }

        // Collect relationships from reference index
        // For each symbol, get its forward references (typing, specialization, etc.)
        let reference_index = self.workspace.reference_index();
        for symbol in &symbols {
            let targets = reference_index.get_targets(&symbol.qualified_name);
            for target in targets {
                // Determine relationship type based on context
                // For now, we'll mark all as "typing" - this can be refined later
                // by storing relationship types in the reference index
                relationships.push(DiagramRelationship {
                    rel_type: "typing".to_string(),
                    source: symbol.qualified_name.clone(),
                    target: target.to_string(),
                });
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
        } => {
            let definition_kind = match kind.as_str() {
                "part" => Some("Part".to_string()),
                "port" => Some("Port".to_string()),
                "action" => Some("Action".to_string()),
                "state" => Some("State".to_string()),
                "item" => Some("Item".to_string()),
                "attribute" => Some("Attribute".to_string()),
                "requirement" => Some("Requirement".to_string()),
                "concern" => Some("Concern".to_string()),
                "case" => Some("Case".to_string()),
                "analysis case" => Some("AnalysisCase".to_string()),
                "verification case" => Some("VerificationCase".to_string()),
                "use case" => Some("UseCase".to_string()),
                "view" => Some("View".to_string()),
                "viewpoint" => Some("Viewpoint".to_string()),
                "rendering" => Some("Rendering".to_string()),
                "allocation" => Some("Allocation".to_string()),
                "calculation" => Some("Calculation".to_string()),
                "connection" => Some("Connection".to_string()),
                "constraint" => Some("Constraint".to_string()),
                "enumeration" => Some("Enumeration".to_string()),
                "flow" => Some("Flow".to_string()),
                "individual" => Some("Individual".to_string()),
                "interface" => Some("Interface".to_string()),
                "occurrence" => Some("Occurrence".to_string()),
                "metadata" => Some("Metadata".to_string()),
                _ => None,
            };

            Some(DiagramSymbol {
                name: name.clone(),
                qualified_name: qualified_name.clone(),
                kind: "Definition".to_string(),
                definition_kind,
                usage_kind: None,
                features: None,
                typed_by: None,
                direction: None,
            })
        }
        Symbol::Usage {
            name,
            qualified_name,
            kind,
            usage_type,
            ..
        } => {
            let usage_kind = match kind.as_str() {
                "part" => Some("Part".to_string()),
                "port" => Some("Port".to_string()),
                "action" => Some("Action".to_string()),
                "item" => Some("Item".to_string()),
                "attribute" => Some("Attribute".to_string()),
                "requirement" => Some("Requirement".to_string()),
                "concern" => Some("Concern".to_string()),
                "case" => Some("Case".to_string()),
                "view" => Some("View".to_string()),
                "enumeration" => Some("Enumeration".to_string()),
                "satisfy" => Some("SatisfyRequirement".to_string()),
                "perform" => Some("PerformAction".to_string()),
                "exhibit" => Some("ExhibitState".to_string()),
                "include" => Some("IncludeUseCase".to_string()),
                "state" => Some("State".to_string()),
                "occurrence" => Some("Occurrence".to_string()),
                "individual" => Some("Individual".to_string()),
                "snapshot" => Some("Snapshot".to_string()),
                "timeslice" => Some("Timeslice".to_string()),
                "ref" | "reference" => Some("Reference".to_string()),
                "constraint" => Some("Constraint".to_string()),
                "calculation" | "calc" => Some("Calculation".to_string()),
                "connection" => Some("Connection".to_string()),
                "interface" => Some("Interface".to_string()),
                "allocation" | "allocate" => Some("Allocation".to_string()),
                "flow" => Some("Flow".to_string()),
                _ => None,
            };

            Some(DiagramSymbol {
                name: name.clone(),
                qualified_name: qualified_name.clone(),
                kind: "Usage".to_string(),
                definition_kind: None,
                usage_kind,
                features: None,
                typed_by: usage_type.clone(),
                direction: None,
            })
        }
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
        // Skip other symbol types (Classifier, Feature, Alias, Import)
        // as they don't map directly to diagram nodes
        _ => None,
    }
}
