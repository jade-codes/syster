use crate::semantic::symbol_table::Symbol;
use crate::semantic::types::SemanticError;
use crate::syntax::sysml::ast::{DefinitionKind, UsageKind};

use super::SymbolTablePopulator;

impl<'a> SymbolTablePopulator<'a> {
    pub(super) fn qualified_name(&self, name: &str) -> String {
        if self.current_namespace.is_empty() {
            name.to_string()
        } else {
            format!("{}::{}", self.current_namespace.join("::"), name)
        }
    }

    pub(super) fn enter_namespace(&mut self, name: String) {
        self.current_namespace.push(name);
        self.symbol_table.enter_scope();
    }

    pub(super) fn exit_namespace(&mut self) {
        self.current_namespace.pop();
        self.symbol_table.exit_scope();
    }

    pub(super) fn insert_symbol(&mut self, name: String, symbol: Symbol) {
        if let Err(e) = self.symbol_table.insert(name.clone(), symbol) {
            self.errors
                .push(SemanticError::duplicate_definition(name, None));
            if let Some(last_error) = self.errors.last_mut() {
                last_error.message = e;
            }
        }
    }

    pub(super) fn map_definition_kind(kind: &DefinitionKind) -> String {
        match kind {
            DefinitionKind::Part => "Part".to_string(),
            DefinitionKind::Port => "Port".to_string(),
            DefinitionKind::Action => "Action".to_string(),
            DefinitionKind::State => "State".to_string(),
            DefinitionKind::Item => "Item".to_string(),
            DefinitionKind::Attribute => "Attribute".to_string(),
            DefinitionKind::Requirement => "Requirement".to_string(),
            DefinitionKind::Concern => "UseCase".to_string(),
            DefinitionKind::Case => "UseCase".to_string(),
            DefinitionKind::AnalysisCase => "UseCase".to_string(),
            DefinitionKind::VerificationCase => "VerificationCase".to_string(),
            DefinitionKind::UseCase => "UseCase".to_string(),
            DefinitionKind::View => "View".to_string(),
            DefinitionKind::Viewpoint => "Viewpoint".to_string(),
            DefinitionKind::Rendering => "Rendering".to_string(),
        }
    }

    pub(super) fn map_usage_kind(kind: &UsageKind) -> String {
        match kind {
            UsageKind::Part => "Part".to_string(),
            UsageKind::Port => "Port".to_string(),
            UsageKind::Action => "Action".to_string(),
            UsageKind::Item => "Item".to_string(),
            UsageKind::Attribute => "Attribute".to_string(),
            UsageKind::Requirement => "Requirement".to_string(),
            UsageKind::Concern => "Concern".to_string(),
            UsageKind::Case => "Case".to_string(),
            UsageKind::View => "View".to_string(),
            UsageKind::SatisfyRequirement => "SatisfyRequirement".to_string(),
            UsageKind::PerformAction => "PerformAction".to_string(),
            UsageKind::ExhibitState => "ExhibitState".to_string(),
            UsageKind::IncludeUseCase => "IncludeUseCase".to_string(),
        }
    }
}
