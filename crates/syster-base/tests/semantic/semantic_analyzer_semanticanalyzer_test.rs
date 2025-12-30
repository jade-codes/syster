#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use syster::semantic::analyzer::SemanticAnalyzer;
use syster::semantic::graphs::RelationshipGraph;
use syster::semantic::symbol_table::{Symbol, SymbolTable};

// Tests for relationship_graph() method

#[test]
fn test_relationship_graph_returns_reference() {
    let analyzer = SemanticAnalyzer::new();
    let graph = analyzer.relationship_graph();

    // Verify that we get a reference to RelationshipGraph
    // For an empty graph, querying any relationship should return None
    assert!(
        graph
            .get_one_to_many("specialization", "NonExistent")
            .is_none()
    );
}

#[test]
fn test_relationship_graph_with_empty_graph() {
    let table = SymbolTable::new();
    let graph = RelationshipGraph::new();
    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);

    let retrieved_graph = analyzer.relationship_graph();

    // Empty graph should have no relationships
    assert!(
        retrieved_graph
            .get_one_to_many("specialization", "A")
            .is_none()
    );
    assert!(retrieved_graph.get_one_to_one("typing", "B").is_none());
}

#[test]
fn test_relationship_graph_with_specialization_relationships() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add symbols
    table
        .insert(
            "Vehicle".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Vehicle".to_string(),
                qualified_name: "Vehicle".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table
        .insert(
            "Car".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Car".to_string(),
                qualified_name: "Car".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Add relationship: Car specializes Vehicle
    graph.add_one_to_many(
        "specialization",
        "Car".to_string(),
        "Vehicle".to_string(),
        None,
    );

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let retrieved_graph = analyzer.relationship_graph();

    // Verify relationship exists
    let specializations = retrieved_graph.get_one_to_many("specialization", "Car");
    assert!(specializations.is_some());
    let targets = specializations.unwrap();
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0], "Vehicle");
}

#[test]
fn test_relationship_graph_with_typing_relationships() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add a definition
    table
        .insert(
            "VehicleDef".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "VehicleDef".to_string(),
                qualified_name: "VehicleDef".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Add a usage
    table
        .insert(
            "myVehicle".to_string(),
            Symbol::Usage {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "myVehicle".to_string(),
                qualified_name: "myVehicle".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: None,
            },
        )
        .unwrap();

    // Add typing relationship
    graph.add_one_to_one(
        "typing",
        "myVehicle".to_string(),
        "VehicleDef".to_string(),
        None,
    );

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let retrieved_graph = analyzer.relationship_graph();

    // Verify typing relationship exists
    let typing = retrieved_graph.get_one_to_one("typing", "myVehicle");
    assert!(typing.is_some());
    assert_eq!(typing.unwrap(), "VehicleDef");
}

#[test]
fn test_relationship_graph_with_multiple_relationship_types() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add symbols
    table
        .insert(
            "Base".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Base".to_string(),
                qualified_name: "Base".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table
        .insert(
            "Derived".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Derived".to_string(),
                qualified_name: "Derived".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table
        .insert(
            "instance".to_string(),
            Symbol::Usage {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "instance".to_string(),
                qualified_name: "instance".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: None,
            },
        )
        .unwrap();

    // Add multiple relationship types
    graph.add_one_to_many(
        "specialization",
        "Derived".to_string(),
        "Base".to_string(),
        None,
    );
    graph.add_one_to_one(
        "typing",
        "instance".to_string(),
        "Derived".to_string(),
        None,
    );
    graph.add_one_to_many(
        "subsetting",
        "instance".to_string(),
        "Derived".to_string(),
        None,
    );

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let retrieved_graph = analyzer.relationship_graph();

    // Verify all relationships exist
    assert!(
        retrieved_graph
            .get_one_to_many("specialization", "Derived")
            .is_some()
    );
    assert!(
        retrieved_graph
            .get_one_to_one("typing", "instance")
            .is_some()
    );
    assert!(
        retrieved_graph
            .get_one_to_many("subsetting", "instance")
            .is_some()
    );
}

#[test]
fn test_relationship_graph_immutability() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    table
        .insert(
            "Test".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Test".to_string(),
                qualified_name: "Test".to_string(),
            },
        )
        .unwrap();

    graph.add_one_to_many("specialization", "A".to_string(), "B".to_string(), None);

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);

    // Get multiple references to the graph
    let graph1 = analyzer.relationship_graph();
    let graph2 = analyzer.relationship_graph();

    // Both should point to the same data
    let count1 = graph1
        .get_one_to_many("specialization", "A")
        .map(|v| v.len())
        .unwrap_or(0);
    let count2 = graph2
        .get_one_to_many("specialization", "A")
        .map(|v| v.len())
        .unwrap_or(0);
    assert_eq!(count1, count2);
}

