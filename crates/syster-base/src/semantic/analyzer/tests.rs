#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::*;
use crate::semantic::RelationshipGraph;
use crate::semantic::SemanticErrorKind;
use crate::semantic::create_validator;
use crate::semantic::symbol_table::{Symbol, SymbolTable};
use crate::semantic::types::{SemanticError, SemanticResult, SemanticRole};

#[test]
fn test_analyzer_creation() {
    let analyzer = SemanticAnalyzer::new();
    assert_eq!(analyzer.symbol_table().current_scope_id(), 0);
}

#[test]
fn test_analyze_empty_table() {
    let analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_analyze_with_valid_symbols() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "MyPackage".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyPackage".to_string(),
                qualified_name: "MyPackage".to_string(),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_context_error_handling() {
    let table = SymbolTable::new();
    let graph = RelationshipGraph::new();
    let mut context = AnalysisContext::new(&table, &graph);

    assert!(!context.has_errors());

    context.add_error(SemanticError::undefined_reference("Test".to_string()));

    assert!(context.has_errors());
    assert_eq!(context.errors.len(), 1);
}

#[test]
fn test_context_into_result_success() {
    let table = SymbolTable::new();
    let graph = RelationshipGraph::new();
    let context = AnalysisContext::new(&table, &graph);

    let result: SemanticResult<i32> = context.into_result(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_context_into_result_error() {
    let table = SymbolTable::new();
    let graph = RelationshipGraph::new();
    let mut context = AnalysisContext::new(&table, &graph);

    context.add_error(SemanticError::undefined_reference("Test".to_string()));

    let result: SemanticResult<i32> = context.into_result(42);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().len(), 1);
}

#[test]
fn test_context_multiple_errors() {
    let table = SymbolTable::new();
    let graph = RelationshipGraph::new();
    let mut context = AnalysisContext::new(&table, &graph);

    context.add_error(SemanticError::undefined_reference("Test1".to_string()));
    context.add_error(SemanticError::undefined_reference("Test2".to_string()));
    context.add_error(SemanticError::invalid_type("BadType".to_string()));

    assert!(context.has_errors());
    assert_eq!(context.errors.len(), 3);

    let result: SemanticResult<()> = context.into_result(());
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 3);
}

#[test]
fn test_analyzer_with_multiple_symbols() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Pkg".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Pkg".to_string(),
                qualified_name: "Pkg".to_string(),
            },
        )
        .unwrap();

    table
        .insert(
            "MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClass".to_string(),
                qualified_name: "MyClass".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    table
        .insert(
            "MyDef".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyDef".to_string(),
                qualified_name: "MyDef".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_analyzer_with_nested_scopes() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Root".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Root".to_string(),
                qualified_name: "Root".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Child".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Child".to_string(),
                qualified_name: "Root::Child".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "GrandChild".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "GrandChild".to_string(),
                qualified_name: "Root::Child::GrandChild".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_context_resolver_access() {
    let mut table = SymbolTable::new();
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

    let graph = RelationshipGraph::new();
    let context = AnalysisContext::new(&table, &graph);
    let resolved = context.resolver.resolve("Test");
    assert!(resolved.is_some());

    let Some(Symbol::Package { name, .. }) = resolved else {
        panic!("Expected Package symbol, got: {resolved:?}");
    };
    assert_eq!(name, "Test");
}

#[test]
fn test_analyzer_default() {
    let analyzer = SemanticAnalyzer::default();
    assert_eq!(analyzer.symbol_table().current_scope_id(), 0);
}

#[test]
fn test_analyzer_symbol_table_access() {
    let mut analyzer = SemanticAnalyzer::new();

    analyzer
        .symbol_table_mut()
        .insert(
            "NewSymbol".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "NewSymbol".to_string(),
                qualified_name: "NewSymbol".to_string(),
            },
        )
        .unwrap();

    assert!(analyzer.symbol_table().lookup("NewSymbol").is_some());
}

#[test]
fn test_analyze_with_all_symbol_types() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Pkg".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Pkg".to_string(),
                qualified_name: "Pkg".to_string(),
            },
        )
        .unwrap();

    table
        .insert(
            "MyClass".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClass".to_string(),
                qualified_name: "MyClass".to_string(),
                kind: "Behavior".to_string(),
                is_abstract: true,
            },
        )
        .unwrap();

    // Define Integer type for feature to reference
    table
        .insert(
            "Integer".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Integer".to_string(),
                qualified_name: "Integer".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table
        .insert(
            "myFeature".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "myFeature".to_string(),
                qualified_name: "MyClass::myFeature".to_string(),
                feature_type: Some("Integer".to_string()),
            },
        )
        .unwrap();

    table
        .insert(
            "MyDef".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyDef".to_string(),
                qualified_name: "MyDef".to_string(),
                kind: "Requirement".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table
        .insert(
            "MyUsage".to_string(),
            Symbol::Usage {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyUsage".to_string(),
                qualified_name: "MyUsage".to_string(),
                kind: "Action".to_string(),
                semantic_role: None,
                usage_type: None,
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_context_into_result_with_unit() {
    let table = SymbolTable::new();
    let graph = RelationshipGraph::new();
    let context = AnalysisContext::new(&table, &graph);

    let result: SemanticResult<()> = context.into_result(());
    assert!(result.is_ok());
}

#[test]
fn test_context_error_accumulation() {
    let table = SymbolTable::new();
    let graph = RelationshipGraph::new();
    let mut context = AnalysisContext::new(&table, &graph);

    for i in 0..10 {
        context.add_error(SemanticError::undefined_reference(format!("Symbol{i}")));
    }

    assert_eq!(context.errors.len(), 10);
    let result: SemanticResult<()> = context.into_result(());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().len(), 10);
}

#[test]
fn test_analyzer_immutable_access() {
    let mut table = SymbolTable::new();
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

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let sym_table = analyzer.symbol_table();
    assert!(sym_table.lookup("Test").is_some());
}

#[test]
fn test_analyze_idempotent() {
    let mut table = SymbolTable::new();
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

    let analyzer = SemanticAnalyzer::with_symbol_table(table);

    let result1 = analyzer.analyze();
    let result2 = analyzer.analyze();

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[test]
fn test_context_no_errors_initially() {
    let table = SymbolTable::new();
    let graph = RelationshipGraph::new();
    let context = AnalysisContext::new(&table, &graph);

    assert!(!context.has_errors());
    assert_eq!(context.errors.len(), 0);
}

#[test]
fn test_analyzer_with_features() {
    let mut table = SymbolTable::new();

    // Define a type that can be referenced
    table
        .insert(
            "String".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "String".to_string(),
                qualified_name: "String".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table
        .insert(
            "feature1".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "feature1".to_string(),
                qualified_name: "feature1".to_string(),
                feature_type: None,
            },
        )
        .unwrap();

    table
        .insert(
            "feature2".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "feature2".to_string(),
                qualified_name: "feature2".to_string(),
                feature_type: Some("String".to_string()),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_analyzer_with_different_classifier_kinds() {
    let mut table = SymbolTable::new();

    for (idx, kind) in [
        "Type",
        "Class",
        "DataType",
        "Structure",
        "Association",
        "Behavior",
        "Function",
    ]
    .iter()
    .enumerate()
    {
        table
            .insert(
                format!("Classifier{idx}"),
                Symbol::Classifier {
                    scope_id: 0,
                    source_file: None,
                    span: None,
                    references: Vec::new(),
                    name: format!("Classifier{idx}"),
                    qualified_name: format!("Classifier{idx}"),
                    kind: (*kind).to_string(),
                    is_abstract: false,
                },
            )
            .unwrap();
    }

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_analyzer_with_different_definition_kinds() {
    let mut table = SymbolTable::new();

    for (idx, kind) in ["Part", "Port", "Action", "State", "Requirement", "Item"]
        .iter()
        .enumerate()
    {
        table
            .insert(
                format!("Def{idx}"),
                Symbol::Definition {
                    scope_id: 0,
                    source_file: None,
                    span: None,
                    references: Vec::new(),
                    name: format!("Def{idx}"),
                    qualified_name: format!("Def{idx}"),
                    kind: (*kind).to_string(),
                    semantic_role: None,
                },
            )
            .unwrap();
    }

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_analyzer_with_different_usage_kinds() {
    let mut table = SymbolTable::new();

    for (idx, kind) in ["Part", "Port", "Action", "State", "Requirement"]
        .iter()
        .enumerate()
    {
        table
            .insert(
                format!("Usage{idx}"),
                Symbol::Usage {
                    scope_id: 0,
                    source_file: None,
                    span: None,
                    references: Vec::new(),
                    name: format!("Usage{idx}"),
                    qualified_name: format!("Usage{idx}"),
                    kind: (*kind).to_string(),
                    usage_type: None,
                    semantic_role: None,
                },
            )
            .unwrap();
    }

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_context_with_qualified_name_resolution() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "A".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "A".to_string(),
                qualified_name: "A".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "B".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "B".to_string(),
                qualified_name: "A::B".to_string(),
            },
        )
        .unwrap();

    let graph = RelationshipGraph::new();
    let context = AnalysisContext::new(&table, &graph);
    let resolved = context.resolver.resolve("A::B");
    assert!(resolved.is_some());
}

#[test]
fn test_analyzer_with_abstract_classifiers() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Abstract1".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Abstract1".to_string(),
                qualified_name: "Abstract1".to_string(),
                kind: "Class".to_string(),
                is_abstract: true,
            },
        )
        .unwrap();

    table
        .insert(
            "Concrete1".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Concrete1".to_string(),
                qualified_name: "Concrete1".to_string(),
                kind: "Class".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_context_into_result_preserves_value() {
    let table = SymbolTable::new();
    let graph = RelationshipGraph::new();
    let context = AnalysisContext::new(&table, &graph);

    let test_value = vec![1, 2, 3, 4, 5];
    let result: SemanticResult<Vec<i32>> = context.into_result(test_value.clone());

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), test_value);
}

#[test]
fn test_context_symbol_table_reference() {
    let mut table = SymbolTable::new();
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

    let graph = RelationshipGraph::new();
    let context = AnalysisContext::new(&table, &graph);
    let lookup_result = context.symbol_table.lookup("Test");
    assert!(lookup_result.is_some());
}

#[test]
fn test_analyzer_with_deeply_nested_structure() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "L0".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "L0".to_string(),
                qualified_name: "L0".to_string(),
            },
        )
        .unwrap();

    for i in 1..=5 {
        table.enter_scope();
        let parent = if i == 1 {
            "L0".to_string()
        } else {
            format!(
                "L0::{}",
                (1..i)
                    .map(|j| format!("L{j}"))
                    .collect::<Vec<_>>()
                    .join("::")
            )
        };
        table
            .insert(
                format!("L{i}"),
                Symbol::Package {
                    scope_id: 0,
                    source_file: None,
                    span: None,
                    references: Vec::new(),
                    name: format!("L{i}"),
                    qualified_name: format!("{parent}::L{i}"),
                },
            )
            .unwrap();
    }

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_context_error_types_variety() {
    let table = SymbolTable::new();
    let graph = RelationshipGraph::new();
    let mut context = AnalysisContext::new(&table, &graph);

    context.add_error(SemanticError::undefined_reference("Ref1".to_string()));
    context.add_error(SemanticError::invalid_type("Type1".to_string()));
    context.add_error(SemanticError::duplicate_definition(
        "Dup1".to_string(),
        None,
    ));
    context.add_error(SemanticError::type_mismatch(
        "Int".to_string(),
        "Str".to_string(),
        "test".to_string(),
    ));
    context.add_error(SemanticError::circular_dependency(vec![
        "A".to_string(),
        "B".to_string(),
    ]));

    assert_eq!(context.errors.len(), 5);
}

