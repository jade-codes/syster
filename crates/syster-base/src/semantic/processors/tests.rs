use super::*;
use crate::core::constants::{
    REL_REDEFINITION, REL_REFERENCE_SUBSETTING, REL_SPECIALIZATION, REL_SUBSETTING, REL_TYPING,
};
use crate::core::{Position, Span};
use crate::language::sysml::syntax::{
    Alias, Definition, DefinitionKind, Element, Package, Relationships, SysMLFile, Usage, UsageKind,
};
use crate::semantic::graphs::RelationshipGraph;
use crate::semantic::processors::{NoOpValidator, RelationshipValidator};
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use crate::semantic::workspace::WorkspaceFile;
use std::path::PathBuf;

#[test]
fn test_noop_validator_accepts_all_relationships() {
    let validator = NoOpValidator;
    let source = Symbol::Package {
        name: "Source".to_string(),
        qualified_name: "Source".to_string(),
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
    };
    let target = Symbol::Package {
        name: "Target".to_string(),
        qualified_name: "Target".to_string(),
        scope_id: 0,
        source_file: None,
        span: None,
        references: Vec::new(),
    };

    let result = validator.validate_relationship("any_type", &source, &target);
    assert!(result.is_ok());
}

#[test]
fn test_noop_validator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<NoOpValidator>();
}

