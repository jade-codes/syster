use super::*;
use crate::core::constants::{
    REL_REDEFINITION, REL_REFERENCE_SUBSETTING, REL_SPECIALIZATION, REL_SUBSETTING, REL_TYPING,
};
use crate::core::{Position, Span};
use crate::semantic::graphs::RelationshipGraph;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use crate::semantic::{NoOpValidator, RelationshipValidator};
use crate::syntax::sysml::ast::{
    Alias, Definition, DefinitionKind, Element, Package, Relationships, SysMLFile, Usage, UsageKind,
};

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
                semantic_role: None,
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
    graph.add_one_to_one(REL_TYPING, "myCar".to_string(), "Vehicle".to_string(), None);

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
    graph.add_one_to_many(
        REL_SPECIALIZATION,
        "Car".to_string(),
        "Vehicle".to_string(),
        None,
    );

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
    graph.add_one_to_one(REL_TYPING, "speed".to_string(), "Integer".to_string(), None);
    graph.add_one_to_one(REL_TYPING, "count".to_string(), "Integer".to_string(), None);
    graph.add_one_to_one(REL_TYPING, "index".to_string(), "Integer".to_string(), None);

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
        None,
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
        None,
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
                semantic_role: None,
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
                semantic_role: None,
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
        None,
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
                semantic_role: None,
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
    graph.add_one_to_one(REL_TYPING, "Source".to_string(), "Target".to_string(), None);

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
                semantic_role: None,
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
        None,
    );
    graph.add_one_to_one(REL_TYPING, "instance".to_string(), "Base".to_string(), None);

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
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;

    let pkg = Package {
        name: Some("TestPackage".to_string()),
        span: Some(Span::new(Position::new(0, 8), Position::new(0, 19))),
        elements: vec![],
    };
    let sysml_file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Package(pkg)],
    };
    let syntax_file = crate::syntax::SyntaxFile::SysML(sysml_file);

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    symbol_table.set_current_file(Some("test.sysml".to_string()));
    populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();

    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "test.sysml");

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].line, 0);
    assert_eq!(tokens[0].column, 8);
    assert_eq!(tokens[0].length, 11);
    assert_eq!(tokens[0].token_type, TokenType::Namespace);
}

#[test]
fn test_collect_definition_tokens() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;

    let mut def = Definition::new(
        DefinitionKind::Part,
        Some("MyDef".to_string()),
        Relationships::default(),
        vec![],
    );
    def.span = Some(Span::new(Position::new(1, 4), Position::new(1, 9)));

    let sysml_file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Definition(def)],
    };
    let syntax_file = crate::syntax::SyntaxFile::SysML(sysml_file);

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    symbol_table.set_current_file(Some("test.sysml".to_string()));
    populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();

    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "test.sysml");

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Type);
}

#[test]
fn test_collect_usage_tokens() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;

    let mut usage = Usage::new(
        UsageKind::Part,
        Some("myUsage".to_string()),
        Relationships::default(),
        vec![],
    );
    usage.span = Some(Span::new(Position::new(2, 4), Position::new(2, 11)));

    let sysml_file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Usage(usage)],
    };
    let syntax_file = crate::syntax::SyntaxFile::SysML(sysml_file);

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    symbol_table.set_current_file(Some("test.sysml".to_string()));
    populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();

    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "test.sysml");

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Property);
}

#[test]
fn test_collect_alias_tokens() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;

    let alias = Alias {
        name: Some("myAlias".to_string()),
        target: "SomeTarget".to_string(),
        span: Some(Span::new(Position::new(3, 6), Position::new(3, 13))),
    };
    let sysml_file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Alias(alias)],
    };
    let syntax_file = crate::syntax::SyntaxFile::SysML(sysml_file);

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    symbol_table.set_current_file(Some("test.sysml".to_string()));
    populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();

    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "test.sysml");

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, TokenType::Variable);
}

#[test]
fn test_collect_nested_tokens() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;

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
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Package(pkg)],
    };
    let syntax_file = crate::syntax::SyntaxFile::SysML(sysml_file);

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    symbol_table.set_current_file(Some("test.sysml".to_string()));
    populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();

    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "test.sysml");

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Namespace); // Outer
    assert_eq!(tokens[1].token_type, TokenType::Type); // Inner
}