#[test]
fn test_analyzer_table_mutation_after_creation() {
    let mut analyzer = SemanticAnalyzer::new();

    let table_mut = analyzer.symbol_table_mut();
    table_mut
        .insert(
            "Added".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Added".to_string(),
                qualified_name: "Added".to_string(),
            },
        )
        .unwrap();

    let result = analyzer.analyze();
    assert!(result.is_ok());
}

// Type validation tests

#[test]
fn test_type_validation_valid_type_reference() {
    let mut table = SymbolTable::new();

    // Define a type
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

    // Feature referencing the type
    table
        .insert(
            "myVehicle".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "myVehicle".to_string(),
                qualified_name: "myVehicle".to_string(),
                feature_type: Some("Vehicle".to_string()),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_type_validation_undefined_type_reference() {
    let mut table = SymbolTable::new();

    // Feature referencing undefined type
    table
        .insert(
            "myFeature".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "myFeature".to_string(),
                qualified_name: "myFeature".to_string(),
                feature_type: Some("UndefinedType".to_string()),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(errors[0].kind, SemanticErrorKind::UndefinedReference { .. }),
        "Expected UndefinedReference error kind"
    );
}

#[test]
fn test_type_validation_invalid_type_reference() {
    let mut table = SymbolTable::new();

    // Define a non-type symbol (Package)
    table
        .insert(
            "NotAType".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "NotAType".to_string(),
                qualified_name: "NotAType".to_string(),
            },
        )
        .unwrap();

    // Feature referencing a package (not a type)
    table
        .insert(
            "myFeature".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "myFeature".to_string(),
                qualified_name: "myFeature".to_string(),
                feature_type: Some("NotAType".to_string()),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(errors[0].kind, SemanticErrorKind::InvalidType { .. }),
        "Expected InvalidType error kind"
    );
}

#[test]
fn test_type_validation_classifier_as_type() {
    let mut table = SymbolTable::new();

    // Define a classifier
    table
        .insert(
            "MyClassifier".to_string(),
            Symbol::Classifier {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyClassifier".to_string(),
                qualified_name: "MyClassifier".to_string(),
                kind: "Type".to_string(),
                is_abstract: false,
            },
        )
        .unwrap();

    // Feature referencing the classifier
    table
        .insert(
            "myFeature".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "myFeature".to_string(),
                qualified_name: "myFeature".to_string(),
                feature_type: Some("MyClassifier".to_string()),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok(), "Classifiers should be valid types");
}

#[test]
fn test_type_validation_multiple_features() {
    let mut table = SymbolTable::new();

    // Define types
    table
        .insert(
            "Type1".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Type1".to_string(),
                qualified_name: "Type1".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table
        .insert(
            "Type2".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Type2".to_string(),
                qualified_name: "Type2".to_string(),
                kind: "Port".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Features with valid types
    table
        .insert(
            "feature1".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "feature1".to_string(),
                qualified_name: "feature1".to_string(),
                feature_type: Some("Type1".to_string()),
            },
        )
        .unwrap();

    table
        .insert(
            "feature2".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "feature2".to_string(),
                qualified_name: "feature2".to_string(),
                feature_type: Some("Type2".to_string()),
            },
        )
        .unwrap();

    // Feature with no type
    table
        .insert(
            "feature3".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "feature3".to_string(),
                qualified_name: "feature3".to_string(),
                feature_type: None,
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_type_validation_qualified_type_reference() {
    let mut table = SymbolTable::new();

    // Define a type
    table
        .insert(
            "SubType".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SubType".to_string(),
                qualified_name: "Package::SubType".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Feature referencing with simple name (resolver handles lookup)
    table
        .insert(
            "myFeature".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "myFeature".to_string(),
                qualified_name: "myFeature".to_string(),
                feature_type: Some("SubType".to_string()),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_type_validation_multiple_errors() {
    let mut table = SymbolTable::new();

    // Multiple features with invalid type references
    table
        .insert(
            "feature1".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "feature1".to_string(),
                qualified_name: "feature1".to_string(),
                feature_type: Some("Undefined1".to_string()),
            },
        )
        .unwrap();

    table
        .insert(
            "feature2".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "feature2".to_string(),
                qualified_name: "feature2".to_string(),
                feature_type: Some("Undefined2".to_string()),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2, "Should have 2 undefined type errors");
}

#[test]
fn test_type_validation_mixed_errors() {
    let mut table = SymbolTable::new();

    // Define a package (not a type)
    table
        .insert(
            "SomePackage".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SomePackage".to_string(),
                qualified_name: "SomePackage".to_string(),
            },
        )
        .unwrap();

    // Feature referencing undefined type
    table
        .insert(
            "feature1".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "feature1".to_string(),
                qualified_name: "feature1".to_string(),
                feature_type: Some("UndefinedType".to_string()),
            },
        )
        .unwrap();

    // Feature referencing non-type symbol
    table
        .insert(
            "feature2".to_string(),
            Symbol::Feature {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "feature2".to_string(),
                qualified_name: "feature2".to_string(),
                feature_type: Some("SomePackage".to_string()),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(
        errors.len(),
        2,
        "Should have 1 undefined + 1 invalid type error"
    );
}

#[test]
fn test_type_validation_scope_aware_resolution() {
    // This tests that type resolution happens from the scope where the symbol is defined,
    // not from the root scope. Pkg1::feature should resolve "Vehicle" to Pkg1::Vehicle,
    // not Pkg2::Vehicle, even though both exist.

    let mut table = SymbolTable::new();

    // Enter Pkg1 scope
    let pkg1_scope = table.enter_scope();

    // Define Vehicle in Pkg1
    table
        .insert(
            "Vehicle".to_string(),
            Symbol::Definition {
                name: "Vehicle".to_string(),
                qualified_name: "Pkg1::Vehicle".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
                scope_id: pkg1_scope,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Define feature in Pkg1 referencing Vehicle
    table
        .insert(
            "myFeature".to_string(),
            Symbol::Feature {
                name: "myFeature".to_string(),
                qualified_name: "Pkg1::myFeature".to_string(),
                feature_type: Some("Vehicle".to_string()),
                scope_id: pkg1_scope,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Exit Pkg1
    table.exit_scope();

    // Enter Pkg2 scope
    let pkg2_scope = table.enter_scope();

    // Define a different Vehicle in Pkg2
    table
        .insert(
            "Vehicle".to_string(),
            Symbol::Definition {
                name: "Vehicle".to_string(),
                qualified_name: "Pkg2::Vehicle".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
                scope_id: pkg2_scope,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Exit Pkg2
    table.exit_scope();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();

    // Should succeed because Pkg1::myFeature resolves Vehicle from Pkg1 scope,
    // finding Pkg1::Vehicle (not Pkg2::Vehicle)
    assert!(
        result.is_ok(),
        "Type resolution should be scope-aware: Pkg1::myFeature should find Pkg1::Vehicle"
    );
}

#[test]
fn test_type_validation_scope_aware_undefined() {
    // Test that a feature in Pkg1 referencing a type that only exists in Pkg2 fails

    let mut table = SymbolTable::new();

    // Enter Pkg1 scope
    let pkg1_scope = table.enter_scope();

    // Define feature in Pkg1 referencing Vehicle (which doesn't exist in this scope)
    table
        .insert(
            "myFeature".to_string(),
            Symbol::Feature {
                name: "myFeature".to_string(),
                qualified_name: "Pkg1::myFeature".to_string(),
                feature_type: Some("Vehicle".to_string()),
                scope_id: pkg1_scope,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Exit Pkg1
    table.exit_scope();

    // Enter Pkg2 scope
    let pkg2_scope = table.enter_scope();

    // Define Vehicle in Pkg2 (not visible to Pkg1)
    table
        .insert(
            "Vehicle".to_string(),
            Symbol::Definition {
                name: "Vehicle".to_string(),
                qualified_name: "Pkg2::Vehicle".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
                scope_id: pkg2_scope,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Exit Pkg2
    table.exit_scope();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();

    // Should fail because Pkg1::myFeature can't see Pkg2::Vehicle
    assert!(
        result.is_err(),
        "Pkg1::myFeature should not be able to resolve Vehicle from Pkg2"
    );

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(errors[0].kind, SemanticErrorKind::UndefinedReference { .. }),
        "Expected UndefinedReference error kind"
    );
}

#[test]
fn test_type_validation_feature_without_type() {
    // Features without type references should pass validation
    let mut table = SymbolTable::new();

    table
        .insert(
            "untyped".to_string(),
            Symbol::Feature {
                name: "untyped".to_string(),
                qualified_name: "untyped".to_string(),
                feature_type: None,
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();

    assert!(
        result.is_ok(),
        "Features without type references should pass validation"
    );
}

#[test]
fn test_type_validation_usage_not_valid_type() {
    // Usages should not be valid types for features to reference
    let mut table = SymbolTable::new();

    // Define a Usage
    table
        .insert(
            "myUsage".to_string(),
            Symbol::Usage {
                name: "myUsage".to_string(),
                qualified_name: "myUsage".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: None,
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Feature trying to reference the Usage as a type
    table
        .insert(
            "myFeature".to_string(),
            Symbol::Feature {
                name: "myFeature".to_string(),
                qualified_name: "myFeature".to_string(),
                feature_type: Some("myUsage".to_string()),
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();

    assert!(result.is_err(), "Usage should not be a valid type");

    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(
        matches!(errors[0].kind, SemanticErrorKind::InvalidType { .. }),
        "Expected InvalidType error kind"
    );
}

#[test]
fn test_type_validation_shadowing() {
    // Test that nested scope can shadow parent's type and resolution works correctly
    let mut table = SymbolTable::new();

    // Define Vehicle in root scope
    table
        .insert(
            "Vehicle".to_string(),
            Symbol::Definition {
                name: "Vehicle".to_string(),
                qualified_name: "Vehicle".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Enter nested scope
    let nested_scope = table.enter_scope();

    // Shadow Vehicle with a different definition
    table
        .insert(
            "Vehicle".to_string(),
            Symbol::Definition {
                name: "Vehicle".to_string(),
                qualified_name: "Nested::Vehicle".to_string(),
                kind: "Port".to_string(),
                semantic_role: None,
                scope_id: nested_scope,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Feature in nested scope referencing Vehicle (should find shadowed one)
    table
        .insert(
            "myFeature".to_string(),
            Symbol::Feature {
                name: "myFeature".to_string(),
                qualified_name: "Nested::myFeature".to_string(),
                feature_type: Some("Vehicle".to_string()),
                scope_id: nested_scope,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    // Exit nested scope
    table.exit_scope();

    // Feature in root scope referencing Vehicle (should find root one)
    table
        .insert(
            "rootFeature".to_string(),
            Symbol::Feature {
                name: "rootFeature".to_string(),
                qualified_name: "rootFeature".to_string(),
                feature_type: Some("Vehicle".to_string()),
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();

    assert!(
        result.is_ok(),
        "Shadowing should work correctly - each scope finds its appropriate Vehicle"
    );
}

#[test]
fn test_relationship_validation_undefined_target() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add a definition
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

    // Add a specialization relationship to a non-existent target
    graph.add_one_to_many(
        "specialization",
        "Vehicle".to_string(),
        "NonExistent".to_string(),
        None,
    );

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let result = analyzer.analyze();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(!errors.is_empty());
    assert!(
        errors.iter().any(|e| matches!(&e.kind, SemanticErrorKind::UndefinedReference { name } if name.contains("NonExistent"))),
        "Expected undefined reference error, got: {errors:?}"
    );
}

#[test]
fn test_relationship_validation_typing_undefined_target() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add a usage
    table
        .insert(
            "myPart".to_string(),
            Symbol::Usage {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "myPart".to_string(),
                qualified_name: "myPart".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
                usage_type: None,
            },
        )
        .unwrap();

    // Add a typing relationship to a non-existent definition
    graph.add_one_to_one(
        "typing",
        "myPart".to_string(),
        "UndefinedDef".to_string(),
        None,
    );

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let result = analyzer.analyze();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(!errors.is_empty());
    assert!(
        errors.iter().any(|e| matches!(&e.kind, SemanticErrorKind::UndefinedReference { name } if name.contains("UndefinedDef"))),
        "Expected undefined reference error for typing target"
    );
}

#[test]
fn test_circular_specialization_detection() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Create three definitions in a circular specialization chain
    table
        .insert(
            "A".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "A".to_string(),
                qualified_name: "A".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table
        .insert(
            "B".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "B".to_string(),
                qualified_name: "B".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table
        .insert(
            "C".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "C".to_string(),
                qualified_name: "C".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Create circular specialization: A -> B -> C -> A
    graph.add_one_to_many("specialization", "A".to_string(), "B".to_string(), None);
    graph.add_one_to_many("specialization", "B".to_string(), "C".to_string(), None);
    graph.add_one_to_many("specialization", "C".to_string(), "A".to_string(), None);

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let result = analyzer.analyze();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    // Should detect at least one circular dependency (could be 3, one for each element in the cycle)
    assert!(!errors.is_empty());
    assert!(
        errors
            .iter()
            .any(|e| matches!(&e.kind, SemanticErrorKind::CircularDependency { .. })),
        "Expected circular dependency error"
    );
}

#[test]
fn test_self_specialization_detection() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Create a definition that specializes itself
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

    // Vehicle specializes itself
    graph.add_one_to_many(
        "specialization",
        "Vehicle".to_string(),
        "Vehicle".to_string(),
        None,
    );

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let result = analyzer.analyze();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(!errors.is_empty());
    assert!(
        errors
            .iter()
            .any(|e| matches!(&e.kind, SemanticErrorKind::CircularDependency { .. })),
        "Expected circular dependency error for self-specialization"
    );
}

#[test]
fn test_valid_specialization_chain() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Create a valid specialization chain: Vehicle -> Car -> SportsCar
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

    table
        .insert(
            "SportsCar".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SportsCar".to_string(),
                qualified_name: "SportsCar".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Create linear specialization chain (no cycles)
    graph.add_one_to_many(
        "specialization",
        "Car".to_string(),
        "Vehicle".to_string(),
        None,
    );
    graph.add_one_to_many(
        "specialization",
        "SportsCar".to_string(),
        "Car".to_string(),
        None,
    );

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let result = analyzer.analyze();

    assert!(
        result.is_ok(),
        "Valid specialization chain should not produce errors"
    );
}

#[test]
fn test_multiple_relationship_types_validation() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add definitions and usages
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

    // Valid typing relationship
    graph.add_one_to_one(
        "typing",
        "myVehicle".to_string(),
        "VehicleDef".to_string(),
        None,
    );

    // Invalid subsetting relationship (target doesn't exist)
    graph.add_one_to_many(
        "subsetting",
        "myVehicle".to_string(),
        "NonExistent".to_string(),
        None,
    );

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let result = analyzer.analyze();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(!errors.is_empty());
    assert!(
        errors.iter().any(|e| e.message.contains("NonExistent")),
        "Error should mention the undefined target"
    );
}

#[test]
fn test_relationship_validation_with_qualified_names() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add a package
    table
        .insert(
            "Vehicles".to_string(),
            Symbol::Package {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Vehicles".to_string(),
                qualified_name: "Vehicles".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();

    // Add a definition in the package - use qualified name as key for lookup
    table
        .insert(
            "Vehicles::Car".to_string(),
            Symbol::Definition {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Car".to_string(),
                qualified_name: "Vehicles::Car".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table
        .insert(
            "Vehicles::SportsCar".to_string(),
            Symbol::Definition {
                scope_id: 1,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SportsCar".to_string(),
                qualified_name: "Vehicles::SportsCar".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Valid relationship using qualified names
    graph.add_one_to_many(
        "specialization",
        "Vehicles::SportsCar".to_string(),
        "Vehicles::Car".to_string(),
        None,
    );

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let result = analyzer.analyze();

    assert!(
        result.is_ok(),
        "Qualified name relationships should be validated correctly"
    );
}

#[test]
fn test_satisfy_relationship_validation_valid() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add a case
    table
        .insert(
            "SafetyCase".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SafetyCase".to_string(),
                qualified_name: "SafetyCase".to_string(),
                kind: "Case".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Add a requirement
    table
        .insert(
            "SafetyReq".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "SafetyReq".to_string(),
                qualified_name: "SafetyReq".to_string(),
                kind: "Requirement".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Valid satisfy relationship
    graph.add_one_to_many(
        "satisfy",
        "SafetyCase".to_string(),
        "SafetyReq".to_string(),
        None,
    );

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let result = analyzer.analyze();

    assert!(
        result.is_ok(),
        "Valid satisfy relationship should pass validation"
    );
}

#[test]
fn test_satisfy_relationship_validation_invalid_target() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add a case
    table
        .insert(
            "MyCase".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "MyCase".to_string(),
                qualified_name: "MyCase".to_string(),
                kind: "Case".to_string(),
                semantic_role: Some(SemanticRole::AnalysisCase),
            },
        )
        .unwrap();

    // Add a part (not a requirement)
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
                semantic_role: Some(SemanticRole::Component),
            },
        )
        .unwrap();

    // Invalid satisfy relationship - targeting a part instead of requirement
    graph.add_one_to_many("satisfy", "MyCase".to_string(), "Vehicle".to_string(), None);

    let validator = create_validator("sysml");
    let analyzer = SemanticAnalyzer::with_validator(table, graph, validator);
    let result = analyzer.analyze();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(
        errors
            .iter()
            .any(|e| matches!(&e.kind, SemanticErrorKind::InvalidType { .. })),
        "Should detect invalid satisfy target type"
    );
}

#[test]
fn test_perform_relationship_validation() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add a part
    table
        .insert(
            "Robot".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Robot".to_string(),
                qualified_name: "Robot".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Add an action
    table
        .insert(
            "Move".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Move".to_string(),
                qualified_name: "Move".to_string(),
                kind: "Action".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Valid perform relationship
    graph.add_one_to_many("perform", "Robot".to_string(), "Move".to_string(), None);

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let result = analyzer.analyze();

    assert!(
        result.is_ok(),
        "Valid perform relationship should pass validation"
    );
}

#[test]
fn test_perform_relationship_invalid_target() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add a part
    table
        .insert(
            "Robot".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Robot".to_string(),
                qualified_name: "Robot".to_string(),
                kind: "Part".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Add another part (not an action)
    table
        .insert(
            "Tool".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Tool".to_string(),
                qualified_name: "Tool".to_string(),
                kind: "Part".to_string(),
                semantic_role: Some(SemanticRole::Component),
            },
        )
        .unwrap();

    // Invalid perform relationship - targeting a part instead of action
    graph.add_one_to_many("perform", "Robot".to_string(), "Tool".to_string(), None);

    let validator = create_validator("sysml");
    let analyzer = SemanticAnalyzer::with_validator(table, graph, validator);
    let result = analyzer.analyze();

    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(
        errors
            .iter()
            .any(|e| matches!(&e.kind, SemanticErrorKind::InvalidType { .. })),
        "Should detect invalid perform target type"
    );
}

#[test]
fn test_exhibit_relationship_validation() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add a part
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

    // Add a state
    table
        .insert(
            "Moving".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Moving".to_string(),
                qualified_name: "Moving".to_string(),
                kind: "State".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Valid exhibit relationship
    graph.add_one_to_many("exhibit", "Vehicle".to_string(), "Moving".to_string(), None);

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let result = analyzer.analyze();

    assert!(
        result.is_ok(),
        "Valid exhibit relationship should pass validation"
    );
}

#[test]
fn test_include_relationship_validation() {
    let mut table = SymbolTable::new();
    let mut graph = RelationshipGraph::new();

    // Add use cases
    table
        .insert(
            "ManageAccount".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "ManageAccount".to_string(),
                qualified_name: "ManageAccount".to_string(),
                kind: "UseCase".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    table
        .insert(
            "Login".to_string(),
            Symbol::Definition {
                scope_id: 0,
                source_file: None,
                span: None,
                references: Vec::new(),
                name: "Login".to_string(),
                qualified_name: "Login".to_string(),
                kind: "UseCase".to_string(),
                semantic_role: None,
            },
        )
        .unwrap();

    // Valid include relationship
    graph.add_one_to_many(
        "include",
        "ManageAccount".to_string(),
        "Login".to_string(),
        None,
    );

    let analyzer = SemanticAnalyzer::with_symbol_table_and_relationships(table, graph);
    let result = analyzer.analyze();

    assert!(
        result.is_ok(),
        "Valid include relationship should pass validation"
    );
}