#[test]
fn test_typing_relationship_reference() {
    let mut table = SymbolTable::new();

    // Create a classifier
    table
        .insert(
            "Vehicle".to_string(),
            Symbol::Classifier {
                name: "Vehicle".to_string(),
                qualified_name: "Vehicle".to_string(),
                kind: "class".to_string(),
                is_abstract: false,
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 1, column: 0 },
                    end: Position {
                        line: 1,
                        column: 20,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create a usage that types by Vehicle
    table
        .insert(
            "myCar".to_string(),
            Symbol::Usage {
                name: "myCar".to_string(),
                qualified_name: "myCar".to_string(),
                kind: "part".to_string(),
                usage_type: None,
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 5, column: 0 },
                    end: Position {
                        line: 5,
                        column: 20,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create relationship graph with typing relationship
    let mut graph = RelationshipGraph::new();
    graph.add_one_to_one(REL_TYPING, "myCar".to_string(), "Vehicle".to_string());

    // Collect references
    let mut collector = ReferenceCollector::new(&mut table, &graph);
    collector.collect();

    // Verify Vehicle has a reference from myCar
    let vehicle = table.lookup("Vehicle").unwrap();
    assert_eq!(vehicle.references().len(), 1);
    assert_eq!(vehicle.references()[0].span.start.line, 5);
    assert_eq!(vehicle.references()[0].span.start.column, 0);
}

#[test]
fn test_specialization_relationship_reference() {
    let mut table = SymbolTable::new();

    // Create base classifier
    table
        .insert(
            "Vehicle".to_string(),
            Symbol::Classifier {
                name: "Vehicle".to_string(),
                qualified_name: "Vehicle".to_string(),
                kind: "class".to_string(),
                is_abstract: false,
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 1, column: 0 },
                    end: Position {
                        line: 1,
                        column: 20,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create specialized classifier
    table
        .insert(
            "Car".to_string(),
            Symbol::Classifier {
                name: "Car".to_string(),
                qualified_name: "Car".to_string(),
                kind: "class".to_string(),
                is_abstract: false,
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 3, column: 0 },
                    end: Position {
                        line: 3,
                        column: 30,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create specialization relationship
    let mut graph = RelationshipGraph::new();
    graph.add_one_to_many(REL_SPECIALIZATION, "Car".to_string(), "Vehicle".to_string());

    // Collect references
    let mut collector = ReferenceCollector::new(&mut table, &graph);
    collector.collect();

    // Verify Vehicle has a reference from Car
    let vehicle = table.lookup("Vehicle").unwrap();
    assert_eq!(vehicle.references().len(), 1);
    assert_eq!(vehicle.references()[0].span.start.line, 3);
}

#[test]
fn test_multiple_references_to_same_symbol() {
    let mut table = SymbolTable::new();

    // Create base type
    table
        .insert(
            "Integer".to_string(),
            Symbol::Classifier {
                name: "Integer".to_string(),
                qualified_name: "Integer".to_string(),
                kind: "datatype".to_string(),
                is_abstract: false,
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 1, column: 0 },
                    end: Position {
                        line: 1,
                        column: 15,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create multiple usages that reference Integer
    table
        .insert(
            "speed".to_string(),
            Symbol::Feature {
                name: "speed".to_string(),
                qualified_name: "speed".to_string(),
                feature_type: Some("Integer".to_string()),
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 5, column: 4 },
                    end: Position {
                        line: 5,
                        column: 25,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    table
        .insert(
            "count".to_string(),
            Symbol::Feature {
                name: "count".to_string(),
                qualified_name: "count".to_string(),
                feature_type: Some("Integer".to_string()),
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 6, column: 4 },
                    end: Position {
                        line: 6,
                        column: 25,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    table
        .insert(
            "index".to_string(),
            Symbol::Feature {
                name: "index".to_string(),
                qualified_name: "index".to_string(),
                feature_type: Some("Integer".to_string()),
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 7, column: 4 },
                    end: Position {
                        line: 7,
                        column: 25,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create typing relationships
    let mut graph = RelationshipGraph::new();
    graph.add_one_to_one(REL_TYPING, "speed".to_string(), "Integer".to_string());
    graph.add_one_to_one(REL_TYPING, "count".to_string(), "Integer".to_string());
    graph.add_one_to_one(REL_TYPING, "index".to_string(), "Integer".to_string());

    // Collect references
    let mut collector = ReferenceCollector::new(&mut table, &graph);
    collector.collect();

    // Verify Integer has references from all three features
    let integer = table.lookup("Integer").unwrap();
    assert_eq!(integer.references().len(), 3);

    let lines: Vec<_> = integer
        .references()
        .iter()
        .map(|r| r.span.start.line)
        .collect();
    assert!(lines.contains(&5));
    assert!(lines.contains(&6));
    assert!(lines.contains(&7));
}

#[test]
fn test_redefinition_reference() {
    let mut table = SymbolTable::new();

    // Create base feature
    table
        .insert(
            "Vehicle::mass".to_string(),
            Symbol::Feature {
                name: "mass".to_string(),
                qualified_name: "Vehicle::mass".to_string(),
                feature_type: Some("Real".to_string()),
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 2, column: 4 },
                    end: Position {
                        line: 2,
                        column: 20,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create redefining feature
    table
        .insert(
            "Car::mass".to_string(),
            Symbol::Feature {
                name: "mass".to_string(),
                qualified_name: "Car::mass".to_string(),
                feature_type: Some("Real".to_string()),
                scope_id: 1,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 6, column: 4 },
                    end: Position {
                        line: 6,
                        column: 35,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create redefinition relationship
    let mut graph = RelationshipGraph::new();
    graph.add_one_to_many(
        REL_REDEFINITION,
        "Car::mass".to_string(),
        "Vehicle::mass".to_string(),
    );

    // Collect references
    let mut collector = ReferenceCollector::new(&mut table, &graph);
    collector.collect();

    // Verify Vehicle::mass has a reference from Car::mass
    let base_mass = table.lookup("Vehicle::mass").unwrap();
    assert_eq!(base_mass.references().len(), 1);
    assert_eq!(base_mass.references()[0].span.start.line, 6);
}

#[test]
fn test_subsetting_reference() {
    let mut table = SymbolTable::new();

    // Create general feature
    table
        .insert(
            "parts".to_string(),
            Symbol::Feature {
                name: "parts".to_string(),
                qualified_name: "parts".to_string(),
                feature_type: Some("Part".to_string()),
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 2, column: 0 },
                    end: Position {
                        line: 2,
                        column: 20,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create subsetting feature
    table
        .insert(
            "engineParts".to_string(),
            Symbol::Feature {
                name: "engineParts".to_string(),
                qualified_name: "engineParts".to_string(),
                feature_type: Some("EnginePart".to_string()),
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 4, column: 0 },
                    end: Position {
                        line: 4,
                        column: 30,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create subsetting relationship
    let mut graph = RelationshipGraph::new();
    graph.add_one_to_many(
        REL_SUBSETTING,
        "engineParts".to_string(),
        "parts".to_string(),
    );

    // Collect references
    let mut collector = ReferenceCollector::new(&mut table, &graph);
    collector.collect();

    // Verify parts has a reference from engineParts
    let parts = table.lookup("parts").unwrap();
    assert_eq!(parts.references().len(), 1);
    assert_eq!(parts.references()[0].span.start.line, 4);
}

#[test]
fn test_reference_subsetting() {
    let mut table = SymbolTable::new();

    // Create base reference
    table
        .insert(
            "vehicle".to_string(),
            Symbol::Usage {
                name: "vehicle".to_string(),
                qualified_name: "vehicle".to_string(),
                kind: "ref".to_string(),
                usage_type: None,
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 2, column: 0 },
                    end: Position {
                        line: 2,
                        column: 20,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create subsetting reference
    table
        .insert(
            "car".to_string(),
            Symbol::Usage {
                name: "car".to_string(),
                qualified_name: "car".to_string(),
                kind: "ref".to_string(),
                usage_type: None,
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 4, column: 0 },
                    end: Position {
                        line: 4,
                        column: 25,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create reference subsetting relationship
    let mut graph = RelationshipGraph::new();
    graph.add_one_to_many(
        REL_REFERENCE_SUBSETTING,
        "car".to_string(),
        "vehicle".to_string(),
    );

    // Collect references
    let mut collector = ReferenceCollector::new(&mut table, &graph);
    collector.collect();

    // Verify vehicle has a reference from car
    let vehicle = table.lookup("vehicle").unwrap();
    assert_eq!(vehicle.references().len(), 1);
    assert_eq!(vehicle.references()[0].span.start.line, 4);
}

#[test]
fn test_no_references() {
    let mut table = SymbolTable::new();

    // Create a standalone symbol with no relationships
    table
        .insert(
            "StandaloneClass".to_string(),
            Symbol::Classifier {
                name: "StandaloneClass".to_string(),
                qualified_name: "StandaloneClass".to_string(),
                kind: "class".to_string(),
                is_abstract: false,
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 1, column: 0 },
                    end: Position {
                        line: 1,
                        column: 30,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Empty relationship graph
    let graph = RelationshipGraph::new();

    // Collect references
    let mut collector = ReferenceCollector::new(&mut table, &graph);
    collector.collect();

    // Verify no references collected
    let standalone = table.lookup("StandaloneClass").unwrap();
    assert_eq!(standalone.references().len(), 0);
}

#[test]
fn test_symbol_without_span() {
    let mut table = SymbolTable::new();

    // Create target symbol
    table
        .insert(
            "Target".to_string(),
            Symbol::Classifier {
                name: "Target".to_string(),
                qualified_name: "Target".to_string(),
                kind: "class".to_string(),
                is_abstract: false,
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 1, column: 0 },
                    end: Position {
                        line: 1,
                        column: 20,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create source symbol without span
    table
        .insert(
            "Source".to_string(),
            Symbol::Usage {
                name: "Source".to_string(),
                qualified_name: "Source".to_string(),
                kind: "part".to_string(),
                usage_type: None,
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: None, // No span
                references: Vec::new(),
            },
        )
        .ok();

    // Create relationship
    let mut graph = RelationshipGraph::new();
    graph.add_one_to_one(REL_TYPING, "Source".to_string(), "Target".to_string());

    // Collect references
    let mut collector = ReferenceCollector::new(&mut table, &graph);
    collector.collect();

    // Verify no reference collected (source has no span)
    let target = table.lookup("Target").unwrap();
    assert_eq!(target.references().len(), 0);
}

#[test]
fn test_mixed_relationships() {
    let mut table = SymbolTable::new();

    // Create base type
    table
        .insert(
            "Base".to_string(),
            Symbol::Classifier {
                name: "Base".to_string(),
                qualified_name: "Base".to_string(),
                kind: "class".to_string(),
                is_abstract: false,
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 1, column: 0 },
                    end: Position {
                        line: 1,
                        column: 15,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create derived type (specialization)
    table
        .insert(
            "Derived".to_string(),
            Symbol::Classifier {
                name: "Derived".to_string(),
                qualified_name: "Derived".to_string(),
                kind: "class".to_string(),
                is_abstract: false,
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 3, column: 0 },
                    end: Position {
                        line: 3,
                        column: 25,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create usage (typing)
    table
        .insert(
            "instance".to_string(),
            Symbol::Usage {
                name: "instance".to_string(),
                qualified_name: "instance".to_string(),
                kind: "part".to_string(),
                usage_type: None,
                scope_id: 0,
                source_file: Some("model.sysml".to_string()),
                span: Some(Span {
                    start: Position { line: 5, column: 0 },
                    end: Position {
                        line: 5,
                        column: 20,
                    },
                }),
                references: Vec::new(),
            },
        )
        .ok();

    // Create multiple relationship types
    let mut graph = RelationshipGraph::new();
    graph.add_one_to_many(
        REL_SPECIALIZATION,
        "Derived".to_string(),
        "Base".to_string(),
    );
    graph.add_one_to_one(REL_TYPING, "instance".to_string(), "Base".to_string());

    // Collect references
    let mut collector = ReferenceCollector::new(&mut table, &graph);
    collector.collect();

    // Verify Base has references from both relationships
    let base = table.lookup("Base").unwrap();
    assert_eq!(base.references().len(), 2);

    let lines: Vec<_> = base
        .references()
        .iter()
        .map(|r| r.span.start.line)
        .collect();
    assert!(lines.contains(&3)); // from Derived
    assert!(lines.contains(&5)); // from instance
}

#[test]
fn test_collect_package_tokens() {
    let pkg = Package {
        name: Some("TestPackage".to_string()),
        span: Some(Span::new(Position::new(0, 8), Position::new(0, 19))),
        elements: vec![],
    };
    let sysml_file = SysMLFile {
        namespace: None,
        elements: vec![Element::Package(pkg)],
    };
    let workspace_file = WorkspaceFile::new(
        PathBuf::from("test.sysml"),
        crate::language::LanguageFile::SysML(sysml_file),
    );

    let tokens = SemanticTokenCollector::collect(&workspace_file);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].line, 0);
    assert_eq!(tokens[0].column, 8);
    assert_eq!(tokens[0].length, 11);
    assert_eq!(tokens[0].token_type, TokenType::Namespace);
}

#[test]
fn test_collect_definition_tokens() {
    let mut def = Definition::new(
        DefinitionKind::Part,
        Some("MyDef".to_string()),
        Relationships::default(),
        vec![],
    );
    def.span = Some(Span::new(Position::new(1, 4), Position::new(1, 9)));

    let sysml_file = SysMLFile {
        namespace: None,
        elements: vec![Element::Definition(def)],
    };
    let workspace_file = WorkspaceFile::new(
        PathBuf::from("test.sysml"),
        crate::language::LanguageFile::SysML(sysml_file),
    );

    let tokens = SemanticTokenCollector::collect(&workspace_file);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Type);
}

#[test]
fn test_collect_usage_tokens() {
    let mut usage = Usage::new(
        UsageKind::Part,
        Some("myUsage".to_string()),
        Relationships::default(),
        vec![],
    );
    usage.span = Some(Span::new(Position::new(2, 4), Position::new(2, 11)));

    let sysml_file = SysMLFile {
        namespace: None,
        elements: vec![Element::Usage(usage)],
    };
    let workspace_file = WorkspaceFile::new(
        PathBuf::from("test.sysml"),
        crate::language::LanguageFile::SysML(sysml_file),
    );

    let tokens = SemanticTokenCollector::collect(&workspace_file);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Property);
}

#[test]
fn test_collect_alias_tokens() {
    let alias = Alias {
        name: Some("myAlias".to_string()),
        target: "SomeTarget".to_string(),
        span: Some(Span::new(Position::new(3, 6), Position::new(3, 13))),
    };
    let sysml_file = SysMLFile {
        namespace: None,
        elements: vec![Element::Alias(alias)],
    };
    let workspace_file = WorkspaceFile::new(
        PathBuf::from("test.sysml"),
        crate::language::LanguageFile::SysML(sysml_file),
    );

    let tokens = SemanticTokenCollector::collect(&workspace_file);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Variable);
}

#[test]
fn test_collect_nested_tokens() {
    let mut inner_def = Definition::new(
        DefinitionKind::Part,
        Some("Inner".to_string()),
        Relationships::default(),
        vec![],
    );
    inner_def.span = Some(Span::new(Position::new(1, 4), Position::new(1, 9)));

    let pkg = Package {
        name: Some("Outer".to_string()),
        span: Some(Span::new(Position::new(0, 8), Position::new(0, 13))),
        elements: vec![Element::Definition(inner_def)],
    };
    let sysml_file = SysMLFile {
        namespace: None,
        elements: vec![Element::Package(pkg)],
    };
    let workspace_file = WorkspaceFile::new(
        PathBuf::from("test.sysml"),
        crate::language::LanguageFile::SysML(sysml_file),
    );

    let tokens = SemanticTokenCollector::collect(&workspace_file);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Namespace); // Outer
    assert_eq!(tokens[1].token_type, TokenType::Type); // Inner
}

#[test]
fn test_tokens_sorted_by_position() {
    let mut def1 = Definition::new(
        DefinitionKind::Part,
        Some("First".to_string()),
        Relationships::default(),
        vec![],
    );
    def1.span = Some(Span::new(Position::new(1, 0), Position::new(1, 5)));

    let mut def2 = Definition::new(
        DefinitionKind::Part,
        Some("Second".to_string()),
        Relationships::default(),
        vec![],
    );
    def2.span = Some(Span::new(Position::new(2, 0), Position::new(2, 6)));

    let sysml_file = SysMLFile {
        namespace: None,
        // Elements should be in document order (AST order)
        elements: vec![Element::Definition(def1), Element::Definition(def2)],
    };
    let workspace_file = WorkspaceFile::new(
        PathBuf::from("test.sysml"),
        crate::language::LanguageFile::SysML(sysml_file),
    );

    let tokens = SemanticTokenCollector::collect(&workspace_file);

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].line, 1); // First comes first in document order
    assert_eq!(tokens[1].line, 2);
}