#[test]
fn test_tokens_sorted_by_position() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;

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
        namespaces: vec![],
        namespace: None,
        // Elements should be in document order (AST order)
        elements: vec![Element::Definition(def1), Element::Definition(def2)],
    };
    let syntax_file = crate::syntax::SyntaxFile::SysML(sysml_file);

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    symbol_table.set_current_file(Some("test.sysml".to_string()));
    populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();

    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "test.sysml");

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].line, 1); // First comes first in document order
    assert_eq!(tokens[1].line, 2);
}

#[test]
fn test_semantic_tokens_with_absolute_path() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;
    use std::env;

    let mut def = Definition::new(
        DefinitionKind::Part,
        Some("TestDef".to_string()),
        Relationships::default(),
        vec![],
    );
    def.span = Some(Span::new(Position::new(1, 4), Position::new(1, 11)));

    let sysml_file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Definition(def)],
    };
    let syntax_file = crate::syntax::SyntaxFile::SysML(sysml_file);

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();

    // Use absolute path for the file
    let absolute_path = env::current_dir()
        .unwrap()
        .join("test_dir")
        .join("test.sysml");
    let absolute_path_str = absolute_path.to_string_lossy().to_string();

    symbol_table.set_current_file(Some(absolute_path_str.clone()));
    populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();

    // Query with absolute path
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, &absolute_path_str);
    assert_eq!(
        tokens.len(),
        1,
        "Should find token when querying with absolute path"
    );

    // Query with relative path - should also work after normalization
    let relative_path = "test_dir/test.sysml";
    let tokens_relative =
        SemanticTokenCollector::collect_from_symbols(&symbol_table, relative_path);
    assert_eq!(
        tokens_relative.len(),
        1,
        "Should find token when querying with relative path"
    );
}

#[test]
fn test_semantic_tokens_path_mismatch() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;

    let mut def = Definition::new(
        DefinitionKind::Part,
        Some("TestDef".to_string()),
        Relationships::default(),
        vec![],
    );
    def.span = Some(Span::new(Position::new(1, 4), Position::new(1, 11)));

    let sysml_file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Definition(def)],
    };
    let syntax_file = crate::syntax::SyntaxFile::SysML(sysml_file);

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();

    // Store with one file path
    symbol_table.set_current_file(Some("path/to/file.sysml".to_string()));
    populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();

    // Query with different file path - should return 0 tokens
    let tokens =
        SemanticTokenCollector::collect_from_symbols(&symbol_table, "different/file.sysml");
    assert_eq!(
        tokens.len(),
        0,
        "Should return no tokens when file path doesn't match"
    );
}

#[test]
fn test_semantic_tokens_no_source_file() {
    use crate::semantic::symbol_table::SymbolTable;

    let mut symbol_table = SymbolTable::new();

    // Insert a symbol without a source file
    let _ = symbol_table.insert(
        "NoSourceSymbol".to_string(),
        Symbol::Definition {
            name: "NoSourceSymbol".to_string(),
            qualified_name: "NoSourceSymbol".to_string(),
            kind: "part".to_string(),
            semantic_role: None,
            scope_id: 0,
            source_file: None, // No source file
            span: Some(Span::new(Position::new(0, 0), Position::new(0, 14))),
            references: vec![],
        },
    );

    // Should return 0 tokens since symbol has no source file
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "any_file.sysml");
    assert_eq!(
        tokens.len(),
        0,
        "Should return no tokens for symbols without source_file"
    );
}