#[test]
fn test_relationship_graph_with_no_relationships_for_source() {
    let mut graph = RelationshipGraph::new();
    graph.add_one_to_many("specialization", "A".to_string(), "B".to_string(), None);

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(SymbolTable::new(), graph);
    let retrieved_graph = analyzer.relationship_graph();

    // Query for a source that has no relationships
    assert!(
        retrieved_graph
            .get_one_to_many("specialization", "NonExistent")
            .is_none()
    );
    assert!(
        retrieved_graph
            .get_one_to_one("typing", "NonExistent")
            .is_none()
    );
}

// Tests for relationship_graph_mut() method

#[test]
fn test_relationship_graph_mut_returns_mutable_reference() {
    let mut analyzer = SemanticAnalyzer::new();

    // Get mutable reference and add a relationship
    let graph_mut = analyzer.relationship_graph_mut();
    graph_mut.add_one_to_many(
        "specialization",
        "Child".to_string(),
        "Parent".to_string(),
        None,
    );

    // Verify the change persisted
    let graph = analyzer.relationship_graph();
    let result = graph.get_one_to_many("specialization", "Child");
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 1);
}

#[test]
fn test_relationship_graph_mut_can_add_specialization() {
    let mut analyzer = SemanticAnalyzer::new();

    // Add symbols to symbol table
    analyzer
        .symbol_table_mut()
        .insert(
            "Parent".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Parent".to_string(),
                qualified_name: "Parent".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    analyzer
        .symbol_table_mut()
        .insert(
            "Child".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Child".to_string(),
                qualified_name: "Child".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Add specialization through mutable reference
    analyzer.relationship_graph_mut().add_one_to_many(
        "specialization",
        "Child".to_string(),
        "Parent".to_string(),
        None,
    );

    // Verify through immutable reference
    let specializations = analyzer
        .relationship_graph()
        .get_one_to_many("specialization", "Child");
    assert!(specializations.is_some());
    let targets = specializations.unwrap();
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0], "Parent");
}

#[test]
fn test_relationship_graph_mut_can_add_typing() {
    let mut analyzer = SemanticAnalyzer::new();

    // Add definition and usage to symbol table
    analyzer
        .symbol_table_mut()
        .insert(
            "TypeDef".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "TypeDef".to_string(),
                qualified_name: "TypeDef".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    analyzer
        .symbol_table_mut()
        .insert(
            "instance".to_string(),
            Symbol::Usage {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "instance".to_string(),
                qualified_name: "instance".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: None,
            },
        )
        .unwrap();

    // Add typing through mutable reference
    analyzer.relationship_graph_mut().add_one_to_one(
        "typing",
        "instance".to_string(),
        "TypeDef".to_string(),
        None,
    );

    // Verify through immutable reference
    let typing = analyzer
        .relationship_graph()
        .get_one_to_one("typing", "instance");
    assert!(typing.is_some());
    assert_eq!(typing.unwrap(), "TypeDef");
}

#[test]
fn test_relationship_graph_mut_can_add_multiple_relationships() {
    let mut analyzer = SemanticAnalyzer::new();

    // Add relationships one by one
    let graph_mut = analyzer.relationship_graph_mut();
    graph_mut.add_one_to_many("specialization", "A".to_string(), "B".to_string(), None);
    graph_mut.add_one_to_many("specialization", "C".to_string(), "D".to_string(), None);
    graph_mut.add_one_to_one("typing", "X".to_string(), "Y".to_string(), None);

    // Verify all were added
    let graph = analyzer.relationship_graph();
    assert!(graph.get_one_to_many("specialization", "A").is_some());
    assert!(graph.get_one_to_many("specialization", "C").is_some());
    assert!(graph.get_one_to_one("typing", "X").is_some());
}

#[test]
fn test_relationship_graph_mut_can_modify_existing_graph() {
    let table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Start with one relationship
    graph.add_one_to_many(
        "specialization",
        "Initial".to_string(),
        "Base".to_string(),
        None,
    );

    let mut analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);

    // Verify initial state
    assert!(
        analyzer
            .relationship_graph()
            .get_one_to_many("specialization", "Initial")
            .is_some()
    );

    // Add more relationships
    analyzer.relationship_graph_mut().add_one_to_many(
        "specialization",
        "Added".to_string(),
        "Base".to_string(),
        None,
    );

    // Verify modification - both relationships should exist
    assert!(
        analyzer
            .relationship_graph()
            .get_one_to_many("specialization", "Initial")
            .is_some()
    );
    assert!(
        analyzer
            .relationship_graph()
            .get_one_to_many("specialization", "Added")
            .is_some()
    );
}

