use crate::semantic::symbol_table::Symbol;
use crate::semantic::types::{SemanticError, SemanticRole};
use crate::syntax::sysml::ast::{DefinitionKind, UsageKind};

use crate::semantic::adapters::SysmlAdapter;

impl<'a> SysmlAdapter<'a> {
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
            DefinitionKind::AnalysisCase => "AnalysisCase".to_string(),
            DefinitionKind::VerificationCase => "VerificationCase".to_string(),
            DefinitionKind::UseCase => "UseCase".to_string(),
            DefinitionKind::View => "View".to_string(),
            DefinitionKind::Viewpoint => "Viewpoint".to_string(),
            DefinitionKind::Allocation => "Allocation".to_string(),
            DefinitionKind::Calculation => "Calculation".to_string(),
            DefinitionKind::Connection => "Connection".to_string(),
            DefinitionKind::Constraint => "Constraint".to_string(),
            DefinitionKind::Enumeration => "Enumeration".to_string(),
            DefinitionKind::Flow => "Flow".to_string(),
            DefinitionKind::Individual => "Individual".to_string(),
            DefinitionKind::Interface => "Interface".to_string(),
            DefinitionKind::Occurrence => "Occurrence".to_string(),
            DefinitionKind::Metadata => "Metadata".to_string(),
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
            UsageKind::Enumeration => "Enumeration".to_string(),
            UsageKind::Occurrence => "Occurrence".to_string(),
            UsageKind::Individual => "Individual".to_string(),
            UsageKind::Snapshot => "Snapshot".to_string(),
            UsageKind::Timeslice => "Timeslice".to_string(),
            UsageKind::SatisfyRequirement => "SatisfyRequirement".to_string(),
            UsageKind::PerformAction => "PerformAction".to_string(),
            UsageKind::ExhibitState => "ExhibitState".to_string(),
            UsageKind::IncludeUseCase => "IncludeUseCase".to_string(),
            UsageKind::Reference => "Reference".to_string(),
            UsageKind::Constraint => "Constraint".to_string(),
            UsageKind::Calculation => "Calculation".to_string(),
            UsageKind::State => "State".to_string(),
            UsageKind::Connection => "Connection".to_string(),
            UsageKind::Interface => "Interface".to_string(),
            UsageKind::Allocation => "Allocation".to_string(),
            UsageKind::Flow => "Flow".to_string(),
        }
    }

    /// Maps a SysML DefinitionKind directly to a semantic role.
    /// This is the ONLY place where language-specific AST types are translated to semantic concepts.
    pub(super) fn definition_kind_to_semantic_role(kind: &DefinitionKind) -> SemanticRole {
        match kind {
            DefinitionKind::Requirement => SemanticRole::Requirement,
            DefinitionKind::Action => SemanticRole::Action,
            DefinitionKind::State => SemanticRole::State,
            DefinitionKind::UseCase | DefinitionKind::Case | DefinitionKind::Concern => {
                SemanticRole::UseCase
            }
            DefinitionKind::VerificationCase => SemanticRole::VerificationCase,
            DefinitionKind::AnalysisCase => SemanticRole::AnalysisCase,
            DefinitionKind::Part => SemanticRole::Component,
            DefinitionKind::Port => SemanticRole::Port,
            DefinitionKind::Attribute => SemanticRole::Attribute,
            DefinitionKind::Item => SemanticRole::Item,
            DefinitionKind::View => SemanticRole::View,
            DefinitionKind::Viewpoint => SemanticRole::Metadata,
            DefinitionKind::Rendering => SemanticRole::View,
            DefinitionKind::Allocation => SemanticRole::Component,
            DefinitionKind::Calculation => SemanticRole::Action,
            DefinitionKind::Connection => SemanticRole::Component,
            DefinitionKind::Constraint => SemanticRole::Constraint,
            DefinitionKind::Enumeration => SemanticRole::Component,
            DefinitionKind::Flow => SemanticRole::Component,
            DefinitionKind::Individual => SemanticRole::Component,
            DefinitionKind::Interface => SemanticRole::Interface,
            DefinitionKind::Occurrence => SemanticRole::Component,
            DefinitionKind::Metadata => SemanticRole::Metadata,
        }
    }

    /// Maps a SysML UsageKind directly to a semantic role.
    pub(super) fn usage_kind_to_semantic_role(kind: &UsageKind) -> SemanticRole {
        match kind {
            UsageKind::Requirement => SemanticRole::Requirement,
            UsageKind::Action => SemanticRole::Action,
            UsageKind::Part => SemanticRole::Component,
            UsageKind::Port => SemanticRole::Port,
            UsageKind::Attribute => SemanticRole::Attribute,
            UsageKind::Item => SemanticRole::Item,
            UsageKind::View => SemanticRole::View,
            UsageKind::Concern => SemanticRole::UseCase,
            UsageKind::Case => SemanticRole::UseCase,
            UsageKind::Enumeration => SemanticRole::Item, // Treat enum values as items
            // Domain relationship usages map to their target roles
            UsageKind::SatisfyRequirement => SemanticRole::Requirement,
            UsageKind::PerformAction => SemanticRole::Action,
            UsageKind::ExhibitState => SemanticRole::State,
            UsageKind::IncludeUseCase => SemanticRole::UseCase,
            // Occurrence-based usages
            UsageKind::Occurrence => SemanticRole::Component,
            UsageKind::Individual => SemanticRole::Component,
            UsageKind::Snapshot => SemanticRole::Component,
            UsageKind::Timeslice => SemanticRole::Component,
            // Reference usages (parameters with direction like `in`, `out`, `inout`)
            UsageKind::Reference => SemanticRole::Attribute,
            // Additional usage types
            UsageKind::Constraint => SemanticRole::Constraint,
            UsageKind::Calculation => SemanticRole::Action,
            UsageKind::State => SemanticRole::State,
            UsageKind::Connection => SemanticRole::Component,
            UsageKind::Interface => SemanticRole::Interface,
            UsageKind::Allocation => SemanticRole::Component,
            UsageKind::Flow => SemanticRole::Component,
        }
    }
}