#[test]
fn test_semantic_tokens_path_with_spaces() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;
    use std::env;

    let mut def = Definition::new(
        DefinitionKind::Part,
        Some("MyPart".to_string()),
        Relationships::default(),
        vec![],
    );
    def.span = Some(Span::new(Position::new(2, 5), Position::new(2, 11)));

    let sysml_file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Definition(def)],
    };
    let syntax_file = crate::syntax::SyntaxFile::SysML(sysml_file);

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();

    // Use path with spaces
    let path_with_spaces = "my folder/test file.sysml";
    let absolute_path = env::current_dir().unwrap().join(path_with_spaces);
    let absolute_path_str = absolute_path.to_string_lossy().to_string();

    symbol_table.set_current_file(Some(absolute_path_str.clone()));
    populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();

    // Query with the same path - should work
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, &absolute_path_str);
    assert_eq!(tokens.len(), 1, "Should find token when path has spaces");
    assert_eq!(tokens[0].line, 2);
    assert_eq!(tokens[0].column, 5);

    // Query with relative path with spaces - should also work
    let tokens_relative =
        SemanticTokenCollector::collect_from_symbols(&symbol_table, path_with_spaces);
    assert_eq!(
        tokens_relative.len(),
        1,
        "Should find token with relative path containing spaces"
    );

    // Simulate URL-encoded path (like VS Code sends)
    let _url_encoded_relative = "my%20folder/test%20file.sysml";
    // Note: Our normalize_path doesn't decode URLs, but the LSP layer should handle that
    // This test documents the expected behavior at this layer

    // Query with a different path that also has spaces - should not match
    let different_path = "other folder/test file.sysml";
    let tokens_different =
        SemanticTokenCollector::collect_from_symbols(&symbol_table, different_path);
    assert_eq!(
        tokens_different.len(),
        0,
        "Should not find tokens for different path even with spaces"
    );
}

#[test]
fn test_semantic_tokens_stdlib_different_locations() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;

    let pkg = Package {
        name: Some("AnalysisTooling".to_string()),
        span: Some(Span::new(Position::new(5, 8), Position::new(5, 24))),
        elements: vec![],
    };
    let sysml_file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Package(pkg)],
    };
    let syntax_file = crate::syntax::SyntaxFile::SysML(sysml_file);

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();

    // Simulate stdlib loaded from build directory
    let build_path = "/workspaces/syster/target/release/sysml.library/Domain Libraries/Analysis/AnalysisTooling.sysml";
    symbol_table.set_current_file(Some(build_path.to_string()));
    populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();

    // Query with source directory path (like VS Code opening the file)
    let source_path = "/workspaces/syster/crates/syster-base/sysml.library/Domain Libraries/Analysis/AnalysisTooling.sysml";
    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, source_path);

    assert_eq!(
        tokens.len(),
        1,
        "Should find token when stdlib file is loaded from build dir but queried from source dir"
    );
    assert_eq!(tokens[0].line, 5);
    assert_eq!(tokens[0].column, 8);

    // Also test the reverse - query with build path when stored as source path
    let mut symbol_table2 = SymbolTable::new();
    let mut relationship_graph2 = RelationshipGraph::new();
    symbol_table2.set_current_file(Some(source_path.to_string()));
    populate_syntax_file(&syntax_file, &mut symbol_table2, &mut relationship_graph2).ok();

    let tokens2 = SemanticTokenCollector::collect_from_symbols(&symbol_table2, build_path);
    assert_eq!(
        tokens2.len(),
        1,
        "Should find token when stdlib file is loaded from source dir but queried from build dir"
    );
}

#[test]
fn test_semantic_tokens_shows_what_symbols_have_spans() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;

    // Create a file with multiple types of symbols
    let mut inner_def = Definition::new(
        DefinitionKind::Part,
        Some("InnerPart".to_string()),
        Relationships::default(),
        vec![],
    );
    inner_def.span = Some(Span::new(Position::new(2, 4), Position::new(2, 13)));

    let mut usage = Usage::new(
        UsageKind::Part,
        Some("myUsage".to_string()),
        Relationships::default(),
        vec![],
    );
    usage.span = Some(Span::new(Position::new(3, 4), Position::new(3, 11)));

    let pkg = Package {
        name: Some("TestPackage".to_string()),
        span: Some(Span::new(Position::new(0, 8), Position::new(0, 19))),
        elements: vec![Element::Definition(inner_def), Element::Usage(usage)],
    };

    let sysml_file = SysMLFile {
        namespaces: vec![],
        namespace: None,
        elements: vec![Element::Package(pkg)],
    };
    let syntax_file = crate::syntax::SyntaxFile::SysML(sysml_file);

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    symbol_table.set_current_file(Some("test.sysml".to_string()));
    populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();

    // Check what symbols were created
    for (_name, symbol) in symbol_table.all_symbols() {
        if let Some(source_file) = symbol.source_file()
            && source_file == "test.sysml"
        {}
    }

    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "test.sysml");
    for _token in &tokens {}

    // We should get tokens for: TestPackage, InnerPart, myUsage
    assert_eq!(
        tokens.len(),
        3,
        "Should have 3 tokens (package, definition, usage)"
    );

    // Verify they're in the right order
    assert_eq!(tokens[0].line, 0); // TestPackage
    assert_eq!(tokens[0].token_type, TokenType::Namespace);

    assert_eq!(tokens[1].line, 2); // InnerPart
    assert_eq!(tokens[1].token_type, TokenType::Type);

    assert_eq!(tokens[2].line, 3); // myUsage
    assert_eq!(tokens[2].token_type, TokenType::Property);
}