#[test]
fn test_relationship_graph_mut_can_add_subsetting() {
    let mut analyzer = SemanticAnalyzer::new();

    // Add subsetting relationship
    analyzer.relationship_graph_mut().add_one_to_many(
        "subsetting",
        "Refined".to_string(),
        "Original".to_string(),
        None,
    );

    // Verify
    let subsettings = analyzer
        .relationship_graph()
        .get_one_to_many("subsetting", "Refined");
    assert!(subsettings.is_some());
    let targets = subsettings.unwrap();
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0], "Original");
}

#[test]
fn test_relationship_graph_mut_can_add_redefinition() {
    let mut analyzer = SemanticAnalyzer::new();

    // Add redefinition relationship
    analyzer.relationship_graph_mut().add_one_to_many(
        "redefinition",
        "Override".to_string(),
        "Original".to_string(),
        None,
    );

    // Verify
    let redefinitions = analyzer
        .relationship_graph()
        .get_one_to_many("redefinition", "Override");
    assert!(redefinitions.is_some());
    let targets = redefinitions.unwrap();
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0], "Original");
}

#[test]
fn test_relationship_graph_mut_changes_persist() {
    let mut analyzer = SemanticAnalyzer::new();

    // Make a change
    {
        let graph_mut = analyzer.relationship_graph_mut();
        graph_mut.add_one_to_many("specialization", "A".to_string(), "B".to_string(), None);
    } // Mutable reference dropped

    // Verify change persists after mutable reference is dropped
    let graph = analyzer.relationship_graph();
    assert!(graph.get_one_to_many("specialization", "A").is_some());

    // Make another change
    {
        let graph_mut = analyzer.relationship_graph_mut();
        graph_mut.add_one_to_many("specialization", "C".to_string(), "D".to_string(), None);
    }

    // Verify both changes persist
    let graph = analyzer.relationship_graph();
    assert!(graph.get_one_to_many("specialization", "A").is_some());
    assert!(graph.get_one_to_many("specialization", "C").is_some());
}

#[test]
fn test_relationship_graph_mut_with_empty_initial_graph() {
    let mut analyzer = SemanticAnalyzer::new();

    // Verify empty initially
    assert!(
        analyzer
            .relationship_graph()
            .get_one_to_many("specialization", "A")
            .is_none()
    );

    // Add relationship
    analyzer.relationship_graph_mut().add_one_to_many(
        "specialization",
        "A".to_string(),
        "B".to_string(),
        None,
    );

    // Verify it was added to initially empty graph
    assert!(
        analyzer
            .relationship_graph()
            .get_one_to_many("specialization", "A")
            .is_some()
    );
}

#[test]
fn test_relationship_graph_mut_multiple_consecutive_mutations() {
    let mut analyzer = SemanticAnalyzer::new();

    // Perform multiple mutations in sequence
    analyzer.relationship_graph_mut().add_one_to_many(
        "specialization",
        "A".to_string(),
        "B".to_string(),
        None,
    );

    analyzer.relationship_graph_mut().add_one_to_many(
        "specialization",
        "B".to_string(),
        "C".to_string(),
        None,
    );

    analyzer.relationship_graph_mut().add_one_to_one(
        "typing",
        "X".to_string(),
        "Y".to_string(),
        None,
    );

    // Verify all mutations applied
    let graph = analyzer.relationship_graph();
    assert!(graph.get_one_to_many("specialization", "A").is_some());
    assert!(graph.get_one_to_many("specialization", "B").is_some());
    assert!(graph.get_one_to_one("typing", "X").is_some());
}

#[test]
fn test_relationship_graph_mut_can_add_multiple_targets_to_same_source() {
    let mut analyzer = SemanticAnalyzer::new();

    // Add multiple specialization targets for the same source
    analyzer.relationship_graph_mut().add_one_to_many(
        "specialization",
        "Child".to_string(),
        "Parent1".to_string(),
        None,
    );

    analyzer.relationship_graph_mut().add_one_to_many(
        "specialization",
        "Child".to_string(),
        "Parent2".to_string(),
        None,
    );

    // Verify both targets exist
    let specializations = analyzer
        .relationship_graph()
        .get_one_to_many("specialization", "Child");
    assert!(specializations.is_some());
    let targets = specializations.unwrap();
    assert_eq!(targets.len(), 2);
    assert!(targets.contains(&&"Parent1".to_string()));
    assert!(targets.contains(&&"Parent2".to_string()));
}

// Edge case tests