#[test]
fn test_semantic_tokens_symbols_without_spans_are_skipped() {
    use crate::semantic::symbol_table::SymbolTable;

    let mut symbol_table = SymbolTable::new();

    // Insert symbols with and without spans
    let _ = symbol_table.insert(
        "WithSpan".to_string(),
        Symbol::Definition {
            name: "WithSpan".to_string(),
            qualified_name: "WithSpan".to_string(),
            kind: "part".to_string(),
            semantic_role: None,
            scope_id: 0,
            source_file: Some("test.sysml".to_string()),
            span: Some(Span::new(Position::new(1, 0), Position::new(1, 8))),
            references: vec![],
        },
    );

    let _ = symbol_table.insert(
        "NoSpan".to_string(),
        Symbol::Definition {
            name: "NoSpan".to_string(),
            qualified_name: "NoSpan".to_string(),
            kind: "part".to_string(),
            semantic_role: None,
            scope_id: 0,
            source_file: Some("test.sysml".to_string()),
            span: None, // No span!
            references: vec![],
        },
    );

    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "test.sysml");
    for _token in &tokens {}

    // Should only get 1 token (the one with a span)
    assert_eq!(
        tokens.len(),
        1,
        "Should only generate tokens for symbols with spans"
    );
    assert_eq!(tokens[0].line, 1);
}

#[test]
fn test_semantic_tokens_parse_real_stdlib_file() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;
    use crate::syntax::parser::parse_content;
    use std::path::Path;

    // Try to load an actual stdlib file
    let stdlib_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("sysml.library/Systems Library/Views.sysml");

    if !stdlib_path.exists() {
        return;
    }

    let content = std::fs::read_to_string(&stdlib_path).expect("Failed to read stdlib file");
    let syntax_file = parse_content(&content, &stdlib_path).expect("Failed to parse stdlib file");

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let file_path = stdlib_path.to_string_lossy().to_string();
    symbol_table.set_current_file(Some(file_path.clone()));

    let _result = populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph);

    // Check what symbols were created
    let mut symbol_count = 0;
    let mut _symbols_with_spans = 0;
    for (_name, symbol) in symbol_table.all_symbols() {
        if let Some(source_file) = symbol.source_file()
            && source_file.contains("Views.sysml")
        {
            symbol_count += 1;
            let has_span = symbol.span().is_some();
            if has_span {
                _symbols_with_spans += 1;
            }
            if symbol_count <= 5 {}
        }
    }

    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, &file_path);
    for (i, _token) in tokens.iter().enumerate() {
        if i < 10 {}
    }

    // The file should have at least some tokens
    assert!(
        !tokens.is_empty(),
        "Should generate at least some tokens from stdlib file"
    );
}

#[test]
fn test_semantic_tokens_what_are_we_missing() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;

    // Create a complex structure with nested elements
    let content = r#"
package TestPkg {
    part def Vehicle {
        part engine;
        attribute mass;
        port powerPort;
    }
    
    part myCar : Vehicle {
        part specificEngine;
    }
}
"#;

    let syntax_file =
        crate::syntax::parser::parse_content(content, std::path::Path::new("test.sysml"))
            .expect("Failed to parse");

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    symbol_table.set_current_file(Some("test.sysml".to_string()));
    populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();
    for (_name, symbol) in symbol_table.all_symbols() {
        if let Some(source_file) = symbol.source_file()
            && source_file == "test.sysml"
        {}
    }

    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, "test.sysml");
    for _token in &tokens {}

    if tokens.len() < 7 {}
}

#[test]
fn test_semantic_tokens_all_stdlib_files() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;
    use crate::syntax::parser::parse_content;
    use std::fs;
    use std::path::Path;

    let stdlib_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("sysml.library");

    if !stdlib_dir.exists() {
        return;
    }

    let mut total_files = 0;
    let mut files_with_zero_tokens = Vec::new();
    let mut files_with_one_token = Vec::new();
    let mut files_with_good_tokens = Vec::new();

    // Recursively collect all .sysml and .kerml files
    fn collect_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    collect_files(&path, files)?;
                } else if let Some(ext) = path.extension()
                    && (ext == "sysml" || ext == "kerml")
                {
                    files.push(path);
                }
            }
        }
        Ok(())
    }

    let mut all_files = Vec::new();
    collect_files(&stdlib_dir, &mut all_files).expect("Failed to read stdlib directory");

    // Just test first 20 files
    for path in all_files.iter().take(20) {
        total_files += 1;
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let syntax_file = match parse_content(&content, path) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let mut symbol_table = SymbolTable::new();
        let mut relationship_graph = RelationshipGraph::new();
        let file_path = path.to_string_lossy().to_string();
        symbol_table.set_current_file(Some(file_path.clone()));
        populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();

        let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, &file_path);

        let filename = path.file_name().unwrap().to_string_lossy();

        match tokens.len() {
            0 => {
                files_with_zero_tokens.push(filename.to_string());
            }
            1 => {
                files_with_one_token.push(filename.to_string());
            }
            n => {
                files_with_good_tokens.push((filename.to_string(), n));
            }
        }
    }

    if !files_with_zero_tokens.is_empty() {
        for _f in &files_with_zero_tokens {}
    }

    if !files_with_one_token.is_empty() {
        for _f in &files_with_one_token {}
    }

    // This is clearly broken if most files have 0 or 1 token
    let bad_files = files_with_zero_tokens.len() + files_with_one_token.len();
    if bad_files > total_files / 2 {
        panic!(
            "MORE THAN HALF THE FILES ({bad_files}/{total_files}) HAVE 0 OR 1 TOKENS - SOMETHING IS VERY WRONG!"
        );
    }
}

#[test]
fn test_allocation_definition_parsing() {
    use crate::semantic::adapters::syntax_factory::populate_syntax_file;
    use crate::semantic::graphs::RelationshipGraph;
    use crate::semantic::symbol_table::SymbolTable;
    use crate::syntax::parser::parse_content;
    use std::path::Path;

    let content = r#"standard library package Allocations {
	allocation def Allocation :> BinaryConnection {
		end source: Anything;
	}
}"#;

    let path = Path::new("test.sysml");
    let syntax_file = parse_content(content, path).expect("Should parse");
    let (_sysml_file, file_elements) = match &syntax_file {
        crate::syntax::SyntaxFile::SysML(file) => (file, &file.elements),
        crate::syntax::SyntaxFile::KerML(_file) => panic!("Expected SysML, got KerML"),
    };
    for (i, elem) in file_elements.iter().enumerate() {
        match elem {
            crate::syntax::sysml::ast::Element::Package(_pkg) => {}
            crate::syntax::sysml::ast::Element::Definition(_def) => {}
            crate::syntax::sysml::ast::Element::Usage(_usage) => {}
            _ => println!("Element {i}: {elem:?}"),
        }
    }

    let mut symbol_table = SymbolTable::new();
    let mut relationship_graph = RelationshipGraph::new();
    let file_path = path.to_string_lossy().to_string();
    symbol_table.set_current_file(Some(file_path.clone()));
    populate_syntax_file(&syntax_file, &mut symbol_table, &mut relationship_graph).ok();
    let all_symbols = symbol_table.all_symbols();
    for (_name, _symbol) in &all_symbols {}

    let tokens = SemanticTokenCollector::collect_from_symbols(&symbol_table, &file_path);
    for _token in tokens.iter() {}

    assert!(!file_elements.is_empty(), "Should have at least package");
    assert!(
        tokens.len() >= 2,
        "Should have at least 2 tokens (package + allocation def), got {}",
        tokens.len()
    );
}