#[test]
fn test_relationship_graph_with_qualified_names() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add symbols with qualified names
    table
        .insert(
            "Pkg::Base".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Base".to_string(),
                qualified_name: "Pkg::Base".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table
        .insert(
            "Pkg::Derived".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Derived".to_string(),
                qualified_name: "Pkg::Derived".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Add relationship with qualified names
    graph.add_one_to_many(
        "specialization",
        "Pkg::Derived".to_string(),
        "Pkg::Base".to_string(),
        None,
    );

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let retrieved_graph = analyzer.relationship_graph();

    // Verify relationship with qualified names
    let specializations = retrieved_graph.get_one_to_many("specialization", "Pkg::Derived");
    assert!(specializations.is_some());
    assert_eq!(specializations.unwrap()[0], "Pkg::Base");
}

#[test]
fn test_relationship_graph_mut_with_different_relationship_kinds() {
    let mut analyzer = SemanticAnalyzer::new();

    let graph_mut = analyzer.relationship_graph_mut();

    // Add various SysML/KerML relationship kinds
    graph_mut.add_one_to_many("specialization", "A".to_string(), "B".to_string(), None);
    graph_mut.add_one_to_many("subsetting", "C".to_string(), "D".to_string(), None);
    graph_mut.add_one_to_many("redefinition", "E".to_string(), "F".to_string(), None);
    graph_mut.add_one_to_one("typing", "G".to_string(), "H".to_string(), None);
    graph_mut.add_one_to_many("satisfy", "I".to_string(), "J".to_string(), None);
    graph_mut.add_one_to_many("perform", "K".to_string(), "L".to_string(), None);

    // Verify all different kinds were added
    let graph = analyzer.relationship_graph();
    assert!(graph.get_one_to_many("specialization", "A").is_some());
    assert!(graph.get_one_to_many("subsetting", "C").is_some());
    assert!(graph.get_one_to_many("redefinition", "E").is_some());
    assert!(graph.get_one_to_one("typing", "G").is_some());
    assert!(graph.get_one_to_many("satisfy", "I").is_some());
    assert!(graph.get_one_to_many("perform", "K").is_some());
}

#[test]
fn test_relationship_graph_after_analyzer_with_validator() {
    use syster::semantic::create_validator;

    let table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add a relationship
    graph.add_one_to_many(
        "specialization",
        "Child".to_string(),
        "Parent".to_string(),
        None,
    );

    let validator = create_validator("sysml");
    let analyzer = SemanticAnalyzer::with_validator(table, graph, validator);

    // Verify relationship is accessible
    let retrieved_graph = analyzer.relationship_graph();
    let result = retrieved_graph.get_one_to_many("specialization", "Child");
    assert!(result.is_some());
}

#[test]
fn test_relationship_graph_query_different_relationship_types() {
    let mut graph = RelationshipGraph::new();

    // Add different types of relationships
    graph.add_one_to_many("specialization", "A".to_string(), "B".to_string(), None);
    graph.add_one_to_one("typing", "X".to_string(), "Y".to_string(), None);

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(SymbolTable::new(), graph);
    let retrieved_graph = analyzer.relationship_graph();

    // Verify we can query different types independently
    assert!(
        retrieved_graph
            .get_one_to_many("specialization", "A")
            .is_some()
    );
    assert!(retrieved_graph.get_one_to_one("typing", "X").is_some());

    // Verify wrong type/source combinations return None
    assert!(retrieved_graph.get_one_to_many("typing", "X").is_none());
    assert!(
        retrieved_graph
            .get_one_to_one("specialization", "A")
            .is_none()
    );
}

#[test]
fn test_relationship_graph_mut_modify_then_query() {
    let mut analyzer = SemanticAnalyzer::new();

    // Add a relationship
    analyzer.relationship_graph_mut().add_one_to_many(
        "specialization",
        "Child".to_string(),
        "Parent".to_string(),
        None,
    );

    // Query immediately after modification
    let result = analyzer
        .relationship_graph()
        .get_one_to_many("specialization", "Child");
    assert!(result.is_some());
    assert_eq!(result.unwrap()[0], "Parent");

    // Add another relationship
    analyzer.relationship_graph_mut().add_one_to_many(
        "specialization",
        "GrandChild".to_string(),
        "Child".to_string(),
        None,
    );

    // Query again
    let result2 = analyzer
        .relationship_graph()
        .get_one_to_many("specialization", "GrandChild");
    assert!(result2.is_some());
    assert_eq!(result2.unwrap()[0], "Child");

    // Original relationship should still exist
    let result3 = analyzer
        .relationship_graph()
        .get_one_to_many("specialization", "Child");
    assert!(result3.is_some());
}
